# Objective
Exploit a SQL injection vulnerability to extract user credentials from the database.

# Access
URL: http://127.0.0.1:8081
Username: admin
Password: password

# Steps
1. Open http://127.0.0.1:8081/setup.php and click 'Create / Reset Database'.
2. Log in using the credentials above.
3. Navigate to DVWA Security on the left menu and set the security level to 'low'.
4. Navigate to SQL Injection in the left menu.
5. Manipulate the user ID input field to extract the usernames and passwords from the database.
6. Locate the admin password hash in the resulting output.

# Cleanup
When finished, run: `labnet down dvwa`