use std::sync::Mutex;

use crate::errors::ErrorResponse;
use dotenv::dotenv;
use errors::Error;
use kafka::{consumer::Consumer, producer::Record};
use non_system_commands::NonSystemCommands;
use serde::{Serialize, Deserialize};

mod common;
mod errors;
mod heartbeat;
mod kafka_connectivity;
mod non_system_commands;
mod system_commands;
mod systeminfo;


#[derive(Debug, Serialize, Deserialize)]
struct Envelope {
    uuid: String,
    message: String,
    timestamp: i64,
}

impl Envelope {
    fn copy_with_new_message(self, new_message: String) -> Self {
        Envelope {
            message: new_message,
            ..self
        }
    }
}

fn main() {
    dotenv().ok();

    let group = common::get_env_var(common::GROUP_VAR);
    let broker = common::get_env_var(common::BROKER_ENV_VAR);
    let consume_topic = common::get_env_var(common::CONSUME_TOPIC_ENV_VAR);
    let response_topic: String = common::get_env_var(common::PUBLISH_TOPIC_ENV_VAR);
    let error_topic: String = common::get_env_var(common::ERROR_TOPIC);
    let registration_topic: String = common::get_env_var(common::REGISTRATION_TOPIC);
    let heartbeat_topic: String = common::get_env_var(common::HEARTBEAT_TOPIC);
    let brokers = vec![broker];

    register_agent(registration_topic.as_str(), &brokers);
    heartbeat::start_heartbeat(heartbeat_topic.as_str(), &brokers);

    start_listening(
        group.as_str(),
        consume_topic.as_str(),
        error_topic.as_str(),
        response_topic.as_str(),
        &brokers,
    );
}

fn register_agent(registration_topic: &str, brokers: &[String]) {
    send_response(common::CLIENT_ID.to_owned().as_str(), registration_topic, brokers);
}

fn start_listening(
    group: &str,
    topic: &str,
    error_topic: &str,
    response_topic: &str,
    brokers: &[String],
) {
    let duration = common::TEN_SECS;

    println!("My client Id is {}", *common::CLIENT_ID);

    let mut con_opt: Mutex<Option<Consumer>> = Mutex::new(None);

    loop {
        con_opt = get_consumer_with_retry(con_opt, brokers, topic, group, duration);
        
        let mut val = con_opt.lock().unwrap();

        let mut con = val.as_mut().unwrap();

        let Ok(mss) = &mut con.poll() else {
            println!("Could not poll for messages. Retry in {:?}", duration);

            common::sleep(duration);
            continue;
        };

        if mss.is_empty() {
            common::sleep(duration);
            continue;
        }

        for ms in mss.iter() {
            for m in ms.messages() {
                handle_message(m, response_topic, brokers, error_topic);
            }
            consume_messageset(&mut con, ms);
        }
        let Ok(_) = con.commit_consumed() else {
            println!("Could not commit messages");

            drop(val);
            con_opt.lock().unwrap().take();
            
            continue;
        };
    }
}

fn get_consumer_with_retry(
    con:  Mutex<Option<Consumer>>,
    brokers: &[String],
    topic: &str,
    group: &str,
    duration: std::time::Duration,
) ->  Mutex<Option<Consumer>> {
    let option = con.lock().unwrap();

    if option.is_none() {
        loop {
            let Ok(consumer) = kafka_connectivity::get_consumer(brokers, topic, group) else {
                println!("Could not open connection to kafka broker. Retry in {:?}", duration);

                common::sleep(duration);
                continue;
            };            
            return Mutex::new(Some(consumer));
        }
    }
    drop(option);
    con
}

fn handle_message(
    m: &kafka::consumer::Message<'_>,
    response_topic: &str,
    brokers: &[String],
    error_topic: &str,
) {
    let Ok(json) = std::string::String::from_utf8(m.value.to_vec()) else {
        send_response("Could get json from message", error_topic, brokers);
        return;
    };

    println!("received message: {}", json);

    let Ok(envelope) = serde_json::from_str::<Envelope>(&json) else {
        send_response("Could not convert an expected json message to an object", error_topic, brokers);
        return;
    };

    match process_command(envelope.message.as_str()) {
        Ok(execution_result) => {
            if let Some(result_value) = execution_result {
                let response_envelope = envelope.copy_with_new_message(result_value);
                println!("sending response envelope {:?}", response_envelope);
                send_response_envelope(response_envelope, response_topic, error_topic, brokers);
            }
        }
        Err(err) => {
            println!("Error while executing command {:?}", err);
            if let Ok(error_message) = serde_json::to_string(&ErrorResponse::from(err)) {
                send_response(error_message.as_str(), error_topic, brokers);
            }
        }
    }
}

fn consume_messageset(con: &mut kafka::consumer::Consumer, ms: kafka::consumer::MessageSet<'_>) {
    match con.consume_messageset(ms) {
        Ok(_) => {}
        Err(err) => {
            println!("Could not consume messages{:?}", err);
        }
    }
}

fn send_response_envelope(envelope: Envelope, topic: &str, error_topic: &str, brokers: &[String]) {
    let Ok(response) = serde_json::to_string(&envelope) else {
        send_response("", error_topic, brokers);
        return;
    };

    send_response(response.as_str(), topic, brokers);
}

fn send_response(response: &str, topic: &str,brokers: &[String]) {
    loop {
        let Ok(mut producer) = kafka_connectivity::get_producer(brokers) else {
            common::sleep(common::FIVE_SECS);
            continue;
        };
        let key: &String = &common::CLIENT_ID;
        

        println!("message {}", response);
        
        let Ok(_) = producer.send(&Record::from_key_value(topic, key.to_owned(), response)) else {
            common::sleep(common::FIVE_SECS);
            continue;
        };
        drop(producer);
        break;
    }
}

fn process_command(command_str: &str) -> Result<Option<String>, Error> {
    match command_str.parse::<NonSystemCommands>() {
        Ok(non_systen_command) => non_systen_command.execute(),
        Err(_) => {
            // not really an error - just no build-in command. Execute it as a system command
            system_commands::execute_command(command_str)
        }
    }
}
