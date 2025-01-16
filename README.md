# Berlin Clock for ESP32-C3

## How to run

```shell
cd app && SSID=<wifi_ssid> PASSWORD=<wifi_password> cargo run --bin <binary>
```

## How to flush

```shell
espflash flash target/riscv32imc-esp-espidf/debug/berlin-clock-hardware
```

## How to monitor

```shell
espflash monitor 
```

## Dev setup

Follow the guide [here](https://docs.esp-rs.org/book/) for std applications

```bash
rustup toolchain install nightly --component rust-src
cargo install espup
espup install
cargo install ldproxy
```
