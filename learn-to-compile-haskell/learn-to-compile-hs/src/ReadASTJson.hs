-- GHC language pragma, tells compiler to enable specific language extenstions
{-# LANGUAGE DeriveGeneric #-}
{-# LANGUAGE OverloadedStrings #-}

module ReadASTJson where

import qualified Data.ByteString.Lazy as B
import GHC.Data.ShortText (ShortText(contents))
import GHC.Generics (Generic)
import Data.Aeson

data ASTNode = ASTNode
    { nodeType :: String
    , value :: String
    , children :: [ASTNode]
    } deriving (Show, Generic) -- thiis tells compiler to gernerate two typeclasses for the data type 
    -- show and Generic - Generic typecalss creates a generic structure for the type that allows other libraies to impl functionality 

-- This is giving specifics on how to parse the json 
instance FromJSON ASTNode where
    parseJSON = withObject "ASTNode" $ \obj -> 
        ASTNode
            -- the $ and * bsiaclly take the json object ASTNode andd acces the values within the apply finctions 
            -- the * being for the already "unwrapped funtion from the $ call"
            -- .: indicates a required feild 
            <$> obj .: "NodeType"
            <*> obj .: "Value"
            -- .:? optional field witha default value 
            <*> obj .:? "Children" .!= []

-- func name :: arg1 :: output 
readAST :: FilePath -> IO (Either String ASTNode)
-- function logic 
readAST path = do
    contents <- B.readFile path
    return (eitherDecode contents)