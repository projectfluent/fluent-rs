select-expression =
    { $sel ->
       *[a] A
        [b] B
    }
multiline-variant =
    { $sel ->
       *[a]
            AAA
            BBBB
    }
variant-key-number =
    { $sel ->
       *[a] A
        [b] B
    }
select-expression-in-block-value =
    Foo { $sel ->
       *[a] A
        [b] B
    }
select-expression-in-multiline-value =
    Foo
    Bar { $sel ->
       *[a] A
        [b] B
    }
nested-select-expression =
    { $a ->
       *[a]
            { $b ->
               *[b] Foo
            }
    }
selector-external-argument =
    { $bar ->
       *[a] A
    }
selector-number-expression =
    { 1 ->
       *[a] A
    }
selector-string-expression =
    { "bar" ->
       *[a] A
    }
selector-attribute-expression =
    { -bar.baz ->
       *[a] A
    }
