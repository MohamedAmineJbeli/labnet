# Solution

The underlying SQL query executed by the application is:
`SELECT first_name, last_name FROM users WHERE user_id = '$id'`

To extract credentials, we inject a `UNION` based payload. The `UNION` operator combines the result-set of two or more `SELECT` statements. 

Input the following into the User ID field:
`1' UNION SELECT user, password FROM users-- -`

**Breakdown:**
* `1'`: Closes the original string quote and completes the first query (returning ID 1).
* `UNION SELECT user, password FROM users`: Appends a new query that fetches all usernames and passwords from the `users` table.
* `-- -`: Comments out the trailing quote and semicolon (`'`) that the application appends after our input, preventing a syntax error.

The resulting First Name and Surname columns on the page will now display the database `user` and `password` columns. The admin password hash will be visible in the output.