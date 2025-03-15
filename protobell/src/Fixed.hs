module Fixed where

import Data.Bits (shiftL, shiftR, (.&.))
import Data.Int
import Data.Word
import qualified Data.ByteString as BS

class Fixed a where
    fixedEncode :: a -> BS.ByteString
    fixedDecode :: BS.ByteString -> a

instance Fixed Word32 where
    fixedEncode n = BS.pack
        [ fromIntegral (n .&. 0xFF)
        , fromIntegral ((n `shiftR` 8) .&. 0xFF)
        , fromIntegral ((n `shiftR` 16) .&. 0xFF)
        , fromIntegral ((n `shiftR` 24) .&. 0xFF)
        ]
    
    fixedDecode bs = case BS.unpack bs of
        [b1, b2, b3, b4] -> 
            fromIntegral b1
            + (fromIntegral b2 `shiftL` 8)
            + (fromIntegral b3 `shiftL` 16)
            + (fromIntegral b4 `shiftL` 24)
        _ -> error "Invalid ByteString length for Word32"

instance Fixed Word64 where
    fixedEncode n = BS.pack
        [ fromIntegral (n .&. 0xFF)
        , fromIntegral ((n `shiftR` 8) .&. 0xFF)
        , fromIntegral ((n `shiftR` 16) .&. 0xFF)
        , fromIntegral ((n `shiftR` 24) .&. 0xFF)
        , fromIntegral ((n `shiftR` 32) .&. 0xFF)
        , fromIntegral ((n `shiftR` 40) .&. 0xFF)
        , fromIntegral ((n `shiftR` 48) .&. 0xFF)
        , fromIntegral ((n `shiftR` 56) .&. 0xFF)
        ]
    
    fixedDecode bs = case BS.unpack bs of
        [b1, b2, b3, b4, b5, b6, b7, b8] -> 
            fromIntegral b1
            + (fromIntegral b2 `shiftL` 8)
            + (fromIntegral b3 `shiftL` 16)
            + (fromIntegral b4 `shiftL` 24)
            + (fromIntegral b5 `shiftL` 32)
            + (fromIntegral b6 `shiftL` 40)
            + (fromIntegral b7 `shiftL` 48)
            + (fromIntegral b8 `shiftL` 56)
        _ -> error "Invalid ByteString length for Word64"

instance Fixed Int32 where
    fixedEncode n = fixedEncode (fromIntegral n :: Word32)
    fixedDecode bs = fromIntegral (fixedDecode bs :: Word32)

instance Fixed Int64 where
    fixedEncode n = fixedEncode (fromIntegral n :: Word64)
    fixedDecode bs = fromIntegral (fixedDecode bs :: Word64)
