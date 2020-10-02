# Idasen control

This is a simple program to control the Ikea `IDASEN` desk via bluetooth. It allows to save two
positions - one for sitting and one for standing, and move the desk between them.

## Usage

First, you need to save the preferred heights by moving the desk manually, and then run one of 
the following commands:
```shell
desk save-up
```

```shell
desk save-down
```

After you save both positions, you can move desk between them by running one of the commands below:  
```shell
desk up
```

```shell
desk down
```

To get the desk's MAC address or position use the following command:  
```shell
desk info
```

## Installation

This program can be compiled from source and installed with [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
by running this command:  
```shell
cargo install --git https://github.com/aklajnert/idasen-control.git
```