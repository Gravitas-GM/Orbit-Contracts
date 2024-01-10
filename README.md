## Orbit Contracts
This is the primary repository for Orbit's contracts libraries. Packages are built from this repository using the Rust application `/codegen`.

## Usage
You can use the `/build.sh` script to interface with the codegen application, which is used to generate the required contract files for each package.

```shell
# Build all contracts, for all languages
$ ./build.sh

# Build only the "permission" contract
$ ./build.sh -c permission

# Build only the PHP package's "permission" contract
$ ./build.sh -c permission -l php
```

The `-c` and `-l` options can be repeated to specify multiple items.

## Packages
The directories in `/packages` are available as subtree splits on Github as `Orbit-<Lang>-Contracts`. See the README in each package root for more information.