# srs-exporter

Attempt to be a srs exporter.

Inspired by [srs-exporter](https://github.com/chaoswest-tv/srs-exporter), thank you.

## important

**fake http response**: Be careful with HEADER [no space before]

## Metrics

| name                 | help                              |
| -------------------- | --------------------------------- |
| stream_active_total  | Total amount of active streams    |
| stream_clients_total | Total amount of connected clients |

## Next Step

Consider Collect other important metrics (e.g. host resource status) for load balancing.

## run srs

```sh
docker run --rm -it -p 1935:1935 -p 1985:1985 -p 8080:8080 registry.cn-hangzhou.aliyuncs.com/ossrs/srs:4 ./objs/srs -c conf/docker.conf
```

## run nacos

```sh
docker run --rm -it -p 8848:8848 -e PREFER_HOST_MODE=hostname -e MODE=standalone nacos/nacos-server:v2.0.4
```

## Podman Instructions

1. `podman run -it rust:1.59.0-buster /bin/sh`
2. `podman cotainer cp [container:]src_file [container:]target_file`
3. `podman container commit [container] image_name`

## Problem

### WSL reqwest

1. 裸着使用 tokio::TcpStream，并伪装 HTTP 响应，虽然成功瞒过了浏览器，但是无法被 prometheus 的 scraper 正常识别，所以还是集成了 Web 库[axum](https://github.com/tokio-rs/axum) 【例子很多，后面好好学习一下】
2. 在 Windows 环境下运行项目，无法使用 reqwest 库访问 WSL 内部运行的程序。【服了，排查了白天。。】

## Reference

- [How to create small Docker images for Rust](https://kerkour.com/rust-small-docker-image)
