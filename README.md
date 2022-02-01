# wiring-py
Wiring for Python

1. Project Setup
    1. `git clone git@github.com:ontodev/wiring.py.git`
    2. `cd wiring.py`
    3. `git clone git@github.com:ontodev/wiring.rs.git`
    4. `mv python_module.rs wiring.rs/src/`
    5. `cd wiring.rs`
    6. add the line `mod python_module;` to the end of file `src/lib.rs`
    7. modify `Cargo.toml` so that it contains the following lines: 
    ```
    [lib]
    name="wiring_rs"
    crate-type = ["cdylib", "lib"]

    [[bin]]
    name = "mybin"
    path = "src/main.rs"

    [dependencies.pyo3]
    version = "0.14.5"
    # "extension-module" tells pyo3 we want to build an extension module (skips linking against libpython.so)
    # "abi3-py36" tells pyo3 (and maturin) to build using the stable ABI with minimum Python version 3.6
    features = ["extension-module", "abi3-py36"]
    ```
2. Installing Maturin 

    1. `python3 -m venv .venv`
    2. `source .venv/bin/activate`
    3. `pip install -U pip maturin`

3. Build
    1. `maturin develop` for local installation
    2. `maturin build` for creating a wheel

4. Test
    1. `cd ..`
    2. `python demo.py`


