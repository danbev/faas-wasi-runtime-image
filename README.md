# FaaS WASI Runtime Image
__This project is investigation/playground to figure out what might work__

The idea here is to enable WebAssembly modules, possibly using the WebAssembly
System Interface (WASI), to be executed as OpenShift Cloud Function. An end
user would have a WASM module they would like to expose as a function. This
WASM module could either be a module written and bundled with the users
project, or could be a WASM module in the Web Assembly Package Manager
([wapm](https://wapm.io)) or in any other package manager, for example Node.js
Package Manager ([npm](https://www.npmjs.com/)).

The idea is that an end user in this case would write the code needed to extract
any required parameters the `.wasm` module takes from the HTTP request, and take
the result from the execution of the `.wasm` module and place it into the HTTP
response. Exactly how this would look still needs to be throught through.


## Building

To build the image, run the following command:
```console
$ make build
```

You should end up with an image at `oscf/wasi-runtime`:
```console
$ docker images
REPOSITORY         TAG    IMAGE ID     CREATED        SIZE
oscf/wasi-runtime  0.0.1  4e5d82b8c6b8 3 minutes ago  13.9MB
```

### WASM test module
The .wasm module used is located in `module/add.wasm`, and looks like this:
```console
$ wasm2wat add.wasm
(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    get_local 0
    get_local 1
    i32.add)
  (export "add" (func 0)))
```

## Running locally
```console
$ docker run -p 8080:8080 -ti oscf/wasi-runtime:0.0.1
WASI Runtime started. Module name: add.wasm
```
From a different terminal session:
```console
$ curl http://localhost:8080/data
/home/wasi/module/add.wasm returned 44: i32
```

To stop the container:
```console
$ docker ps
$ docker stop <CONTAINER_ID>
```
The go server can be run locally without using Docker with the following
command:
```console
$ FUNCTION_NAME=add PORT=8080 MODULE_PATH=./module/add.wasm cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/faas-wasm-runtime-image`
WASI Runtime started. Port: 8080, Module path: ./module/add.wasm
```
And then from a second terminal you can call the service:
```console
$ curl http://localhost:8080/data
/home/wasi/module/add.wasm returned 44: i32
```
