
use pyo3::prelude::*;
use pyo3::exceptions::PyException;
use gldf_rs::gldf::GldfProduct;
use gldf_rs::{Logger, fetch_text_from_url_async};
use std::fmt;
use futures::executor::block_on;

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

// Async functionality wrapped for Python - uses blocking executor
// This demonstrates how the underlying async capabilities can be exposed
#[pyfunction]
fn fetch_and_convert_to_json(url: &str) -> PyResult<String> {
    let logger = PythonLogger::new("fetch_and_convert_to_json");
    logger.log(&format!("Fetching content from URL: {}", url));
    
    // Use the async function from gldf-rs with block_on
    let content = block_on(fetch_text_from_url_async(url))
        .map_err(|e| PyException::new_err(format!("Failed to fetch URL: {}", e)))?;
    
    logger.log("Content fetched, parsing as GLDF XML");
    
    let loaded = GldfProduct::from_xml(&content)
        .map_err(anyhow_to_pyerr)?;
    
    logger.log("Converting to JSON");
    loaded.to_json()
        .map_err(anyhow_to_pyerr)
}

/// A Python module implemented in Rust.
#[pymodule]
fn gldf_rs_python(_py: Python, m: &PyModule) -> PyResult<()> {
    // Sync functions with improved error handling and logging
    m.add_function(wrap_pyfunction!(gldf_to_xml, m)?)?;
    m.add_function(wrap_pyfunction!(gldf_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(xml_from_json, m)?)?;
    m.add_function(wrap_pyfunction!(json_from_xml_str, m)?)?;
    
    // Async functionality wrapped in blocking executor
    m.add_function(wrap_pyfunction!(fetch_and_convert_to_json, m)?)?;
    
    Ok(())
}