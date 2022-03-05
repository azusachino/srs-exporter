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

FROM ubuntu:22.04
LABEL maintainer="azusachino <azusa146@gmail.com>"

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /srs

COPY --from=builder /app/target/release/srs_exporter ./

USER srs:srs
EXPOSE 9717
CMD ["/srs/srs_exporter"]