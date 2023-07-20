--[[
Tools for Success:

-- Global Variables
local input = "" 
local output = ""

-- Functions

function inputGet() 
	-- Get the user input 
	input = io.read()
end

function inputCheck() 
	-- Check the user input for validity 
	if input == "" then 
		print("Please enter a valid input")
		inputGet()
	end	
end

function outputPrint() 
	-- Print the output 
	print(output)
end

function convertInput() 
	-- Convert the user input to a valid output 
	output = input 
end

-- Main Loop
while true do 
	inputGet()
	inputCheck()
	convertInput()
	outputPrint()
end

-- Helper Functions
function printHelp() 
	-- Print out instructions for the user 
	print("Please enter input to be converted:")
end

function checkFlag() 
	-- Check if the user has prompted for help 
	if input == "-h" then 
		printHelp() 
		inputGet()
	end
end

-- Main Loop
while true do 
	inputGet()
	checkFlag()
	inputCheck()
	convertInput()
	outputPrint()
end

-- 	Event Listeners

function onInput() 
	-- Listen for user input 
	inputGet()
	checkFlag()
end

-- Main Loop
while true do 
	onInput()
	inputCheck()
	convertInput()
	outputPrint()
end

-- Streamline Conversion

function convertStreamline() 
	-- Streamline the conversion process 
	output = string.gsub(input, ",", "")
end

-- Main Loop
while true do 
	onInput()
	inputCheck()
	convertStreamline()
	outputPrint()
end

-- Error Handling

local errors = {
	["invalid"] = "Please enter a valid input",
	["help"] = "Please enter input to be converted"
}

function errorHandle() 
	-- Check for errors and give a corresponding response 
	if input == "" then 
		print(errors["invalid"]) 
	elseif input == "-h" then 
		print(errors["help"])
	end	
end

-- Main Loop
while true do 
	onInput()
	errorHandle()
	convertStreamline()
	outputPrint()
end

-- Refine Output

function refineOutput() 
	-- Refine and format the output 
	output = "Converted input is: "..output
end

-- Main Loop
while true do 
	onInput()
	errorHandle()
	convertStreamline()
	refineOutput()
	outputPrint()
end

-- Menu Systems

function menu() 
	-- Print a menu to the user 
	print("1. Convert input")
	print("2. Exit")
	inputGet()
end

function optionCheck()
	-- Check for menu option selection
	if input == "1" then 
		-- If 1 is selected, run the main loop 
		onInput()
		errorHandle()
		convertStreamline()
		refineOutput()
		outputPrint()
	else 
		-- Otherwise, exit the program 
		break 
	end	
end

-- Main Loop
while true do 
	menu()
	optionCheck()
end
--]]