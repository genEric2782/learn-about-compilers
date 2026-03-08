public class Parser
{
    // Should make properties? 
    private readonly List<Token> _tokens;
    private int _position;

    // Constuctor 
    public Parser(List<Token> tokens)
    {
        _tokens = tokens;
        _position = 0; // Maybe dont conusme and start a -1 ? 
        // Should i just consume the first token at construction? v
        // Consume();
        
    }

    // is current position is less than the size of the list of tokens get token at value position
    private Token? currentToken => _position < _tokens.Count ? _tokens[_position] : null;

    // private Token peekNextToken => (_position + 1) < _tokens.Count ? _tokens[_position++] : null;
    private Token? peekNextToken => (_position + 1) < _tokens.Count ? (_tokens[_position++].Kind == TokenKind.Whitespace ? _tokens[_position + 2] : _tokens[_position++]) : null;


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
        // Starting node 
        var Node = createParsedTokenNode();
    
        // Current scheme doesnt do order of operations well.... i.e. not at all 
        while (currentToken.Kind != TokenKind.EOF)
        {
            // Will need additional or better logic when paraens are added for now first token should always be int (error otherwise)
            // which implies the next token will be an operator (error otherwise) 
            if (Enum.IsDefined(typeof(Operators), peekNextToken.Kind))
            {
                // TODO maybe needs updating ?
                var operate = Consume();

                createParsedTreeNode(operate, Node);

                if (peekNextToken.Kind == TokenKind.Integer)
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
    // TODO
    public ASTNode createParsedASTreeNode(Token head, ASTNode? LNode, ASTNode? RNode = null)
    {

        // TODO handle precedence in here? 
        // Check for precidence i.e. if the next token is a * or / operator
        if(RNode is null && Enum.IsDefined(typeof(Operators), currentToken.Kind))
        {
            RNode = createParsedTokenNode();

            if (peekNextToken.Kind == TokenKind.Multiply || peekNextToken.Kind == TokenKind.Divide)
            {
                var precedence = Consume();
                createParsedTreeNode(precedence, LNode, RNode);
            }
            else
            {
                return new ASTNode
                {
                    NodeType = head.Kind.ToString(),
                    Value = head.Span.Literal,
                    Children = new List<ASTNode> { LNode, RNode }
                };
            }
        }
        // RNode is defined  
        else
        {
            return new ASTNode
            {
                NodeType = head.Kind.ToString(),
                Value = head.Span.Literal,
                Children = new List<ASTNode> { LNode, RNode }
            };
        }

        // return new ASTNode
        // {
        //     NodeType = head.Kind.ToString(),
        //     Value = head.Span.Literal,
        //     Children = new List<ASTNode> { LNode, RNode }
        // };
    }

// TODO
    public ASTNode createParsedMDTreeNode(Token head, ASTNode? LNode, ASTNode? RNode = null)
    {

        // TODO handle precedence in here? 
        // Check for precidence i.e. if the next token is a * or / operator
        if(RNode is null && Enum.IsDefined(typeof(Operators), currentToken.Kind))
        {
            RNode = createParsedTokenNode();

            if (peekNextToken.Kind == TokenKind.Multiply || peekNextToken.Kind == TokenKind.Divide)
            {
                var precedence = Consume();
                createParsedTreeNode(precedence, LNode, RNode);
            }
            else
            {
                return new ASTNode
                {
                    NodeType = head.Kind.ToString(),
                    Value = head.Span.Literal,
                    Children = new List<ASTNode> { LNode, RNode }
                };
            }
        }
        // RNode is defined  
        else
        {
            return new ASTNode
            {
                NodeType = head.Kind.ToString(),
                Value = head.Span.Literal,
                Children = new List<ASTNode> { LNode, RNode }
            };
        }

        // return new ASTNode
        // {
        //     NodeType = head.Kind.ToString(),
        //     Value = head.Span.Literal,
        //     Children = new List<ASTNode> { LNode, RNode }
        // };
    }
}
