# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.5 - 2025-07-22
### Fixed
- Include plain Message-IDs in the JSON-LD output

## 0.1.4 - 2025-07-19
### Added
- Support JSONL output with `-o jsonl` (by @artob)
### Fixed
- Fix building for Windows (#1 by @imunproductive)

## 0.1.3 - 2025-07-15
### Changed
- Default to anonymous login if no credentials were configured (by @artob)

## 0.1.2 - 2025-07-15
### Changed
- Permit specifying nested mailbox names

## 0.1.1 - 2025-07-14
### Added
- Define the `-b, --order-by` option
- Discover available IMAP server capabilities
- Support server-side sorting
- Implement client-side sorting
### Changed
- Decode email message subjects in encoded-words form

## 0.1.0 - 2025-07-11
### Added
- Instruct Gmail configuration
- Add a cloud email provider reference
- Document credentials configuration
- Support `ASIMOV_IMAP_USER`
- Support `ASIMOV_IMAP_PASSWORD`
### Changed
- Disable Windows builds due to a build problem

## 0.0.1 - 2025-07-10
### Added
- `asimov-imap-cataloger`
- `asimov-imap-fetcher`

## 0.0.0 - 2025-07-05
