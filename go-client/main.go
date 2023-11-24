package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
)

func listen(conn net.Conn) {
	for {
		buffer := make([]byte, 1024)
		n, err := conn.Read(buffer)
		if err != nil {
			fmt.Println("Error reading from connection:", err)
			return
		}
		fmt.Println("Message received:", string(buffer[:n]))
	}
}

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:8080")
	if err != nil {
		fmt.Println("Could not connect to the server:", err)
		return
	}
	defer conn.Close()
	reader := bufio.NewReader(os.Stdin)
	
	go listen(conn)

	for {
		fmt.Print("Enter message (type 'quit' to exit): ")
		input, err := reader.ReadString('\n')
		input = strings.TrimRight(input, "\n")

		if input == "quit" || err != nil {
			fmt.Println("Exiting chat room...")
			break
		}

		_, err = conn.Write([]byte(input))
		if err != nil {
			fmt.Println("Error sending message:", err)
			break
		}
	}
}
