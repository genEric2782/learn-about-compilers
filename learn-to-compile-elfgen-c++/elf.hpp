#pragma once 
#include <cstdint>
#include <vector>
#include <string>

namespace elf {
    constexpr uint64_t BASE_ADDR = 0x400000; // constexpr - evaluate at compiletime
    constexpr uint64_t PAGE_SIZE = 0x1000;

    // push - save current alignment settings on stack 
    // 1 - set packing alignment to one byte 
    #pragma pack(push, 1) // Change memory alignment telling the compiler to pack data members together without padding 

    // Elf Header metadata about the file 
    // 
    struct Elf64_Ehdr {
        unsigned char e_ident[16]; // identity block - lets linux know this is an elf file 
        uint16_t e_type; // type of file i.e. executable 
        uint16_t e_machine; // kernerl cpu type i.e. instruction set x86_64
        uint32_t e_version; // always 1 current elf version 
        uint64_t e_entry; // where to start executing code - takes a virtual address 
        uint64_t e_phoff; // offset to where program header start 
        uint64_t e_shoff; // section headers 
        uint32_t e_flags; // Arch specific flags (might not be used for x86)
        uint16_t e_ehsize; // size of elf header 
        uint16_t e_phentsize; // size  of one program header 
        uint16_t e_phnum; // size of number of progam headers 
        uint16_t e_shentsize; // section header field TODO 
        uint16_t e_shnum; // section header field TODO 
        uint16_t e_shstrndx; // section header field TODO
    };

    // ELf program header 
    // used by kernel to load program into memory 
    struct Elf64_Phdr {
        uint32_t p_type; // 1 - to load this segment into memory 
        uint32_t p_flags; // readable plus executable
        uint64_t p_offset; // where in the file the segment starts 
        uint64_t p_vaddr; // virtual address to map this segment - must match e_entry 
        uint64_t p_paddr; // physical address - ignored on modenr linux? 
        uint64_t p_filesz; // number of bytes to to read from a file 
        uint64_t p_memsz; // number of bytes to allocate to memory 
        uint64_t p_align; // alignment (usually 0x1000 = 4KB page)
    };

    // restore previous structure alignment settings for stack (was svaed onto by push aboe )
    #pragma pack(pop)

    void write_elf(const char* filename, std::vector<uint8_t>);
}