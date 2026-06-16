package rpc

import "testing"

type EncodingExample struct {
	Test bool
}

func TestEndcodeMessage(t *testing.T){
	expected := "Content-Length: 13\r\n\r\n{\"Test\":true}"
	content, err := EncodeMessage(EncodingExample{Test: true})
	if err != nil {
		t.Errorf("failed with: %v ", err)
	}
	
	if string(content) != expected {
		t.Errorf("expected %s:  got %s:", expected, content)
		
	}
}


func TestDecodeMessage(t *testing.T){
	msg := "Content-Length: 15\r\n\r\n{\"method\":\"hi\"}"
	method,content, err := DecodeMessage([]byte(msg))
	if err != nil {
		t.Errorf("error: %v ", err)
	}
	
	contentLength := len(content)
	
	if contentLength != 15 {
		t.Errorf("expected: %d got: %d",15, contentLength)
	}
	
	if method != "hi"{
		t.Errorf("expected: %s got: %s", "hi", method)
	}
}