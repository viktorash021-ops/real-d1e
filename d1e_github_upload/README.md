# d1e Programming Language

An open-source, cybersecurity-focused domain-specific language written in Rust.

## Syntax Overview
Programs must be wrapped inside `start` and `finish` tags:

```d1e
start
    target = "admin123"
    hash sha256 target as secure_hash
    print secure_hash
    print "Security audit completed successfully."
finish
```

## Running locally
```bash
cargo run -- examples/test_security.d1e
```
