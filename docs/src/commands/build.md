# Build Project

```
USAGE:
    android build [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -r, --release    Should build in release mode
```

## What it does

For now, it just execs `./gradlew assemble[Debug|Release]`.

## Examples

### Building in Debug mode

```sh
$ android build
```

### Building in Release mode

```sh
$ android build --release
```