# Toolchain for Jots

## Requirement

- bazelisk
- libomp
- cmake
- openblas

## Windows

Install chocolatey

```bash
choco install bazelisk
choco install cmake
choco install rustup
```

## Build

```sh
cargo build -F build-faiss
cargo build -F build-tensorflow
```
