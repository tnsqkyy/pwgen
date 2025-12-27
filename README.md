# Password Generator

`pwgen` is a simple, secure, and customizable command-line password generator built with Rust.

## Features

*   **Customizable Length:** Generate passwords with user-defined lengths (10-128 characters).
*   **Strong Passwords:** Includes a mix of uppercase, lowercase, digits, and special characters.
*   **Easy to Use:** A straightforward CLI.

## Installation

Clone the repository:

```bash
git clone https://github.com/tnsqkyy/pwgen.git
cd pwgen
```

## Usage

Run the application and enter your desired password length when prompted:

```bash
cargo run
```

Example:
```
Enter password length (10-128): 16
Generated Password: xY7@pQ!9zK$mR2oV
```

## Build

To build the executable:

```bash
cargo build --release
```

The executable will be located at `target/release/pwgen`.

## License

This project is licensed under the MIT License.
