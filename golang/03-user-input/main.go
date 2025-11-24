package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	welcomeMessage := "Welcome to the user input program"
	fmt.Println(welcomeMessage)

	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Enter your name: ")
	// comma ok syntax or comma error syntax
	name, _ := reader.ReadString('\n')
	fmt.Println("Hello, ", name)
	fmt.Printf("Type of name is %T\n", name)
}
