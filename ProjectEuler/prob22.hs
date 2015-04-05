import Text.ParserCombinators.Parsec
import Control.Applicative hiding ((<|>), many)
import Data.Char
import Data.List

name ::Parser String
name= char '"' *> many letter <* char '"'

names :: Parser [String]
names= do
	x<-name
	try $ do{char ','; y<-names;return $ x:y;} <|> return [x]

calcScore :: Integer->Integer->[String]->Integer
calcScore m b l|m`seq`b`seq`l`seq`False=undefined
calcScore m _ []=m
calcScore m b (x:xs)=calcScore (m+b*score x) (b+1) xs

score ::String->Integer
score ""=0
score (x:xs)=score xs+fromIntegral (ord x-64)

main=do
	ln<-getLine
	let (Right nms)=parse names "***" ln in print (calcScore 0 1 (sort nms))
