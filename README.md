# srs-exporter

tempt to be a srs exporter

## important

**fake http response**: Be careful with HEADER [no space before]

## run srs

```sh
docker run --rm -it -p 1935:1935 -p 1985:1985 -p 8080:8080 \
    registry.cn-hangzhou.aliyuncs.com/ossrs/srs:4 ./objs/srs -c conf/docker.conf
```
