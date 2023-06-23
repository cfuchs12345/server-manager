use std::{net::IpAddr, vec};

use crate::{datastore, models::error::AppError};
use chrono::{Duration, Utc};
use kafka::{
    consumer::{Consumer, FetchOffset, GroupOffsetStorage},
    producer::{Producer, Record, RequiredAcks},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

pub fn get_producer() -> Result<Producer, AppError> {
    let brokers = get_bootstrap_brokers()?;

    let producer = Producer::from_hosts(brokers)
        .with_ack_timeout(std::time::Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .map_err(AppError::from)?;

    Ok(producer)
}

pub fn get_consumer(topic: &str) -> Result<Consumer, AppError> {
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

///
pub async fn execute_kafka_request(
    ipaddress: Option<IpAddr>,
    topic: &str,
    response_topic: &str,
    command: &str,
    timeout: Duration,
) -> Result<String, AppError> {
    let mut producer = get_producer()?;
    let mut consumer = get_consumer(response_topic)?;

    let envelope = Envelope::new(command);

    let request = serde_json::to_string(&envelope)?;

    log::debug!("send message to kafka topic {} msg: {}", topic, request);

    producer.send(&Record::from_key_value(
        topic,
        format!("{:?}", ipaddress),
        request,
    ))?;

    let response = await_response(envelope, &mut consumer, timeout)?;

    Ok(response)
}

fn await_response(
    envelope: Envelope,
    consumer: &mut Consumer,
    timeout: Duration,
) -> Result<String, AppError> {
    let start_time = Utc::now();

    let mut found: Option<Envelope> = None;

    'outer: loop {
        let mss = consumer.poll().map_err(AppError::from)?;

        if !mss.is_empty() {
            for ms in mss.iter() {
                for m in ms.messages() {
                    let response = std::string::String::from_utf8(m.value.to_vec())?;

                    log::debug!(
                        "received message from kafka topic {} msg: {}",
                        ms.topic(),
                        response
                    );

                    if let Ok(return_envelope) = serde_json::from_str::<Envelope>(&response) {
                        if return_envelope.uuid == envelope.uuid {
                            log::debug!("matched uuid {}", envelope.uuid);
                            let _ = found.insert(return_envelope.clone());
                        }
                    }
                }
                consumer.consume_messageset(ms)?;
            }
            if found.is_some() {
                break 'outer;
            }
        } else if Utc::now().signed_duration_since(start_time).cmp(&timeout)
            == std::cmp::Ordering::Greater
        {
            break 'outer;
        }
    }
    consumer.commit_consumed()?;

    Ok(found.map(|e| e.message).unwrap_or("".to_owned()))
}

fn get_bootstrap_brokers() -> Result<Vec<String>, AppError> {
    let config = datastore::get_config()?;
    let broker = config.get_string("kafka_bootstrap_broker")?;
    Ok(vec![broker])
}
