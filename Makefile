IMAGE_TAG    = 0.0.1
IMAGE_NAME   = oscf/wasi-runtime
DOCKER_IMAGE = docker.io/$(IMAGE_NAME):$(IMAGE_TAG)
QUAY_IMAGE   = quay.io/$(IMAGE_NAME):$(IMAGE_TAG)
TEST_IMAGE   = $(IMAGE_NAME):candidate

.PHONY: build test clean

build:
	docker build --build-arg MODULE_NAME=add.wasm -t $(DOCKER_IMAGE) .
	# docker build -t $(QUAY_IMAGE) .

test:
	./run-test.sh $(TEST_IMAGE)

clean:
	docker rmi -f `docker images $(TEST_IMAGE) -q`
	docker rmi -f `docker images $(IMAGE_NAME) -q`

push:
	# docker push $(QUAY_IMAGE)
	docker push $(DOCKER_IMAGE)
