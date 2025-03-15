{-# LANGUAGE FlexibleInstances #-}

module VarInt where

import Data.Bits
import qualified Data.ByteString as BS
import qualified Data.ByteString.Builder as BSB
import qualified Data.ByteString.Lazy as BSL
import Data.Word (Word64)
import Data.List (unfoldr)
import Control.Monad (guard)

class VarInt a where
    varIntEncode :: a -> BS.ByteString
    varIntDecode :: BS.ByteString -> Maybe (a, BS.ByteString)
    varIntEncodeMany :: [a] -> BS.ByteString
    varIntDecodeMany :: BS.ByteString -> [a]

    varIntEncodeMany = BS.concat . map varIntEncode
    varIntDecodeMany bs = unfoldr (fmap (\(x, rest) -> (x, rest)) . varIntDecode) bs

instance VarInt Word64 where
    varIntEncode n = BSL.toStrict $ BSB.toLazyByteString (enc n)
      where
        enc :: Word64 -> BSB.Builder
        enc 0 = BSB.word8 0
        enc v
            | v < 0x80  = BSB.word8 (fromIntegral v)
            | otherwise = BSB.word8 (fromIntegral (v .&. 0x7F) .|. 0x80) <> enc (v `shiftR` 7)

    varIntDecode bs = do
        guard (not $ BS.null bs)
        let (result, rest) = dec 0 0 bs
        Just (result, rest)
      where
        dec :: Word64 -> Int -> BS.ByteString -> (Word64, BS.ByteString)
        dec acc shiftAmt input
          | BS.null input = (acc, BS.empty)
          | otherwise =
              let byte = BS.head input
                  rest = BS.tail input
                  acc' = acc .|. (fromIntegral (byte .&. 0x7F) `shiftL` shiftAmt)
              in if byte .&. 0x80 == 0
                then (acc', rest)
                else dec acc' (shiftAmt + 7) rest
