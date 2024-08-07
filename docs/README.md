# Eclipse zenoh C API documentation

zenoh-c API documentation is available on [Read the Docs](https://zenoh-c.readthedocs.io/en/latest/index.html).

-------------------------------
## How to build it

  -- Ubuntu --

  ```bash
  $ doxygen Doxyfile
  $ cd docs
  $ sphinx-build -b html . _build/html
  ```

  -- MacOS --

  Update conf.py *Config.set_library_file* with the path to your *libclang.dylib* file.

  ```bash
  $ doxygen Doxyfile
  $ cd docs
  $ sphinx-build -b html . _build/html
  ```
