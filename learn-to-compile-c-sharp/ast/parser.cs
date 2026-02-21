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

    private Token peekNextToken => (_position + 1) < _tokens.Count ? _tokens[_position++] : null;

    private Token Consume()
    {
        return _tokens[_position++];
    }



    public ASTNode ParseExpression()
    {

        var first = createParsedToken();

        // Giant if statement? 

        while (currentToken != null)
        {
            if (currentToken.NodeType == TokenKind.Integer.ToString()) // gross 
            {
                // for now if this currentoken is an int its safe to assume that for a proper expression the next token needs to be an operation 
                if (peekNextToken.Span.Literal != )
                {

                }
            }
            else if ()
            {

            }
            else
            {

            }
        }
    }

    public createParsedToken()
    {
        var token = Consume();




        return new ASTNode
        {
            NodeType = token.kind,
            Value = token.Span.Literal
        };
    }
}
