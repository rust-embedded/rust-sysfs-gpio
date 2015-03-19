# Change Log

## [Unreleased][unreleased]

### Changed
- The `core` module has been removed in favor of putting all
  structs, functions, and macros directly in the `sysfs_gpio`
  crate.

### Added
- Project now publishes documentation and has travisci support
- Added `with_exported` method taking a closure for more convenient
  export/unexport in all cases.

### Fixed
- Fixed a critical bug that resulted in `unexport` never actually
  unexporting GPIOs.

## [0.1.1] - 2015-03-17

### Added
- Added `try_unexport!` macro
- Include additional documentation for cross-compilation
- Added `poll` example showing input functionalty

### Fixed
- Fixed bug preventing the correct operation of `get_value`.  In 0.1.1,
  this function would always fail.

## 0.1.0 - 2015-03-15

### Added
- Initial version of the library with basic functionality
- Support for export/unexport/get_value/set_value/set_direction


[unreleased]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.1...HEAD
[0.1.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.0...0.1.1
