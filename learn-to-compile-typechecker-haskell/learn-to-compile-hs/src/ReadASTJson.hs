-- GHC language pragma, tells compiler to enable specific language extenstions
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BlockArguments #-}

module ReadASTJson where

import qualified Data.ByteString.Lazy as B
import GHC.Data.ShortText (ShortText(contents))
import GHC.Generics (Generic)
import Data.Aeson
import Data.Aeson.Types (Parser)
-- import Data.Map.Internal.Debug (node)
import Data.Bool (Bool)
import Data.Char (isNumber)
import Text.Read (readMaybe)

-- One day ill use the real types instead of onyl strings and then i wont need this :D 
stringToIntSafe :: String -> Maybe Int
stringToIntSafe = readMaybe

data ASTNode = ASTNode
    { nodeType :: String
    , value :: String
    , children :: [ASTNode]
    } deriving (Show, Generic) -- this tells compiler to gernerate two typeclasses for the data type 
    -- show and Generic - Generic typecalss creates a generic structure for the type that allows other libraies to impl functionality 

isOperator :: String -> Bool
-- elem function check if a value exists inside a list our element being o 
-- the backtick ` turn a normal function into infix notation 
isOperator o = o `elem` ["Plus", "Minus", "Multiply", "Divide"]

isNumberType :: String -> Bool
isNumberType n = n `elem` ["Integer"]

typeCheckOperation :: [ASTNode] -> Parser()
typeCheckOperation children
    | null children = fail "Operator node must have children"
    | not (all (\c -> isNumberType (nodeType c) || isOperator (nodeType c) ) children) =
        fail "Children of an operator must be an int or anthor operator"
    | otherwise = return ()

typeCheckNumber :: ASTNode -> Parser()
typeCheckNumber node
    | isNumberType (nodeType node) = do
        case stringToIntSafe (value node) of
            Just n ->
                return ()
            Nothing ->
                fail "Type Reported as Interger but wasnt actually an int" -- wiil make more dynamic i.e. {NumberType} instead of hardcoded Interger 
    | otherwise = fail "Something weird happened here..."

typeCheck :: ASTNode -> Parser()
typeCheck node@(ASTNode nodeType value children)
    | isOperator nodeType =
        typeCheckOperation children
    | isNumberType nodeType =
        typeCheckNumber node
    | otherwise = fail $ "Not a valid Node Type: " ++ nodeType

-- This is giving specifics on how to parse the json 
instance FromJSON ASTNode where
    parseJSON = withObject "ASTNode" $ \obj -> do
        node <- ASTNode
            -- the $ and * basically take the json object ASTNode and access the values within the apply functions 
            -- the * being for the already "unwrapped funtion from the $ call"
            -- .: indicates a required feild 
            <$> obj .: "NodeType"
            <*> obj .: "Value"
            -- .:? optional field witha default value 
            <*> obj .:? "Children" .!= []
        typeCheck node
        return node

-- func name :: arg1 :: output 
readAST :: FilePath -> IO (Either String ASTNode)
-- function logic 
readAST path = do
    contents <- B.readFile path
    return (eitherDecode contents)



-- This is applicative style, using do makes it monadic 
-- -- This is giving specifics on how to parse the json 
-- instance FromJSON ASTNode where
--     parseJSON = withObject "ASTNode" $ \obj -> 
--         ASTNode
--             -- the $ and * basically take the json object ASTNode and access the values within the apply functions 
--             -- the * being for the already "unwrapped funtion from the $ call"
--             -- .: indicates a required feild 
--             <$> obj .: "NodeType"
--             <*> obj .: "Value"
--             -- .:? optional field witha default value 
--             <*> obj .:? "Children" .!= []
