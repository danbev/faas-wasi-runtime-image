# FaaS WASI Runtime Image

This image is meant to run in an OpenShift cluster with Knative installed.
It is currently under development and incomplete. 

## Source to Image

This image may also be used as a [source to image builder](https://github.com/openshift/source-to-image).

## Building

To build the image, run the following command:

```console
$ make build
```

You should end up with an image at `oscf/wasi-runtime`.

## Running locally
```console
docker run -p 8080:8080 -ti oscf/wasi-runtime:0.0.1 /bin/bash
```
From a different terminal session:
```console
$ curl http://localhost:8080/data
The answer to your addition was I32(44)$
