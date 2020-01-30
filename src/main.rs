extern crate wasm_executor;
extern crate url;

use wasmtime_jit::{ActionOutcome, RuntimeValue, ActionError};
use hyper::server::{Response};
use hyper::header::ContentLength;

use wasm_executor::RequestExtractor;
use wasm_executor::ResponseHandler;
use wasm_executor::Context;

struct ReqHandler { }

impl RequestExtractor for ReqHandler {
    fn extract_args(&self, _context: Context) -> Vec<RuntimeValue> {
        println!("CloudEvent: {:?}", _context);
        let mut vec = Vec::new();
        vec.push(RuntimeValue::I32(4));
        vec.push(RuntimeValue::I32(14));
        return vec;
    }
}

struct ResHandler { }
impl ResponseHandler for ResHandler {
    fn create_response(&self,
                       result: Result<ActionOutcome, ActionError>,
                       module_path: &str,
                       function_name: &str) -> Response {
        let body = match result.unwrap() {
            ActionOutcome::Returned { values } => 
                format!("module: {}, function: {}, returned {:#}", module_path, function_name, values[0]).to_string().into_bytes(),
            ActionOutcome::Trapped { message } => 
                format!("Trap from within function: {}", message).to_string().into_bytes(),
        };
        println!("WASM Response: {:#?}", String::from_utf8(body.clone()));
        return Response::new().with_header(ContentLength(body.len() as u64)).with_body(body)
    }
}

fn create_request_handler() -> Box::<dyn RequestExtractor> {
    Box::new(ReqHandler{})
}

fn create_response_handler() -> Box::<dyn ResponseHandler> {
    Box::new(ResHandler{})
}

fn main() {
    wasm_executor::start(create_request_handler, create_response_handler);
}
