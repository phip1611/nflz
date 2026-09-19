# v1.1.0 (2026-09-19)
- new: ambiguous file endings (such as a mix of `jpg`, `JPG`, and `HEIC`) can now be ignored.
  The CLI lists them and asks for confirmation with an uppercase `Y`; the library got
  `NFLZAssistant::with_ignore_ambiguous_suffixes()`. Ambiguous prefixes remain an error.
- the CLI now checks whether the renaming is possible **before** it asks for confirmation

# v1.0.2 (2023-02-17)
- dependency updates

# v1.0.1 (2022-09-25)
Minor improvements and bugfixes.

# v1.0.0 (2022-06-15)
- better public API
- much improved code quality
- better documentation + minimal example inside the repository
