extern crate wasm_executor;
extern crate url;

use wasmtime_jit::{ActionOutcome, RuntimeValue, ActionError};
use hyper::server::{Request, Response};
use hyper::header::ContentLength;
use url::form_urlencoded;

use wasm_executor::handler::RequestExtractor;
use wasm_executor::handler::ResponseHandler;

struct ReqHandler { }

impl RequestExtractor for ReqHandler {
    fn extract_args(&self, request: Request) -> Vec<RuntimeValue> {
        let params = form_urlencoded::parse(request.uri().query().unwrap().as_bytes());
        let mut vec = Vec::new();
        for p in params.into_iter() {
          vec.push(RuntimeValue::I32(p.1.parse::<i32>().unwrap()));
        }
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
