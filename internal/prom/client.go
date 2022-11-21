package prom

import (
	"fmt"
	"io"

	"github.com/azusachino/srs-exporter/internal/request"
	"github.com/azusachino/srs-exporter/internal/yml"
	"github.com/gin-gonic/gin"
	"github.com/goccy/go-json"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promhttp"
)

const STREAM_URL = "api/v1/streams"
const SUMMARY_URL = "api/v1/summaries"

type StreamResponse struct {
	Code    int16          `json:"code"`
	Server  string         `json:"server"`
	Streams []StreamStatus `json:"streams"`
}

type StreamStatus struct {
	Clients   uint32 `json:"clients"`
	Frames    uint32 `json:"frames"`
	SendBytes uint32 `json:"send_bytes"`
	RecvBytes uint32 `json:"recv_bytes"`
	LiveMs    uint64 `json:"live_ms"`
	Id        string `json:"id"`
	Name      string `json:"name"`
	Vhost     string `json:"vhost"`
	App       string `json:"app"`
}

type SummaryResponse struct {
	Code   int16       `json:"code"`
	Server string      `json:"server"`
	Data   SummaryData `json:"data"`
}

type SummaryData struct {
	Ok    bool       `json:"ok"`
	NowMs uint64     `json:"now_ms"`
	Self  SelfStatus `json:"self"`
}

type SelfStatus struct {
	MemPercent float64 `json:"mem_percent"`
	CpuPercent float64 `json:"cpu_percent"`
}

type PromClient struct {
	srsConfig         *yml.SrsConfig
	streamActiveTotal prometheus.Gauge
	streamClientTotal prometheus.Gauge
	memPercent        prometheus.Gauge
	cpuPercent        prometheus.Gauge
}

func (pc *PromClient) collectStreamMetrics() {
	url := fmt.Sprintf("http://%s:%d/%s", pc.srsConfig.Host, pc.srsConfig.HttpPort, STREAM_URL)
	res, err := request.Get(url)

	if err != nil {
		return
	}

	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	// fail to read response
	if err != nil {
		panic(err.Error())
	}

	var sr StreamResponse
	json.Unmarshal(body, &sr)
	ln := len(sr.Streams)
	if ln > 0 {
		pc.streamActiveTotal.Set(float64(ln))
		var cnt uint32 = 0
		for _, st := range sr.Streams {
			cnt += st.Clients
		}
		pc.streamClientTotal.Set(float64(cnt))
	} else {
		pc.streamActiveTotal.Set(0)
		pc.streamClientTotal.Set(0)
	}
}

func (pc *PromClient) collectSummaryMetrics() {
	url := fmt.Sprintf("http://%s:%d/%s", pc.srsConfig.Host, pc.srsConfig.HttpPort, SUMMARY_URL)
	res, err := request.Get(url)

	if err != nil {
		return
	}

	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	// fail to read response
	if err != nil {
		panic(err.Error())
	}

	var sr SummaryResponse
	err = json.Unmarshal(body, &sr)
	if err != nil {
		panic(err.Error())
	}

	pc.cpuPercent.Set(sr.Data.Self.CpuPercent)
	pc.memPercent.Set(sr.Data.Self.MemPercent)
}

var promClient *PromClient
var r *prometheus.Registry

// 初始化 promClient
func InitMetrics(srsConfig *yml.SrsConfig) {
	r = prometheus.NewRegistry()
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
	r.MustRegister(promClient.streamActiveTotal,
		promClient.streamClientTotal, promClient.memPercent, promClient.cpuPercent)
}

func GetHttpHandler() gin.HandlerFunc {
	h := promhttp.HandlerFor(r, promhttp.HandlerOpts{})
	return func(ctx *gin.Context) {
		// 每次抓取的时候，设置新的值
		promClient.collectStreamMetrics()
		promClient.collectSummaryMetrics()
		h.ServeHTTP(ctx.Writer, ctx.Request)
	}
}
