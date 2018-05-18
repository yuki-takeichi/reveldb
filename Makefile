IMAGE=reveldb-devel

build:
	docker build -t $(IMAGE) .
run:
	docker run --security-opt seccomp:unconfined \
		-v `pwd`/:/home/reveldb-devel/ \
		-it $(IMAGE) \
		/bin/bash
