#pragma once 

#include <unordered_map>
#include <string>
#include <algorithm>

enum class Instructions 
{
    mov,
    add,
    unknown
    // TODO More commands
};

struct InstructionEnumUtils 
{
    inline static const std::unordered_map<std::string, Instructions> instruction_map = 
    {
        {"mov", Instructions::mov},
        {"add", Instructions::add},
        {"unknown", Instructions::unknown}
    };

    static Instructions stringToInstructions(const std::string& str) 
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

enum class Registers 
{
    r15,
    r14,
    r13,
    r12,
    r11,
    r10,
    r9,
    r8,
    rsp,
    rbp,
    rdi,
    rsi,
    rdx,
    rcx,
    rbx,
    rax,
    unknown
};

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

    static Registers stringToRegisters(const std::string& str) 
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