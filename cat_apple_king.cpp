//Includes
#include <iostream>
#include <string>
#include <fstream>
#include <vector>

//Namespaces
using namespace std;

//Definitions
#define SUCCESS true

//Functions
void printMenu() {
    cout << "Menu" << endl;
    cout << "1) View all tools" << endl;
    cout << "2) Add a tool" << endl;
    cout << "3) Remove a tool" << endl;
    cout << "4) Exit program" << endl;
}

vector<string> readFile() {
    vector<string> tools;

    string line;
    ifstream file("tools.txt");
    
    if (file.is_open()) {
        while (getline(file, line)) {
            tools.push_back(line);
        }
        file.close();
    }
    
    return tools;
}

bool writeFile(vector<string> tools) {
    ofstream file("tools.txt");

    if (file.is_open()) {
        for (string tool : tools) {
            file << tool << endl;
        }
        file.close();
        return SUCCESS;
    }
    return !SUCCESS;
}

void viewTools(vector<string> tools) {
    for (string tool : tools) {
        cout << tool << endl;
    }
    cout << endl;
}

void addTool(vector<string>& tools) {
    string newTool;
    cout << "Enter a tool to be added: ";
    cin >> newTool;

    tools.push_back(newTool);

    cout << "Tool added!" << endl << endl;
}

void removeTool(vector<string>& tools) {
    string removeTool;
    bool found = false;
    cout << "Enter a tool to be removed: ";
    cin >> removeTool;

    for (int i = 0; i < tools.size(); i++) {
        if (tools[i] == removeTool) {
            tools.erase(tools.begin() + i);
            found = true;
            break;
        }
    }

    if (found) {
        cout << "Tool removed!" << endl << endl;
    } else {
        cout << "Tool not found!" << endl << endl;
    }
}

int main() {
    vector<string> tools;
    bool running = true;
    int option;

    tools = readFile();
    
    while (running) {
        printMenu();
        cin >> option;

        switch (option) {
            case 1:
                viewTools(tools);
                break;
            case 2:
                addTool(tools);
                break;
            case 3: 
                removeTool(tools);
                break;
            case 4:
                writeFile(tools);
                running = false;
                break;
            default:
                break;
        }
    }
}