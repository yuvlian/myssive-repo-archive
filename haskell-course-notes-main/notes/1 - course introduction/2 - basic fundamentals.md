# expresisons
its kinda like math order
```
> 3 + 2 * 2
7
```

```
> (3 + 2) * 2
10
```

```
-- even is function to check if number is even or not
> even(62)
true
```

```
-- parentheses are not necessary
> even 62
true
```

```
-- negative number must have parentheses
> 1 + (-1)
0
```

```
-- this is rounded because integer / integer
> div 14 4
3
```

```
-- type error
> even "Hi, I am string"
```

# type
can check type of soemthing by doing
```
> :type 'D'
'D' :: Char
```

can also do it with functions
```
> :type even
even :: Integral a => a -> Bool
```

# functione example: factorial
```
-- explicit type
factorial :: Integer -> Integer
-- if calling function with 0 it will return 1
factorial 0 = 1
-- other wise do this, this is recursive function
factorial n = n * factorial (n - 1)
```
```
> factorial 5
120
```
```
-- map does something for each item
-- here is doing factorial
-- 0..5 is 0 1 2 3 4 5
> map factorial [0..5]
[1, 1, 2, 6, 24, 120]