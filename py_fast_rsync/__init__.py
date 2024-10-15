"""
A faster implementation of librsync in pure Rust, wrapped for Python.

This module offers three major APIs:

1. `signature.calculate()`, which takes a block of data and returns a
   "signature" of that data which is much smaller than the original data.
2. `diff()`, which takes a signature for some block A, and a block of data B, and
   returns a delta between block A and block B. If A and B are "similar", then
   the delta is usually much smaller than block B.
3. `apply()`, which takes a block A and a delta (as constructed by `diff()`), and
   (usually) returns the block B.

This Python module wraps the Rust implementation, providing a high-performance
solution for efficient data synchronization and comparison.
"""

from .py_fast_rsync import *
