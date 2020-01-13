use wasmtime_jit::{ActionOutcome, RuntimeValue, ActionError};
use hyper::server::{Request, Response};
use hyper::header::ContentLength;

use crate::RequestExtractor;
use crate::ResponseHandler;
use crate::WasmExecutor;

impl RequestExtractor for WasmExecutor {
    fn extract(&self, _request: Request) -> Vec<RuntimeValue> {
        let mut vec = Vec::new();
        vec.push(RuntimeValue::I32(42));
        vec.push(RuntimeValue::I32(2));
        return vec;
    }
}

impl ResponseHandler for WasmExecutor {
    fn result_handler(&self, result: Result<ActionOutcome, ActionError>) -> Response {
        let body = match result.unwrap() {
            ActionOutcome::Returned { values } => format!("{} returned {:#}", &self.module_path, values[0]).to_string().into_bytes(),
            ActionOutcome::Trapped { message } => format!("Trap from within function: {}", message).to_string().into_bytes(),
        };
        return Response::new().with_header(ContentLength(body.len() as u64)).with_body(body)
    }
}
