package nacos

import (
	"fmt"
	"log"

	"github.com/azusachino/srs-exporter/internal/toml"
	"github.com/nacos-group/nacos-sdk-go/clients"
	"github.com/nacos-group/nacos-sdk-go/clients/naming_client"
	"github.com/nacos-group/nacos-sdk-go/common/constant"
	"github.com/nacos-group/nacos-sdk-go/vo"
)

var client naming_client.INamingClient

func initTestClient(nacosCfg toml.NacosConfig) {
	sl := len(nacosCfg.Servers)
	if sl < 1 {
		log.Fatal("please config nacos server")
	}

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
		log.Fatal(err)
	}

}

func RegisterInstance() {

	ok, err := client.RegisterInstance(vo.RegisterInstanceParam{
		Ip:          "10.0.0.12",
		Port:        8848,
		ServiceName: "demo.go",
		Weight:      10,
		// ClusterName: "cluster-b",
		Enable:    true,
		Healthy:   true,
		Ephemeral: true,
		GroupName: "scv",
	})
	if err != nil {
		log.Println(err)
	}
	if ok {
		fmt.Println("ok")
	}
}
