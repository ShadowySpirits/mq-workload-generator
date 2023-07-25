use clap::Parser;
use slog::{Logger, o};
use slog::Drain;
use slog_async::OverflowStrategy;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct PressureOption {
    /// Access point of the mq cluster
    #[arg(short, long, default_value = "localhost:8081")]
    pub access_point: String,

    /// Target topic
    #[arg(short, long)]
    pub topic: String,

    /// Number of the client
    #[arg(short, long, default_value_t = 1)]
    pub parallelism: i32,

    /// Send tps of the sum of all producers
    #[arg(short, long, default_value_t = 100)]
    pub qps: usize,

    /// Mode of the pressure test, available values: producer, consumer, producer_and_consumer
    #[arg(long, default_value = "producer_and_consumer")]
    pub mode: String,

    /// Access Key to the topic
    #[arg(long, default_value = "")]
    pub access_key: String,

    /// Secret Key to the topic
    #[arg(long, default_value = "")]
    pub secret_key: String,
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
