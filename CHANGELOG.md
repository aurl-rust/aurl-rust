# Change Log

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](http://keepachangelog.com/) and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Fix authserver HTTP Response

## [0.1.3]

### Changed

- Add LocalServer for getting Authorization Code automaticaly
- Remove redandunt Test Code
- toolchains to 1.60.0

## [0.1.2]

### Changed

- update toolchain and depencency crates
    - rust editons to 2021
    - toolchains to 1.59.0
- Delete verbose log / fix log levels of some messages.

## [0.1.1]

### Added

Add some options (see README for details)

- `--data` option for sending request (raw) body
- `---timeout` option for setting request timeout
- `--output` option for showing curl snipets of the request
- `--auth_header_template` option for modify header name of access token
