# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/probaku1234/cli_prompt_rs/compare/v0.2.0...v0.2.1) - 2024-02-13

### Added
- change name of spinner function
- add new example for spinner functionality
- add finish_message as parameter
- add cursor_hidden, current_cursor for better testing
- print success message when task done
- add spinner functionality for unstable feature
- exposure  prompt_multi_select_with_max_choice_num as public
- add new error type for multi_select
- implement prompt_multi_selected_with_max_num
- return error when options is empty
- change return type using CliPromptError
- implement custom error for entire lib

### Fixed
- wrap the code with unstable feature to fix build fail when unstable feature not enabled
- add mock clear_line function to run test
- change wrong path of doc
- remove redundant loop and make return joinhandler
- tag spinner test for unstable feature
- add custom error for spinner functionality
- add missing docs
- add missing import
- implement Display and std Error for CliPromptError

### Other
- edit docs
- wrap import with feature flag for removing warning
- add more test
- add more test for mock_term
- add more tests for mock_term
- auto format from fmt
- add mocking cursor functionality and apply it to existing tests
- update docs for spinner
- change time of spinner
- reformatting
- reformatting
- remove redundant code
- add docs for spinner error
- tag s_success as unstable
- add underscore to unused variables
- add docs for spinner error
- add doc for spinner
- handle error from join
- auto format from fmt
- add unstable feature
- add steps for building, testing unstable feature
- rename function name
- add test for spinner
- group import statements with unstable feature
- change signature of spinner_example
- auto format by fmt
- auto format from fmt
- move mock_term to separate file
- update doc
- format
- add doc for InvalidMaxChoiceNumError
- add test for prompt_multi_select_with_max_choice_num
- add arguments section on rust doc
- return error when invalid max_choice_num given
- add docs for error return
- expose CliPromptError public
- add docs for CliPromptError
- auto formatting from fmt
- add test for returning empty options error
- change unwrap to ? operator so that it won't panic
- change get max length of split messages to more idiomatic way in print_note
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/probaku1234/cli_prompt_rs/compare/v0.1.1...v0.1.2) - 2023-10-11

### Added
- add prompt_multi_select
- add print note
- print empty line at end of each operation

## [0.1.1](https://github.com/probaku1234/cli_prompt_rs/compare/v0.1.0...v0.1.1) - 2023-10-09

### Other
- update README file
- update README.md file

## [0.1.0](https://github.com/probaku1234/cli_prompt_rs/releases/tag/v0.1.0) - 2023-10-07

### Added
- intro
- outro
- cancel
- log
- prompt_text
- prompt_confirm
- prompt_select
