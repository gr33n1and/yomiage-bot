FROM rust:1

WORKDIR /app

COPY . .

RUN apt update && apt install -y cmake

CMD ["cargo", "run"]