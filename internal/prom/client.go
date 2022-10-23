package prom

import (
	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
	StreamActiveTotalGauge = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "srs_stream_active_total",
			Help: "Total amount of SRS active streams",
		},
		nil,
	)
	StreamClientTotalGauge = prometheus.NewGaugeVec(
		prometheus.GaugeOpts{
			Name: "srs_stream_clients_total",
			Help: "Total amount of SRS connected clients",
		},
		nil,
	)
)

func InitMetrics() {
	prometheus.MustRegister(StreamActiveTotalGauge)
	prometheus.MustRegister(StreamClientTotalGauge)
}

func GetHttpHandler() gin.HandlerFunc {
	h := promhttp.Handler()
	return func(ctx *gin.Context) {
		h.ServeHTTP(ctx.Writer, ctx.Request)
	}
}
