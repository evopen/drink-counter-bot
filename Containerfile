FROM rust

RUN git clone https://github.com/evopen/drink-counter-bot

WORKDIR drink-counter-bot

RUN cargo install --path .
