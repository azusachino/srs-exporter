FROM golang:1.18 as builder
LABEL maintainer="az <azusachino@yahoo.com>"

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
COPY go.mod go.sum ./

# for cn setup proxy
ENV GO111MODULE on
ENV GOPROXY https://goproxy.cn,direct
RUN ["go", "mod", "download"]

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -a -installsuffix cgo -o srs_exporter .

# setup running environment
FROM alpine:3.13
# replace mirror at first
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk add --update --no-cache ca-certificates

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /srs

COPY --from=builder /app/srs_exporter .

USER srs:srs
CMD ["/srs/srs_exporter"]