package tac

object Main {
  def main(args: Array[String]): Unit = {
    println("Hello, world!")

    // When calling code from a class need to create on object using its contructor first
    // this is becuase classes are non static 
    // static methods defined in an object as opposed to a class do not need to be instantiated
    val ASTObject = JsonASTReader.readJsonASTFile()
    println(ASTObject)

    var tacRepresentation = new IRConverter
    val _ = tacRepresentation.generateTacInstrFromAST(ASTObject)
    // tacRepresentation.printTac(tacRepresentation.tacInstrList)
    tacRepresentation.writeToTacJson(tacRepresentation.tacInstrList)
  }
}