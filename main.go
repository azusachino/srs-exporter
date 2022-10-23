package main

import (
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/azusachino/srs-exporter/internal/log"
	"github.com/azusachino/srs-exporter/internal/nacos"
	"github.com/azusachino/srs-exporter/internal/prom"
	"github.com/azusachino/srs-exporter/internal/yml"

	"github.com/gin-gonic/gin"
)

var cfgFile string
var err error

func init() {
	// treat first arg as config file relative location & parse config
	args := flag.Args()
	if len(args) > 1 {
		cfgFile = args[1]
	} else {
		cfgFile = "config.yaml"
	}
}

func main() {

	err = log.InitLogrus()
	if err != nil {
		fmt.Println("failed to init logger")
		os.Exit(1)
	}

	// 0. load the config
	// log.Sugar.Infof("config location: %s \n", cfgFile)
	cfg := yml.GetCfg(cfgFile)
	gin.SetMode(gin.ReleaseMode)
	srv := gin.Default()

	// 1. init nacos client, fetch srs information, register to nacos
	nacos.InitClient(cfg.Nacos)
	nacos.RegisterInstance(cfg)

	// 2. init prom client
	prom.InitMetrics()
	srv.GET("/metrics", prom.GetHttpHandler())

	// 3. register routers
	srv.GET("/", func(ctx *gin.Context) {
		ret := make(map[string]string)
		ret["msg"] = "Welcome to srs-exporter"
		ret["tip"] = "Please go to `/metrics`"
		ctx.JSON(200, ret)
	})
	
	// 4. goroutine to check srs status (deregister)
	go func(config *yml.SrsExporterConfig) {
		for {
			time.Sleep(2 * time.Second)
			nacos.CheckInstance(config)
		}
	}(&cfg)

	// 5. start gin server
	addr := fmt.Sprintf("%s:%d", cfg.App.Host, cfg.App.Port)
	srv.Run(addr)
	log.Logger.Info("SRS Exporter started in %s", addr)
}
