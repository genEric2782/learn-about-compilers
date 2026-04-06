-- module Main (main) where

-- -- import Lib
-- import GHC.Data.ShortText (ShortText(contents))
-- import Data.Binary (decode)
-- import Foreign.C (errnoToIOError)
-- import ReadASTJson

-- main :: IO ()
-- main = do  
--     result <- readASTFromFile "/home/generic/compiler_project/learn-about-compilers/ASTree.json"
--     case result of 
--             Left err -> putStrLn err
--             Right ast -> print ast

module Main where
import Foreign.C.Types (CInt)
import Foreign.C.String (CString)

main :: IO ()
main = return ()
