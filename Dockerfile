FROM debian:stable-slim
LABEL authors="sspirits"

WORKDIR /root
COPY target/release/mq-workload-generator mq-workload-generator
RUN apt-get update && apt-get install -y dumb-init && apt-get clean
ENTRYPOINT ["dumb-init", "--"]
CMD /root/mq-workload-generator --access-point ${ACCESS_POINT} --mode ${MODE} --parallelism ${PARALLELISM} --qps ${QPS} --topic ${TOPIC} --access-key ${ACCESS_KEY} --secret-key ${SECRET_KEY}