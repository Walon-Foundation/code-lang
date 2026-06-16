package rpc

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
)

type BaseMessage struct {
	// Rpc string
	// ID int
	Method string `json:"method"`
}

func EncodeMessage(msg any)(string, error){
	content, err := json.Marshal(msg)
	if err != nil {
		return "", err
	}
	
	return fmt.Sprintf("Content-Length: %d\r\n\r\n%s", len(content), string(content)), nil
}

func DecodeMessage(msg []byte)(string, []byte, error){
	header, content, found := bytes.Cut(msg, []byte{'\r','\n', '\r', '\n'})
	if !found {
		return "",nil, errors.New("invalid json ")
	}

	contentLength, err := parseContentLength(header)
	if err != nil {
		return "", nil ,err
	}
	
	var result BaseMessage
	
	if err := json.Unmarshal(content[:contentLength], &result); err != nil {
		return "", nil ,err
	}
	
	return result.Method, content[:contentLength] ,nil
}

func Spilt(data []byte, _ bool)(advance int, token []byte, err error){
	header, content, found := bytes.Cut(data, []byte{'\r','\n', '\r', '\n'})
	if !found {
		return 0, nil, nil
	}
	
	contentLength,err := parseContentLength(header)
	if err != nil {
		return 0, nil, err
	}
	
	if len(content) < contentLength {
		return 0, nil, nil
	}
	
	totalLength := len(header) + 4 + contentLength
	return totalLength, data[:totalLength], nil
}

func parseContentLength(header []byte) (int, error) {
	lines := strings.Split(string(header), "\r\n")
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if strings.HasPrefix(strings.ToLower(line), "content-length:") {
			value := strings.TrimSpace(line[len("Content-Length:"):])
			return strconv.Atoi(value)
		}
	}
	return 0, errors.New("missing Content-Length header")
}
