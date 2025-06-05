# This script connects to a MySQL database and performs a simple query.
# It not available to be run this script in the current version of the language.
import mysql

let dsn = "mysql://user:password@localhost:3306/db"
let db = mysql.connect(dsn)

let query = "SELECT * FROM users"
let rows = mysql.query(db, query)

println(rows)
