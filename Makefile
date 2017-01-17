STABLE_EXAMPLES = 	json
NIGHTLY_EXAMPLES =	rocket

all: build test

build: build-stable \
	build-nightly \
	build-examples

test: test-stable \
	test-nightly \
	test-examples

build-stable:
	@echo "building stable"
	@rustup run stable cargo build

build-examples: build-stable-examples build-nightly-examples

build-stable-examples: $(foreach x, $(STABLE_EXAMPLES), build-stable-example-$(x))

build-stable-example-%: $(CURDIR)/examples/stable/%
	@echo "building stable example $*"
	@cd $(CURDIR)/examples/stable/$*; rustup run stable cargo build

build-nightly-examples: $(foreach x, $(NIGHTLY_EXAMPLES), build-nightly-example-$(x))

build-nightly:
	@echo "building nightly"
	@rustup run nightly cargo build

build-nightly-example-%: $(CURDIR)/examples/nightly/%
	@echo "building nightly example $*"
	@cd $(CURDIR)/examples/nightly/$*; rustup run nightly cargo build

test-stable:
	@echo "testing stable"
	@rustup run stable cargo test

test-examples: test-stable-examples test-nightly-examples

test-stable-examples: $(foreach x, $(STABLE_EXAMPLES), test-stable-example-$(x))

test-stable-example-%: $(CURDIR)/examples/stable/%
	@echo "testing stable example $*"
	@cd $(CURDIR)/examples/stable/$*; rustup run stable cargo test

test-nightly-examples: $(foreach x, $(NIGHTLY_EXAMPLES), test-nightly-example-$(x))

test-nightly:
	@echo "testing nightly"
	@rustup run nightly cargo test

test-nightly-example-%: $(CURDIR)/examples/nightly/%
	@echo "testing nightly example $*"
	@cd $(CURDIR)/examples/nightly/$*; rustup run nightly cargo test
