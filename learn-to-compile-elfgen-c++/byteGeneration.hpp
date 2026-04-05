#ifndef BYTEGENERATION_HPP
#define BYTEGENERATION_HPP

#include <vector> 
#include <string>   
#include <cstdint>  

std::vector<std::vector<std::string>> generateInstructionBytes(std::vector<std::vector<std::string>> lines);

std::vector<uint8_t> convertToByteVector(const std::vector<std::vector<std::string>>& bytestring);

#endif