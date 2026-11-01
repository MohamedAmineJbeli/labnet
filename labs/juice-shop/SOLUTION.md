# Solution

The login form builds a query similar to:
`SELECT * FROM Users WHERE email = '<email>' AND password = '<password hash>'`

Enter the following in the Email field:
`' or 1=1--`

Enter any value in the Password field.

**Breakdown:**
* `'`: Closes the string literal around the Email value.
* `or 1=1`: Makes the condition true for every row in the `Users` table.
* `--`: Comments out the remaining password check.

The query returns the first user in the table, which is the administrator. The application logs you in and solves the "Log in with the administrator's user account" challenge.

If the error message reveals the administrator email, `admin@juice-sh.op'--` also works.
