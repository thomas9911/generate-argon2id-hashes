# Generate argon2id hashes
Program to generate argon2id password hashes.

## Build:
`cargo build --release`

`cd target/release` to your freshly baked executable 🥧

## Usage:
```argon2-hash "your secret password"```

### output:
`$argon2id$v=x$m=y,t=z,p=w$salt$hash`
