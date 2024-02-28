use std::sync::atomic::Ordering;
use std::time::Duration;

use futures::{future, StreamExt};
use leaky_bucket::RateLimiter;
use rand::thread_rng;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::ToBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use slog::{error, Logger};

use crate::{RECIEVE_COUNT, SEND_COUNT};
use crate::common::{gen_payload, WorkloadOption};

pub(crate) async fn start_producer(logger: &Logger, option: &WorkloadOption, rate_limiter: &RateLimiter) {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &option.access_point)
        .set("linger.ms", "5")
        .set("batch.size", "1048576") // 1MB
        .create()
        .expect("Kafka producer creation error");

    let payload_size_range = option.payload_size_range();

    loop {
        rate_limiter.acquire(1).await;
        SEND_COUNT.fetch_add(1, Ordering::Relaxed);

        let producer_clone = producer.clone();
        let logger_clone = logger.clone();
        let topic = option.topic.to_string();
        let verbose = option.verbose;
        let payload_size_range_clone = payload_size_range.clone();
        tokio::spawn(async move {
            let payload = gen_payload(thread_rng(), payload_size_range_clone);
            let message: FutureRecord<'_, (), [u8]> = FutureRecord::to(&topic)
                .payload(payload.to_bytes());
            let result = producer_clone.send(message, Duration::from_secs(0)).await;
            if let Err(error) = result {
                if verbose {
                    error!(logger_clone, "send message failed: {:?}", error)
                }
            }
        });
    }
}

pub(crate) async fn start_consumer(logger: &Logger, option: &WorkloadOption) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &option.group)
        .set("bootstrap.servers", &option.access_point)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Kafka consumer creation failed");

    consumer
        .subscribe(&[&option.topic])
        .expect("Kafka consumer can't subscribe to specified topic");


    consumer.stream()
        .for_each(|result| {
            match result {
                Err(error) => {
                    if option.verbose {
                        error!(logger, "receive message failed: {:?}", error);
                    }
                }
                Ok(_message) => {
                    RECIEVE_COUNT.fetch_add(1, Ordering::Relaxed);
                }
            }
            future::ready(())
        }).await;
}
