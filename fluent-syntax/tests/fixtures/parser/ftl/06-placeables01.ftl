key1 = AA { $num } BB

key2 = { brand-name }

key4 = { $num ->
   *[one] One
    [two] Two
  }

key5 = { LEN($num) ->
   *[one] One
    [two] Two
  }

key6 = { LEN(NEL($num)) ->
   *[one] One
    [two] Two
  }

key7 = { LIST($user1, $user2) }

key9 = { LEN(2, 2.5, -3.12, -1.00) }

key11 = { LEN() }

key12 = { LEN(1) }

key13 = { LEN(-1) }

key14 = { LEN($foo) }

key15 = { LEN(foo) }

key19 = { LEN(bar: 1) }

key20 = { LEN(bar: -1) }

key21 = { LEN(bar: "user") }

key22 = { brand-name[masculine] }

key23 = { NUMBER(style: "percent") }

key24 = { NUMBER_SPECIAL($num, style: "percent", foo: "bar") }
