-- GHC language pragma, tells compiler to enable specific language extenstions
{-# LANGUAGE ForeignFunctionInterface #-}
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BlockArguments #-}
{-# LANGUAGE ScopedTypeVariables #-}

module ReadAST where

import Foreign.C.Types
import Foreign.C.String (CString, peekCString)
import qualified Data.ByteString.Lazy.Char8 as Bc
import GHC.Data.ShortText (ShortText(contents))
import GHC.Generics (Generic)
import Data.Aeson
import Data.Aeson.Types (Parser)
-- import Data.Map.Internal.Debug (node)
import Data.Bool (Bool)
import Data.Char (isNumber)
import Text.Read (readMaybe)
import qualified Data.ByteString as B
import Text.XHtml (input)
import Foreign.C (CString)
import Control.Exception (try, SomeException)

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
-- exposes this method for rust to call via ffi
foreign export ccall readAST :: CString -> IO CInt 
readAST :: CString -> IO CInt
-- function logic 
readAST cstr = do 
    result <- try $ do 
        input <- peekCString cstr -- since CString is a pointer to a C-style string we call this method to convert it to  a String haskell can work with
        let jsonAsByte = Bc.pack input -- this cshould conver the string to a byte string that can be parsed 
        case decode jsonAsByte :: Maybe ASTNode of -- decode ast 
            Just node -> do -- Valid json 
                print node -- print the node for debuggin (dont think this will do anything )
                -- hFlush stdout -- this might be needed to see the stdout when calling from rust 
                return True -- valid tree = return 
            Nothing -> return False -- Invalid json 
    case result of
        Left (_ :: SomeException) -> return 0 -- SomeException catch all , _ wildcard 
        Right val ->  return (if val then 1 else 0) 

