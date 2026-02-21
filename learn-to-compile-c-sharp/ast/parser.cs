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

        var Node1 = createParsedTokenNode();

        // Giant if statement? 

        while (currentToken != null)
        {
            //if (currentToken.NodeType == TokenKind.Integer.ToString()) // gross
            if (Enum.IsDefind(typeof(Operators), currentToken.Kind))
            {
                // for now if this currentoken is an int its safe to assume that for a proper expression the next token needs to be an operation 
                if (peekNextToken.Span.Literal == TokenKind.Integer.ToString())
                {
                    var operate = Consume();

                    var Node2 = createParsedTokenNode();

                    createParsedTreeNode(operate, Node1, Node2);
                }
                else if ()
                {

                }// Another operator or paraen
            }
            else if ()
            {

            }
            else
            {

            }
        }
    }

    // Creates a Node that will later be added to a tree 
    public createParsedTokenNode()
    {
        var token = Consume();

        return new ASTNode
        {
            NodeType = token.kind,
            Value = token.Span.Literal
        };
    }

    // Creates a node with children 
    public createParsedTreeNode(Token head, ASTNode LNode, ASTNode RNode)
    {

        return new ASTNode
        {
            NodeType = head.kind,
            Value = head.Span.Literal,
            Children = new List<ASTNode> { LNode, RNode }
        };
    }
}
