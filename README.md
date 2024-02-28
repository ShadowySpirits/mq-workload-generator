# RocketMQ Workload Generator

## Introduction

A tool for testing the performance of Apache RocketMQ.

## Quick Start

### Install

#### Install via cargo

```shell
cargo install mq-workload-generator
```

#### Install manually

Download the binary from [release page](https://github.com/ShadowySpirits/mq-workload-generator/releases). Currently, only Linux and macOS are supported. For other platforms, you need to build from source on your own.

### Basic usage

send and receive 100 messages per second to the topic `test`:

```shell
mq-workload-generator --topic test --qps 100
# example output: 
# this tool will print the current send and receive tps every second
Jul 25 10:38:44.203 INFO[src/main.rs:32:5] Begin generating workload and wait for the server to come online...
Jul 25 10:38:44.204 INFO[src/main.rs:82:17] current send tps: 100, receive tps: 100
Jul 25 10:38:45.206 INFO[src/main.rs:82:17] current send tps: 100, receive tps: 100
Jul 25 10:38:46.205 INFO[src/main.rs:82:17] current send tps: 100, receive tps: 100
Jul 25 10:38:47.205 INFO[src/main.rs:82:17] current send tps: 100, receive tps: 100
```

All available options:

```shell
A tool for testing the performance of Apache RocketMQ and Apache Kafka

Usage: mq-workload-generator [OPTIONS] --topic <TOPIC>

Options:
  -d, --driver <DRIVER>
          Work load driver, available option: rocketmq, kafka [env: DRIVER=] [default: rocketmq]
  -a, --access-point <ACCESS_POINT>
          Access point of the mq cluster [env: ACCESS_POINT=] [default: localhost:8081]
  -t, --topic <TOPIC>
          Target topic [env: TOPIC=]
  -g, --group <GROUP>
          Group used by consumer [env: GROUP=] [default: mq_workload_generator]
  -p, --parallelism <PARALLELISM>
          Number of the client [env: PARALLELISM=] [default: 1]
  -q, --qps <QPS>
          Send tps of the sum of all producers [env: QPS=] [default: 100]
      --min-payload-size <MIN_PAYLOAD_SIZE>
          Minimum message payload size, measured in bytes [env: MIN_PAYLOAD_SIZE=]
      --max-payload-size <MAX_PAYLOAD_SIZE>
          Maximum message payload size, measured in bytes [env: MAX_PAYLOAD_SIZE=]
      --payload-size <PAYLOAD_SIZE>
          Fixed message payload size, measured in bytes. The priority is given to the dynamic payload size, which ranges from **min_payload_size** to **max_payload_size** [env: PAYLOAD_SIZE=] [default: 4096]
      --mode <MODE>
          Mode of the workload test, available values: producer, consumer, producer_and_consumer [env: MODE=] [default: producer_and_consumer]
      --access-key <ACCESS_KEY>
          Access Key to the topic [env: ACCESS_KEY=] [default: ]
      --secret-key <SECRET_KEY>
          Secret Key to the topic [env: SECRET_KEY=] [default: ]
  -v, --verbose
          Print detail error [env: VERBOSE=]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Use in Kubernetes

There is an out-of-the-box Kubernetes manifest file available for deploying the workload generator in Kubernetes.

```shell
kubectl create namespace benchmark
kubectl apply -f deployment-consumer.yaml
kubectl apply -f deployment-producer.yaml
```

## TODO

- [x] Add more options: user-specific consumer group, consume time, lag message count, etc.
- [x] Support more platform: Apache Kafka, etc.
- [ ] Add more metrics: send/receive latency, etc.
- [ ] Add more test cases: send/receive with large message or delay/transaction message.
