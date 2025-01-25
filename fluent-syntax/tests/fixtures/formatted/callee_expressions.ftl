## Callees in placeables.

function-callee-placeable = { FUNCTION() }
term-callee-placeable = { -term() }

# ERROR Messages cannot be parameterized.

# ERROR Equivalent to a MessageReference callee.

# ERROR Message attributes cannot be parameterized.

# ERROR Term attributes may not be used in Placeables.

# ERROR Variables cannot be parameterized.

## Callees in selectors.

function-callee-selector =
    { FUNCTION() ->
       *[key] Value
    }
term-attr-callee-selector =
    { -term.attr() ->
       *[key] Value
    }

# ERROR Messages cannot be parameterized.

# ERROR Equivalent to a MessageReference callee.

# ERROR Message attributes cannot be parameterized.

# ERROR Term values may not be used as selectors.

# ERROR Variables cannot be parameterized.

