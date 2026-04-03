package tac

import os._
import upickle.default._
import ujson._

case class jsonAST(NodeType: String, Value: String, Children: List[jsonAST] = Nil) 
object jsonAST 
{
    implicit val rw: ReadWriter[jsonAST] = macroRW
}

object JsonASTReader {

    // Unit is similar to specifying void for a return value
    def readJsonASTFile(): tac.jsonAST = 
    {
        val astFilePath =  os.Path("../../ASTree.json", os.pwd)
        val jsonString = os.read(astFilePath)
        val parsedJson = upickle.default.read[jsonAST](jsonString)

        // println(parsedJson)
        return parsedJson
    }
}
