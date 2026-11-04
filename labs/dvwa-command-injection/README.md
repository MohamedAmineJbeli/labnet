# DVWA: Command Injection

This scenario uses the Command Injection module of Damn Vulnerable Web Application (DVWA), a deliberately vulnerable PHP/MySQL web application.

The module passes user input directly to a shell command, allowing command injection when the security level is low.

Official Repository: https://github.com/digininja/DVWA

## LabNet Implementation
This scenario uses the official `vulnerables/web-dvwa` image. It is bound to `127.0.0.1:8082` to prevent accidental exposure to the local network and to avoid colliding with the other DVWA scenario on port 8081.
