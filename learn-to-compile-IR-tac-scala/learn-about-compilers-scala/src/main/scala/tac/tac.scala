package tac // package is like a name space 

abstract class TACInstr

// Class definition also servers as the primary constructor
// in scala var (mutable) val (immutable)
case class TACOp(var tacTempValue: String, var op: String, var arg1: String = "", var arg2: String = "") extends TACInstr
{

}

case class TACConst(var tacTempValue: String, var value: String) extends TACInstr 
{

}

class TACTempGenerator 
{
    private var _count: Int = 0

    def NewTacInstr(): String =
    {
        _count += 1
        return s"t${_count}"
    }
} 