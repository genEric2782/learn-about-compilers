using System;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Diagnostics;
using System.IO;
using System.Threading.Tasks;
using System.Diagnostics.CodeAnalysis;
using System.Text.Json.Serialization.Metadata;
using System.Collections.Generic;
using System.Text;
using System.Text.Json.Serialization;  

// This is needed for the json serializer it tell the compiler exactly what is getting serialized and deserialzed 
// this helps get rid of reflection at runtime, which is what normally allows C# to inspect metadata about itself at runtime
// Since the lib is AOT this needs to be done during compile time 
[JsonSerializable(typeof(List<Token>))]   
[JsonSerializable(typeof(List<FlatASTNode>))]
[JsonSourceGenerationOptions(
    WriteIndented = true,
    PropertyNameCaseInsensitive = true,
    AllowTrailingCommas = true)]
internal partial class AstJsonContext : JsonSerializerContext { }

// In order to allow rust to run this C# code without kicking off a process 
// rust needs a .dll (.so) is can call the C# code from 
// Specifically in need a NativeAOT (Ahead of time) lib instead of C# normal JIT compiling
// This gets tricky because AOT and recursion (like the revustive json ASTTree object) dont play well together 
// All methos communicating with the C ABI (Application Binary Interface) need to be static 
// C doesnt have object instances so it expects to be handed a pointer to the direct function 
public static class Exports
{
    // This decorator exposes (as a raw function pointer) these methods from the .so to other code in this case rust
    // Allows the rust code the unmanged caller to use it
    [UnmanagedCallersOnly(EntryPoint = "parse_to_json")]
    // IntPtr is a pointer to unmanaged memeory (used for Rust and C# to communicate via C)
    public static IntPtr ParseToJson(IntPtr input)
    {
        // Marchalling the ptr to a C# string 
        string code = Marshal.PtrToStringAnsi(input);

        var tokens = JsonSerializer.Deserialize<List<Token>>(code, AstJsonContext.Default.ListToken);


        var parser = new Parser(tokens);
        var ast = parser.ParseExpression();
        // Becuase our normal ast object is recusrive and thus normally gets figured out dynamically during runtime 
        // we need to flatten the ast into a known object before we can Serialize it and give it back to rust
        var flat_ast = Parser.FlattenAST(ast);

        // await JsonSerializer.SerializeAsync(Console.OpenStandardOutput(), ast, options); // Dont want to do this 
        string ast_json = JsonSerializer.Serialize(flat_ast, AstJsonContext.Default.ListFlatASTNode);

        return Marshal.StringToHGlobalAnsi(ast_json);
    }

    // Clean up the memory used to communicate 
    [UnmanagedCallersOnly(EntryPoint = "free_string")]
    public static void FreeString(IntPtr ptr)
    {
        Marshal.FreeHGlobal(ptr);
    }
}