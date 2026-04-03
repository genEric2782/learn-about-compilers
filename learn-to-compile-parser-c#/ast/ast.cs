public class AST
{
    public List<ASTNode> Children {get; set;} = new(); 
}

public class ASTNode
{
    public string NodeType { get; set; } = "";
    public string Value { get; set; } = "";
    public List<ASTNode> Children { get; set; } = new();
}