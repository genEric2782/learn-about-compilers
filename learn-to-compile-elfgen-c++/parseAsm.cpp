#include "parseAsm.hpp"
#include "elf.hpp"
#include <sstream>
#include <string>
#include <vector>
#include "byteGeneration.hpp"

// Replaces readInAsmFile() — parses from a string instead of a file
static std::vector<std::vector<std::string>> parseAsmString(const char* asm_src) {
    std::vector<std::vector<std::string>> parsedAsmInstructions;
    std::istringstream stream(asm_src);
    std::string line, asm_instruction;

    while (std::getline(stream, line)) {

        std::vector<std::string> asm_instructions;
        std::istringstream lineStream(line);
        while (lineStream >> asm_instruction)
            asm_instructions.push_back(asm_instruction);

        parsedAsmInstructions.push_back(asm_instructions);
    }

    return parsedAsmInstructions;
}

extern "C" {

bool compile_asm_to_elf(const char* asm_src, const char* output_path) {
    if (!asm_src || !output_path) return false;

    try {
        // Parse the raw assembly string into tokens
        std::vector<std::vector<std::string>> lines = parseAsmString(asm_src);

        // Generate machine code bytes from the parsed instructions
        std::vector<std::vector<std::string>> bytestring = generateInstructionBytes(lines);

        // Flatten into a raw byte vector
        std::vector<uint8_t> code = convertToByteVector(bytestring);

        // Write the ELF binary
        elf::write_elf(output_path, code);

        return true;
    } catch (...) {
        return false;
    }
}

} // extern "C"