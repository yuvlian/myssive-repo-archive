module Fraction where

import qualified Data.ByteString as BS
import qualified Data.ByteString.Lazy as BSL
import Data.Binary.Put (runPut, putFloatle, putDoublele)
import Data.Binary.Get (runGet, getFloatle, getDoublele)

class Fraction a where
    fractedEncode :: a -> BS.ByteString
    fractedDecode :: BS.ByteString -> a

instance Fraction Float where
    fractedEncode = BS.concat . BSL.toChunks . runPut . putFloatle
    fractedDecode = runGet getFloatle . BSL.fromChunks . (:[])

instance Fraction Double where
    fractedEncode = BS.concat . BSL.toChunks . runPut . putDoublele
    fractedDecode = runGet getDoublele . BSL.fromChunks . (:[])
