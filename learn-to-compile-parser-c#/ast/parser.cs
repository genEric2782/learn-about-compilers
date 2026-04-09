using System.Runtime.CompilerServices;

public class Parser
{
    // Should make properties? 
    private readonly List<Token> _tokens;
    private int _position;

    // Constuctor 
    public Parser(List<Token> tokens)
    {
        _tokens = tokens;
        _position = -1;       
    }

    // if current position is less than the size of the list of tokens get token at value position
    private Token? currentToken => _position < _tokens.Count ? _tokens[_position] : null;

    // Extra logic to avoid whitespace tokens for now 
    private Token? peekNextToken => (_position + 1) < _tokens.Count ? (_tokens[_position + 1].Kind.Type == TokenKindEnum.Whitespace.ToString() ? _tokens[_position + 2] : _tokens[_position++]) : null;


    private Token Consume()
    {
        // Ignore whitespace token for now
        if (_tokens[_position + 1].Kind.Type == TokenKindEnum.Whitespace.ToString())
        {
            _position = _position + 2;
            return _tokens[_position];
        }
        else
        {
            _position = _position + 1;
            return _tokens[_position];
        }
    }

    public ASTNode ParseExpression()
    {
        // Starting node 
        ASTNode Node = createParsedTokenNode();
    
        while (currentToken.Kind.Type != TokenKindEnum.EOF.ToString())
        {
            // Will need additional or better logic when paraens are added for now first token should always be int (error otherwise)
            // which implies the next token will be an operator (error otherwise) 
            if (Enum.IsDefined(typeof(Operators), peekNextToken?.Kind.Type))
            {
                Token operate = Consume();

                Node = createParsedASTreeNode(operate, Node);

            } 
            else if (TokenKindEnum.Integer.ToString() == peekNextToken?.Kind.Type)
            {
                ASTNode nextNode = createParsedTokenNode(); 
                // if there is another operator after 
                if(Enum.IsDefined(typeof(Operators), peekNextToken?.Kind.Type)) 
                {
                    Token nextOperator = Consume();
                    Node = createParsedASTreeNode(nextOperator, nextNode);
                }
                // No next operator just one last number 
                else if (TokenKindEnum.EOF.ToString() == peekNextToken?.Kind.Type)
                {
                    // TODO: attach single node to tree?
                    ASTNode finalNode = createParsedTokenNode();
                    Node.Children.Add(finalNode);
                } 
                else if (TokenKindEnum.EOF.ToString() == currentToken.Kind.Type)
                {
                    Node.Children.Add(nextNode);
                }
                else
                {
                    // TODO: Create exceptions 
                    throw new System.Exception("Why Did I Get Here?");
                }
  
            }
            else if (TokenKindEnum.EOF.ToString() == peekNextToken?.Kind.Type || TokenKindEnum.EOF.ToString() == currentToken.Kind.Type)
            {
                break;
            }
            else
            {
                throw new System.Exception("Invalid expression was expecting an Operator");
            }
        }
        return Node;
    }


    // Creates a Node that will later be added to a tree 
    public ASTNode createParsedTokenNode()
    {
        Token token = Consume();

        return new ASTNode
        {
            NodeType = token.Kind.Type,
            Value = token.Span.Literal
        };
    }

    // Creates a node with children 
    public ASTNode createParsedASTreeNode(Token head, ASTNode LNode, ASTNode? RNode = null)
    {

        // Check for precidence i.e. if the next token is a * or / operator
        if(RNode is null && Enum.IsDefined(typeof(Operators), currentToken.Kind.Type))
        {
            // Check to make sre there is another node first 
            if (TokenKindEnum.EOF.ToString() != peekNextToken?.Kind.Type)
            {
                ASTNode tmpNode = createParsedTokenNode();   
                // TODO exponents and paren
                if (peekNextToken?.Kind.Type == TokenKindEnum.Multiply.ToString() || peekNextToken?.Kind.Type == TokenKindEnum.Divide.ToString())
                {
                    Token precedence = Consume();

                    RNode = createParsedTokenNode();

                    ASTNode rightNode = createParsedMDTreeNode(precedence, tmpNode, RNode);

                    return new ASTNode
                    {
                        NodeType = head.Kind.Type,
                        Value = head.Span.Literal,
                        Children = new List<ASTNode> { LNode, rightNode }
                    };
                }
                else
                {
                    return new ASTNode
                    {
                        NodeType = head.Kind.Type,
                        Value = head.Span.Literal,
                        Children = new List<ASTNode> { LNode, tmpNode }
                    };
                } 
            } 
            // Add final operation i.e .. + 5
            else
            {
                return new ASTNode
                {
                    NodeType = head.Kind.Type,
                    Value = head.Span.Literal,
                    Children = new List<ASTNode> { LNode }
                };
            }
            
        }
        // RNode is defined  
        else
        {
            return new ASTNode
            {
                NodeType = head.Kind.Type,
                Value = head.Span.Literal,
                Children = new List<ASTNode> { LNode, RNode }
            };
        }

    }

    public ASTNode createParsedMDTreeNode(Token head, ASTNode LNode, ASTNode RNode)
    {
        return new ASTNode
        {
            NodeType = head.Kind.Type,
            Value = head.Span.Literal,
            Children = new List<ASTNode> { LNode, RNode }
        };
    }

    public static List<FlatASTNode> FlattenAST(ASTNode head)
    {
        var flatAST = new List<FlatASTNode>();

        void Visit(ASTNode node)
        {
            int index = flatAST.Count;
            flatAST.Add(new FlatASTNode
            {
                NodeType = node.NodeType,
                Value = node.Value,
                ChildrenIndices = new List<int>()
            });

            foreach (var child in node.Children)
            {
                Visit(child);
                flatAST[index].ChildrenIndices.Add(flatAST.Count - 1); // index of last added child
            }
        }

        Visit(head);
        return flatAST;
    }

}


