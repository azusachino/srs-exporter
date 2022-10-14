package main

import (
	"fmt"
	"io"
	"log"
	"net/http"

	"github.com/azusachino/srs-exporter/internal/toml"
)


func main() {

	// 1. init nacos client, fetch srs information, register to nacos

	// 2. goroutine
	cfg := toml.GetCfg("config.toml")
	fmt.Printf("%+v", cfg)
}

func _http_request() {
	client := &http.Client{}
	url := "http://www.baidu.com"
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}

	req.Header.Add("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
	req.Header.Add("Accept-Charset", "utf-8")
	//req.Header.Add("Accept-Encoding","br, gzip, deflate")
	req.Header.Add("Accept-Language", "zh-cn")
	req.Header.Add("Connection", "keep-alive")
	//req.Header.Add("Cookie","xxxxxxxxxxxxxxx")
	//req.Header.Add("Content-Lenght",xxx)
	req.Header.Add("Host", "www.baidu.com")
	req.Header.Add("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.100 Safari/537.36")
	rep, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	data, err := io.ReadAll(rep.Body)
	rep.Body.Close()
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("%s", data)
}
