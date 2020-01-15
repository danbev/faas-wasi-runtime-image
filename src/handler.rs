use wasmtime_jit::{ActionOutcome, RuntimeValue, ActionError};
use hyper::server::{Request, Response};

pub trait RequestExtractor {
    fn extract_args(&self, request: Request) -> Vec<RuntimeValue>;
}

pub trait ResponseHandler {
    fn create_response(&self,
                       result: Result<ActionOutcome, ActionError>,
                       module_path: &str,
                       function_name: &str) -> Response;
}
