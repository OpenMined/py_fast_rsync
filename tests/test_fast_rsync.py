import py_fast_rsync
from py_fast_rsync import signature


def test_diff():
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
    # This function should not be used with untrusted input,

    probably_data_b = py_fast_rsync.apply(data_a, delta)
    assert probably_data_b == data_b


def test_optional_params():
    data_a = b"hello world"
    sig = signature.calculate(data_a, block_size=2048, crypto_hash_size=16)

    # 2. take the signature for data_a and data_b
    # and return a delta between data_a and data_b.
    data_b = b"hello world!"
    delta = py_fast_rsync.diff(sig, data_b)

    # 3. apply the delta to data_a
    # (usually) return data_b
    # This function should not be used with untrusted input,

    probably_data_b = py_fast_rsync.apply(data_a, delta)
    assert probably_data_b == data_b
