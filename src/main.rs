extern crate hyper;
extern crate futures;

use std::env;
use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::server::{Http, Service, Request, Response};

use cranelift_codegen::settings;
use cranelift_native;
use std::fs::File;
use std::io::Read;
use wasmtime_jit::{Context, ActionOutcome, RuntimeValue, ActionError};

mod add;
mod handler;
use crate::handler::RequestExtractor;
use crate::handler::ResponseHandler;

//#[derive(Debug)]
struct WasmExecutor {
    function_name: String,
    module_path: String,
    module_binary: Vec<u8>,
}

impl WasmExecutor {
    fn new(function_name: String, module_path: String, module_binary: Vec<u8>) -> WasmExecutor {
        WasmExecutor { function_name, module_path, module_binary }
    }
}
/*
pub trait RequestExtractor {
    fn extract(&self, request: Request) -> Vec<RuntimeValue> ;
}

pub trait ResponseHandler {
    fn result_handler(&self, result: Result<ActionOutcome, ActionError>) -> Response;
}
*/

impl Service for WasmExecutor {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/data") => {
                let isa_builder = cranelift_native::builder().unwrap();
                let flag_builder = settings::builder();
                let isa = isa_builder.finish(settings::Flags::new(flag_builder));
                let mut context = Context::with_isa(isa);

                let mut instance = context.instantiate_module(None, &self.module_binary).unwrap();

                let args = &self.extract(req);
                let result = context.invoke(&mut instance, &self.function_name, &args);
                self.result_handler(result)
            },
            _ => Response::new().with_status(StatusCode::NotFound),})
        }
}

fn main() {
    let port = env::var("PORT").expect("PORT environment variable not set");
    let addr_port = format!("0.0.0.0:{}", port);
    let addr = addr_port.parse().unwrap();
    let function_name = env::var("FUNCTION_NAME").expect("FUNCTION_NAME environment variable not set");
    let module_path = env::var("MODULE_PATH").expect("MODULE_PATH environment variable not set");
    println!("WASI Runtime started. Port: {}, Module path: {}", port, module_path);
    let binary: Vec<u8> = read_module(&module_path);

    let server = Http::new().bind(&addr, move || Ok(WasmExecutor::new(function_name.clone(), module_path.clone(), binary.clone()))).unwrap();
    server.run().unwrap();
}

fn read_module(module_path: &str) -> Vec<u8> {
    let mut module_file = File::open(module_path).expect("wasm not found");
    let mut binary: Vec<u8> = Vec::new();
    module_file.read_to_end(&mut binary).unwrap();
    return binary;
}
