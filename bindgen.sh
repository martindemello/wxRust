BINDGEN=/home/martin/code/rust-bindgen/bindgen
$BINDGEN -allow-bitfields -x c++ wxHaskell/wxc/src/include/wxc.h `wx-config-2.9 --cflags` -I/home/martin/opt/rust/src/llvm/tools/clang/lib/Headers --include stddef.h --include stdint.h --include time.h -o native.rs
