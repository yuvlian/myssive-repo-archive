# basic types

booleans: `Bool`

integers: `Int`, `Integer`

reals: `Float`, `Double`

characters: `Char`


# booleans

type: `Bool`

literals: `False` `True`

operations:
```
not :: Bool -> Bool -- negation
(||) :: Bool -> Bool -> Bool -- disjunction
(&&) :: Bool -> Bool -> Bool -- conjunction
```

examples:
```
not True -- will output False
not (not True) -- will output True
not not true -- error, because read as (not not) True

True || False -- will output True
True && False -- will output False
(False || True) && True -- will output True
```

# integers
type: 
- `Int`: integers of 64 bits
- `Integer`: integers (arbitralily long. use this when very very very long number)

reminder: negative number must be in parentheses

literals: `15`, `(-22)`, `9999999999999`

operations: `+`, `-`, `*`, `div`, `mod` (modulus), `rem` (remainder), `^`

relational operatiors : `<, >, >=, <=, ==, /=` (no !=)

examples:
```
3 + 4 * 5 -> 23
(3 + 4) * 5 -> 35
2 ^ 10 -> 1024
3 + 1 /= 4 -> False

div 11 2 -> 5
mod 11 2 -> 1
rem 11 2 -> 1
mod (-11) 2 -> 1
rem (-11) 2 -> -1
```

# Reaals

type: 
- `Float` 32 bit floating point reals
- `Double`: 64 bit

literals: `3.14, 1e-9, -3.0`

e is used for scientific notation in haskell

operations: `+`, `-`, `*`, `/`, `**`

relational operatiors : `<, >, >=, <=, ==, /=` (no !=)

integer to real conversion: `fromIntegral`

real to integer conersion: `round`, `floor`, `ceiling`

round = closest to rounding, if equal distance to higher or lower, it returns the Even one

examples:
```
10.0 / 3.0 -> 3.3333333333333335
2.0 ** 3.0 -> 8.0
fromIntegral 4 -> 4.0

round 3.6 -> 4
round (-3.6) -> -4
map round [3.5, 4.5] -> [4, 4]

ceiling 2.1 -> 3
ceiling 2.4 -> 3
ceiling 2.6 -> 3
ceiling 2.9 -> 3

floor 2.1 -> 2
floor 2.4 -> 2
floor 2.6 -> 2
floor 2.9 -> 2
```