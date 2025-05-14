let not_empty = fn row -> not empty(row)

let data = read_file("examples/users.csv")

# convert CSV data to a list of lists
let rows =
  split("\n", data)
  |> filter(not_empty)
  |> map(split(","))

let ids = tail(rows) |> map(first)
let names = tail(rows) |> map(second)

println(ids)
println(names)
