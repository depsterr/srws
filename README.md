# srws

srws (Simple rust web server) is a simple HTTP only web server I'm writing in rust in order to learn the language.

I've tried to make the server somewhat safe, however I cannot guarantee it's safety, so if you wish to use it then do so at your own risk. That being said, I've tried using relative paths to read files, both with curl and with netcat and both have just retrieved 404 pages.

# Options / Configuration

To configure the web server you can set these values in /etc/srws.conf

* address
	* The address that the server will listen on. The default value covers all connections on port 80

* allow_sym
	* Allow opening symlinks? (Note that symlink paths are not blocked by this option.

* directory
	* The base directory for the webpage.

* multiple_hosts
	* If set to true, the server will serve webpages from a subdirectory with the name of the host. For example, if you were to connect to examplewebsite.com then the server would use the folder /var/www/html/examplewebsite.com/ as it's base directory. This is useful if you want to host multiple website on one server.

* not_found_page
	* The page to show in case of a 404 Not Found error

If not configuration file is found these defaults will be used:

```
address           0.0.0.0:80
directory         /var/www/html
not_found_page    /var/www/404.html
allow_sym         false
multiple_hosts    false
```

# Installation

To install the binary into /bin/srws and enable the service run the following commands.

```sh
sh install.sh
sudo systemctl enable srws
sudo systemctl start srws
```

If you simply wish to compile the binary you can run

```sh
cargo build
```

and if you want to install the program localy you can run

```sh
cargo install --path .
```
