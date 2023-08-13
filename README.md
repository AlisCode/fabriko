# Fabriko

A Rust library to easily write safe and flexible factories for your test fixtures.
Inspired by Ruby's `factory_bot`, but all in safe Rust.
Flexible so you can use it too.

Supports:
* Associations between resources
* Dependant attributes
* Mixins to share attributes between factories
* Declarative bundles to easily share your test setup between your test fixtures

## TODO:

### v0.1

* One-to-one relationship
* Documentation - most importantly for the core crate `fabriko`
* Cleanup
* Proper README.md
* Tests
    * Unit tests for proc macros namely
    * UI tests
* More/clearer examples
* CI
* Changelog
* Release :)

### v0.2

* Diesel integration (?)
* Async
    * SQLx integration (?)
* Bevy integration (?)
