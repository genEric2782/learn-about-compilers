public class Parser
{
    // Should make properties? 
    private readonly List<Token> _tokens;
    private int _position;

    // Constuctor 
    public Parser(List<Token> tokens)
    {
        _tokens = tokens;
        _position = 0;
    }

    // is current position is less than the size of the list of tokens get token at value position
    private Token currentToken => _position < _tokens.Count ? _tokens[_position] : null;

    private Token Consume()
    {
        return _tokens[_position++];
    }

    public ASTNode ParseExpression()
    {

        var left = createParsedToken();

        while (currentToken != null && currentToken.Type)
        {

        }
    }

    public createParsedToken()
    {
        var token = Consume();

        return new ASTNode
        {
            NodeType = token.kind,
            Value = toekn.value
        };
    }
}
