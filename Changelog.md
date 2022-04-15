# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0]

### Changed

- Removed `CapriceError::IO`.
- Validate keywords. Caprice keywords are now validated against the "^[_a-zA-Z][A-Za-z_0-9]*$" regex.
- `Caprice::set_keywords()` will now receive a `Vec` of keywords by value.

## [0.2.5]

### Changed

- Added support for proceeding arguments. (@berdon)

### Fixed

- Various bug fixes around scanner/buffer divergence (@berdon)

## [0.2.4]

### Changed

### Fixed

- Fix print command not moving cursor to new line
- Fix slow printing of keywords using the `/list` command

## [0.2.3] - 2021-03-06

### Changed

### Fixed

- Fix bug when leaving alternate screen.

## [0.2.2] - 2021-03-06

### Changed

### Fixed

- Fixed an extra empty line in terminal after a token is entered by the user

## [0.2.1] - 2020-02-23

### Fixed

- Spelling errors

## [0.2.0] - 2020-02-23

### Added

### Changed

- Change list command to '/list' from '#list' to bypass AZERTY keyboard bug
- Caprice now uses its own CapriceError and not Anyhow::Result
- set_keywords function returns self and should be chained at initialization

### Deprecated

### Removed

### Fixed

- Scrolling behavior in windows terminals

### Security

## [0.1.0] - 2020-01-05

Initial Release
