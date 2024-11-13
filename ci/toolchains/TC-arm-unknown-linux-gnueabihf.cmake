set(CMAKE_SYSTEM_NAME Linux)
set(CMAKE_SYSTEM_PROCESSOR armhf)
set(DEBARCH armhf)
set(ZENOHC_CUSTOM_TARGET arm-unknown-linux-gnueabihf)
set(ZENOHC_CARGO_FLAGS "--config=target.arm-unknown-linux-gnueabihf.linker=\"arm-linux-gnueabihf-gcc\"")
set(CMAKE_C_COMPILER arm-linux-gnueabihf-gcc)
set(CMAKE_CXX_COMPILER arm-linux-gnueabihf-g++)