package main

import (
	"fmt"
	"log"

	"github.com/nacos-group/nacos-sdk-go/clients"
	"github.com/nacos-group/nacos-sdk-go/clients/naming_client"
	"github.com/nacos-group/nacos-sdk-go/common/constant"
	"github.com/nacos-group/nacos-sdk-go/vo"
)

var testClient naming_client.INamingClient

func initTestClient() {
	sc := []constant.ServerConfig{
		*constant.NewServerConfig("172.31.103.161", 18848),
	}

	cc := *constant.NewClientConfig(
		constant.WithNamespaceId("scv"),
		constant.WithTimeoutMs(5000),
		constant.WithNotLoadCacheAtStart(true),
		constant.WithLogDir("/tmp/nacos/log"),
		constant.WithCacheDir("/tmp/nacos/cache"),
		// constant.WithRotateTime("1h"),
		// constant.WithMaxAge(3),
		constant.WithLogLevel("debug"),
	)
	var err error
	testClient, err = clients.NewNamingClient(
		vo.NacosClientParam{
			ClientConfig:  &cc,
			ServerConfigs: sc,
		},
	)
	if err != nil {
		log.Fatal(err)
	}

}

func testNacos() {

	initTestClient()

	ok, err := client.RegisterInstance(vo.RegisterInstanceParam{
		Ip:          "10.0.0.12",
		Port:        8848,
		ServiceName: "demo.go",
		Weight:      10,
		ClusterName: "cluster-b",
		Enable:      true,
		Healthy:     true,
		Ephemeral:   true,
		GroupName:   "scv",
	})
	if err != nil {
		log.Println(err)
	}
	if ok {
		fmt.Println("ok")
	}
}
