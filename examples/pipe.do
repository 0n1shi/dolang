let a = [1, 2, 3, 4, 5] |> filter(fn x -> x % 2 == 0) |> map(fn x -> x + 3)
println(a)
