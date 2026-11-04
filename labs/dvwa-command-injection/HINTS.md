# Hint 1
The application passes your input to a shell command. Consider how the shell separates one command from another.

# Hint 2
Place a semicolon after a valid IP address and observe whether the response includes output from a second command.

# Hint 3
Other separators work as well, including `|` and `&&`. The goal is to run `id` on the server.

# Hint 4
Use `127.0.0.1; id` as the IP address.
