key1 = AA { $num } BB

key2 = { brand-name }

key4 = { $num ->
  [one] One
  [two] Two
}

key5 = { LEN($num) ->
  [one] One
  [two] Two
}

key6 = { LEN(NEL($num)) ->
  [one] One
  [two] Two
}

key7 = { $user1, $user2 }

key9 = { LEN(2, 2.5, -3.12, -1.00) }

key11 = { len() }

key12 = { len(1) }

key13 = { len(-1) }

key14 = { len($foo) }

key15 = { len(foo) }

key19 = { len(bar: 1) }

key20 = { len(bar: -1) }

key21 = { len(bar: $user) }

key22 = { brand-name[masculine][nominative] }

key23 = { number(style: "percent") }

key24 = { number($num, style: "percent", foo: "bar") }
