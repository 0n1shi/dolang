let users = [{ name: "Mike", age: 30 }, { name: "John", age: 25 }]

let first_user = first(users)
println(first_user) # Mike

let names = users |> map(fn u -> u.name)
println(names) # ["Mike", "John"]
