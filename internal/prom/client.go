package prom

import (
	"fmt"

	"net/http"
	"github.com/azusachino/srs-exporter/internal/yml"
	"github.com/gin-gonic/gin"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

const STREAM_URL = "/api/v1/streams"
const SUMMARY_URL = "/api/v1/summaries"

type StreamResponse struct {
	code    int16
	server  string
	clients []StreamStatus
}

type StreamStatus struct {
	clients   uint32
	frames    uint32
	sendBytes uint32
	recvBytes uint32
	liveMs    uint64
	id        string
	name      string
	vhost     string
	app       string
}

type SummaryResponse struct {
	code   int16
	server string
	data   SummaryData
}

type SummaryData struct {
	ok    bool
	nowMs uint64
	self  SelfStatus
}

type SelfStatus struct {
	memPercent float64
	cpuPercent float64
}

type PromClient struct {
	srsConfig *yml.SrsConfig
	streamActiveTotal prometheus.Gauge
	streamClientTotal prometheus.Gauge
	memPercent        prometheus.Gauge
	cpuPercent        prometheus.Gauge
}

func (pc *PromClient) collectStreamMetrics() {
	url := fmt.Sprintf("http://%s:%d/%s", pc.srsConfig.Host, pc.srsConfig.HttpPort, STREAM_URL)
	res, err := http.Get(url)
	if err != nil {

	}
	defer res.Body.Close()

	
}

func (pc *PromClient) collectSummaryMetrics() {

}

var promClient *PromClient

func InitMetrics(srsConfig *yml.SrsConfig) {
	promClient = &PromClient{
		srsConfig: srsConfig,
		streamActiveTotal: prometheus.NewGauge(prometheus.GaugeOpts{
			Name: "srs_stream_active_total",
			Help: "Total amount of SRS active streams",
		}),
		streamClientTotal: prometheus.NewGauge(prometheus.GaugeOpts{
			Name: "srs_stream_clients_total",
			Help: "Total amount of SRS connected clients",
		}),
		memPercent: prometheus.NewGauge(prometheus.GaugeOpts{
			Name: "srs_mem_percent",
			Help: "Memory usage percent of SRS",
		}),
		cpuPercent: prometheus.NewGauge(prometheus.GaugeOpts{
			Name: "srs_cpu_percent",
			Help: "Cpu usage percent of SRS",
		}),
	}
}

func GetHttpHandler() gin.HandlerFunc {
	h := promhttp.Handler()

	return func(ctx *gin.Context) {
		promClient.collectStreamMetrics()
		promClient.collectSummaryMetrics()
		h.ServeHTTP(ctx.Writer, ctx.Request)
	}
}
