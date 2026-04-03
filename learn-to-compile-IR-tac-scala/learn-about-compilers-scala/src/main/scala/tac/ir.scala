package tac

import scala.collection.mutable.ListBuffer
import scala.collection.mutable.Stack
import os._
import upickle.default._

class IRConverter 
{
    // TODO Do proper getting and setting for this list
    val outputFilePath = os.Path("../../TacJson.json", os.pwd)
    var tacInstrList = ListBuffer.empty[(TACInstr)]
    var InstrJsonList = ListBuffer.empty[(InstructionMetaData)]
    private var tmpFortmp = Stack[String]()
    private var TacTemp = new TACTempGenerator()
    val operators = List("Plus", "Minus", "Multiply", "Divide")

    def generateTacInstrFromAST(ast: jsonAST): String = 
    {
        ast.NodeType match 
        {
            case nt if operators.contains(nt) =>
                println("")
                var iter: Int = 0
                while (!ast.Children.isEmpty && iter < ast.Children.length)
                {
                    var tmp: String = generateTacInstrFromAST(ast.Children(iter))
                    iter += 1 
                    tmpFortmp.push(tmp)
                }
                val t: String = TacTemp.NewTacInstr()
                val v2: String = tmpFortmp.pop()
                val v1: String = tmpFortmp.pop()

                tacInstrList.addOne(TACOp(t, ast.Value, v1, v2))
                return t

            case "Integer" =>
                val t: String = TacTemp.NewTacInstr()
                tacInstrList.addOne(TACConst(t, ast.Value))
                return t
            case _ => 
                throw new Exception("Don't think im ever suppose to get here....")
        }
    }

    def printTac(tacInstr: ListBuffer[TACInstr]): Unit = 
    {
        // TODO this should make a new file every time it runs not append to the old one but also this shouldnt be writing to a file so who cares
        tacInstr.foreach(instr => instr match {
            case TACConst(tmp, v) => 
                println(s"${tmp} = ${v}")
                os.write.append(outputFilePath, s"${tmp} = ${v}\n")
            case TACOp(tmp, o, v1, v2) =>
                println(s"${tmp} = ${v1} ${o} ${v2}")
                os.write.append(outputFilePath, s"${tmp} = ${v1} ${o} ${v2}\n")
            case _ => 
                println("Whoops")
        })
    }

    def writeToTacJson(tacInstr: ListBuffer[TACInstr]): Unit = 
    {
        // TODO Make Formatted json 
        tacInstr.foreach(instr => instr match {
            case TACConst(tmp, v) => 
                val obj = InstructionMetaData(OPCodes.LOAD_CONSTANT, instr)
                // val jsonString: String = upickle.default.write(obj, indent = 4)
                InstrJsonList.addOne(obj)
                // println(jsonString)
                // os.write.append(outputFilePath, s"${jsonString}\n")
            case TACOp(tmp, o, v1, v2) =>
                val obj = InstructionMetaData(OPCodes.ADD, instr)
                // val jsonString: String = upickle.default.write(obj, indent = 4)
                InstrJsonList.addOne(obj)
                // println(jsonString)
                // os.write.append(outputFilePath, s"${jsonString}\n")
            case _ => 
                println("Whoops")
        })

        val jsonList = upickle.default.write(InstrJsonList, indent = 1)
        os.write.append(outputFilePath, s"${jsonList}\n")
        // println(jsonList)
    }
}