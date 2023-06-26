package main

import (
	"fmt"
)

// ToolsForSuccess is an example of code written in Go
// to illustrate the use of tools for success.
func ToolsForSuccess() {
	// Set up some variables for our tools
	var energyLevel int
	var enthusiasmLevel int
	var knowledgeLevel int
	var resiliencyLevel int
	var timeManagementLevel int
	var skillsLevel int
	var determinationLevel int
	
	// Set energy level to 10
	energyLevel = 10

	// Set enthusiasm level to 8
	enthusiasmLevel = 8

	// Set knowledge level to 10
	knowledgeLevel = 10

	// Set resiliency level to 7
	resiliencyLevel = 7

	// Set time management level to 9
	timeManagementLevel = 9

	// Set skills level to 10
	skillsLevel = 10

	// Set determination level to 8
	determinationLevel = 8

	// Print out the following message to the console
	fmt.Println("The combination of energy, enthusiasm, knowledge, resiliency, time management, skills, and determination are essential tools for success.")

	// Calculate a total level of success based on the various tools
	totalLevel := energyLevel + enthusiasmLevel + knowledgeLevel + resiliencyLevel + timeManagementLevel + skillsLevel + determinationLevel

	// Print out the total level of success
	fmt.Printf("The total level of success is %d\n", totalLevel)
}

func main() {
	// Call the function which will execute the code
	ToolsForSuccess()
}