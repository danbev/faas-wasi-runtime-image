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
```console
$ docker images
REPOSITORY         TAG    IMAGE ID     CREATED        SIZE
oscf/wasi-runtime  0.0.1  f302f05fd2b4 56 seconds ago 9.77MB
```

## Running locally
```console
docker run -p 8080:8080 -ti oscf/wasi-runtime:0.0.1 /bin/bash
```
From a different terminal session:
```console
$ curl http://localhost:8080/data
The answer to your addition was I32(44)$

To stop the container:
```console
$ docker ps
$ docker stop <CONTAINER_ID>
```
