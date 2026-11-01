# Hint 1
The login form builds a SQL query from the Email and Password fields. Consider what happens if the Email input is not properly escaped.

# Hint 2
Enter a single quote in the Email field and observe the error. The quote terminates the string literal that the application places around your input.

# Hint 3
You need a condition that is always true, followed by a comment that removes the password check from the query.

# Hint 4
Use `' or 1=1--` as the Email and any value as the Password. The first user in the table is the administrator.
