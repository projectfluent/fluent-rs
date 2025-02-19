## Function names

valid-func-name-01 = { FUN1() }
valid-func-name-02 = { FUN_FUN() }
valid-func-name-03 = { FUN-FUN() }

# JUNK 0 is not a valid Identifier start

# JUNK Function names may not be lowercase

# JUNK Function names may not contain lowercase character

# JUNK ? is not a valid Identifier character

## Arguments

positional-args = { FUN(1, "a", msg) }
named-args = { FUN(x: 1, y: "Y") }
dense-named-args = { FUN(x: 1, y: "Y") }
mixed-args = { FUN(1, "a", msg, x: 1, y: "Y") }

# ERROR Positional arg must not follow keyword args

# ERROR Named arguments must be unique

## Whitespace around arguments

sparse-inline-call = { FUN("a", msg, x: 1) }
empty-inline-call = { FUN() }
multiline-call = { FUN("a", msg, x: 1) }
sparse-multiline-call = { FUN("a", msg, x: 1) }
empty-multiline-call = { FUN() }
unindented-arg-number = { FUN(1) }
unindented-arg-string = { FUN("a") }
unindented-arg-msg-ref = { FUN(msg) }
unindented-arg-term-ref = { FUN(-msg) }
unindented-arg-var-ref = { FUN($var) }
unindented-arg-call = { FUN(OTHER()) }
unindented-named-arg = { FUN(x: 1) }
unindented-closing-paren = { FUN(x) }

## Optional trailing comma

one-argument = { FUN(1) }
many-arguments = { FUN(1, 2, 3) }
inline-sparse-args = { FUN(1, 2, 3) }
mulitline-args = { FUN(1, 2) }
mulitline-sparse-args = { FUN(1, 2) }

## Syntax errors for trailing comma

## Whitespace in named arguments

sparse-named-arg = { FUN(x: 1, y: 2, z: 3) }
unindented-colon = { FUN(x: 1) }
unindented-value = { FUN(x: 1) }
