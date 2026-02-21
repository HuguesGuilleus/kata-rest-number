package main

import (
	"io"
	"log"
	"net/http"
	"strconv"
	"strings"
)

func main() {
	log.Println("listen ...")
	log.Fatal(http.ListenAndServe(":8000", &Server{42}))
}

type Server struct {
	nb int
}

func (s *Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	log.Printf("%s %q", r.Method, r.URL)
	w.Header().Add("Content-Type", "application/json")
	if r.URL.Path == "/" {
		switch r.Method {
		case http.MethodGet, http.MethodHead:
			// ok
		case http.MethodDelete:
			s.nb = 0
		case http.MethodPost:
			s.nb++
		}
	} else if after, ok := strings.CutPrefix(r.URL.Path, "/set-by-path/"); ok {
		if r.Method != http.MethodPut {
			w.WriteHeader(http.StatusMethodNotAllowed)
			io.WriteString(w, "\"405 Method not allowed\"\r\n")
			return
		}
		if s.setInteger(after) {
			w.WriteHeader(400)
			io.WriteString(w, "\"400 Need a path as /set-by-path/:int\"\r\n")
			return
		}
	} else if r.URL.Path == "/set-by-query" {
		if r.Method != http.MethodPut {
			w.WriteHeader(http.StatusMethodNotAllowed)
			io.WriteString(w, "\"405 Method not allowed\"\r\n")
			return
		}
		if s.setInteger(r.URL.Query().Get("nb")) {
			w.WriteHeader(400)
			io.WriteString(w, "\"400 Need integer in query with name nb\"\r\n")
			return
		}
	} else if r.URL.Path == "/set-by-header" {
		if r.Method != http.MethodPut {
			w.WriteHeader(http.StatusMethodNotAllowed)
			io.WriteString(w, "\"405 Method not allowed\"\r\n")
			return
		}
		if s.setInteger(r.Header.Get("x-nb")) {
			w.WriteHeader(400)
			io.WriteString(w, "\"400 Need integer in header with name x-nb\"\r\n")
			return
		}
	} else if r.URL.Path == "/set-by-body" {
		if r.Method != http.MethodPut {
			w.WriteHeader(http.StatusMethodNotAllowed)
			io.WriteString(w, "\"405 Method not allowed\"\r\n")
			return
		}
		buff := [10]byte{}
		n, _ := r.Body.Read(buff[:])
		if s.setInteger(string(buff[:n])) {
			w.WriteHeader(400)
			io.WriteString(w, "\"400 Need integer in the body\"\r\n")
			return
		}
	} else {
		w.WriteHeader(404)
		io.WriteString(w, "\"404 Path not found\"\r\n")
		return
	}
	io.WriteString(w, strconv.Itoa(s.nb))
}

func (s *Server) setInteger(str string) (fail bool) {
	if str == "" {
		return true
	}
	nb, err := strconv.Atoi(str)
	if err != nil {
		return true
	}
	s.nb = nb
	return false
}
