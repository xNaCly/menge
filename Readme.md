# Menge

Menge is a programming language operating solely on sets in the mathematical
sense. It is completly pure, except for the `@print` builtin, which allows for
outgoing IO - otherwise it would be a blackbox. 

See below for an example:

```julia
a := {1, 2}
b := {2, 3}

# basic operations
    # {2}
    intersection := a @intersect b
    # {1,2,3}
    union := a @union b
    # {1, 3}
    diff := (a @diff b) @union (b @diff a)

    subset := a @in b # 1 (true)
    subset := {1} @in b # @E (false)
    subset := {1, 2} @in a # lhs is returned

# predefined sets
    # empty set
    @E 
    # natural numbers
    @N 
    # integers
    @Z
    # rational numbers
    @Q
    # real numbers
    @R

# building sets
    s0 := { 0..19 } # 0..19 = {0,1,2,...,19}
    s1 := { s0..64 } # (0..19)..64 {0,1,2,...,19,...,64}
    s8 := { 0..? } # infinite set, starting from 0
    s9 := { -?..? } # infinite set, starting from -infinity

    # more abstract usecases:
    #
    # use <set builder variable> <- <set> to feed lambda
    s1_excluding_0to10 := { n<-s1 | n @notin 0..10 } # {10..19}
    s3 := { n<-s0 | n @notin 0..10 } # {10..19}
```
