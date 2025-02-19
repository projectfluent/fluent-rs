new-messages =
    { BUILTIN() ->
        [0] Zero
       *[other] { "" }Other
    }
valid-selector-term-attribute =
    { -term.case ->
       *[key] value
    }

# ERROR Term values are not valid selectors

# ERROR CallExpressions on Terms are similar to TermReferences

# ERROR Nested expressions are not valid selectors

# ERROR Select expressions are not valid selectors

empty-variant =
    { $sel ->
       *[key] { "" }
    }
reduced-whitespace =
    { FOO() ->
       *[key] { "" }
    }
nested-select =
    { $sel ->
       *[one]
            { $sel ->
               *[two] Value
            }
    }

# ERROR Missing selector

# ERROR Missing line end after variant list

