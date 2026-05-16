# build 指令

```BASH
mkdir build && cd build

cmake .. \
  -DZENOHC_BUILD_WITH_UNSTABLE_API=true \
  -DZENOHC_BUILD_WITH_SHARED_MEMORY=true \
  -DCMAKE_INSTALL_PREFIX="$(pwd)/../install-prefix"

cmake --build . --config Release

cmake --build . --target examples

cmake --build . --target tests

cmake --install . # or make install
```
