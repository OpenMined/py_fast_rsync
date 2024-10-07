import py_fast_rsync


def test_diff():
    byte_data1 = b"hello world"
    byte_data2 = b"hello world!"
    diff = py_fast_rsync.diff(byte_data1, byte_data2)

    new_data = py_fast_rsync.apply(byte_data1, diff)
    assert new_data == byte_data2
