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
        ASTNode Node = createParsedTokenNode();
    
        // Current scheme doesnt do order of operations well.... i.e. not at all 
        while (currentToken.Kind != TokenKind.EOF)
        {
            // Will need additional or better logic when paraens are added for now first token should always be int (error otherwise)
            // which implies the next token will be an operator (error otherwise) 
            if (Enum.IsDefined(typeof(Operators), peekNextToken.Kind))
            {
                Token operate = Consume();

                Node = createParsedASTreeNode(operate, Node);

            } 
            else if (TokenKind.Integer == peekNextToken.Kind)
            {
                ASTNode nextNode = createParsedTokenNode(); 
                // if there is another operator after 
                if(Enum.IsDefined(typeof(Operators), peekNextToken.Kind)) 
                {
                    Token nextOperator = Consume();
                    Node = createParsedASTreeNode(nextOperator, nextNode);
                }
                // No next operator just one last number 
                else if (TokenKind.EOF == peekNextToken.Kind)
                {
                    // TODO: attach single node to tree?
                    ASTNode finalNode = createParsedTokenNode();
                    Node.Children.Add(finalNode);
                } 
                else
                {
                    // TODO IDK 
                    throw new System.Exception("Why Did I Get Here?");
                }
  
            }
            else
            {
                throw new System.Exception("Invalid expression was expecting an Operator");
            }
        }
        // TODO return the AST
        return Node;
    }


    // Creates a Node that will later be added to a tree 
    public ASTNode createParsedTokenNode()
    {
        Token token = Consume();

        return new ASTNode
        {
            NodeType = token.Kind.ToString(),
            Value = token.Span.Literal
        };
    }

    // Creates a node with children 
    public ASTNode createParsedASTreeNode(Token head, ASTNode? LNode, ASTNode? RNode = null)
    {

        // TODO handle precedence in here? 
        // Check for precidence i.e. if the next token is a * or / operator
        if(RNode is null && Enum.IsDefined(typeof(Operators), currentToken.Kind))
        {
            // Check to make sre there is another node first 
            if (TokenKind.EOF != peekNextToken.Kind)
            {
                RNode = createParsedTokenNode();   
                // TODO exponents and paren
                if (peekNextToken.Kind == TokenKind.Multiply || peekNextToken.Kind == TokenKind.Divide)
                {
                    Token precedence = Consume();
                    ASTNode rightNode = createParsedMDTreeNode(precedence, LNode, RNode);

                    return new ASTNode
                    {
                        NodeType = head.Kind.ToString(),
                        Value = head.Span.Literal,
                        Children = new List<ASTNode> { LNode, rightNode }
                    };
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
            // Add final operation i.e .. + 5
            else
            {
                return new ASTNode
                {
                    NodeType = head.Kind.ToString(),
                    Value = head.Span.Literal,
                    Children = new List<ASTNode> { LNode }
                };
            }
            
        }
        // RNode is defined  
        else // this might actually need to be a throw 
        {
            return new ASTNode
            {
                NodeType = head.Kind.ToString(),
                Value = head.Span.Literal,
                Children = new List<ASTNode> { LNode, RNode }
            };
        }

    }

    public ASTNode createParsedMDTreeNode(Token head, ASTNode LNode, ASTNode RNode)
    {

        // TODO handle precedence in here? 
        // Check for precidence i.e. exponenets and paraen

        return new ASTNode
        {
            NodeType = head.Kind.ToString(),
            Value = head.Span.Literal,
            Children = new List<ASTNode> { LNode, RNode }
        };

    }
}
