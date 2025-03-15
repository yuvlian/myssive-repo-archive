functions can be defined with patterns

for example, is the factorial function

```
factorial :: Integer -> Integer
factorial 0 = 1 -- base case, important!
factorial n = n * factorial (n - 1)
```

the evaluation is from top to bottom, returns
result of first match

it is more elegant than if-then-else and have better applications.

# _
_ is anonymous variable, anonymous variable is
variable that we don't care about the value

```
nand :: Bool -> Bool -> Bool
-- If both true, False. Anything else is True,
-- so we just use anonymous variable.
-- As stated before, order is important
-- If we put the _ _ first, all become True.
nand True True = False
nand _ _ = True
```

