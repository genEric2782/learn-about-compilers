package tac;

import org.graalvm.nativeimage.IsolateThread;
import org.graalvm.nativeimage.c.function.CEntryPoint;
import org.graalvm.nativeimage.c.type.CCharPointer;
import org.graalvm.nativeimage.c.type.CTypeConversion;
import org.graalvm.nativeimage.UnmanagedMemory;
import org.graalvm.word.WordFactory;

public class NativeEntryPoints {

    @CEntryPoint(name = "process_ast")
    public static CCharPointer processAst(
        IsolateThread thread,
        CCharPointer jsonPtr
    ) {
        System.out.println("process_ast called");  // add this
        String inputJson = CTypeConversion.toJavaString(jsonPtr);

        String resultJson;
        try {
            resultJson = Main$.MODULE$.runPipeline(inputJson);
        } catch (Exception e) {
            resultJson = "{\"error\": \"" + e.getMessage().replace("\"", "'") + "\"}";
        }

        byte[] bytes = resultJson.getBytes(java.nio.charset.StandardCharsets.UTF_8);
        CCharPointer ptr = UnmanagedMemory.malloc(WordFactory.unsigned(bytes.length + 1));
        // CTypeConversion.toCString(resultJson, ptr, WordFactory.unsigned(bytes.length + 1));
        for (int i = 0; i < bytes.length; i++) {
            ptr.write(i, bytes[i]);
        }
        ptr.write(bytes.length, (byte) 0); // null terminator
        return ptr;
    }

    // @CEntryPoint(name = "free_result")
    // public static void freeResult(
    //     IsolateThread thread,
    //     CCharPointer ptr
    // ) {
    //     if (ptr.isNonNull()) {
    //         UnmanagedMemory.free(ptr);
    //     }
    // }
}