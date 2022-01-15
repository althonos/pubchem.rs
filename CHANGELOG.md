# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased
[Unreleased]: https://github.com/althonos/pubchem.rs/compare/v0.1.1...HEAD

## [v0.1.1] - 2021-01-15
[v0.1.1]: https://github.com/althonos/pubchem.rs/compare/v0.1.0...v0.1.1

### Added
- `pubchem::Compound::synonyms` to retrieve synonym names for a compound.
- `pubchem::Compounds` to query properties from multiple compounds.
- `Deref`, `DerefMut` and `IntoIterator` implementations for `pubchem::model::rest::PropertyTable`.

## [v0.1.0] - 2021-01-15
[v0.1.0]: https://github.com/althonos/pubchem.rs/compare/7f3bb8b...v0.1.0

Initial release.
