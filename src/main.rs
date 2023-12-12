use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use clap::Parser;
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use leaky_bucket::RateLimiter;
use rand::Rng;
use rocketmq::conf::{ClientOption, LoggingFormat, ProducerOption, SimpleConsumerOption};
use rocketmq::model::common::{FilterExpression, FilterType};
use rocketmq::model::message::MessageBuilder;
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;
use slog::{error, info, Logger};
use tokio::task::JoinSet;

use crate::common::PressureOption;

mod common;

lazy_static! {
    static ref SEND_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ref RECIEVE_COUNT: AtomicUsize = AtomicUsize::new(0);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let option = PressureOption::parse();

    let logger = common::terminal_logger();
    info!(logger, "Begin generating workload and wait for the server to come online...");

    let producer_rate_limiter = RateLimiter::builder()
        .max(option.qps)
        .refill(option.qps)
        .interval(Duration::from_secs(1))
        .build();
    let producer_rate_limiter = Arc::new(producer_rate_limiter);

    let mut join_set = JoinSet::new();

    if option.mode.contains("producer") {
        for _ in 0..option.parallelism {
            let logger = logger.clone();
            let option = option.clone();
            let rate_limiter = producer_rate_limiter.clone();
            join_set.spawn(async move {
                start_producer(&logger, &option, &rate_limiter).await;
            });
        }
    }

    if option.mode.contains("consumer") {
        for _ in 0..option.parallelism {
            let logger = logger.clone();
            let option = option.clone();
            join_set.spawn(async move {
                start_consumer(&logger, &option).await;
            });
        }
    }

    let mut signals =
        Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT]).expect("register signal failed");

    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut send_qps = 0;
                _ = SEND_COUNT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                    send_qps = old;
                    Some(0)
                });
                let mut receive_qps = 0;
                _ = RECIEVE_COUNT.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |old| {
                    receive_qps = old;
                    Some(0)
                });
                info!(logger, "current send tps: {}, receive tps: {}", send_qps, receive_qps)
            },
            Some(res) = join_set.join_next() => {
                if let Err(error) = res {
                    error!(logger, "task end with error: {:?}", error);
                    break;
                }
            },
            Some(signal) = signals.next() => {
                info!(logger, "receive signal: {:?}", rustix::process::Signal::from_raw(signal).unwrap());
                join_set.shutdown().await;
                break;
            }
        }
    }
}

async fn start_producer(logger: &Logger, option: &PressureOption, rate_limiter: &RateLimiter) {
    let mut producer_option = ProducerOption::default();
    producer_option.set_logging_format(LoggingFormat::Terminal);
    producer_option.set_topics(vec![option.topic.clone()]);

    let mut client_option = ClientOption::default();
    client_option.set_access_url(&option.access_point);
    client_option.set_enable_tls(false);
    client_option.set_access_key(&option.access_key);
    client_option.set_secret_key(&option.secret_key);

    let mut producer = rocketmq::Producer::new(producer_option, client_option.clone()).unwrap();
    if let Err(error) = producer.start().await {
        panic!("start producer failed: {:?}", error)
    }

    loop {
        rate_limiter.acquire(1).await;
        SEND_COUNT.fetch_add(1, Ordering::Relaxed);

        let mut body = [0u8; 4096];
        rand::thread_rng().fill(&mut body[..]);

        let message = MessageBuilder::builder()
            .set_topic(option.topic.clone())
            .set_body(Vec::from(body))
            .build()
            .unwrap();
        let result = producer.send(message).await;
        if let Err(error) = result {
            if option.verbose {
                error!(logger, "send message failed: {:?}", error)
            }
        }
    }
}

async fn start_consumer(logger: &Logger, option: &PressureOption) {
    let mut consumer_option = SimpleConsumerOption::default();
    consumer_option.set_logging_format(LoggingFormat::Terminal);
    consumer_option.set_consumer_group(option.group.clone());
    consumer_option.set_topics(vec![option.topic.clone()]);

    let mut client_option = ClientOption::default();
    client_option.set_access_url(&option.access_point);
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
