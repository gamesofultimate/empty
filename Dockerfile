FROM debian:stable-slim

WORKDIR /opt/app

RUN apt-get update -y && apt-get install -y ca-certificates
RUN dpkg-reconfigure -p critical ca-certificates

COPY ./.env ./.env
COPY ./resources ./resources
COPY ./target/server/release/mayhem_server ./dist/mayhem_server

CMD ["/opt/app/dist/mayhem_server"]
