# Objective
Exploit a command injection vulnerability to execute system commands on the web server.

# Access
URL: http://127.0.0.1:8082
Username: admin
Password: password

# Steps
1. Open http://127.0.0.1:8082/setup.php and click 'Create / Reset Database'.
2. Log in using the credentials above.
3. Navigate to DVWA Security on the left menu and set the security level to 'low'.
4. Navigate to Command Injection in the left menu.
5. The form pings an IP address. Manipulate the input so the server also runs `id`.
6. Identify the user account the web server runs as.

# Cleanup
When finished, run: `labnet down dvwa-command-injection`
