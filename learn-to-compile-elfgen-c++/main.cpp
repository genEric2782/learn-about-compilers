#include <iostream>
#include "parseAsmFile.hpp"
#include "byteGeneration.hpp"
#include "elf.hpp"
#include <string>
#include <vector>

int main() {
    // std::cout << "Hello, World!" << std::endl;
    std::vector<std::vector<std::string>> parsedAsmFile;
    std::vector<std::vector<std::string>> parsedAsmInstructions;

    parsedAsmFile = readInAsmFile();

    parsedAsmInstructions = generateInstructionBytes(parsedAsmFile);

    std::vector<uint8_t> vectorOfBytes = convertToByteVector(parsedAsmInstructions);

    std::cout << "--------------------------------------" << std::endl;
    elf::write_elf("a.out", vectorOfBytes);


    // sanity check 
    // for (const auto& row : parsedAsmInstructions) {
    //     for (const auto& str : row) {
    //         std::cout << str << " ";
    //     }
    //     std::cout << std::endl;
    // }

    return 0;
}