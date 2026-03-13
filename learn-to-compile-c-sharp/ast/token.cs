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
    public string Type { get; set; }
    public long? value { get; set; }
}

public class TextSpan
{
    public int Start { get; set; }
    public int End { get; set; }
    public string Literal { get; set; } = "";
}

public class Token
{
    public TokenKind Kind { get; set; }
    public TextSpan Span { get; set; } = new();
}
