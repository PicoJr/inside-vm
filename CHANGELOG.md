# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0](https://crates.io/crates/inside-vm/0.2.0) Jul 11, 2021

### Changed

* replace "hack" preventing compiler from optimizing away call to `__cpuid` with call to `std::ptr::read_volatile`.

## [0.1.0](https://crates.io/crates/inside-vm/0.1.0) Jul 09, 2021

### Added

* `cpuid_cycle_count_avg`
* `inside_vm`
* `inside_vm_custom`
