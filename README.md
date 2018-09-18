# Generate argon2id hashes
Program to generate argon2id password hashes.

## Build:
`cargo build --release`

`cd target/release` to your freshly baked executable 🥧

## Usage:
```argon2-hash "your secret password"```

### output:
`$argon2id$v=x$m=y,t=z,p=w$salt$hash`

```
argon2-hash 0.2.0
Generates Argon2id hashes with random salt and using some defaults. The first argument is the password which you use to
generate the hash

USAGE:
    argon2-hash.exe [OPTIONS] <PASSWD>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -T, --hashlength <RETURN_BYTES>       Desired number of returned bytes
    -t, --iterations <NUM_OF_ITER>        Number of iterations to perform
    -m, --memory <SIZE_KB>                Amount of memory (in kilobytes) to use
    -p, --parallelism <NUM_OF_THREADS>    Degree of parallelism (i.e. number of threads)

ARGS:
    <PASSWD>    The password to hash
```