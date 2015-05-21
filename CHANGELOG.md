# Change Log

## [Unreleased][unreleased]

## [0.3.1] - 2015-05-21

### Fixed

- Converting from a nix error to an io::Error has been simplified and
  updated to work with future versions of nix

### Changed

- Documentation now refers to package as `sysfs-gpio` with a dash
  instead of an underscore as per common convention.  The package
  name on crates.io cannot be changed right now, however.
- Documentation updates.

## [0.3.0] - 2015-04-20

### Fixed

- Updates to work with latest rust nightlies

### Added

- Support for interrupts on pins was added via epoll.  This is an
  efficient and performant way to wait for a pin to change state
  before performing some operation.

## [0.2.1] - 2015-04-06

### Fixed

- Library updated to work with latest nightlies (~1.0.0 beta).  Due to
  std situation, still need to depend on a few deprecated features for
  the examples (synchronous timers).

## [0.2.0] - 2015-03-19

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

## [0.1.0] - 2015-03-15

### Added
- Initial version of the library with basic functionality
- Support for `export`/`unexport`/`get_value`/`set_value`/`set_direction`

[unreleased]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.1...HEAD
[0.3.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/posborne/rust-sysfs-gpio/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/posborne/rust-sysfs-gpio/compare/33b28ae3115d91ae6612245e5b8d8c636dcdb69c...0.1.0