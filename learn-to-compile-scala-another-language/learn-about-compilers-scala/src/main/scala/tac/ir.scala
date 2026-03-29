package tac

import scala.collection.mutable.ListBuffer
import scala.collection.mutable.Stack

class IRConverter 
{
    // TODO Do proper getting and setting for this list
    var tacInstrList = ListBuffer.empty[(TACInstr)]
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
        tacInstr.foreach(instr => instr match {
            case TACConst(tmp, v) => 
                println(s"${tmp} = ${v}")
            case TACOp(tmp, o, v1, v2) =>
                println(s"${tmp} = ${v1} ${o} ${v2}")
            case _ => 
                println("Whoops")
        })
    }
}