[![rtw crate](https://img.shields.io/crates/v/inside-vm.svg)](https://crates.io/crates/inside-vm)
[![rtw documentation](https://docs.rs/inside-vm/badge.svg)](https://docs.rs/inside-vm)

# inside-vm

Detect if code is running inside a virtual machine.

> Only works on x86 and x86-64.

## How does it work

Measure average cpu cycles when calling [`cpuid`](https://en.wikipedia.org/wiki/CPUID) and compare to a threshold, if the value is high assume code is running inside a VM.

## Quick Start

```
git clone https://github.com/PicoJr/inside-vm
cd inside-vm/
cargo run --example test-inside-vm
```

output: `avg cycles for __cpuid: 108`

vs inside VM

```
[vagrant@archlinux vagrant]$ ./target/release/examples/test-inside-vm 
```

output: `avg cycles for __cpuid: 30578`

## API

``` rust
use inside_vm::inside_vm;

let inside = inside_vm();
println!("inside vm: {}", inside);
```

## Credits

https://evasions.checkpoint.com/techniques/timing.html#difference-vm-hosts

## Changelog

Please see the [CHANGELOG](CHANGELOG.md) for a release history.

## License

Dual-licensed under MIT or the Apache License V2.0.
