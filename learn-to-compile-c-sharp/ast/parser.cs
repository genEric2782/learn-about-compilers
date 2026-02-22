public class Parser
{
    // Should make properties? 
    private readonly List<Token> _tokens;
    private int _position;

    // Constuctor 
    public Parser(List<Token> tokens)
    {
        _tokens = tokens;
        _position = -1; // Start at 0? 
    }

    // is current position is less than the size of the list of tokens get token at value position
    private Token currentToken => _position < _tokens.Count ? _tokens[_position] : null;

    private Token peekNextToken => (_position + 1) < _tokens.Count ? _tokens[_position++] : null;

    private Token Consume()
    {
        // Ignore whitespace token for now if i need it later remoe this and do something else
        if (_tokens[_position++].Kind == TokenKind.Whitesace.ToString())
        {
            return _tokens[_position + 2];
        }
        else
        {
            return _tokens[_position++];
        }
    }



    public ASTNode ParseExpression()
    {

        if (!Enum.IsDefind(typeof(Operators), peekNextToken.Kind))
        {
            var Node1 = createParsedTokenNode();
        }
        else
        {
            // TODO Do i want to consume here ?
            var foo = Consume();
        }
        // Giant if statement? 

        // Current scheme doesnt do order of operations well.... i.e. not at all 
        while (currentToken.Kind != TokenKind.EOF)
        {

            //if (currentToken.NodeType == TokenKind.Integer.ToString()) // gross
            if (Enum.IsDefind(typeof(Operators), peekNextToken.Kind))
            {
                // for now if this currentoken is an int its safe to assume that for a proper expression the next token needs to be an operation 
                //if (peekNextToken.Span.Literal == TokenKind.Integer.ToString())
                //{
                // TODO maybe needs updating ?
                var operate = Consume();

                if (peekNextToken.Span.Literal == TokenKind.Integer.ToString())
                {
                    var Node2 = createParsedTokenNode();
                }
                else
                {
                    // TODO error? 
                }

                // Make sure there insnt another operand in the string before we make this part of the tree
                if (Enum.IsDefind(typeof(Operators), peekNextToken.Kind))
                {
                    // TODO       
                    var something = ParseExpression();


                    // Old
                    // var nextOp = Consume();

                    //var Node3 = createParsedTokenNode();

                    //createParsedTreeNode(nextOp, Node2, Node3);
                }
                else
                {
                    createParsedTreeNode(operate, Node1, Node2);
                }
            }
        }
            // TODO: maybe make Numbers Enum? 
            else if (Enum.IsDefind(typeof(), peekNextToken.Kind))
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
