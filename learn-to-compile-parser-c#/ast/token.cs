using System.Text.Json.Serialization;
public enum TokenKindEnum
{
    Integer,
    // Uinterger(u64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    Whitespace,
    EOF,
    Bad,
}

public enum Operators
{
    Plus,
    Minus,
    Multiply,
    Divide,
}

public class TokenKind
{
    [JsonInclude]
    public string Type { get; set; }
    [JsonInclude]
    public long? value { get; set; }
}

public class TextSpan
{
    [JsonInclude]
    public int Start { get; set; }
    [JsonInclude]
    public int End { get; set; }
    [JsonInclude]
    public string Literal { get; set; } = "";
}

public class Token
{
    [JsonInclude]
    public TokenKind Kind { get; set; }
    [JsonInclude]
    public TextSpan Span { get; set; } = new();
}
