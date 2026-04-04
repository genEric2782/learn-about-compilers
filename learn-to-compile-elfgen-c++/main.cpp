#include <iostream>
#include "parseAsmFile.hpp"
#include "byteGeneration.hpp"
#include <string>
#include <vector>

int main() {
    // std::cout << "Hello, World!" << std::endl;
    std::vector<std::vector<std::string>> parsedAsmInstructions;

    parsedAsmInstructions = readInAsmFile();

    generateInstructionBytes(parsedAsmInstructions);
    // sanity check 
    // for (const auto& row : parsedAsmInstructions) {
    //     for (const auto& str : row) {
    //         std::cout << str << " ";
    //     }
    //     std::cout << std::endl;
    // }

    return 0;
}