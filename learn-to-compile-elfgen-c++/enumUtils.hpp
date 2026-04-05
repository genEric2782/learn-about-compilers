#pragma once 

#include <unordered_map>
#include <string>
#include <algorithm>
#include <cstdint>

enum class instructionType
{
    instruction,
    register_name, // since register is a reserved keyword
    literal,
    unkown
};

std::ostream& operator<<(std::ostream& os, const instructionType it) {
    return os << static_cast<int>(it);
}

enum class Instructions 
{
    mov,
    add,
    unknown
    // TODO More commands
};

std::ostream& operator<<(std::ostream& os, const Instructions i) {
    return os << static_cast<int>(i);
}

struct InstructionEnumUtils 
{
    inline static const std::unordered_map<std::string, Instructions> instruction_map = 
    {
        {"mov", Instructions::mov},
        {"add", Instructions::add},
        {"unknown", Instructions::unknown}
    };

    static Instructions stringToInstructions(const std::string str) 
    {
        std::string test = str; 
        std::transform(test.begin(), test.end(), test.begin(), ::tolower); // ensuring string is lowoer case for match

        auto iter = instruction_map.find(test);
        return (iter != instruction_map.end()) ? iter->second : Instructions::unknown;
    }

    static bool isStringInInstructionsEnum(const std::string& str) 
    {
        std::string test = str; 
        std::transform(test.begin(), test.end(), test.begin(), ::tolower); // ensuring string is lowoer case for match

        auto iter = instruction_map.find(test);
        return (iter != instruction_map.end()) ? true : false;
    }
};

// std::ostream& operator<<(std::ostream& os, const InstructionEnumUtils ieu) {
//     for (int i : ieu.instruction_map)
//     return os << ieu;
// }

enum class Registers : uint8_t
{
    r15 = 0x0F,
    r14 = 0x0E,
    r13 = 0x0D,
    r12 = 0x0C,
    r11 = 0x0B,
    r10 = 0x0A,
    r9 = 0x09,
    r8 = 0x08,
    rsp = 0x04,
    rbp = 0x05,
    rdi = 0x07,
    rsi = 0x06,
    rdx = 0x02,
    rcx = 0x01,
    rbx = 0x03,
    rax = 0x00,
    unknown = 0xFF
};

std::ostream& operator<<(std::ostream& os, const Registers r) {
    return os << static_cast<int>(r);
}
struct RegistersEnumUtils {
    inline static const std::unordered_map<std::string, Registers> register_map = 
    {
        {"r15", Registers::r15},
        {"r14", Registers::r14},
        {"r13", Registers::r13},
        {"r12", Registers::r12},
        {"r11", Registers::r11},
        {"r10", Registers::r10},
        {"r9",  Registers::r9},
        {"r8",  Registers::r8},
        {"rsp", Registers::rsp},
        {"rbp", Registers::rbp},
        {"rdi", Registers::rdi},
        {"rsi", Registers::rsi},
        {"rdx", Registers::rdx},
        {"rcx", Registers::rcx},
        {"rbx", Registers::rbx},
        {"rax", Registers::rax},
        {"unknown", Registers::unknown}
    };

    static Registers stringToRegisters(const std::string str) 
    {
        std::string test = str;
        if (!test.empty() && test.back() == ',')
        {
            test.pop_back();
        }
        std::transform(test.begin(), test.end(), test.begin(), ::tolower); // ensuring string is lowoer case for match

        auto iter = register_map.find(test);
        return (iter != register_map.end()) ? iter->second : Registers::unknown;
    }

    static bool isStringInRegistersEnum(const std::string& str) 
    {
        std::string test = str; // weird casting shennagins 
        if (!test.empty() && test.back() == ',')
        {
            test.pop_back();
        }
        std::transform(test.begin(), test.end(), test.begin(), ::tolower); // ensuring string is lowoer case for match

        auto iter = register_map.find(test);
        return (iter != register_map.end()) ? true : false;
    }

};