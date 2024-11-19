# MD5

MD5 (Message Digest Algorithm 5) is a cryptographic hashing algorithm developed by [Ronald Rivest](https://datatracker.ietf.org/doc/html/rfc1321) in 1991. It takes an input of arbitrary length (message) and produces a fixed-length 128-bit digest. This digest is used to verify the integrity of data, ensuring its originality and detecting alterations.

## Why MD5 Is No Longer Used
Despite its early popularity for verifying file integrity and securing passwords, MD5 is now considered insecure. It is vulnerable to:

- Collision Attacks: Two different inputs can produce the same digest.
- Pre-image Attacks: Given a hash, it is feasible to find an input that generates it.
These vulnerabilities make MD5 unsuitable for cryptographic purposes such as digital signatures or secure hashing.

### Alternatives to MD5
For modern applications, consider secure hashing algorithms like:

SHA-256: A widely-used, collision-resistant algorithm.
SHA-3: Designed for future-proof cryptographic needs.
Argon2: Recommended for password hashing.




Note: This codebase is intended solely for educational purposes. It is not recommended to use this code in production.

