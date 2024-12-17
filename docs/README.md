# Eclipse zenoh C API documentation

zenoh-c API documentation is available on [Read the Docs](https://zenoh-c.readthedocs.io/en/latest/index.html).

-------------------------------

## How to build it

```bash
cargo check --all-features
cd docs
doxygen
sphinx-build -b html . _build/html
```
