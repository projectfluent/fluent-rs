-brand-short-name = Firefox
    .gender = masculine

key = Test { -brand-short-name }

key2 = Test { -brand-short-name.gender ->
    [masculine] Foo
   *[feminine] Foo 2
 }

key3 = Test { -brand[one] }
