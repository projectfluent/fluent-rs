## Member expressions in placeables.

# OK Message attributes may be interpolated in values.
message-attribute-expression-placeable = { msg.attr }

# ERROR Term attributes may not be used for interpolation.

## Member expressions in selectors.

# OK Term attributes may be used as selectors.
term-attribute-expression-selector =
    { -term.attr ->
       *[key] Value
    }

# ERROR Message attributes may not be used as selectors.

