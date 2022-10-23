package nacos

import (
	"fmt"
	"net/http"

	"github.com/azusachino/srs-exporter/internal/log"
	"github.com/azusachino/srs-exporter/internal/yml"
	"github.com/nacos-group/nacos-sdk-go/clients"
	"github.com/nacos-group/nacos-sdk-go/clients/naming_client"
	"github.com/nacos-group/nacos-sdk-go/common/constant"
	"github.com/nacos-group/nacos-sdk-go/vo"
)

const SRS = "srs"

var client naming_client.INamingClient

func InitClient(nacosCfg yml.NacosConfig) {
	sl := len(nacosCfg.Servers)
	if sl < 1 {
		log.Logger.Fatalln("please config nacos server")
	}

	var sc []constant.ServerConfig
	for _, srv := range nacosCfg.Servers {
		sc = append(sc, *constant.NewServerConfig(srv.Host, srv.Port))
	}
	opts := []constant.ClientOption{
		constant.WithNamespaceId(nacosCfg.NamespaceId),
		constant.WithTimeoutMs(5000),
		constant.WithNotLoadCacheAtStart(true),
		// constant.WithLogDir("/tmp/nacos/log"),
		// constant.WithCacheDir("/tmp/nacos/cache"),
		// constant.WithRotateTime("1h"),
		// constant.WithMaxAge(3),
		// constant.WithLogLevel("debug"),
	}

	if nacosCfg.Auth {
		opts = append(opts,
			constant.WithUsername(nacosCfg.Username),
			constant.WithPassword(nacosCfg.Password))
	}

	cc := *constant.NewClientConfig(opts...)

	var err error
	client, err = clients.NewNamingClient(
		vo.NacosClientParam{
			ClientConfig:  &cc,
			ServerConfigs: sc,
		},
	)
	if err != nil {
		log.Logger.Fatal(err)
	}
}

func RegisterInstance(cfg yml.SrsExporterConfig) {
	// construct metadata
	metadata := make(map[string]string)
	metadata["cluster_mode"] = cfg.Srs.Mode
	metadata["intranet_host"] = cfg.Srs.Host
	metadata["metric_host"] = cfg.App.Host
	metadata["metric_port"] = fmt.Sprintf("%d", cfg.App.Port)

	ok, err := client.RegisterInstance(vo.RegisterInstanceParam{
		Ip:          cfg.Srs.Domain,
		Port:        cfg.Srs.RtmpPort,
		ServiceName: SRS,
		Weight:      10,
		Metadata:    metadata,
		Enable:      true,
		Healthy:     true,
		Ephemeral:   true,
		GroupName:   cfg.Nacos.GroupName,
	})

	if err != nil {
		log.Logger.Error(err)
	}

	if ok {
		log.Logger.Info("Register to nacos successfully")
	}
}

// 检查 SRS 是否存活
func CheckInstance(cfg *yml.SrsExporterConfig) {
	url := fmt.Sprintf("http://%s:%d/api/v1/summaries", cfg.Srs.Host, cfg.Srs.HttpPort)
	res, err := http.Get(url)

	if err != nil {
		log.Logger.Error(err)
	}

	if res != nil && http.StatusOK == res.StatusCode {
		log.Logger.Info("srs is online")
		// TODO 检查是否要重新注册
		// 1. Get Instance to check healthy

		// 2. If not healthy, redo register
	} else {
		log.Logger.Error("srs is not healthy, deregister it")
		client.DeregisterInstance(vo.DeregisterInstanceParam{
			Ip:          cfg.Srs.Domain,
			Port:        cfg.Srs.RtmpPort,
			ServiceName: SRS,
			Ephemeral:   true,
			GroupName:   cfg.Nacos.GroupName,
		})
	}
}
