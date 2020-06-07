.DEFAULT_GOAL := show-help
THIS_FILE := $(lastword $(MAKEFILE_LIST))
ROOT_DIR:=$(shell dirname $(realpath $(THIS_FILE)))

.PHONY: show-help
# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
## This help screen
show-help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=29 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'

.PHONY: test
## Test it was built ok
test:
	unset GIT_AUTHORS_EXEC && RUST_BACKTRACE=1 cargo test --locked

.PHONY: mdtest
## Test the markdown in the usage directory
mdtest: build
	./bin/mdtest ./usage/**/*.md

.PHONY: smoke-test
## Run a smoke test and see if the app runs
smoke-test:
	cargo run --locked --bin pb-prepare-commit-msg -- -h
	cargo run --locked --bin pb-pre-commit -- -h
	cargo run --locked --bin pb-commit-msg -- -h
	cargo run --locked --bin git-authors -- -h

.PHONY: build
## Build release version
build:
	cargo build --locked --release

.PHONY: lint
## Lint it
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic
	cargo check
	cargo audit
	find . \( -iname "*.yml" -o -iname "*.yaml" \) -exec npx prettier --check --write {} \;

.PHONY: fmt
## Format what can be formatted
fmt:
	cargo fix --allow-dirty
	cargo clippy --allow-dirty --fix -Z unstable-options --all-features -- -D warnings -Dclippy::all -D clippy::pedantic
	cargo fmt --all
	find . \( -iname "*.yml" -o -iname "*.yaml" \) -exec npx prettier --write {} \;

.PHONY: clean
## Clean the build directory
clean:
	cargo clean
