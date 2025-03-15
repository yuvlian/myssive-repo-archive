create a \<FileName\>.hs

then u can create a function in that file or something. for exampole lets create `Add.hs`

in there write
```
test :: Integer -> Integer -> Integer
test x y = x + y 
```

and u can load in interpreter by doing
`:load Add.hs`

u can now use `test` function in ghci

then if changed file u can do

`:reload`

this will reload loaded files.

to quit ghci, do `::quit`