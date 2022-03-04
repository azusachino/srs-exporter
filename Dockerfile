FROM rust:1.59.0-proxy as builder
WORKDIR /app
# COPY config /root/.cargo/
COPY . .
# ENV RUSTUP_DIST_SERVER "https://rsproxy.cn"
# ENV RUSTUP_UPDATE_ROOT "https://rsproxy.cn/rustup"
RUN ["cargo", "install", "--path", "."]

FROM alpine:3.15
LABEL maintainer="azusachino <azusa146@gmail.com>"
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --update --no-cache ca-certificates
WORKDIR /root
COPY config.toml .
COPY --from=builder /app/srs_exporter .
EXPOSE 9007
ENTRYPOINT ["./srs_exporter"]