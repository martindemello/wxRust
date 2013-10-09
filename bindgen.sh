BINDGEN=/home/martin/code/rust-bindgen/bindgen
STDDEF_PATH=/usr/lib/gcc/i686-pc-linux-gnu/4.8.1/include
$BINDGEN -allow-bitfields -x c++ wxHaskell/wxc/src/include/wxc.h `wx-config-2.9 --cflags` -I$STDDEF_PATH --include stddef.h --include stdint.h --include time.h -o native.rs
