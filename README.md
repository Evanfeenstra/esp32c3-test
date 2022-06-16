

export CC=$PWD/.embuild/espressif/tools/riscv32-esp-elf/esp-2021r2-patch3-8.4.0/riscv32-esp-elf/bin/riscv32-esp-elf-gcc

cargo build

espflash target/riscv32imc-esp-espidf/debug/esp32c3-test --monitor