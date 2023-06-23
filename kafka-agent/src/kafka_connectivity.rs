use core::time;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::producer::{Producer, RequiredAcks};
use kafka::Error;

use crate::common;

pub fn get_consumer(brokers: &[String], topic: &str, group: &str) -> Result<Consumer, Error> {
    let con = Consumer::from_hosts(brokers.to_owned())
        .with_topic(topic.to_owned())
        .with_client_id(common::CLIENT_ID.clone())
        .with_group(group.to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()?;

    Ok(con)
}

pub fn get_producer(brokers: &[String]) -> Result<Producer, Error> {
    let producer = Producer::from_hosts(brokers.to_owned())
        .with_ack_timeout(time::Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()?;
    Ok(producer)
}
