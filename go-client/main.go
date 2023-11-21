package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
)

func main() {
	conn, err := net.Dial("tcp", "127.0.0.1:8080")
	if err != nil {
		fmt.Println("Could not connect to the server")
		return
	}
	defer conn.Close()

	reader := bufio.NewReader(os.Stdin)

	for {
		input, err := reader.ReadString('\n')
		input = strings.TrimSpace(input)

		if input == "quit" || err != nil {
			fmt.Println("Exiting chat room ...")
			break
		}

		_, err = conn.Write([]byte(input))
		if err != nil {
			fmt.Println("ERORR:", err.Error())
			break
		}

	}

}
