let data = read_file("examples/users.csv")

# convert CSV data to a list of lists
let rows =
  data
  |> split("\n")
  |> filter(not empty)
  |> map(split(","))
  |> tail # remove header

let ids = rows |> map(first)
let names = rows |> map(second)

println(ids)
println(names)
