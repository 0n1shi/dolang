let not_empty = fn row -> not empty(row)

let data = read_file("examples/users.csv")
let rows = split("\n", data) |> filter(not_empty) |> map(fn row -> split(",", row))

let ids = rows[1..] |> map(fn row -> row[0])
let names = rows[1..] |> map(fn row -> row[1])

println(ids)
println(names)
