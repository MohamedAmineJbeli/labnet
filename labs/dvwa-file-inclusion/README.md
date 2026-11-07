# DVWA: File Inclusion

This scenario uses the File Inclusion module of Damn Vulnerable Web Application (DVWA), a deliberately vulnerable PHP/MySQL web application.

The module passes the `page` parameter to a file inclusion function without validating the path, allowing local file inclusion when the security level is low.

Official Repository: https://github.com/digininja/DVWA

## LabNet Implementation
This scenario uses the `vulnerables/web-dvwa` image. It is bound to `127.0.0.1:8083` to prevent accidental exposure to the local network and to avoid colliding with the other DVWA scenarios on ports 8081 and 8082.
