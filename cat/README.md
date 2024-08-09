# cat

### Overview
This project is a Rust implementation of the [Unix cat](https://github.com/coreutils/coreutils/blob/master/src/cat.c) command. The tool reads and outputs the contents of files or standard input (stdin) and supports several options, including displaying line numbers and concatenating multiple files.


### Features

1. **Basic File Output**: Outputs the contents of a file to the standard output.

```bash
cargo run -- some-text.txt
```

2. **Line Numbers**: Outputs the contents of a file with each line preceded by its line number.

```bash
cargo run -- -n some-text.txt
```

3. **Standard Input (stdin)**: Reads from the standard input stream and outputs the content.

```bash
cat some-text.txt | cargo run -- -
```

4. **Concatenation**: Concatenates and outputs the contents of multiple files.

```bash
cargo run -- file.txt data.txt
```

### Requirements

Rust programming language
