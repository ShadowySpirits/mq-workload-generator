# RocketMQ Pressure Generator

## Introduction

This is a tool to test rocketmq server performance.

## Quick Start

### Local Test

Download the binary from [release page](https://github.com/ShadowySpirits/mq-pressure-generator/releases). Currently only Linux and macOS are supported. For other platforms, you need to build from source on your own.

Basic usage:

send and receive 100 messages per second to the topic `test`:

```shell
mq-pressure-generator --topic test --qps 100
```

All available options:

```shell
Usage: mq-pressure-generator [OPTIONS] --topic <TOPIC>

Options:
  -a, --access-point <ACCESS_POINT>  Access point of the mq cluster [default: localhost:8081]
  -t, --topic <TOPIC>                Target topic
  -p, --parallelism <PARALLELISM>    Number of the client [default: 1]
  -q, --qps <QPS>                    Send tps of the sum of all producers [default: 100]
      --mode <MODE>                  Mode of the pressure test, available values: producer, consumer, producer_and_consumer [default: producer_and_consumer]
      --access-key <ACCESS_KEY>      Access Key to the topic [default: ]
      --secret-key <SECRET_KEY>      Secret Key to the topic [default: ]
  -h, --help                         Print help
  -V, --version                      Print version
```

### Test in Kubernetes

There is a out of box kubernetes manifest file for you to deploy the pressure generator in kubernetes.

```shell
kubectl apply -f mq-pressure-generator-consumer.yaml
kubectl apply -f mq-pressure-generator-producer.yaml
```