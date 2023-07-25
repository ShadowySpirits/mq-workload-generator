FROM debian:stable-slim
LABEL authors="sspirits"

WORKDIR /root
COPY target/release/mq-pressure-generator mq-pressure-generator
RUN apt-get update && apt-get install -y dumb-init && apt-get clean
ENTRYPOINT ["dumb-init", "--"]
CMD /root/mq-pressure-generator --access-point ${ACCESS_POINT} --mode ${MODE} --parallelism ${PARALLELISM} --qps ${QPS} --topic ${TOPIC} --access-key ${ACCESS_KEY} --secret-key ${SECRET_KEY}