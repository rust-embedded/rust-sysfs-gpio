# Change Log

## [0.4.4] - 2016-08-26

### Fixed

- A couple issues were fixed that limited the circumstances where
  `Pin::from_path` would work in various environments.

## [0.4.3] - 2016-06-22

### Changed

- Added `from_path` constructor to allow for use of the library with symlinked
  GPIOs to interact with things like the IOs exported by
  [gpio-utils](https://github.com/rust-embedded/gpio-utils).
- Bumped Nix dependency to version 0.6.0 (removes some compile warnings)

## [0.4.2] - 2016-04-17

### Changed

- `Pin` now has an `is_exported()` function

### Fixed

- Moved to nix 0.5 which fixes problems on some architectures

## [0.4.1] - 2016-04-8

### Changed

- A few additional traits are derived for types exposed by the library.
- Links/Docs updated with move to rust-embedded Github org

## [0.4.0] - 2015-12-04

### Changed

- We now expose our own `Error` type than using io::Result.
  This allows for better errors to be provided from the library.
  This breaks backwards compatability.

### Fixed

- Only open file for reading when reading value from file

## [0.3.3] - 2015-09-25

### Added

- `get_pin` method added for accessing pin number
- `get_direction` method added for querying Pin direction

### Changed

- Updates/Fixes to documentation
- Non-functional code refactoring and cleanup

## [0.3.2] - 2015-07-13

### Fixed

- We now work with the latest version of nix

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

[0.4.3]: https://github.com/posborne/rust-sysfs-gpio/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/posborne/rust-sysfs-gpio/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/posborne/rust-sysfs-gpio/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.3...0.4.0
[0.3.3]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/posborne/rust-sysfs-gpio/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/posborne/rust-sysfs-gpio/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/posborne/rust-sysfs-gpio/compare/33b28ae3115d91ae6612245e5b8d8c636dcdb69c...0.1.0
