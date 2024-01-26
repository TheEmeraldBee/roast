# What is roast?
Roast is a binary that allows you to host other binaries as a website!

# Why Might I Want It?
I originally made it for myself, as a binary that would allow me to 
host a minecraft server, and allow my friends to open the server with the
`main_user` and `main_pass`

# Installation

Currently, running `cargo install roast-bin` is the only way to get the program.

# Usage

Typing `roast`, it will attempt to run your server.

However, without having both a roast-options.toml file, a runnable script, and cert.pem and key.pem, the server will be unable to run.
In whatever directory you want to host your server, you can type

```bash
roast --gen-tls # This will generate the cert.pem and key.pem files.
roast --gen-config # This will put an EXAMPLE config file into your directory (Please change the passwords at least.)
```

After running these two commands, open the config with your favorite text editor, and change some of the parameters.

## Running a Server
Ok, so now that you have your server ready to run, how do I run it?
In the roast-options.toml file you have, change the run path to be to a local script folder which runs your server.

Example for a minecraft server:
```bash
java -jar server.jar
```

Put this into a script file, and change the run path to be the location of the file!
