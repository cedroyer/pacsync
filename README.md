# Pacsync

An utility to simplify your package management.

**Main idea:** have a file with all needed packages on your computer. The tools synchronize the package list with your system.

## Installation

`rua install pacsync`

## Usage

Add file(s) in `/etc/pacsync.d` with required packages for your system.

And run:
```bash
$ pacsync
```

## Example of configuration files:

`/etc/pacsync.d/target/console`
```
# you can add comments
vim

# you can add group
base
base-devel
```

`/etc/pacsync.d/target/desktop`
```
# separate by file allow you to reuse some files on multiple machines.
gnome
```
