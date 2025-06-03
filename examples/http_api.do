# This script is a simple HTTP API server that connects to a MySQL database
# It not available to be run this script in the current version of the language.

let list_users = fn db ->
  let users = mysql.query(db, "SELECT * FROM users")
  json(users)

let main = fn ->
  let db = mysql.connect("mysql://user:password@localhost:3306/db")

  let handlers = {
    "GET /ping": fn -> {"message": "pong"},
    "GET /users": list_users(db),
  }

  let server = http.create_server(handlers)
  server.listen(8080, fn ->
    println("Server is running on :8080")
  )

main()
