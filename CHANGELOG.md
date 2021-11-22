# Change Log

## [master] - Unreleased

## [0.6.1] - 2021-11-22

### Changed

- Updated `nix` to version `0.23`.
- Updated `mio` to version `0.8`.

## [0.6.0] - 2021-09-24

### Changed

- [breaking-change] Renamed `use_tokio` feature `async-tokio`.
- Migrated to `tokio` crate version 1.
- Updated `nix` to version 0.22.
- Updated `mio` to version 0.7.
- Updated `futures` to version 0.3.
- Minimmum supported Rust version updated to 1.46.0.

## [0.5.3] - 2018-04-19

### Fixed

- Relaxed path restrictions on `Pin::from_path` to allow for directories
  outside of `/sys/class/gpio`, required for some SOCs that symlink outside of
  that directory.

## [0.5.2] - 2018-03-02

### Changed

- Add support for `active_low` configuration.
- Remove dependency on regexp
- Update nix to 0.10.0

## [0.5.1] - 2016-06-17

### Changed

- The crate now compiles (to more-or-less nothing) on OSX which can be useful in some
  contexts.
- Multiple warnings from clippy were addressed
- Support for some older versions of rust was dropped and things like `?` are now used in the codebase.
- Additional traits like `Copy` added where appropriate in some places.

## [0.5.0] - 2016-12-18

### Changed

- Support for asynchronous polling for changes using tokio/futures
  is now supported.
- Minimum supported version Rust for 0.5.0+ is now 1.10.0

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

[master]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.6.1...master
[0.6.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.5.3...0.6.0
[0.5.3]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.5.2...0.5.3
[0.5.2]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.4.4...0.5.0
[0.4.4]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.4.1...0.4.2
[0.4.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.3.3...0.4.0
[0.3.3]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.1.1...0.2.0
[0.1.1]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/rust-embedded/rust-sysfs-gpio/compare/33b28ae3115d91ae6612245e5b8d8c636dcdb69c...0.1.0
