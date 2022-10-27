FROM golang:1.18 as builder
LABEL maintainer="az <azusachino@duck.com>"

WORKDIR /app
COPY go.mod go.sum ./

# for cn setup proxy
ENV GO111MODULE on
ENV GOPROXY https://goproxy.cn,direct
RUN ["go", "mod", "download"]

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o srs_exporter .

# setup running environment
FROM debian:stable-slim

ENV PORT=9717

WORKDIR /srs

COPY --from=builder /app/srs_exporter .

EXPOSE $PORT
CMD ["/srs/srs_exporter"]
