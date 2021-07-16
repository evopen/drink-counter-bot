FROM docker.io/archlinux:base-devel

# RUN apt-get update
# RUN apt-get install -y ca-certificates
# RUN update-ca-certificates

COPY ./target/release/drink-counter-bot /bin/

ENV TELOXIDE_TOKEN=NONE

ENTRYPOINT ["/bin/drink-counter-bot"]

