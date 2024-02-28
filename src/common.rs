use std::ops::Range;

use clap::Parser;
use rand::prelude::ThreadRng;
use rand::Rng;
use slog::{Logger, o};
use slog::Drain;
use slog_async::OverflowStrategy;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct WorkloadOption {
    /// Work load driver, available option: rocketmq, kafka
    #[arg(short, long, env, default_value = "rocketmq")]
    pub driver: String,

    /// Access point of the mq cluster
    #[arg(short, long, env, default_value = "localhost:8081")]
    pub access_point: String,

    /// Target topic
    #[arg(short, long, env)]
    pub topic: String,

    /// Group used by consumer
    #[arg(short, long, env, default_value = "mq_workload_generator")]
    pub group: String,

    /// Number of the client
    #[arg(short, long, env, default_value_t = 1)]
    pub parallelism: i32,

    /// Send tps of the sum of all producers
    #[arg(short, long, env, default_value_t = 100)]
    pub qps: usize,

    /// Minimum message payload size, measured in bytes
    #[arg(long, env)]
    pub min_payload_size: Option<usize>,

    /// Maximum message payload size, measured in bytes
    #[arg(long, env)]
    pub max_payload_size: Option<usize>,

    /// Fixed message payload size, measured in bytes. The priority is given to the dynamic payload size, which ranges from **min_payload_size** to **max_payload_size**.
    #[arg(long, env, default_value_t = 4096)]
    pub payload_size: usize,

    /// Mode of the workload test, available values: producer, consumer, producer_and_consumer
    #[arg(long, env, default_value = "producer_and_consumer")]
    pub mode: String,

    /// Access Key to the topic
    #[arg(long, env, default_value = "")]
    pub access_key: String,

    /// Secret Key to the topic
    #[arg(long, env, default_value = "")]
    pub secret_key: String,

    /// Print detail error
    #[arg(short, long, env, default_value_t = false)]
    pub verbose: bool,
}

impl WorkloadOption {
    pub(crate) fn payload_size_range(&self) -> Range<usize> {
        let default = self.payload_size..self.payload_size + 1;
        if let (Some(min_payload_size), Some(max_payload_size)) = (self.min_payload_size, self.max_payload_size) {
            if max_payload_size > min_payload_size {
                return min_payload_size..max_payload_size
            }
        }
        default
    }
}

pub(crate) fn terminal_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .use_file_location()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain)
        .overflow_strategy(OverflowStrategy::Block)
        .chan_size(1)
        .build()
        .fuse();
    Logger::root(drain, o!())
}


pub(crate) fn gen_payload(mut rng: ThreadRng, payload_size_range: Range<usize>) -> Vec<u8> {
    let body_size = rng.gen_range(payload_size_range);
    let mut body = vec![0u8; body_size];
    rng.fill(&mut body[..]);
    body
}
