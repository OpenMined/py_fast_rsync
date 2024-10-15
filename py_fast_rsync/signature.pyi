def calculate(data: bytes, block_size: int = 4096, crypto_hash_size=8) -> bytes:
    """
    Compute an MD4 signature for the given data.

    This function calculates an MD4 signature for the input data using the specified block size and crypto hash size.

    Args:
        data (bytes): The input data to compute the signature for.
        block_size (int, optional): The granularity of the signature. Smaller block sizes yield larger,
                                    but more precise, signatures. Defaults to 4096.
        crypto_hash_size (int, optional): The number of bytes to use from the MD4 hash. Must be at most 16.
                                          The larger this is, the less likely that a delta will be mis-applied.
                                          Defaults to 8.

    Returns:
        bytes: The computed MD4 signature.

    Raises:
        ValueError: If block_size is not greater than zero or if crypto_hash_size is greater than 16.

    Note:
        This function may panic if the provided options are invalid.
    """
