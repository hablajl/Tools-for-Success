-- Variable declaration
local tools = {};

-- Utility functions
local function checkValue(value, expectedType, message)
	if type(value) ~= expectedType then
		error(message);
	end
end

local function createMessage(message, ...)
	return string.format(message, ...);
end

-- Tools

-- Create a new tool
-- @param name string representing the tool's name
-- @param handler function that handles the tool's execution
-- @return tool object
function tools.create(name, handler)
	checkValue(name, "string", createMessage("Expected %s value for parameter 'name', got %s", "string", type(name)));
	checkValue(handler, "function", createMessage("Expected %s value for parameter 'handler', got %s", "function", type(handler)));

	return {
		name = name,
		handler = handler
	};
end

-- Run a tool
-- @param tool object representing the tool
-- @param ... variable list of parameters to pass to the tool
-- @return any value that is returned in the tool's execution
function tools.run(tool, ...)
	checkValue(tool, "table", createMessage("Expected %s value for parameter 'tool', got %s", "table", type(tool)));
	
	return tool.handler(...);
end

-- List all tools that have been created
-- @return array of tool objects
function tools.list()
	local toolsList = {};

	for key, tool in pairs(tools) do
		if type(tool) == "table" and type(tool.name) == "string" then
			table.insert(toolsList, tool);
		end
	end

	return toolsList;
end

-- Remove a tool
-- @param tool object representing the tool
function tools.remove(tool)
	checkValue(tool, "table", createMessage("Expected %s value for parameter 'tool', got %s", "table", type(tool)));
	
	tools[tool.name] = nil;
end

-- Add a tool
-- @param tool object representing the tool
function tools.add(tool)
	checkValue(tool, "table", createMessage("Expected %s value for parameter 'tool', got %s", "table", type(tool)));

	tools[tool.name] = tool;
end

return tools;