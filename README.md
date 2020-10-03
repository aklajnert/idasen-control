# Idasen control

This is a simple program to control the Ikea `IDASEN` desk via bluetooth. It allows to save multiple
positions, and move the desk between them.

## Usage

First, you need to save the preferred heights by moving the desk manually, and then run the following command:
```shell
desk save <position-name>
```
Calling `desk save` again on already saved position will overwrite it. 

After you save the position, you can move desk to the saved state by running the command below:  
```shell
desk <position-name>
```

To get the desk's MAC address or position use the following command:  
```shell
desk info
```

To remove the position that is no longer useful, call:  
```shell
desk delete <position-name>
```

To see all available commands (including saved positions) run:  
```shell
desk help
```

## Installation

This program can be compiled from source and installed with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
by running this command:  
```shell
cargo install --git https://github.com/aklajnert/idasen-control.git
```