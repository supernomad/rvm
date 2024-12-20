.PHONY: default build check coverage coverage-html docker amd64-ci arm64-ci

default: check

build:
	cargo build --all

check:
	cargo test

coverage:
	cargo llvm-cov

coverage-html:
	cargo llvm-cov --html

docker:
	docker build -t rvmd:latest -f build/Dockerfile .

amd64-ci:
	/bin/bash ./build/compile.sh amd64

arm64-ci:
	/bin/bash ./build/compile.sh arm64
