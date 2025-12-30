# python-graphql-core-rs

A high-performance GraphQL parser for Python, powered by Rust.

This library provides Python bindings to the [graphql-parser](https://crates.io/crates/graphql-parser) Rust crate, offering fast and reliable GraphQL query parsing through native extensions.

## Features

- Fast GraphQL query parsing using Rust's graphql-parser
- Native Python extension built with PyO3
- Supports Python 3.7+ (CPython and PyPy)

## Installation

### From Source

Requires [Rust](https://rustup.rs/) and [Maturin](https://github.com/PyO3/maturin):

```bash
pip install maturin
maturin develop
```

## Usage

```python
import graphql_core_rs

# Parse a GraphQL query
query = """
query GetUser($id: ID!) {
    user(id: $id) {
        name
        email
    }
}
"""

result = graphql_core_rs.parse(query)
print(result)
```

## Development

Build the extension in development mode:

```bash
pip install -r requirements.txt
maturin develop
```

Run with release optimizations:

```bash
maturin develop --release
```

## Requirements

- Python 3.7+
- Rust toolchain
- Maturin 0.13.x

## License

Apache-2.0
