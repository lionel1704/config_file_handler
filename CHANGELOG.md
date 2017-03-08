# Config File Handler - Change Log

## [0.4.0]
- Modify file search paths for various paltforms. The path returned would either be the potential path where files can be read from or created, or will contain the default file already created by this crate, depending on function invoked.

## [0.3.1]
- Migrate quick-error to 1.1.0.
- various docs update

## [0.3.0]
- Implemented std::fmt::Display and std::error::Error for Error.

## [0.2.1]
- Fix: existing files are now overwritten, not appended to.
- Added file locks to protect concurrent access.

## [0.2.0]
- Added `open` function and made `cleanup` function public.

## [0.1.0]
- Removed dependency on CBOR.
- Updated dependencies.

## [0.0.1]
- Initial implementation.