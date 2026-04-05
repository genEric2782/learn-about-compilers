#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

std::vector<std::vector<std::string>> readInAsmFile() {
    std::ifstream file("../output.asm");
    std::string line, instruction;
    std::vector<std::vector<std::string>> parsedAsmInstructions;

    if (file.is_open())
    {
        while(std::getline(file, line)) 
        {
            // std::cout << line << std::endl;
            std::stringstream lineStream(line); // converts string into a stream
            
            std::vector<std::string> instructionsLine;
            while( lineStream >> instruction ) 
            {
                instructionsLine.push_back(instruction);
            }
            parsedAsmInstructions.push_back(instructionsLine);
        }
        file.close();
    } else {
        std::cerr << "Couldnt open file" << std::endl;
    }

    return parsedAsmInstructions;
}



