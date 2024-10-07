# py_fast_rsync

```py_fast_rsync``` is a Python library implemented in Rust using the ```pyo3``` and ```fast_rsync``` crates. This library provides functions for calculating the difference between two data sets and applying those differences to create updated data sets, as well as a simple function to sum two numbers and return the result as a string.

## Usage

Here's how you can use the functions provided by ```py_fast_rsync```:

### Example

```python
import py_fast_rsync

# Calculate the diff between two data sets
data = b"Hello, world!"
new_data = b"Hello, Rust!"
diff_result = py_fast_rsync.diff(data, new_data)

# Apply the diff to the original data
applied_result = py_fast_rsync.apply(data, diff_result)
print(applied_result)  # Output should be b"Hello, Rust!"
```

## Functions

### ```diff(data: bytes, new_data: bytes) -> bytes```

Calculates the difference between ```data``` and ```new_data``` and returns the result as ```bytes```.

### ```apply(base: bytes, delta: bytes) -> bytes```

Applies the ```delta``` to the ```base``` data and returns the updated data as ```bytes```.


## Building the Project

### Requirements for Development

- Rust
- Python
- ```maturin``` (for building, developing and publishing the package)

First, ensure you have ```maturin``` installed. You can install it via pip:

```sh
pip install maturin
```

To build the project, run:

```sh
maturin develop
```

This will compile the Rust code and install the resulting Python package in your current Python environment.

