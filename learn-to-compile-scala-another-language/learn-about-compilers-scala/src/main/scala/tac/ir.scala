package tac

import scala.collection.mutable.ListBuffer

class IRConverter 
{
    private var tacInstrList = ListBuffer.empty[(TACTempGenerator, TACInstr)]
    private var  TacTemp = new TACTempGenerator()

    def generateTacInstrFromAST(ast: jsonAST): Unit = 
    {

    }
}