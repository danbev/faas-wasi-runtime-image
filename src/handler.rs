use wasmtime_jit::{ActionOutcome, RuntimeValue, ActionError};
use hyper::server::{Request, Response};

pub trait RequestExtractor {
    fn extract(&self, request: Request) -> Vec<RuntimeValue> ;
}

pub trait ResponseHandler {
    fn result_handler(&self, result: Result<ActionOutcome, ActionError>) -> Response;
}
