function can be defined with guards:

example:
```
valAbs :: Integer -> Integer
valAbs n
    | n >= 0 = n
    | otherwise = -n
```

this is a function that basically does |num| in math

how it work is, it checks if n is over 0 or not. (aka positive)

if so, it will return n. `otherwise` negate it.

In here, the otherwise keyword is equal to `True`, but this way, it is more readable.

the `=` always go after every guard.