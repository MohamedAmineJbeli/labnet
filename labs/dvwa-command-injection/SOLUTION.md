# Solution

The application builds a shell command similar to:
`ping -c 4 <input>`

The input is not escaped, so shell metacharacters are interpreted by the shell.

Enter the following in the IP address field:
`127.0.0.1; id`

**Breakdown:**
* `127.0.0.1`: A valid target, so the first command produces normal output.
* `;`: Ends the `ping` command and starts a new one.
* `id`: Prints the user and group IDs of the process running the command.

The response includes output such as:
`uid=33(www-data) gid=33(www-data) groups=33(www-data)`

This confirms that the injected command runs with the privileges of the web server process, not the user who submitted the form.
