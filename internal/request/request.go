package request

import (
	"io"
	"net/http"
	"time"

	"github.com/azusachino/srs-exporter/internal/log"
)

// 私有化 HttpClient
var DefaultClient = &http.Client{
	Transport: &http.Transport{
		MaxIdleConnsPerHost: 20,
	},
	Timeout: 10 * time.Second,
}

// Get 针对 http.Get 进行封装
// @return statusCode, body, error
func Get(url string) (int, []byte, error) {
	var err error

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Logger.Error("fail to create new http.request", err)
		return 0, nil, err
	}

	res, err := DefaultClient.Do(req)
	if err != nil {
		log.Logger.Error("fail to make the http.request", err)
		return 0, nil, err
	}

	statusCode := res.StatusCode

	defer res.Body.Close()
	body, err := io.ReadAll(res.Body)
	if err != nil {
		log.Logger.Error("fail to extract body", err)
		return 0, nil, err
	}

	return statusCode, body, nil
}
