package tac

import os._
import upickle.default._
import ujson._
// use graalvm instead of the jvm TODO Look into what this is 
import org.graalvm.nativeimage.c.function.CEntryPoint
import org.graalvm.nativeimage.c.`type`.{CCharPointer, CTypeConversion}
import org.graalvm.nativeimage.IsolateThread

case class jsonAST(NodeType: String, Value: String, Children: List[jsonAST] = Nil) 
object jsonAST 
{
    implicit val rw: ReadWriter[jsonAST] = macroRW
}

object JsonASTReader {
    // def readJsonASTFile(jsonString: String): tac.jsonAST = {
    //     upickle.default.read[jsonAST](jsonString)
    // }
    def readJsonASTFromString(jsonString: String): tac.jsonAST = {
        upickle.default.read[jsonAST](jsonString)
    }

}

// Leave for later debugging 
// object JsonASTReader {

//     // Unit is similar to specifying void for a return value
//     def readJsonASTFile(): tac.jsonAST = 
//     {
//         val astFilePath =  os.Path("../ASTree.json", os.pwd)
//         val jsonString = os.read(astFilePath)
//         val parsedJson = upickle.default.read[jsonAST](jsonString)

//         // println(parsedJson)
//         return parsedJson
//     }
// }
