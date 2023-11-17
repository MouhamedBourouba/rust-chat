package main

import (
	"fmt"
	"net"
)

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:8080")
	if err != nil {
		fmt.Println("Could not connect to the server")
		return
	}
	defer conn.Close()

	for {
		var input string

		fmt.Print("Please write message: ")
		fmt.Scanln(&input)

		if input == "quit" {
			break
		}

		_, err = conn.Write([]byte(input))

		if err != nil {
			print("Cloud not send message")
			break
		}

	}

}
