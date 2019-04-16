response-msg =
    { $value ->
        [one] "{ $input }" ma jeden krok Collatza.
        [few] "{ $input }" ma { $value } kroki Collatza.
       *[many] "{ $input }" ma { $value } kroków Collatza.
    }
