public class Parser
{
    // Should make properties? 
    private readonly List<Token> _tokens;
    private int _position;

    // Constuctor 
    public Parser(List<Token> tokens)
    {
        _tokens = tokens;
        _position = 0; // Start at 0?
        Consume();
        // Should i just consume the first token at construction? 
    }

    // is current position is less than the size of the list of tokens get token at value position
    private Token currentToken => _position < _tokens.Count ? _tokens[_position] : null;

    // TODO this wont work with white space fix for later 
    // private Token peekNextToken => (_position + 1) < _tokens.Count ? _tokens[_position++] : null;
    private Token peekNextToken => (_position + 1) < _tokens.Count ? (_tokens[_position++].Kind == TokenKind.Whitespace ? _tokens[_position + 2] : _tokens[_position++]) : null;


    private Token Consume()
    {
        // Ignore whitespace token for now if i need it later remoe this and do something else
        if (_tokens[_position++].Kind.ToString() == TokenKind.Whitespace.ToString())
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

        /*
          if (!Enum.IsDefind(typeof(Operators), peekNextToken.Kind))
          {
              var Node1 = createParsedTokenNode();
          }
          else
          {
              // TODO Do i want to consume here ?
              var foo = Consume();
          }
        */
        // Current scheme doesnt do order of operations well.... i.e. not at all 
        while (currentToken.Kind != TokenKind.EOF)
        {
            // Will need additional or better logic when paraens are added for now first token should always be int (error otherwise)
            // which implies the next token will be an operator (error otherwise) 
            if (Enum.IsDefined(typeof(Operators), peekNextToken.Kind))
            {
                // TODO maybe needs updating ?
                var operate = Consume();

                if (peekNextToken.Kind.ToString() == TokenKind.Integer.ToString())
                {
                    //TODO probalby dont want to create the node yet but also dont want to consume this number....
                    var Node2 = createParsedTokenNode();

                    // Is this where i check for precedence? 
                    if (peekNextToken.Kind == TokenKind.Multiply || peekNextToken.Kind == TokenKind.Divide)
                    {
                        // TODO need to figure out precedence might not want to recurse here?  
                        ParseExpression();
                    }
                    // TODO might need different node add methods for precendence 
                    createParsedTreeNode(operate, Node1, Node2);

                    // TODO return once tree has been built 

                }
                else
                {
                    // TODO create custom exceptions 
                    throw new System.Exception("Invalid expression was expecting an integer");
                }

                // Make sure there insnt another operand in the string before we make this part of the tree
                if (Enum.IsDefined(typeof(Operators), peekNextToken.Kind))
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
                    // TODO 
                    createParsedTreeNode(operate, Node1, Node2);
                }
            }
            else
            {
                throw new System.Exception("Invalid expression was expecting an Operator");
            }
        }
        // TODO return the AST
        return
    }


    // Creates a Node that will later be added to a tree 
    public ASTNode createParsedTokenNode()
    {
        var token = Consume();

        return new ASTNode
        {
            NodeType = token.Kind.ToString(),
            Value = token.Span.Literal
        };
    }

    // Creates a node with children 
    public ASTNode createParsedTreeNode(Token head, ASTNode LNode, ASTNode RNode)
    {

        // TODO handle precedence in here? 
        // CHeck for precidence i.e. if the next token is a * or / operator

        return new ASTNode
        {
            NodeType = head.Kind.ToString(),
            Value = head.Span.Literal,
            Children = new List<ASTNode> { LNode, RNode }
        };
    }
}
