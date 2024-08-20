# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](Https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](Https://semver.org/spec/v2.0.0.html).

### [v0.1.0]

#### Added

- Created a parser for the first line to determine, http version, uri and request method
- Proper http 0.9 support.
- Custom Option type called Polar to efficiently handle the http error codes.
- Custom pages for errors (with changing messages), currently sends it as http 0.9. Will be fixed when http 1.0
  gets implemented.
- An [ideas page](./ideas.md) where I added some interesting ideas in that might get implemented. (create an
  issue/pr if you know something cool :D!).
- Probably some other things lmao.

#### Changed

- Moved macros.rs to needed.rs to fit its new use.

#### Removed

- Removed all the old code and basically rewrote everything http related lmao.

---

---

### Template:

### [vx.y.z]

#### Added

#### Fixed

#### Changed

#### Removed