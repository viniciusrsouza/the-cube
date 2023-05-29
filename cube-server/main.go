package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/viniciusrsouza/the-cube/pkg/websocket"
)

func ws(pool *websocket.Pool, w http.ResponseWriter, r *http.Request) {
	log.Println("Receiving connection")
	conn, err := websocket.Upgrade(w, r)
	if err != nil {
		fmt.Println(err)
		return
	}

	client := websocket.NewClient(conn, pool)

	pool.Register <- &client
	client.Read()
}

func routes() {
	pool := websocket.NewPool()
	go pool.Start()
	http.HandleFunc("/ws", func(w http.ResponseWriter, r *http.Request) {
		ws(pool, w, r)
	})
}

func main() {
	fmt.Println("Starting socket server at port 8080")
	routes()
	http.ListenAndServe("0.0.0.0:8080", nil)
}
