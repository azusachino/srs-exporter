package prom

import "github.com/prometheus/client_golang/prometheus"

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

func init() {
	prometheus.MustRegister(StreamActiveTotalGauge)
	prometheus.MustRegister(StreamClientTotalGauge)
}

func Collect() {
	go func() {
		for {
			// TODO
		}
	}()
}
