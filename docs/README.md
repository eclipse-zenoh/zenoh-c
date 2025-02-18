# Eclipse zenoh C API documentation

zenoh-c API documentation is available on [Read the Docs](https://zenoh-c.readthedocs.io/en/latest/index.html).

-------------------------------

## How to build it

1. generate headers with all API functions/structures enabled
2. run doxygen to parse headers
3. generate documentation

```bash
cargo check --all-features
cd docs
doxygen
sphinx-build -b html . _build/html
```
