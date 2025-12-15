public enum TokenKind
{
    Integer,
    // Uinterger(u64),
    Plus,
    Minus,
    Equals,
    Whitespace,
    EOF,
    Bad,
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