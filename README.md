# gldf-rs-python
WIP python module and binding for the gldf-rs rust library

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
