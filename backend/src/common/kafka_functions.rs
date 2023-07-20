use std::{collections::HashMap, net::IpAddr, sync::Arc, thread, time::Duration, vec};

use crate::{datastore, init, models::error::AppError};

use chrono::Utc;
use kafka::{
    consumer::{Consumer, FetchOffset, GroupOffsetStorage},
    producer::{Producer, Record, RequiredAcks},
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::sleep};
use uuid::Uuid;

lazy_static! {
    static ref RECEIVER: Arc<Mutex<HashMap<String, Receiver>>> =
        Arc::new(Mutex::new(HashMap::new()));
    static ref SENDER: Arc<Mutex<HashMap<String, Sender>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Envelope {
    uuid: String,
    message: String,
    timestamp: i64,
}

impl Envelope {
    fn new(message: &str) -> Self {
        Envelope {
            uuid: Uuid::new_v4().to_string(),
            message: message.to_owned(),
            timestamp: Utc::now().timestamp(),
        }
    }
}

struct Sender {
    producer: Producer,
    topic: String,
}

impl Sender {
    fn new(topic: &str) -> Result<Self, AppError> {
        let producer = Self::get_producer()?;

        Ok(Sender {
            producer,
            topic: topic.to_owned(),
        })
    }

    pub async fn send(&mut self, message: &str, key: &str) -> Result<String, AppError> {
        let envelope = Envelope::new(message);

        let request = serde_json::to_string(&envelope)?;
        log::trace!(
            "|Thread: {:?}|send message to kafka topic {} msg: {}",
            thread::current().name().unwrap_or_default(),
            self.topic,
            request
        );

        self.producer
            .send(&Record::from_key_value(self.topic.as_str(), key, request))?;
        Ok(envelope.uuid)
    }

    fn get_producer() -> Result<Producer, AppError> {
        let brokers = get_bootstrap_brokers()?;

        let producer = Producer::from_hosts(brokers)
            .with_ack_timeout(std::time::Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()
            .map_err(AppError::from)?;

        Ok(producer)
    }
}

struct Receiver {
    topic: String,
    consumer: Consumer,
    received_messages: HashMap<String, String>,
}

impl Receiver {
    async fn new(topic: &str) -> Result<Self, AppError> {
        let consumer = Self::get_consumer(topic)?;

        let receiver = Receiver {
            topic: topic.to_owned(),
            consumer,
            received_messages: HashMap::new(),
        };

        init::register_poll_message_callback(Self::poll_for_messages, topic).await;

        Ok(receiver)
    }

    fn poll_for_messages(topic: &str) {
        match futures::executor::block_on(RECEIVER.lock()).get_mut(topic) {
            Some(receiver) => receiver.poll(),
            None => {
                log::error!("No receiver found for topic {}", topic);
            }
        }
    }

    fn poll(&mut self) {
        match self.consumer.poll() {
            Ok(msg_sets) => {
                if !msg_sets.is_empty() {
                    for ms in msg_sets.iter() {
                        for m in ms.messages() {
                            match std::string::String::from_utf8(m.value.to_vec()) {
                                Ok(response) => {
                                    log::trace!(
                                        "|Thread: {:?}| received message from kafka topic {} msg: {}",
                                        thread::current().name().unwrap_or_default(),
                                        ms.topic(),
                                        response
                                    );

                                    match serde_json::from_str::<Envelope>(&response) {
                                        Ok(envelope) => {
                                            self.received_messages
                                                .insert(envelope.uuid, envelope.message);
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "Error while parsing json {}: {:?}",
                                                response,
                                                err
                                            );
                                        }
                                    }
                                }
                                Err(err) => {
                                    log::error!("Error while getting message: {:?}", err);
                                }
                            }
                        }
                        self.consumer
                            .consume_messageset(ms)
                            .expect("Could not consume messages");
                    }
                }
            }
            Err(err) => {
                log::error!("Error while polling topic {}: {:?}", self.topic, err);
            }
        }
        self.consumer.commit_consumed().expect("Could not commit");
    }

    fn get_consumer(topic: &str) -> Result<Consumer, AppError> {
        let brokers = get_bootstrap_brokers()?;

        let con = Consumer::from_hosts(brokers)
            .with_topic(topic.to_owned())
            .with_client_id("Server".to_owned())
            .with_group("agent".to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
            .create()
            .map_err(AppError::from)?;

        Ok(con)
    }

    async fn get_message(
        &self,
        uuid: &str,
        timeout: chrono::Duration,
        _key: &str,
    ) -> Result<String, AppError> {
        let start = Utc::now();
        let mut result = None;

        loop {
            let opt = self.received_messages.get(uuid);

            match opt {
                Some(response) => result = Some(response.clone()),
                None => {
                    log::trace!(
                        "|Thread: {:?}| received_messages: {}",
                        thread::current().name().unwrap_or_default(),
                        self.received_messages.len()
                    );
                    sleep(Duration::from_millis(200)).await;
                }
            }
            if result.is_some() || Utc::now().signed_duration_since(start).gt(&timeout) {
                break;
            }
        }
        Ok(result.unwrap_or_default())
    }
}

///
pub async fn execute_kafka_request(
    ipaddress: Option<IpAddr>,
    topic: &str,
    response_topic: &str,
    command: &str,
    timeout: chrono::Duration,
) -> Result<String, AppError> {
    let key = ipaddress.map(|a| format!("{:?}", a)).unwrap_or_default();

    let uuid = send(topic, command, &key).await?;
    let response = receive(response_topic, &uuid, &key, timeout).await?;

    Ok(response)
}

async fn send(topic: &str, command: &str, key: &str) -> Result<String, AppError> {
    let mut map = SENDER.lock().await;
    let sender = match map.get_mut(topic) {
        Some(sender) => sender,
        None => {
            let sender = Sender::new(topic)?;
            map.insert(topic.to_owned(), sender);
            map.get_mut(topic).unwrap()
        }
    };
    let uuid = sender.send(command, key).await?;
    Ok(uuid)
}

async fn receive(
    topic: &str,
    uuid: &str,
    key: &str,
    timeout: chrono::Duration,
) -> Result<String, AppError> {
    let mut map = RECEIVER.lock().await;
    let receiver = match map.get_mut(topic) {
        Some(receiver) => receiver,
        None => {
            let receiver = Receiver::new(topic).await?;
            map.insert(topic.to_owned(), receiver);
            map.get_mut(topic).unwrap()
        }
    };
    receiver.get_message(uuid, timeout, key).await
}

fn get_bootstrap_brokers() -> Result<Vec<String>, AppError> {
    let config = datastore::get_config()?;
    let broker = config.get_string("kafka_bootstrap_broker")?;
    Ok(vec![broker])
}
