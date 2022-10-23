package log

import (
	"github.com/gin-gonic/gin"
	"github.com/sirupsen/logrus"
)

var Logger = logrus.New()

func InitLogrus() error {
	// 设置为json格式的日志
	Logger.Formatter = &logrus.JSONFormatter{}
	gin.SetMode(gin.ReleaseMode)
	// gin框架自己记录的日志也会输出
	gin.DefaultWriter = Logger.Out
	// 设置日志级别
	Logger.Level = logrus.InfoLevel
	return nil
}
