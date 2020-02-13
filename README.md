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

This project contains a library that end users can use to implements this and
also contains a base container image to be used in a FAAS environment.

## Building

To build the base image, run the following command:
```console
$ docker build -t dbevenius/wasm-base-image . 
```

Then to publish:
```console
$ docker login
$ docker push dbevenius/wasm-base-image
```

### Publishing
```console
$ cargo login <api access token>
$ cargo publish --dry-run
```
