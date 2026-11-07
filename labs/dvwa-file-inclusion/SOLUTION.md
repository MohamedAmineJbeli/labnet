# Solution

The application includes the file named by the `page` parameter without validating the path.

Open:

`http://127.0.0.1:8083/vulnerabilities/fi/?page=/etc/passwd`

**Breakdown:**
* `page`: The parameter used by the include function.
* `/etc/passwd`: An absolute path to a local file readable by the web server.

The response contains a line such as:

`root:x:0:0:root:/root:/bin/bash`

This confirms local file inclusion. The web server runs as `www-data`, but `/etc/passwd` is world-readable, so no privilege escalation is needed to read it.
