{-# LANGUAGE TypeFamilies #-}

module ZigZag where

import Data.Bits (shiftL, shiftR, xor, (.&.))
import Data.Int
import Data.Word

class ZigZag a where
    type Encoded a
    zigZagEncode :: a -> Encoded a
    zigZagDecode :: Encoded a -> a

instance ZigZag Int32 where
    type Encoded Int32 = Word32
    zigZagEncode (n :: Int32) = fromIntegral ((n `shiftL` 1) `xor` (n `shiftR` 31))
    zigZagDecode (n :: Word32) = fromIntegral ((n `shiftR` 1) `xor` negate (n .&. 1))

instance ZigZag Int64 where
    type Encoded Int64 = Word64
    zigZagEncode (n :: Int64) = fromIntegral ((n `shiftL` 1) `xor` (n `shiftR` 63))
    zigZagDecode (n :: Word64) = fromIntegral ((n `shiftR` 1) `xor` negate (n .&. 1))
