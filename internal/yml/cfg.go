package yml

import (
	"os"

	"github.com/azusachino/srs-exporter/internal/log"
	"gopkg.in/yaml.v3"
)

type SrsExporterConfig struct {
	App   AppConfig   `yaml:"app"`
	Srs   SrsConfig   `yaml:"srs"`
	Nacos NacosConfig `yaml:"nacos"`
}

type AppConfig struct {
	Host string `yaml:"host"`
	Port uint64 `yaml:"port"`
}

type SrsConfig struct {
	Mode     string `yaml:"mode"`
	Domain   string `yaml:"domain"`
	RtmpPort uint64 `yaml:"rtmpPort"`
	Host     string `yaml:"host"`
	HttpPort uint64 `yaml:"httpPort"`
}

type NacosConfig struct {
	Servers     []HostPort `yaml:"servers"`
	NamespaceId string     `yaml:"namespaceId"`
	GroupName   string     `yaml:"groupName"`
	Auth        bool       `yaml:"auth"`
	Username    string     `yaml:"username"`
	Password    string     `yaml:"password"`
}

type HostPort struct {
	Host string `yaml:"host"`
	Port uint64 `yaml:"port"`
}

// Read Cfg from config-file
func GetCfg(f string) SrsExporterConfig {
	data, err := os.ReadFile(f)
	if err != nil {
		log.Logger.Fatal("fail to open config.toml")
	}
	var cfg SrsExporterConfig
	err = yaml.Unmarshal(data, &cfg)
	if err != nil {
		log.Logger.Fatal("fail to unmarshal data")
	}
	return cfg
}
