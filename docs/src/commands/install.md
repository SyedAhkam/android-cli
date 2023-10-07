# Install App

```ignore
USAGE:
    android install [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -r, --release    Should install release APK
```

## What it does

Executes `adb install` under the hood and simply returns the output.

## Examples

### Install in Debug mode

```sh
$ android install
```

### Install in Release mode

```sh
$ android install --release
```