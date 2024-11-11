# Rust File Deduplication Project

This project is a Rust-based utility to detect and handle duplicate files based on their content hash. It can automatically delete duplicate files while keeping the original ones.

## Features

- Detects duplicate files based on SHA-256 hash.
- Deletes the duplicate files keeping the original ones.
- By default, runs in a dry-run mode to show what would be deleted without actually deleting files.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.82.0 or later)
- Cargo (comes with Rust installation)

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/your-repo.git
    cd your-repo
    ```

2. Build and install the project:

    ```sh
    cargo install --path=.
    ```

## Usage

### Basic Usage

To run the file deduplication tool in dry-run mode (default):

```sh
duplicate-remover .
```

Replace `.` with the actual path to the directory you want to deduplicate.

### Real-run Mode

To actually delete the detected duplicate files, pass the `--real-run` flag:

```sh
duplicate-remover . --real-run
```

## Important Notes

- Ensure the directory paths are correct and that you have the necessary permissions to delete files.
- Handle with care; improperly running the tool could result in loss of data.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes. Ensure that your changes are well-tested.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Contact

For any questions or suggestions, please open an issue or contact us at [ardasugur@gmail.com](mailto:ardasugur@gmail.com).