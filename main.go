package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"
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
	sigs := make(chan os.Signal, 1)
	signal.Notify(sigs, os.Interrupt, syscall.SIGTERM)

	go func() {
		<-sigs
		fmt.Println("SIGINT or SIGTERM received, exit...")
		os.Exit(1)
	}()

	err = log.InitLogrus()
	if err != nil {
		fmt.Println("failed to init logger")
		os.Exit(1)
	}

	// 0. load the config
	log.Logger.Info("config location: ", cfgFile)
	cfg := yml.GetCfg(cfgFile)

	// 1. release the beast (server)
	gin.SetMode(gin.ReleaseMode)
	srv := gin.Default()

	// 2. init nacos client, fetch srs information, register to nacos
	nc := nacos.InitClient(&cfg)
	nc.RegisterInstance()

	// 3. init prom client
	prom.InitMetrics(&cfg.Srs)
	srv.GET("/metrics", prom.GetHttpHandler())

	// 4. register routers
	srv.GET("/", func(ctx *gin.Context) {
		ret := make(map[string]string)
		ret["msg"] = "Welcome to srs-exporter"
		ret["tip"] = "Please go to `/metrics`"
		ctx.JSON(200, ret)
	})

	// 5. goroutine to check srs status (deregister)
	go func(config *yml.SrsExporterConfig) {
		for {
			time.Sleep(5 * time.Second)
			nc.CheckInstance()
		}
	}(&cfg)

	// 6. start gin server
	log.Logger.Info("srs-exporter started in ", fmt.Sprintf("%s:%d", cfg.App.Host, cfg.App.Port))
	srv.Run(fmt.Sprintf(":%d", cfg.App.Port))
}
