# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.9](https://github.com/joshka/tui-prompts/compare/v0.3.8...v0.3.9) - 2024-04-01

### Other
- *(deps)* bump clap from 4.5.2 to 4.5.4 ([#37](https://github.com/joshka/tui-prompts/pull/37))

## [0.3.8](https://github.com/joshka/tui-prompts/compare/v0.3.7...v0.3.8) - 2024-03-12

### Other
- add workflow names ([#32](https://github.com/joshka/tui-prompts/pull/32))

## [0.3.7](https://github.com/joshka/tui-prompts/compare/v0.3.6...v0.3.7) - 2024-03-12

### Other
- allow release-plz workflow to be manually run ([#30](https://github.com/joshka/tui-prompts/pull/30))

## [0.3.6](https://github.com/joshka/tui-prompts/compare/v0.3.5...v0.3.6) - 2024-03-12

### Other
- *(deps)* bump clap from 4.5.0 to 4.5.2 ([#23](https://github.com/joshka/tui-prompts/pull/23))
- use release-plz from reusable workflows ([#27](https://github.com/joshka/tui-prompts/pull/27))
- add test.yml workflow ([#26](https://github.com/joshka/tui-prompts/pull/26))
- add check.yml workflow ([#24](https://github.com/joshka/tui-prompts/pull/24))

## [0.3.5](https://github.com/joshka/tui-prompts/compare/v0.3.4...v0.3.5) - 2024-02-13

### Other
- *(deps)* bump ratatui from 0.25.0 to 0.26.1 ([#18](https://github.com/joshka/tui-prompts/pull/18))
- *(deps)* bump clap from 4.4.18 to 4.5.0 ([#19](https://github.com/joshka/tui-prompts/pull/19))

## [0.3.4](https://github.com/joshka/tui-prompts/compare/v0.3.3...v0.3.4) - 2024-01-30

### Other
- *(deps)* bump itertools from 0.12.0 to 0.12.1 ([#15](https://github.com/joshka/tui-prompts/pull/15))

## [0.3.3](https://github.com/joshka/tui-prompts/compare/v0.3.2...v0.3.3) - 2024-01-25

### Other
- *(deps)* bump clap from 4.4.8 to 4.4.18 ([#14](https://github.com/joshka/tui-prompts/pull/14))
- *(deps)* bump ratatui from 0.24.0 to 0.25.0 ([#12](https://github.com/joshka/tui-prompts/pull/12))
- *(deps)* bump actions/checkout from 3 to 4 ([#11](https://github.com/joshka/tui-prompts/pull/11))
- Create dependabot.yml

## [0.3.2](https://github.com/joshka/tui-prompts/compare/v0.3.1...v0.3.2) - 2023-12-15

### Other
- *(deps)* bump zerocopy from 0.7.26 to 0.7.31

## [0.3.1](https://github.com/joshka/tui-prompts/compare/v0.3.0...v0.3.1) - 2023-11-17

### Other
- add debug info to examples and update readme

## [0.3.0](https://github.com/joshka/tui-prompts/compare/v0.2.3...v0.3.0) - 2023-11-17

### Other
- deps update and clippy lint fixes
- make TextState fields private

## [0.2.3](https://github.com/joshka/tui-prompts/compare/v0.2.2...v0.2.3) - 2023-07-25

### Other
- simplifiy wrapping

## [0.2.2](https://github.com/joshka/tui-prompts/compare/v0.2.1...v0.2.2) - 2023-07-18

### Other
- add unit tests for soft wrapping single lines
- add release-plz github action

## [0.2.1](https://github.com/joshka/tui-prompts/compare/v0.2.0...v0.2.1) - 2023-07-18

### Other
- add TODO list and key bindings to readme
- add invisible text prompt to readme
- add coverage job to bacon config

## [0.2.0](https://github.com/joshka/tui-prompts/compare/v0.1.1...v0.2.0) - 2023-07-17

### Added
- handle focus and invisible text prompt

### Other
- tweak text example
- Readme badges / license single file
- Update README.md

## [0.1.1](https://github.com/joshka/tui-prompts/compare/v0.1.0...v0.1.1) - 2023-07-11

### Other
- fix cargo.toml categories and keywords
- release

## [0.1.0](https://github.com/joshka/tui-prompts/releases/tag/v0.1.0) - 2023-07-11

### Added
- add text prompt, shared impl
- *(password)* add password prompt

### Fixed
- correct cursor position

### Other
- Revert "chore: add changelog"
- add changelog
- fix readme for release
- remove unrelased ratatui code
- Update readme and add licenses
- improve text example
- replace PasswordPrompt with render_style
- make TextPrompt::new() const
- tidy up text prompt rendering
- extract Status and Symbols to module
- use feat-stylize-all-the-things branch
- add password example to readme
