STABLE_EXAMPLES = 	json
UNSTABLE_EXAMPLES =	json rocket
UNSTABLE_FEATURES = inclusive_range

all: build test

build: build-stable \
	build-unstable \
	build-examples

test: test-stable \
	test-unstable \
	test-examples

build-stable:
	@echo "building on stable"
	@rustup run stable cargo build

build-examples: build-stable-examples build-unstable-examples

build-stable-examples: $(foreach x, $(STABLE_EXAMPLES), build-stable-example-$(x))

build-stable-example-%: $(CURDIR)/examples/stable/%
	@echo "building stable example $*"
	@cd $(CURDIR)/examples/stable/$*; rustup run stable cargo build

build-unstable-examples: $(foreach x, $(UNSTABLE_EXAMPLES), build-unstable-example-$(x))

build-unstable:
	@echo "building on nightly"
	@rustup run nightly cargo build
	@echo "building on nightly with unstable features: $(UNSTABLE_FEATURES)"
	@rustup run nightly cargo build --features $(UNSTABLE_FEATURES)

build-unstable-example-%: $(CURDIR)/examples/unstable/%
	@echo "building unstable example $*"
	@cd $(CURDIR)/examples/unstable/$*; rustup run nightly cargo build

test-stable:
	@echo "testing stable"
	@rustup run stable cargo test

test-examples: test-stable-examples test-unstable-examples

test-stable-examples: $(foreach x, $(STABLE_EXAMPLES), test-stable-example-$(x))

test-stable-example-%: $(CURDIR)/examples/stable/%
	@echo "testing stable example $*"
	@cd $(CURDIR)/examples/stable/$*; rustup run stable cargo test

test-unstable-examples: $(foreach x, $(UNSTABLE_EXAMPLES), test-unstable-example-$(x))

test-unstable:
	@echo "testing on nightly"
	@rustup run nightly cargo test
	@echo "testing on nightly with unstable features: $(UNSTABLE_FEATURES)"
	@rustup run nightly cargo test --features $(UNSTABLE_FEATURES)

test-unstable-example-%: $(CURDIR)/examples/unstable/%
	@echo "testing unstable example $*"
	@cd $(CURDIR)/examples/unstable/$*; rustup run nightly cargo test
