# py_fast_rsync

```py_fast_rsync``` is a Python library implemented in Rust using the ```pyo3``` and ```fast_rsync``` crates. This library provides functions for calculating the difference between two data sets and applying those differences to create updated data sets, as well as a simple function to sum two numbers and return the result as a string.

### Usage

Here's how you can use the functions provided by ```py_fast_rsync```:


```python
    import py_fast_rsync
    from py_fast_rsync import signature

    # 1. take data_a and return a "signature" of that data
    # which is much smaller than the original data.
    data_a = b"hello world"
    sig = signature.calculate(data_a)

    # 2. take the signature for data_a and data_b
    # and return a delta between data_a and data_b.
    data_b = b"hello world!"
    delta = py_fast_rsync.diff(sig, data_b)

    # 3. apply the delta to data_a
    # (usually) return data_b

    probably_data_b = py_fast_rsync.apply(data_a, delta)
    assert probably_data_b == data_b
```


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
