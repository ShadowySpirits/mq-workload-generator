use std::sync::atomic::Ordering;

use leaky_bucket::RateLimiter;
use rand::thread_rng;
use rocketmq::conf::{ClientOption, LoggingFormat, ProducerOption, SimpleConsumerOption};
use rocketmq::model::common::{FilterExpression, FilterType};
use rocketmq::model::message::MessageBuilder;
use slog::{error, Logger};

use crate::{RECIEVE_COUNT, SEND_COUNT};
use crate::common::{gen_payload, WorkloadOption};

pub(crate) async fn start_producer(logger: &Logger, option: &WorkloadOption, rate_limiter: &RateLimiter) {
    let mut producer_option = ProducerOption::default();
    producer_option.set_logging_format(LoggingFormat::Terminal);
    producer_option.set_topics(vec![option.topic.clone()]);

    let mut client_option = ClientOption::default();
    client_option.set_access_url(option.access_point());
    client_option.set_enable_tls(false);
    client_option.set_access_key(&option.access_key);
    client_option.set_secret_key(&option.secret_key);

    let mut producer = rocketmq::Producer::new(producer_option, client_option.clone()).unwrap();
    if let Err(error) = producer.start().await {
        panic!("start producer failed: {:?}", error)
    }

    let payload_size_range = option.payload_size_range();

    loop {
        rate_limiter.acquire(1).await;

        let payload = gen_payload(thread_rng(), payload_size_range.clone());

        let message = MessageBuilder::builder()
            .set_topic(option.topic.clone())
            .set_body(payload)
            .build()
            .unwrap();
        let result = producer.send(message).await;
        if let Err(error) = result {
            if option.verbose {
                error!(logger, "send message failed: {:?}", error)
            }
        } else {
            SEND_COUNT.fetch_add(1, Ordering::Relaxed);
        }
    }
}

pub(crate) async fn start_consumer(logger: &Logger, option: &WorkloadOption) {
    let mut consumer_option = SimpleConsumerOption::default();
    consumer_option.set_logging_format(LoggingFormat::Terminal);
    consumer_option.set_consumer_group(option.group.clone());
    consumer_option.set_topics(vec![option.topic.clone()]);

    let mut client_option = ClientOption::default();
    client_option.set_access_url(option.access_point());
    client_option.set_enable_tls(false);
    client_option.set_access_key(&option.access_key);
    client_option.set_secret_key(&option.secret_key);

    let mut consumer = rocketmq::SimpleConsumer::new(consumer_option, client_option).unwrap();
    if let Err(error) = consumer.start().await {
        panic!("start consumer failed: {:?}", error)
    }

    loop {
        let result = consumer.receive(option.topic.clone(), &FilterExpression::new(FilterType::Tag, "*")).await;
        if let Err(error) = result {
            if option.verbose {
                error!(logger, "receive message failed: {:?}", error)
            }
        } else {
            let message_vec = result.unwrap();
            RECIEVE_COUNT.fetch_add(message_vec.len(), Ordering::Relaxed);
            for message in message_vec {
                let result = consumer.ack(&message).await;
                if let Err(error) = result {
                    if option.verbose {
                        error!(logger, "ack message {} failed: {:?}", message.message_id(), error)
                    }
                }
            }
        }
    }
}