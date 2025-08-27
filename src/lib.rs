
use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use gldf_rs::gldf::GldfProduct;
use gldf_rs::Logger;
use std::fmt;

// Simple logger implementation for Python integration
#[derive(Clone)]
struct PythonLogger {
    prefix: String,
}

impl PythonLogger {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for PythonLogger {
    fn log(&self, message: &str) {
        println!("[{}] {}", self.prefix, message);
    }
}

impl fmt::Display for PythonLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PythonLogger({})", self.prefix)
    }
}

// Helper function to convert anyhow::Error to PyErr
fn anyhow_to_pyerr(err: anyhow::Error) -> PyErr {
    PyException::new_err(format!("GLDF Error: {}", err))
}

#[pyfunction]
fn gldf_to_xml(path: &str) -> PyResult<String> {
    let logger = PythonLogger::new("gldf_to_xml");
    logger.log(&format!("Loading GLDF file: {}", path));
    
    let loaded = GldfProduct::load_gldf(path)
        .map_err(anyhow_to_pyerr)?;
    
    logger.log("Converting to XML");
    loaded.to_xml()
        .map_err(anyhow_to_pyerr)
}

#[pyfunction]
fn gldf_to_json(path: &str) -> PyResult<String> {
    let logger = PythonLogger::new("gldf_to_json");
    logger.log(&format!("Loading GLDF file: {}", path));
    
    let loaded = GldfProduct::load_gldf(path)
        .map_err(anyhow_to_pyerr)?;
    
    logger.log("Converting to JSON");
    loaded.to_json()
        .map_err(anyhow_to_pyerr)
}

#[pyfunction]
fn json_from_xml_str(xml_str: &str) -> PyResult<String> {
    let logger = PythonLogger::new("json_from_xml_str");
    logger.log("Parsing XML string");
    
    let loaded = GldfProduct::from_xml(&xml_str.to_string())
        .map_err(anyhow_to_pyerr)?;
    
    logger.log("Converting to JSON");
    loaded.to_json()
        .map_err(anyhow_to_pyerr)
}

#[pyfunction]
fn xml_from_json(json_str: &str) -> PyResult<String> {
    let logger = PythonLogger::new("xml_from_json");
    logger.log("Parsing JSON string");
    
    let loaded = GldfProduct::from_json(json_str)
        .map_err(anyhow_to_pyerr)?;
    
    logger.log("Converting to XML");
    loaded.to_xml()
        .map_err(anyhow_to_pyerr)
}

// Async versions will be added in a future update when pyo3-asyncio compatibility is resolved
// For now, we'll focus on improved error handling and logging

// #[pyfunction]
// fn gldf_to_xml_async(py: Python, path: String) -> PyResult<&PyAny> {
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let logger = PythonLogger::new("gldf_to_xml_async");
//         logger.log(&format!("Loading GLDF file async: {}", path));
//         
//         let loaded = GldfProduct::load_gldf(&path)
//             .map_err(anyhow_to_pyerr)?;
//         
//         logger.log("Converting to XML async");
//         loaded.to_xml()
//             .map_err(anyhow_to_pyerr)
//     })
// }

/// A Python module implemented in Rust.
#[pymodule]
fn gldf_rs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    // Sync functions with improved error handling and logging
    m.add_function(wrap_pyfunction!(gldf_to_xml, m)?)?;
    m.add_function(wrap_pyfunction!(gldf_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(xml_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(json_from_xml_str, m)?)?;
    
    // Async functions will be added in a future update
    // when pyo3-asyncio compatibility is resolved
    
    Ok(())
}