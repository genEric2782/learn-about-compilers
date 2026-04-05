#include "elf.hpp"
#include <fstream>
#include <vector>
#include <cstring>

namespace elf {


    void write_elf(const char* filename, std::vector<uint8_t> code) 
    {

        constexpr uint64_t text_offset = PAGE_SIZE;
        constexpr uint64_t text_vaddr = BASE_ADDR + PAGE_SIZE;

        // Setting up elf values 
        Elf64_Ehdr ehdr{};
        // 0x7f - non printable ascii char, chosen so that the file is not seen as a text file 
        // elf converts t o- 45 4c 46 - the elf magic bytes 
        memcpy(ehdr.e_ident, "\x7f""ELF", 4);
        // TODO Look up the byte vaules and what they are acutally setting 
        ehdr.e_ident[4] = 2;
        ehdr.e_ident[5] = 1;
        ehdr.e_ident[6] = 1;

        ehdr.e_type = 2;
        ehdr.e_machine = 0x3E;
        ehdr.e_version = 1;
        ehdr.e_entry = text_vaddr;
        ehdr.e_phoff = sizeof(Elf64_Ehdr);
        ehdr.e_ehsize = sizeof(Elf64_Ehdr);
        ehdr.e_phentsize = sizeof(Elf64_Phdr);
        ehdr.e_phnum = 1;

        Elf64_Phdr phdr{};
        phdr.p_type = 1;
        phdr.p_flags = 5;
        phdr.p_offset = text_offset;
        phdr.p_vaddr = text_vaddr;
        phdr.p_paddr = text_vaddr;
        phdr.p_filesz = code.size();
        phdr.p_memsz = code.size();
        phdr.p_align = PAGE_SIZE;

        // creating the elf file
        std::ofstream out(filename, std::ios::binary);

        out.write((char*)&ehdr, sizeof(ehdr));
        out.write((char*)&phdr, sizeof(phdr));

        std::vector<char> padding(text_offset - sizeof(ehdr) - sizeof(phdr), 0);
        out.write(padding.data(), padding.size());

        out.write((char*)code.data(), code.size());
    }
}