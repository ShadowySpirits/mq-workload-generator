FROM debian:stable-slim
LABEL authors="sspirits"

WORKDIR /root
COPY target/release/mq-workload-generator mq-workload-generator
RUN apt-get update && apt-get install -y dumb-init && apt-get clean
ENTRYPOINT ["dumb-init", "--"]
CMD /root/mq-workload-generator