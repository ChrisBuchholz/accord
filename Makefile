STABLE_EXAMPLES = 	json
NIGHTLY_EXAMPLES =	rocket

all: build test

build: build-stable \
	build-nightly \
	build-stable-examples \
	build-nightly-examples

test: test-stable test-nightly

build-stable:
	@echo "building stable"
	rustup run stable cargo build

build-stable-examples: $(foreach x, $(STABLE_EXAMPLES), build-stable-example-$(x))

build-stable-example-%:
	@echo "building stable example $%"
	cd $(CURDIR)/examples/$% 
	rustup run stable cargo build
	cd ../..

build-nightly-examples: $(foreach x, $(NIGHTLY_EXAMPLES), build-nightly-example-$(x))

build-nightly:
	@echo "building nightly"
	rustup run nightly cargo build

build-nightly-example-%:
	@echo "building nightly example $*"
	cd $(CURDIR)/examples/$* 
	rustup run nightly cargo build
	cd ../..

test-stable:
	@echo "testing stable"
	rustup run stable cargo test

test-nightly:
	@echo "testing nightly"
	rustup run nightly cargo test
