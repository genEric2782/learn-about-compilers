using System.Text.Json.Serialization;
public class AST
{
    [JsonInclude]
    public List<ASTNode> Children {get; set;} = new(); 
}

public class ASTNode
{
    public string NodeType { get; set; } = "";
    public string Value { get; set; } = "";
    public List<ASTNode> Children { get; set; } = new();
}

public class FlatASTNode
{
    [JsonInclude]
    public string NodeType { get; set; } = "";

    [JsonInclude]
    public string Value { get; set; } = "";

    [JsonInclude]
    public List<int> ChildrenIndices { get; set; } = new();
}