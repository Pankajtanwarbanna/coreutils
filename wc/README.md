# wc 

### Overview

This project is a Rust implementation of the [Unix wc](https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html#wc-invocation) (word count) command. The tool counts lines, words, characters, and bytes in text files. It supports the following options:

- `-c`: Print the byte count.
- `-l`: Print the line count.
- `-w`: Print the word count.
- `-m`: Print the character count.

The tool can read from files or standard input (stdin) and is designed to closely mimic the behavior of the wc command found in Unix-like systems.

### Features
- Supports -c, -l, -w, and -m options.
- Handles standard input (stdin) piping.
- Provides default behavior when no options are specified.
- Written in Rust with all functionality implemented in main.rs.

### Requirements
Rust programming language