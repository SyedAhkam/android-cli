# Run App

```
USAGE:
    android run [OPTIONS]

OPTIONS:
    -h, --help       Print help information
    -r, --release    Should build in release mode
```

## What it does

It is a combination of three commands, [`android build`](./build.md), [`android install`](./install.md) and [`android launch`](./launch.md) giving you an illusion of a single operation being executed.

## Examples

### Running in Debug mode

```sh
$ android run
```

### Running in Release mode

```sh
$ android run --release
```