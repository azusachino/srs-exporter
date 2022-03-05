FROM rust:1.59 as builder
# for cn proxy
COPY ./config  /usr/local/cargo
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
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
RUN ["cargo", "build", "--target", "x86_64-unknown-linux-musl", "--release"]

FROM alpine:3.15
LABEL maintainer="azusachino <azusa146@gmail.com>"
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --update --no-cache ca-certificates

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /srs

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/srs_exporter .
USER srs:srs
EXPOSE 9717
CMD ["/srs/srs_exporter"]