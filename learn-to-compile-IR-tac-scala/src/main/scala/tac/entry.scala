package tac

import org.graalvm.nativeimage.UnmanagedMemory
import org.graalvm.word.WordFactory
import org.graalvm.nativeimage.c.function.CEntryPoint
import org.graalvm.nativeimage.c.`type`.{CCharPointer, CTypeConversion}
import org.graalvm.nativeimage.IsolateThread
import upickle.default._

object Main {
  def main(args: Array[String]): Unit = {
    
  }

  def runPipeline(astJsonString: String): String = {
    println(s"Input JSON: $astJsonString")
    val ASTObject = JsonASTReader.readJsonASTFromString(astJsonString)

    val tacRepresentation = new IRConverter
    tacRepresentation.generateTacInstrFromAST(ASTObject)
    tacRepresentation.writeToTacJson(tacRepresentation.tacInstrList)

    // Serialize the TAC instruction list back to JSON to return to Rust
    upickle.default.write(tacRepresentation.InstrJsonList, indent = 1)
  }


// // FFI entry point 
// @CEntryPoint(name = "process_ast")
// def processAst(
//   thread: IsolateThread,
//   jsonPtr: CCharPointer // Json from Rust 
// ): CCharPointer = { // Json to rust 

//   // Convert to a String Scala can work with 
//   val inputJson = CTypeConversion.toJavaString(jsonPtr)

//   val resultJson = try{
//     runPipeline(inputJson) 
//   } catch {
//     case e: Exception =>
//         // Return a safe error JSON instead of crashing the native library
//         s"""{"error": "${e.getMessage.replace("\"", "'")}"}"""
//   }

//   val bytes = resultJson.getBytes("UTF-8")
//   val ptr: CCharPointer = UnmanagedMemory.malloc[CCharPointer](bytes.length + 1)
//   CTypeConversion.toCString(resultJson, ptr, WordFactory.unsigned(bytes.length + 1))
//   ptr
// }


//   // Rust calls this to free the string returned by process_ast
//   @CEntryPoint(name = "free_result")
//   def freeResult(
//     thread: IsolateThread,
//     ptr:    CCharPointer
//   ): Unit = {
//     if (!ptr.isNull) UnmanagedMemory.free(ptr)
//   }
}

// Leave old code for future debugging 
// object Main {
//   def main(args: Array[String]): Unit = {
//     println("Hello, world!")

//     // When calling code from a class need to create on object using its contructor first
//     // this is becuase classes are non static 
//     // static methods defined in an object as opposed to a class do not need to be instantiated
//     val ASTObject = JsonASTReader.readJsonASTFile()
//     println(ASTObject)

//     var tacRepresentation = new IRConverter
//     val _ = tacRepresentation.generateTacInstrFromAST(ASTObject)
//     // tacRepresentation.printTac(tacRepresentation.tacInstrList)
//     tacRepresentation.writeToTacJson(tacRepresentation.tacInstrList)
//   }
// }