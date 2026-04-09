import sys
import json 

# Leave for future debugging 
# def readAST():
#     try:
#         # When running by hand ../../ASTree.json
#         # When debugging ASTree.json
#         with open('../../ASTree.json', 'r') as tree:
#             return json.load(tree)
#     except FileNotFoundError:   
#         print("Could not find the file specified")
#     except json.JSONDecodeError:
#         print("Failed to evalulate json") 

def walkAST(tree, walkList):

    if tree is None:
        return
    
    walkList.append(tree['Value'])
    
    if tree["Children"]:
        for child in tree['Children']:
            walkAST(child, walkList)
            # Santity check
            print(walkList)

def isInt(num):
    try:
        int(num)
        return True
    except ValueError:
        return False

def evaluateExpression(op, num1, num2):
    if op == '+':
        result = num1 + num2
        return result

def evaluateAST(preFixList):

    iter = 0 
    operators = {'+', '-', '*', '/'}

    # for iter, val in enumerate(preFixList):
    while iter <= len(preFixList):
        if preFixList[iter] in operators:
            op = preFixList[iter] 
            if (iter + 1) <= len(preFixList) and isInt(preFixList[iter + 1]):
                if (iter + 2) <= len(preFixList) and isInt(preFixList[iter + 2]):
                    if op == '+':
                      result = evaluateExpression(op, int(preFixList[iter + 1]), int(preFixList[iter + 2]))
                    del preFixList[iter:iter+3] # plus becuase slce in not inclusive 
                    if len(preFixList) == 0:
                        break
                    iter = 0
                elif (iter + 2) <= len(preFixList) and preFixList[iter + 2] in operators:
                    iter + 2
                    continue
                else: # iter + 2 > len 
                    result = evaluateExpression(op, result, int(preFixList[iter + 1]))
                    del preFixList[iter:iter + 2]
                    if len(preFixList) == 0:
                        break
                    iter = 0
            else:
                iter + 1 
                continue
        else:
            raise Exception("Started with a number currently not supported :D")
    return result 
    

def evaluate_ast_from_json(json_str: str) -> int:
    ast_tree = json.loads(json_str) 
    preFixList = []
    walkAST(ast_tree, preFixList)
    result = evaluateAST(preFixList)
    return result

# For debugging 
# def main(args):

#     # Leave in for debugging 
#     # jsonASTree = readAST()
#     # print(jsonASTree)

#     preFixList = []
#     walkAST(jsonASTree, preFixList)
#     result = evaluateAST(preFixList)
#     print(result)
    

#     # Testing stuff 
#     # print(f"NodeType: {jsonASTree['NodeType']}")
#     return 0

# if __name__ == "__main__":

#     exit_code = main(sys.argv[1:])
#     sys.exit(exit_code)
