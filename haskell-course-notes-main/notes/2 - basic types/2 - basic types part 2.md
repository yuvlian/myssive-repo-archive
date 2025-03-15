# charactres
type: `Char`

literals: `'a', 'A', '\n'`

relational operatos: `<, >, <=, >=, ==, /=`

conversion functions: (need to import `Data.Char`)
- ord :: Char -> Int (example: `ord 'A'` will return `65`)
- chr :: Int -> Char (example: `chr 65` will return `'A'`)

# predefind function s
is even/odd
```
even :: Integral a => a -> Bool
odd :: Integral a => a -> Bool
```

min/max of two values:

Ord position here means the a element must belong to the Ord type class (order)
```
min :: Ord a => a -> a -> a
max :: Ord a => a -> a -> a
```

greatest common divisor, least common multiple:
```
gcd :: Integral a => a -> a -> a
lch :: Integral a => a -> a -> a
```

mathematicals:
```
-- absolute
abs :: Num a => a -> a
sqrt :: Floating a => a -> a
log :: Floating a => a -> a
exp :: Floating a => a -> a
cos :: Floating a => a -> a
```