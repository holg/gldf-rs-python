[![Rust](https://github.com/holg/gldf-rs-python/actions/workflows/rust.yml/badge.svg)](https://github.com/holg/gldf-rs-python/actions/workflows/rust.yml)
# gldf-rs-python
Some more descriptive info in the Cargo.toml file
Added the build.rs mostly bcs of macOS
Release notes:

0.2.3
- added new header definition, because FormatVersion changed
- added Logger to pass into some methods
- added test to completely check and compare to URL based gldf
- bumped up versions of dependencies

0.2.2
- added support meta-information.xml

0.2.1
- added better documentation fo the main page
- for wasm support some refactoring was needed, to use reqwest::blocking

v0.2.0
New:
support for file types of url
support for BOM encoded UTF8 product.xml

WIP python module and binding for the gldf-rs rust library  
the github workflow shall make sure all the wheels are build  
and published to pypi  

install via pip:  

```
pip install gldf-rs-python
```


create venv and maturin

maturin develop


```
import gldf_rs_python

xml = gldf_rs_python.gldf_to_xml('tests/data/test.gldf')

json = gldf_rs_python.gldf_to_json('tests/data/test.gldf')

xml2 = gldf_rs_python.xml_from_json(json)

xml == xml2
```
=> True
