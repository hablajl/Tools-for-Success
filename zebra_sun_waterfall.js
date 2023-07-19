//Object Oriented Programming

//1. Define a class
class ToolsForSuccess { 
  constructor(name, value) { 
    this.name = name;
    this.value = value;
  } 
}

//2.Create function for setting values
ToolsForSuccess.prototype.setName = function (name) {
  this.name = name;
}

ToolsForSuccess.prototype.setValue = function (value) {
  this.value = value;
}

//3.Create function for getting values
ToolsForSuccess.prototype.getName = function () {
  return this.name;
}

ToolsForSuccess.prototype.getValue = function () {
  return this.value;
}

//4.Create function for checking if object is empty
ToolsForSuccess.prototype.isEmpty = function () {
  return (this.name === undefined && this.value === undefined);
}

//5.Create function for adding tools
ToolsForSuccess.prototype.addTool = function (name, value) {
  this[name] = value;
}

//6.Create function for removing tools
ToolsForSuccess.prototype.removeTool = function (name) {
  delete this[name];
}

//7.Create function for printing all the tools
ToolsForSuccess.prototype.printTools = function () {
  for (let key in this) {
    if (this.hasOwnProperty(key)) {
      console.log(`${key}: ${this[key]}`);
    }
  }
} 

//8.Create function for running tests
ToolsForSuccess.prototype.runTest = function (name, value) {
  if (this[name] == value) {
    console.log('Passed!');
  } else {
    console.log('Failed!');
  }
}

//9.Create function for cloning object
ToolsForSuccess.prototype.clone = function () {
  let clone = Object.assign({}, this);
  return clone;
}

//10.Create function for serializing object
ToolsForSuccess.prototype.serialize = function () {
  let serialized = JSON.stringify(this);
  return serialized;
}

//11.Create function for validating object
ToolsForSuccess.prototype.validate = function () {
  if (typeof this.name !== 'string' || typeof this.value !== 'number') {
    return false;
  }
  return true;
}

//12.Create function for finding object
ToolsForSuccess.prototype.find = function (name) {
  if (this[name]) {
    return this[name];
  }
  return false;
}

//13.Create function for editing object
ToolsForSuccess.prototype.edit = function (name, value) {
  this[name] = value;
}

//14.Create function for merging objects
ToolsForSuccess.prototype.merge = function (tools) {
  let merged = Object.assign(this, tools);
  return merged;
}

//15.Create function for sorting object
ToolsForSuccess.prototype.sort = function () {
  let sorted = {};
  let keys = Object.keys(this).sort();
  for (let i = 0; i < keys.length; i++) {
    let key = keys[i];
    sorted[key] = this[key];
  }
  return sorted;
}

//16.Create function for comparing objects
ToolsForSuccess.prototype.compare = function (tools) {
  if (JSON.stringify(this) === JSON.stringify(tools)) {
    return true;
  }
  return false;
}

//17.Create function for deserializing object
ToolsForSuccess.prototype.deserialize = function (serialized) {
  let deserialized = JSON.parse(serialized);
  return deserialized;
}

//18.Create function for counting the number of tools
ToolsForSuccess.prototype.count = function () {
  return Object.keys(this).length;
}

//19.Create function for filtering the tools
ToolsForSuccess.prototype.filter = function (query) {
  let filtered = {};
  for (let key in this) {
    if (key.search(query) !== -1) {
      filtered[key] = this[key];
    }
  }
  return filtered;
}

//20.Create function for transforming the object
ToolsForSuccess.prototype.transform = function (transformFunc) {
  let transformed = {};
  for (let key in this) {
    transformed[key] = transformFunc(this[key]);
  }
  return transformed;
}