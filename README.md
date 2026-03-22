# paths

A simple command-line utility to display PATH environment variable entries on separate lines.

## Problem

On Windows, the PATH environment variable is displayed with semicolon (`;`) separators, which makes it hard to read:

```
C:\Windows\System32;C:\Program Files\bin;...
```

Similarly, on Linux and macOS, the PATH is displayed with colon (`:`) separators:

```
/usr/bin:/usr/local/bin:/home/user/bin:...
```

## Solution

The `paths` command displays PATH entries on separate lines, making it much more readable:

```
C:\Windows\System32
C:\Program Files\bin
...
```

## Building

```bash
cargo build --release
```

The binary will be generated at `target/release/paths` (or `paths.exe` on Windows).

## Usage

Simply run the `paths` command:

```bash
paths
```

It will display all entries in your PATH environment variable, one per line.

## Testing

Run the test suite:

```bash
cargo test
```

## Development

The project uses:
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`

Check formatting and linting:

```bash
cargo fmt --check
cargo clippy
```

## License

MIT

## Author

TAKAHASHI Satoshi <hikobae@gmail.com>
