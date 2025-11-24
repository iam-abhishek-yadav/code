package main

import "fmt"

// constants are like variables but they are immutable
const LoginToken string = "gibberish"

func main() {
	var username string = "abhishek"
	fmt.Println("username: ", username)
	fmt.Printf("username is of type: %T \n", username)

	var isLoggedIn bool = true
	fmt.Println("isLoggedIn: ", isLoggedIn)
	fmt.Printf("isLoggedIn is of type: %T \n", isLoggedIn)

	var smallVal uint8 = 255
	fmt.Println("smallVal: ", smallVal)
	fmt.Printf("smallVal is of type: %T \n", smallVal)

	var smallFloat float32 = 255.45544511254451885
	fmt.Println("smallFloat: ", smallFloat) // 255.45544
	fmt.Printf("smallFloat is of type: %T \n", smallFloat)

	var smallDouble float64 = 255.45544511254451885
	fmt.Println("smallDouble: ", smallDouble) // 255.4554451125445
	fmt.Printf("smallDouble is of type: %T \n", smallDouble)

	// default values and some aliases
	var anotherVariable int
	fmt.Println("anotherVariable: ", anotherVariable)
	fmt.Printf("anotherVariable is of type: %T \n", anotherVariable)

	// implicit type
	var website = "https://www.google.com"
	fmt.Println("website: ", website)
	fmt.Printf("website is of type: %T \n", website)

	// no var style
	numberOfUsers := 300000
	fmt.Println("numberOfUsers: ", numberOfUsers)
	fmt.Printf("numberOfUsers is of type: %T \n", numberOfUsers)

	fmt.Println("LoginToken: ", LoginToken)
	fmt.Printf("LoginToken is of type: %T \n", LoginToken)
}
