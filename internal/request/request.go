package request

import (
	"net/http"

	"github.com/azusachino/srs-exporter/internal/log"
)

// 私有化 HttpClient
var DefaultClient = &http.Client{}

// Get 针对 http.Get 进行封装
func Get(url string) (*http.Response, error) {
	var err error

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Logger.Error("fail to create new http.request", err)
		return nil, err
	}

	// force conenction close
	req.Header.Set("Connection", "close")

	res, err := DefaultClient.Do(req)
	if err != nil {
		log.Logger.Error("fail to make the http.request", err)
		return nil, err
	}

	return res, nil
}
