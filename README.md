# ZLS plugin

[ZLS](https://github.com/zigtools/zls) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

Add the following to `.prototools`.

```toml
[plugins]
zls = "github://konomae/zls-plugin"
```

## Configuration

ZLS plugin does not support configuration.

## Hooks

ZLS plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install zls-test
proto list-remote zls-test
```
