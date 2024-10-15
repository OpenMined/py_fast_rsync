def diff(signature_bytes: bytes, data: bytes) -> bytes:
    """
    Calculate a delta and return it as bytes.

    This function computes a delta that can be applied to the base data represented by `signature_bytes`
    to attempt to reconstruct `data`.

    Args:
        signature_bytes (bytes): The signature of the base data.
        data (bytes): The target data to be reconstructed.

    Returns:
        bytes: The calculated delta.

    Security:
        Since this function uses the insecure MD4 hash algorithm, the resulting delta must not be
        trusted to correctly reconstruct `data`. The delta might fail to apply or produce the wrong
        data entirely. Always use another mechanism, like a cryptographic hash function, to validate
        the final reconstructed data.
    """
    ...

def apply(base: bytes, delta: bytes) -> bytes:
    """
    Apply `delta` to the base data `base` and return the result.

    This function applies the provided delta to the base data and returns the reconstructed data.

    Args:
        base (bytes): The original base data.
        delta (bytes): The delta to be applied.

    Returns:
        bytes: The reconstructed data after applying the delta.

    Security:
        This function should not be used with untrusted input, as a delta may create an arbitrarily
        large output which can exhaust available memory. Use `apply_limited()` instead to set an upper
        bound on the size of the output.
    """
    ...
