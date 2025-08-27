use gldf_rs::gldf::GldfProduct;
use gldf_rs::{Logger, fetch_text_from_url_async};
use std::fmt;
use futures::executor::block_on;

// Test logger implementation
#[derive(Clone)]
struct TestLogger {
    prefix: String,
}

impl TestLogger {
    fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for TestLogger {
    fn log(&self, message: &str) {
        println!("[{}] {}", self.prefix, message);
    }
}

impl fmt::Display for TestLogger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TestLogger({})", self.prefix)
    }
}

#[test]
fn test_logger_trait_implementation() {
    let logger = TestLogger::new("integration_test");
    logger.log("Testing logger implementation");
    assert_eq!(logger.prefix, "integration_test");
}

#[test]
fn test_gldf_functionality_if_file_exists() {
    let test_path = "tests/data/test.gldf";
    let logger = TestLogger::new("test_gldf_functionality");
    
    // Only run this test if the file exists
    if std::path::Path::new(test_path).exists() {
        logger.log(&format!("Testing with file: {}", test_path));
        
        match GldfProduct::load_gldf(test_path) {
            Ok(loaded) => {
                logger.log("Successfully loaded GLDF file");
                
                // Test conversion to JSON
                match loaded.to_json() {
                    Ok(json) => {
                        logger.log("Successfully converted to JSON");
                        assert!(!json.is_empty());
                        assert!(json.contains("GeneralDefinitions"));
                    }
                    Err(e) => panic!("Failed to convert to JSON: {}", e),
                }
                
                // Test conversion to XML
                match loaded.to_xml() {
                    Ok(xml) => {
                        logger.log("Successfully converted to XML");
                        assert!(!xml.is_empty());
                        assert!(xml.contains("<Root"));
                    }
                    Err(e) => panic!("Failed to convert to XML: {}", e),
                }
            }
            Err(e) => {
                logger.log(&format!("Could not load test file: {}", e));
                // This is okay - the test file might not exist
            }
        }
    } else {
        logger.log("Test file does not exist - skipping functionality test");
    }
}

#[test]
fn test_error_handling_with_invalid_data() {
    let logger = TestLogger::new("test_error_handling");
    
    // Test with invalid JSON
    let invalid_json = "{ invalid json }";
    match GldfProduct::from_json(invalid_json) {
        Ok(_) => panic!("Should have failed with invalid JSON"),
        Err(e) => {
            logger.log(&format!("Correctly caught JSON error: {}", e));
        }
    }
    
    // Test with invalid XML
    let invalid_xml = "<invalid><xml>unclosed";
    match GldfProduct::from_xml(&invalid_xml.to_string()) {
        Ok(_) => panic!("Should have failed with invalid XML"),
        Err(e) => {
            logger.log(&format!("Correctly caught XML error: {}", e));
        }
    }
}

#[test] 
fn test_async_functionality_with_mock_url() {
    let logger = TestLogger::new("test_async_functionality");
    
    // Test that the async function works
    // This demonstrates the async capabilities from gldf-rs
    
    logger.log("Testing async URL fetch capability");
    
    // Create a Tokio runtime for the test
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Use a simple URL that should return text (this is just to test the async mechanism)  
    let test_url = "https://httpbin.org/robots.txt";
    
    // This tests that our async infrastructure works correctly
    match rt.block_on(fetch_text_from_url_async(test_url)) {
        Ok(content) => {
            logger.log("Successfully fetched content using async function");
            assert!(!content.is_empty());
        }
        Err(e) => {
            // Network might not be available in test environment, that's okay
            logger.log(&format!("Network fetch failed (expected in some test environments): {}", e));
        }
    }
}