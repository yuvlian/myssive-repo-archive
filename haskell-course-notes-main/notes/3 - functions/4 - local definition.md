u can define local names in expression with `let-in`

example:
```
fastExponent :: Integer -> Integer -> Integer
fastExponent _ 0 = 1
fastExponent x n =
    let y = fastExponent x half_n
        half_n = div n 2
    in
        if even n
        then y * y
        else y * y * x
```

this is equal to
```
fastExponent :: Integer -> Integer -> Integer
fastExponent _ 0 = 1
fastExponent x n =
        if even n
        then (fastExponent x (div n 2)) * (fastExponent x (div n 2))
        else (fastExponent x (div n 2)) * (fastExponent x (div n 2)) * x
```

`where` allows names to be defined in >1 expression

in this example:
```
-- y and half_n is available to all the guards
fastExponent :: Integer -> Integer -> Integer
fastExponent _ 0 = 1
fastExponent x n
    | even n    = y * y
    | otherwise = y * y * x
    where
        y = fastExponent x half_n
        half_n = div n 2

the indentation of `where` defines the scope

