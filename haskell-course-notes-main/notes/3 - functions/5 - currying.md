all function have one parameter

the ones with more, **actually return a new function**

no need to pass all parameters (partial application)

example:
```
> sumTwo x y = x + y
> trySum = sumTwo 1
> trySum 4
5
> trySum 2
3
```

another example:

`prod 3 5` what happening is:

- `prod :: Int -> Int -> Int`

- `prod :: Int -> (Int -> Int)`

- `(prod 3) :: (Int -> Int)`

- `(prod 3) 5 :: Int`
