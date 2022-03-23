# srs-exporter

1. Collect SRS metrics from API and transform to Prometheus style.
2. Report SRS metadata to Nacos for service discovery.

Inspired by [srs-exporter](https://github.com/chaoswest-tv/srs-exporter), thanks.

## Metrics

| name                     | help                    |
| ------------------------ | ----------------------- |
| srs_stream_active_total  | SRS active streams      |
| srs_stream_clients_total | SRS connected clients   |
| srs_cpu_percent          | SRS cpu usage (percent) |
| srs_mem_percent          | SRS mem usage (percent) |

## Few Instructions

### run srs

```sh
docker run --rm -it -p 1935:1935 -p 1985:1985 -p 8080:8080 registry.cn-hangzhou.aliyuncs.com/ossrs/srs:4 ./objs/srs -c conf/docker.conf
```

### run nacos

```sh
docker run --rm -it -p 8848:8848 -e PREFER_HOST_MODE=hostname -e MODE=standalone nacos/nacos-server:v2.0.4
```

## Podman Instructions

1. `podman run -it rust:1.59.0-buster /bin/sh`
2. `podman cotainer cp [container:]src_file [container:]target_file`
3. `podman container commit [container] image_name`

## Problems Encountered

1. 裸着使用 tokio::TcpStream，并伪装 HTTP 响应，虽然成功瞒过了浏览器，但是无法被 prometheus 的 scraper 正常识别，所以还是集成了 Web 库[axum](https://github.com/tokio-rs/axum)
2. 在 Windows 环境下运行项目，无法使用 reqwest 库访问 WSL 内部运行的程序。

## Reference

- [How to create small Docker images for Rust](https://kerkour.com/rust-small-docker-image)
