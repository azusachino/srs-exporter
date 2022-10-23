package nacos

import (
	"fmt"
	"net/http"

	"github.com/azusachino/srs-exporter/internal/log"
	"github.com/azusachino/srs-exporter/internal/toml"
	"github.com/nacos-group/nacos-sdk-go/clients"
	"github.com/nacos-group/nacos-sdk-go/clients/naming_client"
	"github.com/nacos-group/nacos-sdk-go/common/constant"
	"github.com/nacos-group/nacos-sdk-go/vo"
)

var client naming_client.INamingClient
var httpClient *http.Client

func InitClient(nacosCfg toml.NacosConfig) {
	sl := len(nacosCfg.Servers)
	if sl < 1 {
		log.Sugar.Fatalln("please config nacos server")
	}
	httpClient = &http.Client{}

	var sc []constant.ServerConfig
	for _, srv := range nacosCfg.Servers {
		sc = append(sc, *constant.NewServerConfig(srv.Host, srv.Port))
	}

	cc := *constant.NewClientConfig(
		constant.WithNamespaceId(nacosCfg.NamespaceId),
		constant.WithTimeoutMs(5000),
		constant.WithNotLoadCacheAtStart(true),
		// constant.WithLogDir("/tmp/nacos/log"),
		// constant.WithCacheDir("/tmp/nacos/cache"),
		// constant.WithRotateTime("1h"),
		// constant.WithMaxAge(3),
		// constant.WithLogLevel("debug"),
	)
	var err error
	client, err = clients.NewNamingClient(
		vo.NacosClientParam{
			ClientConfig:  &cc,
			ServerConfigs: sc,
		},
	)
	if err != nil {
		log.Sugar.Fatal(err)
	}
}

func RegisterInstance(cfg toml.SrsExporterConfig) {
	// construct metadata
	metadata := make(map[string]string)
	metadata["cluster_mode"] = cfg.Srs.Mode
	metadata["intranet_host"] = cfg.Srs.Host
	metadata["metric_host"] = cfg.Srs.Host
	metadata["metric_port"] = cfg.Srs.Host

	ok, err := client.RegisterInstance(vo.RegisterInstanceParam{
		Ip:          cfg.Srs.Domain,
		Port:        cfg.Srs.RtmpPort,
		ServiceName: "srs",
		Weight:      10,
		Metadata:    metadata,
		Enable:      true,
		Healthy:     true,
		Ephemeral:   true,
		GroupName:   cfg.Nacos.GroupName,
	})

	if err != nil {
		log.Sugar.Error(err)
	}

	if ok {
		log.Sugar.Info("Register to nacos successfully")
	}
}

// 检查 SRS 是否存活
func CheckInstance(cfg toml.SrsExporterConfig) {
	url := fmt.Sprintf("http://%s:%d/api/v1/summaries", cfg.Srs.Host, cfg.Srs.HttpPort)
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Sugar.Error(err)
	}
	res, err := httpClient.Do(req)
	if err != nil {
		log.Sugar.Error(err)
	}
	if http.StatusOK == res.StatusCode {
		log.Sugar.Info("srs is still online")
	}
}
