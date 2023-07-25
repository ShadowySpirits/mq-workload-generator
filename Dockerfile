FROM debian:stable-slim
LABEL authors="sspirits"

WORKDIR /root
COPY target/release/mq-housekeeping mq-housekeeping
RUN apt-get update && apt-get install -y dumb-init && apt-get clean
ENTRYPOINT ["dumb-init", "--"]
CMD /root/mq-housekeeping --cluster ${CLUSTER_ID} --replica ${REPLICA} --access-point ${ACCESS_POINT} --topic ${TOPIC_PREFIX} --access-key ${ACCESS_KEY} --secret-key ${SECRET_KEY}