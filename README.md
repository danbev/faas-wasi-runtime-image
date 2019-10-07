# FaaS WASI Runtime Image
This image is meant to run in an OpenShift cluster with Knative installed.

It is currently under development and very incomplete.

The idea here is to enable WebAssembly modules, possibly using the WebAssembly
System Interface (WASI), to be executed as OpenShift Cloud Function. An end
user could have a WASM module they would like to expose as a function.

## Source to Image

This image may also be used as a [source to image builder](https://github.com/openshift/source-to-image).

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

## Running locally
```console
$ docker run -p 8080:8080 -ti oscf/wasi-runtime:0.0.1
```
From a different terminal session:
```console
$ curl http://localhost:8080/data
add.wasm returned 44
```

To stop the container:
```console
$ docker ps
$ docker stop <CONTAINER_ID>
```
