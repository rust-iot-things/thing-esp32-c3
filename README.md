# ESP32-C3 with RISV-V (cargo approach)

Things getting easier using an ESP32-C3 with RISC-V, as cargo espflash can be used instead of idf.

## Installation
Simple:
```
rustup target add riscv32imc-unknown-none-elf
```

## Flashing
```
cargo espflash --target riscv32imc-esp-espidf <e.g. /dev/ttyUSB0>
```

## Flashing and monitoring
```
cargo espflash --monitor --target riscv32imc-esp-espidf <e.g. /dev/ttyUSB0>
```
