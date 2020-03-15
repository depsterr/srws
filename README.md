# srws

srws (Simple rust web server) is a simple HTTP only web server I'm writing in rust in order to learn the language.

I've tried to make the server somewhat safe, however I cannot guarantee it's safety, so if you wish to use it then do so at your own risk. That being said, I've tried using relative paths to read files, both with curl and with netcat and both have just retrieved 404 pages.

# Options / Configuration

```rust
/* Begin options */
const ADDRESS:&str = "0.0.0.0:80";
const DIRECTORY:&str = "/var/www/html";
const NOTFOUNDPAGE:&str = "/var/www/404.html";
const ALLOWSYM:bool = false; /* Potentially dangerous */
const MULTIPLEHOSTS:bool = false;
/* End Options */
```

* Address
	* This is the adress the server will listen on, use the default value if you simply want to listen for all connections on port 80.

* Directory
	* The base directory for the website.

* Not found page
	* The page to display in case of a 404 error.

* Allow symlinks
	* Allow opening symlinks, note that it will follow symlink directories without this, it simply prevents opening symlinks directly.

* Multiple hosts
	* Use directory `<address option>/<full domain name>` as base directory for files. This is usefull if you're hosting multiple websites on one server.

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
