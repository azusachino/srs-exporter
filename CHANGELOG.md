# CHANGE_LOG

## [v1.1.0](https://github.com/azusachino/srs-exporter/releases/tag/v1.1.0)

1. 将 SRS 的内外网信息汇报至 Nacos，以实现内外网分离
2. 集成 Nacos 鉴权功能

## [v1.0.0](https://github.com/azusachino/srs-exporter/releases/tag/v1.0.0)

1. 通过 1985 端口检测 SRS 是否存活，并将 SRS 基础信息汇报至 Nacos
2. 通过 1985 端口查询 SRS 的指标信息，并暴露在服务 HTTP 端口上，待 Prometheus 采集
