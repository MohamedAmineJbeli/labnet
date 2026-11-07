# Objective
Exploit a local file inclusion vulnerability to read files from the web server.

# Access
URL: http://127.0.0.1:8083
Username: admin
Password: password

# Steps
1. Open http://127.0.0.1:8083/setup.php and click 'Create / Reset Database'.
2. Log in using the credentials above.
3. Navigate to DVWA Security on the left menu and set the security level to 'low'.
4. Navigate to File Inclusion in the left menu.
5. Manipulate the `page` parameter so the server includes `/etc/passwd`.
6. Identify the first user account listed in the file.

# Cleanup
When finished, run: `labnet down dvwa-file-inclusion`
