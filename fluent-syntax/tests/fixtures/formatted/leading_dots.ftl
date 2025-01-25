key01 = .Value
key02 = …Value
key03 = { "." }Value
key04 = { "." }Value
key05 =
    Value
    { "." }Continued
key06 = .Value
{ "." }Continued
# MESSAGE (value = "Value", attributes = [])
# JUNK (attr .Continued" must have a value)
key07 = Value
# JUNK (attr .Value must have a value)

# JUNK (attr .Value must have a value)

key10 =
    .Value =
        which is an attribute
        Continued
key11 =
    { "." }Value = which looks like an attribute
    Continued
key12 =
    .accesskey = A
key13 =
    .attribute = .Value
key14 =
    .attribute = { "." }Value
key15 =
    { 1 ->
        [one] .Value
       *[other] { "." }Value
    }

# JUNK (variant must have a value)

# JUNK (unclosed placeable)

# JUNK (attr .Value must have a value)

key19 =
    .attribute =
        Value
        Continued
key20 = { "." }Value
