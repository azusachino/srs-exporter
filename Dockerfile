FROM rust:1.59 as builder
# for cn proxy
COPY ./config  /usr/local/cargo

RUN update-ca-certificates

# Create new user
ENV USER=srs
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app
COPY ./ .
RUN ["cargo", "build", "--release"]

FROM debian:buster-slim
LABEL maintainer="azusachino <azusa146@gmail.com>"

### fix openssl missing problem
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev libc6-dev \
    && apt-get autoremove --purge -y wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /srs

COPY --from=builder /app/target/release/srs_exporter ./

USER srs:srs
EXPOSE 9717
CMD ["/srs/srs_exporter"]