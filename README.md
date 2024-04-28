## grrs - A Simple Grep Clone

**grrs** is a lightweight command-line utility inspired by the classic `grep` tool. It allows you to search for a specific pattern within a file and display the lines that contain the pattern. Thsi tool was built following this handbook: [Rust Handbook](https://rust-cli.github.io/book/index.html)

### Installation
To use **grrs**, you need to have Rust installed on your system. If Rust is not installed, you can get it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install **grrs** by cloning this repository and running the following command:
```
cargo install --path .
```

### Usage
**grrs** provides a simple command-line interface:

```
grrs <pattern> <file>
```

- `<pattern>`: The pattern to search for within the file.
- `<file>`: The path to the file in which to search for the pattern.

### Examples
Search for the word "hello" in a file named `example.txt`:
```
grrs hello example.txt
```

### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

### Acknowledgments
This project was inspired by the `grep` tool and built using the Rust programming language. Special thanks to the Rust community for their excellent libraries and resources.
