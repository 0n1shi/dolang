let tax_rate = 0.1
let cart = [ { name = "apple", price = 100 }, { name = "banana", price = 200 } ]

let sum = for item in cart -> item.price |> total -> total * (1 + tax_rate)
