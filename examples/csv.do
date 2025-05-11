let data = read_file("examples/users.csv")
let rows = split("\n", data)
println(rows[1..])
