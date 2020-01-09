extern crate hyper;
extern crate futures;

use std::env;
use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use cranelift_codegen::settings;
use cranelift_native;
use std::fs::File;
use std::io::Read;
use wasmtime_jit::{ActionOutcome, Context, RuntimeValue};

//#[derive(Debug)]
struct WasmExecutor {
    module_path: String,
    module_binary: Vec<u8>,
}

impl WasmExecutor {
    fn new(module_path: String, module_binary: Vec<u8>) -> WasmExecutor {
        WasmExecutor { module_path, module_binary }
    }
}

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

                let args = [RuntimeValue::I32(42), RuntimeValue::I32(2)];
                let result = context.invoke(&mut instance, "add", &args);
                let body = match result.unwrap() {
                    ActionOutcome::Returned { values } => format!("{} returned {:#}", &self.module_path, values[0]).to_string().into_bytes(),
                    ActionOutcome::Trapped { message } => format!("Trap from within function: {}", message).to_string().into_bytes(),
                };
                Response::new().with_header(ContentLength(body.len() as u64)).with_body(body)
            },
            _ => Response::new().with_status(StatusCode::NotFound),})
        }
}

fn main() {
    let port = env::var("PORT").expect("PORT environment variable not set");
    let addr_port = format!("0.0.0.0:{}", port);
    let addr = addr_port.parse().unwrap();
    let module_path = get_module_path();
    println!("WASI Runtime started. Port: {}, Module path: {}", port, module_path);
    let binary: Vec<u8> = read_module(&module_path);


    let server = Http::new().bind(&addr, move || Ok(WasmExecutor::new(module_path.clone(), binary.clone()))).unwrap();
    server.run().unwrap();
}

fn get_module_path() -> String {
    let module_dir = env::var("MODULE_DIR").expect("MODULE_DIR environment variable was not set");
    let module_name = env::var("MODULE_NAME").expect("MODULE_NAME environment variable not set");
    module_dir.to_owned() + "/" +  &module_name
}

fn read_module(module_path: &str) -> Vec<u8> {
    let mut module_file = File::open(module_path).expect("wasm not found");
    let mut binary: Vec<u8> = Vec::new();
    module_file.read_to_end(&mut binary).unwrap();
    return binary;
}
