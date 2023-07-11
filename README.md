# Fabriko

Easy factories for your test fixtures.
Inspired by `factory_bot`, but all in safe Rust.
Flexible so you can use it too.

Supports:
* Associations between resources
* Dependant attributes
* Mixins to share attributes between factories
* Declarative bundles to easily share your test setup between your test fixtures

## TODO:

### v0.1

* "Context wrapper" to erase the need to mutably borrow & provide the context everytime
* "Factory registry" (linked to the context wrapper)
* Documentation - most importantly for the core crate `fabriko`
* Proc Macro for `WithIdentifier`
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

* Async
