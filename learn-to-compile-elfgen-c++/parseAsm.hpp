#pragma once

#ifdef __cplusplus
extern "C" {
#endif

bool compile_asm_to_elf(const char* asm_src, const char* output_path);

#ifdef __cplusplus
}
#endif