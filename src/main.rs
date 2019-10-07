extern crate wasmi;

use std::env;
use std::fs::File;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};

extern crate hyper;
extern crate futures;

use futures::future::FutureResult;
use hyper::{Get, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

//#[derive(Debug)]
struct WasmExecutor {
    module_name: String,
}

impl WasmExecutor {
    fn new(module_name: String) -> WasmExecutor {
        WasmExecutor { module_name }
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
                let mut buffer = Vec::new();
                {
                    let mut f = File::open("../module/".to_owned() + &self.module_name).expect("wasm not found");
                    f.read_to_end(&mut buffer).expect("wasm read error");
                }
                let module = wasmi::Module::from_buffer(buffer).expect("create Module error");
                let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
                    .expect("Failed to instantiate WASM module")
                    .assert_no_start();
                let mut args = Vec::<RuntimeValue>::new();
                args.push(RuntimeValue::from(42));
                args.push(RuntimeValue::from(2));

                let result: Option<RuntimeValue> =
                    instance.invoke_export("add", &args, &mut NopExternals).expect("invoke error");
                let b = match result {
                    Some(RuntimeValue::I32(v)) => format!("add.wasm returned {}", v).to_string().into_bytes(),
                    Some(RuntimeValue::I64(v)) => format!("add.wasm returned {}", v).to_string().into_bytes(),
                    Some(RuntimeValue::F32(v)) => format!("add.wasm returned {:?}", v).to_string().into_bytes(),
                    Some(RuntimeValue::F64(v)) => format!("add.wasm returned {:?}", v).to_string().into_bytes(),
                    None => String::from("Failed to get a result from wasm invocation")
                            .to_string().into_bytes(),
                };

                Response::new().with_header(ContentLength(b.len() as u64))
                .with_body(b)
            },
            _ => Response::new().with_status(StatusCode::NotFound),})
        }
}

fn main() {
    let port = env::var("PORT").expect("PORT environment variable not set");
    let addr_port = format!("0.0.0.0:{}", port);
    let addr = addr_port.parse().unwrap();
    let module_name = env::var("MODULE_NAME").expect("MODULE_NAME environment variable not set");
    println!("WASI Runtime started. Module name: {}", module_name);
    let server = Http::new().bind(&addr, move || Ok(WasmExecutor::new(module_name.clone()))).unwrap();
    server.run().unwrap();
}
