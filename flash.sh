#!/bin/bash
sudo chmod 777 /dev/ttyUSB0
cargo espflash --monitor --target riscv32imc-esp-espidf /dev/ttyUSB0
