#include <iostream>
#include <unordered_map>
#include "parseAsmFile.hpp"
#include "enumUtils.hpp"
#include <cstdint>
#include <map>
#include <tuple>
#include <iomanip>
#include <bitset>

bool isNumber(std::string str, int& num)
{
    try
    {
        // std::stoi(str);
        num = std::stoi(str);
        return true;
    } 
    catch(...)
    {
        return false;
    } 
}

std::string convertToHexString(uint8_t val)
{
    std::stringstream ss;
    ss << std::hex << std::uppercase << std::setw(2) << std::setfill('0') 
       << static_cast<int>(val);

    return ss.str();
}

std::vector<std::string> convertTo16HexTo8VecString(uint16_t val)
{
    std::vector<std::string> result;
    std::stringstream ss;

    // seperate the bytes 
    uint8_t upperByte = (val >> 8) & 0xFF;
    uint8_t lowerByte = val & 0xFF;

    // for the upper 8 bits 
    ss << std::hex << std::uppercase << std::setw(2) << std::setfill('0') 
       << static_cast<int>(upperByte);
    result.push_back(ss.str());

    // clear out stream 
    ss.str("");
    ss.clear();

    // for the lower 8 bits 
    ss << std::hex << std::uppercase << std::setw(2) << std::setfill('0') 
       << static_cast<int>(lowerByte);
    result.push_back(ss.str());

    return result;
}

int ModR_M_Set(uint8_t mod, uint8_t reg, uint8_t rm)
{
    int mod_r_m = 0x00;

    mod_r_m = mod_r_m | (mod << 6);
    mod_r_m = mod_r_m | (reg << 3);
    mod_r_m = mod_r_m | (rm);

    // std::cout << mod_r_m << std::endl;

    return mod_r_m;
}
// TODO Still need to add padding to move and add? 
std::vector<std::vector<std::string>> generateInstructionBytes(std::vector<std::vector<std::string>> parsedAsmInstructions) 
{
    // Keep for debugging 
    // std::vector<std::vector<std::string>> parsedAsmInstructions = readInAsmFile();
    std::map<int, std::tuple<instructionType, uint8_t>> intruction_type_map;
    std::vector<std::vector<std::string>> bytestring;
    int instruction_val, instruction_count;
    // REX Encoding 0100 WRXB
    // W - when 1 64bit op size is used 
    const uint8_t rexByte = 0x48;
    std::string rexByteString = convertToHexString(rexByte);

    // currently dont care about tags so skipping to the first real instruction  
    for (int i = 4; i < parsedAsmInstructions.size(); i++)
    {
        intruction_type_map.clear();
        int instruction_count = 0;
        for(int j = parsedAsmInstructions[i].size() - 1; j >= 0; j--)
        {
            // Do this backwards so it is known what the operands are for instrcutions like mov 
            // whose opcode changes depending on what it is moving
            std::string instruction = parsedAsmInstructions[i][j];
            // since for now we are going to assume everything is is 64-bit use REX prefixes for opcode 
            if(InstructionEnumUtils::isStringInInstructionsEnum(instruction)) 
            {
                std::cout << "Instruction: " << instruction << std::endl;
                // intruction_type_map.insert({j, instructionType::instruction}); // TODO 

                switch (InstructionEnumUtils::stringToInstructions(instruction)) 
                {      
                    case Instructions::mov:
                        // move opcode changes depending on what is being moved :( 

                        if (intruction_type_map.size() == 2) // mov takes two operands so map should always only be instr op op
                        {
                            // For command mov reg literal 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::literal && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t mov_op_byte = 0xB8; //opcode 
                                // ex rax = 0x00 to to find the opcode for mov rax, ... -> 0xB8 + 0x00 = 0xB8
                                uint8_t mov_op_reg_byte = mov_op_byte + std::get<1>(intruction_type_map[1]);
                                
                                // no Mod value here as moving lit into register 

                                std::string mov_op_reg_byte_string = convertToHexString(mov_op_reg_byte);
                                std::string literal_string = convertToHexString(std::get<1>(intruction_type_map[0]));

                                
                                // vector containing instruction line 
                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(mov_op_reg_byte_string);
                                line.push_back(literal_string);

                                // Padding for string literal to be 8 bytes 
                                int bytes_to_pad = 8 - (literal_string.size() / 2);
                                for (int i = bytes_to_pad; i > 0; i--)
                                {
                                    line.push_back("00");
                                }

                                bytestring.push_back(line);

                                std::cout << "This is the first instruction as Bytes: " << rexByteString << mov_op_reg_byte_string << literal_string << std::endl;

                            }
                            // for mov reg, reg 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::register_name && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t mov_op_byte = 0x89; //opcode , though i guess this can be 8B when mem move into a reg instead of reg to mem
                                std::string mov_op_byte_string = convertToHexString(mov_op_byte);

                                const uint8_t mov_mod_value = 0x03; 

                                // params as follows mod src dest
                                int mod_r_m_reg = ModR_M_Set(mov_mod_value, std::get<1>(intruction_type_map[0]), std::get<1>(intruction_type_map[1]));
                                std::string mod_r_m_reg_string = convertToHexString(mod_r_m_reg);

                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(mov_op_byte_string);
                                line.push_back(mod_r_m_reg_string);
                                bytestring.push_back(line);

                                std::cout << "This is the other mov instruction as Bytes: " << rexByteString << mov_op_byte_string << mod_r_m_reg_string << std::endl;

                            }
                        }

                        break;
                    case Instructions::add:
                        // add opcode changes depending on what is being added :( 
                        if (intruction_type_map.size() == 2) // add takes two operands so map should always only be instr op op
                        {
                            // The add operation also has a mode field which changes depending on whats getting added 
                            // 00 - memory, no displacement 
                            // 01 - mem, 8bit signed displacement
                            // 10 - mem, 32bit signed displacement
                            // 11 - reg direct mode i.e. both operands are registers 

                            // For command add reg reg 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::register_name && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t add_two_reg = 0x01; //opcode
                                std::string add_two_reg_string = convertToHexString(add_two_reg);

                                const uint8_t add_mod_value = 0x03; // register direct mode meaning utilazing to reg instead of liek reg and literal 
                                std::string reg1_string =  convertToHexString(std::get<1>(intruction_type_map[0]));
                                std::string reg2_string =  convertToHexString(std::get<1>(intruction_type_map[1]));
                                // the ModR/M byte is for insturctions and specifies the operanrds for an instruction 
                                int mod_r_m_reg = ModR_M_Set(add_mod_value, std::get<1>(intruction_type_map[0]), std::get<1>(intruction_type_map[1]));
                                std::string mod_r_m_reg_string = convertToHexString(mod_r_m_reg);

                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(add_two_reg_string);
                                line.push_back(mod_r_m_reg_string);
                                bytestring.push_back(line);

                                std::cout << "This is the add instruction as Bytes: " << rexByteString << add_two_reg_string << mod_r_m_reg_string << std::endl;
                                
                            }
                        }
                        break;
                    case Instructions::sub:
                        // add opcode changes depending on what is being added :( 
                        if (intruction_type_map.size() == 2) // add takes two operands so map should always only be instr op op
                        {
                            // The add operation also has a mode field which changes depending on whats getting added 
                            // 00 - memory, no displacement 
                            // 01 - mem, 8bit signed displacement
                            // 10 - mem, 32bit signed displacement
                            // 11 - reg direct mode i.e. both operands are registers 

                            // For command add reg reg 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::register_name && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t sub_two_reg = 0x29; //opcode for subtracting 64bit registers
                                std::string sub_two_reg_string = convertToHexString(sub_two_reg);

                                const uint8_t sub_mod_value = 0x03; // register direct mode meaning utilazing to reg instead of liek reg and literal 
                                std::string reg1_string =  convertToHexString(std::get<1>(intruction_type_map[0]));
                                std::string reg2_string =  convertToHexString(std::get<1>(intruction_type_map[1]));
                                // the ModR/M byte is for insturctions and specifies the operanrds for an instruction 
                                int mod_r_m_reg = ModR_M_Set(sub_mod_value, std::get<1>(intruction_type_map[0]), std::get<1>(intruction_type_map[1]));
                                std::string mod_r_m_reg_string = convertToHexString(mod_r_m_reg);

                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(sub_two_reg_string);
                                line.push_back(mod_r_m_reg_string);
                                bytestring.push_back(line);

                                std::cout << "This is the add instruction as Bytes: " << rexByteString << sub_two_reg_string << mod_r_m_reg_string << std::endl;
                                
                            }
                        }

                        break;
                    // TODO: Multiplication
                    case Instructions::mul:
                        // add opcode changes depending on what is being added :( 
                        if (intruction_type_map.size() == 2) // add takes two operands so map should always only be instr op op
                        {
                            // The add operation also has a mode field which changes depending on whats getting added 
                            // 00 - memory, no displacement 
                            // 01 - mem, 8bit signed displacement
                            // 10 - mem, 32bit signed displacement
                            // 11 - reg direct mode i.e. both operands are registers 

                            // For command add reg reg 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::register_name && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t add_two_reg = 0x01; //opcode
                                std::string add_two_reg_string = convertToHexString(add_two_reg);

                                const uint8_t add_mod_value = 0x03; // register direct mode meaning utilazing to reg instead of liek reg and literal 
                                std::string reg1_string =  convertToHexString(std::get<1>(intruction_type_map[0]));
                                std::string reg2_string =  convertToHexString(std::get<1>(intruction_type_map[1]));
                                // the ModR/M byte is for insturctions and specifies the operanrds for an instruction 
                                int mod_r_m_reg = ModR_M_Set(add_mod_value, std::get<1>(intruction_type_map[0]), std::get<1>(intruction_type_map[1]));
                                std::string mod_r_m_reg_string = convertToHexString(mod_r_m_reg);

                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(add_two_reg_string);
                                line.push_back(mod_r_m_reg_string);
                                bytestring.push_back(line);

                                std::cout << "This is the add instruction as Bytes: " << rexByteString << add_two_reg_string << mod_r_m_reg_string << std::endl;
                                
                            }
                        }

                        break;
                    // TODO: Division
                    case Instructions::div:
                        // add opcode changes depending on what is being added :( 
                        if (intruction_type_map.size() == 2) // add takes two operands so map should always only be instr op op
                        {
                            // The add operation also has a mode field which changes depending on whats getting added 
                            // 00 - memory, no displacement 
                            // 01 - mem, 8bit signed displacement
                            // 10 - mem, 32bit signed displacement
                            // 11 - reg direct mode i.e. both operands are registers 

                            // For command add reg reg 
                            if(std::get<0>(intruction_type_map[0]) == instructionType::register_name && std::get<0>(intruction_type_map[1]) == instructionType::register_name)
                            {
                                const uint8_t add_two_reg = 0x01; //opcode
                                std::string add_two_reg_string = convertToHexString(add_two_reg);

                                const uint8_t add_mod_value = 0x03; // register direct mode meaning utilazing to reg instead of liek reg and literal 
                                std::string reg1_string =  convertToHexString(std::get<1>(intruction_type_map[0]));
                                std::string reg2_string =  convertToHexString(std::get<1>(intruction_type_map[1]));
                                // the ModR/M byte is for insturctions and specifies the operanrds for an instruction 
                                int mod_r_m_reg = ModR_M_Set(add_mod_value, std::get<1>(intruction_type_map[0]), std::get<1>(intruction_type_map[1]));
                                std::string mod_r_m_reg_string = convertToHexString(mod_r_m_reg);

                                std::vector<std::string> line;
                                line.push_back(rexByteString);
                                line.push_back(add_two_reg_string);
                                line.push_back(mod_r_m_reg_string);
                                bytestring.push_back(line);

                                std::cout << "This is the add instruction as Bytes: " << rexByteString << add_two_reg_string << mod_r_m_reg_string << std::endl;
                                
                            }
                        }

                        break;
                    default:
                        std::cout << "The heck" << std::endl;
                        break;
                }
            }
            else if (RegistersEnumUtils::isStringInRegistersEnum(instruction)) 
            {
                std::cout << "Register: " << instruction << std::endl;
                Registers asm_reg = RegistersEnumUtils::stringToRegisters(instruction);

                intruction_type_map.insert({instruction_count, {instructionType::register_name, static_cast<uint8_t>(asm_reg)}});
                instruction_count++;
            } 
            else if (isNumber(instruction, instruction_val)) 
            {
                std::cout << "Number: " << instruction << std::endl;


                intruction_type_map.insert({instruction_count, {instructionType::literal, static_cast<uint64_t>(instruction_val)}});
                instruction_count++;
            }
            else if (KeywordsEnumUtils::isStringInKeywordsEnum(instruction))
            {
                std::cout << "Keyword: " << instruction << std::endl;
                switch(KeywordsEnumUtils::stringToKeywords(instruction)) 
                {
                    case Keywords::syscall:
                    {
                        std::vector<std::string> keyword_string = convertTo16HexTo8VecString(static_cast<uint16_t>(Keywords::syscall));

                        // std::vector<std::string> line;
                        // line.push_back(keyword_string);
                        bytestring.push_back(keyword_string);

                        std::cout << "Keyword Byte: " << static_cast<uint16_t>(Keywords::syscall) << std::endl;
                        break;
                    }
                    default:
                        std::cout << "heck" << std::endl;
                        break;
                }
            }
            else 
            {
                std::cout << "where am I: " << instruction << std::endl;


                intruction_type_map.insert({instruction_count, {instructionType::unkown, 0xFF}});
                instruction_count++;
            }
        }
    }

    return bytestring;
}

std::vector<uint8_t> convertToByteVector(const std::vector<std::vector<std::string>>& bytestring) 
{
    std::vector<uint8_t> byteVector;

    for (const auto& row : bytestring) {
        for (const auto& singlehexStr : row) {
            uint8_t byte = static_cast<uint8_t>(std::stoul(singlehexStr, nullptr, 16));
            byteVector.push_back(byte);
        }
    }

    return byteVector;
}