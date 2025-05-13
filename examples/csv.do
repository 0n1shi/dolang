let not_empty = fn row -> not empty(row)

let data = read_file("examples/users.csv")

let rows = split("\n", data) |> filter(not_empty) |> map(fn row -> split(",", row))

let ids = tail(rows) |> map(first)
let names = tail(rows) |> map(second)

println(ids)
println(names)
