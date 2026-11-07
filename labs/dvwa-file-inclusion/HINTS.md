# Hint 1
The application passes the `page` parameter to a file inclusion function. Consider what happens when the value is a path rather than a file name from the menu.

# Hint 2
The intended files are in the same directory as the module. Try a path outside that directory.

# Hint 3
Use an absolute path. The `page` parameter in the URL is the value being included.

# Hint 4
Request `?page=/etc/passwd`. The response should contain the contents of the file.
