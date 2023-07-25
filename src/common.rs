use std::time::Instant;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct HKOption {
    /// Name of the cluster
    #[arg(short, long)]
    pub cluster: String,

    /// Prefix of the topic
    #[arg(short, long)]
    pub topic: String,

    /// Prefix of the topic
    #[arg(long, default_value = "normal,delay,transaction")]
    pub topic_type: String,

    /// Access point of the cluster
    #[arg(long, default_value = "localhost:8081")]
    pub access_point: String,

    /// Access Key to the topic
    #[arg(long, default_value = "")]
    pub access_key: String,

    /// Secret Key to the topic
    #[arg(long, default_value = "")]
    pub secret_key: String,

    /// Broker replica of the cluster
    #[arg(long, default_value_t = 1)]
    pub replica: i32,

    /// Filter expression
    #[arg(long, default_value = "")]
    pub tag: String,

    /// Consume timeout in seconds, after which the message will be discarded
    #[arg(long, default_value_t = 60)]
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct HKMessage {
    pub message_id: String,
    pub topic: String,
    pub topic_type: String,
    pub body: Option<Vec<u8>>,
    pub tag: Option<String>,
    pub message_group: Option<String>,
    pub delivery_timestamp: Option<i64>,
    pub born_timestamp: Instant,
    pub consume_timestamp: Option<Instant>,
}
