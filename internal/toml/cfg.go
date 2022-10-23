package toml

import (
	"github.com/pelletier/go-toml/v2"
	"log"
	"os"
)

type SrsExporterConfig struct {
	App   AppConfig
	Srs   SrsConfig
	Nacos NacosConfig
}

type AppConfig struct {
	Host string
	Port uint64
}

type SrsConfig struct {
	Mode     string
	Domain   string
	RtmpPort uint64
	Host     string
	HttpPort uint64
}

type NacosConfig struct {
	Servers     []HostPort
	NamespaceId string
	GroupName   string
	Auth        bool
	Username    string
	Password    string
}

type HostPort struct {
	Host string
	Port uint64
}

// Read Cfg from config-file
func GetCfg(f string) SrsExporterConfig {
	data, err := os.ReadFile(f)
	if err != nil {
		log.Fatal("fail to open config.toml")
	}
	var cfg SrsExporterConfig
	err = toml.Unmarshal(data, &cfg)
	if err != nil {
		log.Fatal("fail to unmarshal data")
	}
	return cfg
}
