# in haskell
functions are pure, only return result based off parameters

no side effects because:
- they do not modify paramteers
- they do not modify memory
- they do not modify input /output

function always return same result when same parameters

to make function
1. give type header
2. give definition

function name is camelCase

example:
```
double :: Int -> Int -- this is type header
double x = x * 2 -- this is the definition

-- this type header is read as: 
-- take two ints, return int
perimeter :: Int -> Int -> Int
-- can pass function in function
perimeter width height = double (width + height)

xOr :: Bool -> Bool -> Bool
xOr a b = (a || b) && not (a && b)

addThree :: Int -> Int
addThree n = n + 3

addThreeToInts :: [Int] -> [Int]
addThreeToInts y = map addThree y

-- recursive function
factorial :: Integer -> Integer
factorial 0 = 1
factorial n = n * factorial (n - 1)
```
