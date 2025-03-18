# Rust Notepad 📝

A small project to create a native Linux notepad app in Rust from scratch.

## About

This project aims to develop a simple and lightweight notepad application for Linux using the Rust programming language. The goal is for me to learn the basics of Rust. So the application itself is ass, it is not the next Sublime text.

## Features

- Basic text editing (open, edit, save files)
- Syntax highlighting (planned)
- Search and replace (planned)
- Undo and redo functionality (planned)
- Lightweight and fast

## Installation

To build and run the notepad application, you need to have Rust installed. You can install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/khairizalkhalid/rust_notepad.git
cd rust_notepad
```

Build and run the application:

```bash
cargo run
```

## Usage

Once the application is running, you can open, edit, and save text files using the provided interface. Future updates will include additional features like syntax highlighting and search functionality.

## Project Structure

```
rust_notepad/
├── src/
│   ├── main.rs
│   ├── ui.rs
│   ├── editor.rs
│   └── ...
├── Cargo.toml
└── README.md
```

- **src/**: Contains the Rust source files for the application.
  - **main.rs**: Entry point of the application.
  - **ui.rs**: User interface code.
  - **editor.rs**: Core text editing logic.
- **Cargo.toml**: The Cargo configuration file.

## Contributing

Contributions are welcome! If you have any suggestions or improvements, feel free to open an issue or create a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Thank you for checking out this project! Feel free to reach out if you have any questions or feedback.
