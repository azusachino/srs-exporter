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
