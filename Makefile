.PHONY: all format lint build clean clean-build precommit check-env run help

## Generate sync unit tests, format, and, lint
all: precommit

## Format all files
format:
	cargo fmt

## Check the formatting of all files, run clippy on the source code, then run
## clippy on the tests (but allow expect to be used in tests)
lint:
	cargo fmt -- --check && \
	cargo clippy --all-features -- -D warnings -W clippy::unwrap_used -W clippy::expect_used -W missing_docs && \
	cargo clippy --tests -- -D warnings -W clippy::unwrap_used

## Build project
build:
	cargo build --verbose

## Remove build files
clean:
	cargo clean

## Build project
clean-build: clean build

## Run clean-build as a step before committing.
precommit: clean-build lint

check-env:
	@if [ -z "${MOMENTO_AUTHENTICATION}" ]; then \
		echo "MOMENTO_AUTHENTICATION is not set"; \
		exit 1; \
	fi

run: check-env
	cargo run -- config/momento_proxy.toml

# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=19 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'|more $(shell test $(shell uname) == Darwin && echo '-Xr')
