package websocket

import (
	"log"

	"github.com/google/uuid"
	"github.com/gorilla/websocket"
)

type Client struct {
	ID   string
	Conn *websocket.Conn
	Pool *Pool
}

type Message struct {
	Type   int
	Body   []byte
	Sender string
}

func NewClient(conn *websocket.Conn, pool *Pool) Client {
	return Client{
		ID:   uuid.New().String(),
		Conn: conn,
		Pool: pool,
	}
}

func (c *Client) Read() {
	defer func() {
		c.Pool.Unregister <- c
		c.Conn.Close()
	}()

	for {
		messageType, p, err := c.Conn.ReadMessage()
		if err != nil {
			log.Println(err)
			return
		}

		message := Message{Type: messageType, Body: p, Sender: c.ID}
		c.Pool.Broadcast <- message
		log.Printf("Message Received: %+v", message)
	}
}

func (pool *Pool) Start() {
	for {
		select {
		case client := <-pool.Register:
			pool.Clients[client] = true
			log.Println("Size of Connection Pool: ", len(pool.Clients))
			for client, _ := range pool.Clients {
				log.Println("Client ID: ", client.ID)
			}
			break
		case client := <-pool.Unregister:
			delete(pool.Clients, client)
			log.Println("Size of Connection Pool: ", len(pool.Clients))
			break
		case message := <-pool.Broadcast:
			log.Println("Sending message to all clients in Pool")
			for client, _ := range pool.Clients {
				if client.ID == message.Sender {
					continue
				}
				if err := client.Conn.WriteMessage(message.Type, message.Body); err != nil {
					log.Println(err)
					return
				}
			}
		}
	}
}
