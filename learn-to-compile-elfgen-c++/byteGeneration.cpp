#include <iostream>
#include <unordered_map>
#include "parseAsmFile.hpp"
#include "enumUtils.hpp"

void generateInstructionBytes(std::vector<std::vector<std::string>> lines) 
{
    std::vector<std::vector<std::string>> parsedAsmInstructions = readInAsmFile();

    // currently dont care about tags so skipping to the first real instruction  
    for (int i = 4; i < parsedAsmInstructions.size(); i++)
    {
        for(int j = 0; j < parsedAsmInstructions[i].size(); j++)
        {
            std::string instruction = parsedAsmInstructions[i][j];
            // since for now we are going to assume everything is is 64-bit use REX prefixes for opcode 
            if(InstructionEnumUtils::isStringInInstructionsEnum(instruction)) 
            {
                std::cout << "Instruction: " << instruction << std::endl;
            }
            else if (RegistersEnumUtils::isStringInRegistersEnum(instruction)) 
            {
                std::cout << "Regoster: " << instruction << std::endl;
            } 
            else 
            {
                std::cout << "where am I: " << instruction << std::endl;
            }
        }
    }
}