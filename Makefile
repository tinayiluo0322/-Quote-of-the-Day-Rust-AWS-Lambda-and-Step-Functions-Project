rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

watch:
	cargo lambda watch

build1: 
	cargo lambda build --bin step1 --release 

build2: 
	cargo lambda build --bin step2 --release 

build3: 
	cargo lambda build --bin step3 --release 

deploy1:
	cargo lambda deploy step1

deploy2:
	cargo lambda deploy step2

deploy3:
	cargo lambda deploy step3

# only did one but ideally do all 3
aws-invoke: 
	cargo lambda invoke --remote step1 --data-ascii "{ \"command\": \"get\"}" 

# only did one but ideally do all 3
invoke:
	cargo lambda invoke --data-ascii "{ \"command\": \"get\"}" 

all: format lint test 