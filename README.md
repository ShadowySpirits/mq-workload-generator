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
Usage: mq-workload-generator [OPTIONS] --topic <TOPIC>

Options:
  -a, --access-point <ACCESS_POINT>  Access point of the mq cluster [default: localhost:8081]
  -t, --topic <TOPIC>                Target topic
  -p, --parallelism <PARALLELISM>    Number of the client [default: 1]
  -q, --qps <QPS>                    Send tps of the sum of all producers [default: 100]
      --mode <MODE>                  Mode of the workload test, available values: producer, consumer, producer_and_consumer [default: producer_and_consumer]
      --access-key <ACCESS_KEY>      Access Key to the topic [default: ]
      --secret-key <SECRET_KEY>      Secret Key to the topic [default: ]
  -h, --help                         Print help
  -V, --version                      Print version
```

### Use in Kubernetes

There is an out-of-the-box Kubernetes manifest file available for deploying the workload generator in Kubernetes.

```shell
kubectl apply -f deployment-consumer.yaml
kubectl apply -f deployment-producer.yaml
```

## TODO

- [ ] Add more metrics: send/receive latency, etc.
- [ ] Add more test cases: send/receive with large message or delay/transaction message.
- [ ] Add more options: user-specific consumer group, consume time, lag message count, etc.
