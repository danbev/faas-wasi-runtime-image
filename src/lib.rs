use futures::future::FutureResult;
use hyper::server::{Http, Request, Response, Service};
use hyper::{header::Headers, Body, Get, Method, Post, StatusCode};
use hyper::header::ContentLength;
use std::{env, fs::File, io::Read};
use http::HeaderMap;
use wasmtime::{Engine, Store, Val, Module, Instance, Trap};

use cloudevents::v02::CloudEvent;

pub trait RequestExtractor {
    fn extract_args(&self, context: &Context) -> Vec<Val>;
}

pub struct WasmResponse {
    pub body: Vec<u8>,
    pub headers: Option<HeaderMap>,
}

pub trait ResponseHandler {
    fn create_response(
        &self,
        context: &Context,
        result: Result<Box<[Val]>, Trap>
    ) -> WasmResponse;
}

pub struct WasmExecutor {
    function_name: String,
    module_path: String,
    request_handler: Box<dyn RequestExtractor>,
    response_handler: Box<dyn ResponseHandler>,
    module: Module,
}

impl WasmExecutor {
    fn new(
        function_name: String,
        module_path: String,
        module_binary: Vec<u8>,
        request_handler: Box<dyn RequestExtractor>,
        response_handler: Box<dyn ResponseHandler>,
    ) -> WasmExecutor {
        let store = Store::new(&Engine::default());
        let module = Module::new(&store, module_binary).unwrap();
        WasmExecutor {
            function_name,
            module_path,
            request_handler,
            response_handler,
            module,
        }
    }
}

#[derive(Debug)]
pub struct Context<'a> {
    pub module_path: String,
    pub function_name: String,
    pub user: String,
    pub method: Method,
    pub headers: Headers,
    pub path: String,
    pub query: Option<&'a str>,
    pub body: Option<&'a Body>,
    pub cloudevent: Option<CloudEvent>,
}

impl Service for WasmExecutor {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match req.method() {
            &Get | &Post => {
                let instance = Instance::new(&self.module, &[]).unwrap();
                let func = instance.get_export(&self.function_name).unwrap().func().unwrap();

                let context = create_context(&req, &self);
                let args = self.request_handler.extract_args(&context);

                let result = func.call(&args);
                
                let wasm_response = self.response_handler.create_response(
                    &context,
                    result);
                Response::new()
                    .with_header(ContentLength(wasm_response.body.len() as u64))
                    .with_body(wasm_response.body)
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

fn create_context<'a>(req: &'a Request, executor: &WasmExecutor) -> Context<'a> {
    Context {
        module_path: executor.module_path.clone(),
        function_name: executor.function_name.clone(),
        user: String::from(""),
        method: req.method().clone(),
        headers: req.headers().clone(),
        path: req.path().to_string(),
        query: req.query(),
        body: req.body_ref(),
        cloudevent: None,
    }
}

pub fn start(
    req_handler: fn() -> Box<dyn RequestExtractor>,
    res_handler: fn() -> Box<dyn ResponseHandler>,
) {
    let port = env::var("PORT").expect("PORT environment variable not set");
    let addr_port = format!("0.0.0.0:{}", port);
    let addr = addr_port.parse().unwrap();
    let function_name =
        env::var("FUNCTION_NAME").expect("FUNCTION_NAME environment variable not set");
    let module_path = env::var("MODULE_PATH").expect("MODULE_PATH environment variable not set");
    println!(
        "WASI Runtime started. Port: {}, Module path: {}",
        port, module_path
    );
    let binary: Vec<u8> = read_module(&module_path);
    let server = Http::new()
        .bind(&addr, move || {
            Ok(WasmExecutor::new(
                function_name.clone(),
                module_path.clone(),
                binary.clone(),
                req_handler(),
                res_handler(),
            ))
        })
        .unwrap();
    server.run().unwrap();
}

pub fn read_module(module_path: &str) -> Vec<u8> {
    let mut module_file = File::open(module_path).expect("wasm not found");
    let mut binary: Vec<u8> = Vec::new();
    module_file.read_to_end(&mut binary).unwrap();
    return binary;
}
