# Toolchain for Jots

## Requirement

- bazelisk
- cmake
- libomp
- openblas
- python

## Windows

Install chocolatey
Install rustup
Install cmake
Install python

```bash
#faiss:
vcpkg install openblas
#tensorflow:
pip install numpy
choco install bazelisk
choco install cmake --installargs 'ADD_CMAKE_TO_PATH=System'
```

## Build

```sh
cargo build -F build-faiss
cargo build -F build-tensorflow
```
