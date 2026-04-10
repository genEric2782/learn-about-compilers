package tac // package is like a name space 

import upickle.default._

// TODO Explain all of this 
// Sealed keyword is a special scala keyword esstentially telling the compiler all classes that inhierit from 
// this calls are in the same file helps compiler pattern match 
sealed abstract class TACInstr
object TACInstr
{
    implicit val tacOpRW: ReadWriter[TACOp] = macroRW
    implicit val tacConstRW: ReadWriter[TACConst] = macroRW
    implicit val tacInstrRW: ReadWriter[TACInstr] = ReadWriter.merge(tacConstRW, tacOpRW)
}
// Class definition also servers as the primary constructor
// in scala var (mutable) val (immutable)
case class TACOp(var tacTempValue: String, var op: String, var arg1: String = "", var arg2: String = "") extends TACInstr {}

case class TACConst(var tacTempValue: String, var value: String) extends TACInstr {}

class TACTempGenerator 
{
    private var _count: Int = 0

    def NewTacInstr(): String =
    {
        _count += 1
        return s"t${_count}"
    }
} 

object OPCodes extends Enumeration 
{
    type OPCode = Value 
    val LOAD_CONSTANT, ADD, MINUS, MULTIPLY, DIVIDE = Value 

    // This is essentailly taking my enumaeration and converting it to a string for the json library to properly seraailzie
    // the .bimap is creatinga way to convert between the ReadWriter for the enum and the ReadWriter needed for the string
    implicit val opcodeRW: ReadWriter[OPCodes.Value] =
    readwriter[String].bimap[OPCodes.Value](
        _.toString,
        OPCodes.withName
    )
}

case class InstructionMetaData(var opcode: OPCodes.OPCode, var tacvar: TACInstr)

object InstructionMetaData 
{
    implicit val instructionRw: ReadWriter[InstructionMetaData] = macroRW
}