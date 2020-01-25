FROM ubuntu:latest
RUN apt-get --yes --no-upgrade update
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get --yes --no-upgrade --no-install-recommends install apt-utils
RUN apt-get --yes --no-upgrade --no-install-recommends install cargo
ADD . /home/clarith
WORKDIR /home/clarith
RUN cargo test --release
RUN cargo doc --release --no-deps
