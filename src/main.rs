use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use clap::Parser;
use futures::stream::StreamExt;
use lazy_static::lazy_static;
use leaky_bucket::RateLimiter;
use signal_hook::consts::{SIGHUP, SIGINT, SIGQUIT, SIGTERM};
use signal_hook_tokio::Signals;
use slog::{error, info};
use tokio::task::JoinSet;

use crate::common::WorkloadOption;

mod common;
mod rmq;
mod kafka;

lazy_static! {
    static ref SEND_COUNT: AtomicUsize = AtomicUsize::new(0);
    static ref RECIEVE_COUNT: AtomicUsize = AtomicUsize::new(0);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    let option = WorkloadOption::parse();

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
                match option.driver.as_str() {
                    "rocketmq" => rmq::start_producer(&logger, &option, &rate_limiter).await,
                    "kafka" => kafka::start_producer(&logger, &option, &rate_limiter).await,
                    _ => panic!("Unsupported driver: {}", option.driver),
                }
            });
        }
    }

    if option.mode.contains("consumer") {
        for _ in 0..option.parallelism {
            let logger = logger.clone();
            let option = option.clone();
            join_set.spawn(async move {
                match option.driver.as_str() {
                    "rocketmq" => rmq::start_consumer(&logger, &option).await,
                    "kafka" => kafka::start_consumer(&logger, &option).await,
                    _ => panic!("Unsupported driver: {}", option.driver),
                }
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

