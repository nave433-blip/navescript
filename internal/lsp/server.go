package lsp

import (
	"encoding/json"
	"fmt"
	"io"
	"os"
)

type Request struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      int             `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

type Response struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      int         `json:"id"`
	Result  interface{} `json:"result,omitempty"`
	Error   *Error      `json:"error,omitempty"`
}

type Error struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

type Server struct {
	reader *json.Decoder
	writer *json.Encoder
}

func NewServer(r io.Reader, w io.Writer) *Server {
	return &Server{
		reader: json.NewDecoder(r),
		writer: json.NewEncoder(w),
	}
}

func (s *Server) Run() {
	for {
		var req Request
		if err := s.reader.Decode(&req); err != nil {
			break
		}
		s.handleRequest(req)
	}
}

func (s *Server) handleRequest(req Request) {
	fmt.Fprintf(os.Stderr, "Method: %s\n", req.Method)
	// Dispatcher logic here
}
