# fix the install path for cargo
FROM debian:latest

RUN  apt-get update -y && apt-get upgrade -y && apt-get install curl -y && \
apt-get install build-essential -y && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
mkdir -p app/jeeves && . $HOME/.cargo/env

WORKDIR /app/jeeves

COPY . .

RUN . $HOME/.cargo/env

ENV PATH="/root/.cargo/bin:$PATH"

CMD ["cargo", "run"]

EXPOSE 3000
