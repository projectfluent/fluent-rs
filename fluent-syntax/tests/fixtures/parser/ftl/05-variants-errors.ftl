key1 = {
  [:key] Value
 *[ok] Valid
}

key3 = {
  [] Value
 *[ok] Valid
}

key4 = {
 **[f] Foo
 *[ok] Valid
}

key5 = {
 *fw] Foo
 *[ok] Valid
}

key6 = {
  [ a] A
 *[ok] Valid
}

key7 = {
  [ x/a] XA
 *[ok] Valid
}

key8 = {
  [x y/a] XYA
 *[ok] Valid
}

key10 = {
 [x/a ] XA
 [x/a b ] XAB
 *[ok] Valid
}
