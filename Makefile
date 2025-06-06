# Globals

CARGO := cargo
CARGO_BUILD := $(CARGO) build
CARGO_FLAGS := --verbose

# Phony targets
.PHONY: all format lint build clean clean-build precommit check-env run help

## Generate sync unit tests, format, and, lint
all: precommit

## Format all files
format:
	@echo "Formatting all files..."
	@$(CARGO) fmt

## Check the formatting of all files, run clippy on the source code, then run
## clippy on the tests (but allow expect to be used in tests)
lint:
	@echo "Linting all files..."
	@echo ">>> Checking formatting..."
	@$(CARGO) fmt -- --check
	@echo ">>> Running clippy on source code..."
	@$(CARGO) clippy --all-features -- -D warnings -W clippy::unwrap_used -W clippy::expect_used -W missing_docs
	@echo ">>> Running clippy on tests..."
	@$(CARGO) clippy --tests -- -D warnings -W clippy::unwrap_used

## Build project
build:
	@echo "Building project..."
	@$(CARGO_BUILD) $(CARGO_FLAGS)

## Remove build files
clean:
	@echo "Cleaning project..."
	@$(CARGO) clean

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
	@echo "Running Momento Proxy..."
	@$(CARGO) run -- config/momento_proxy.toml

# See <https://gist.github.com/klmr/575726c7e05d8780505a> for explanation.
# This is a way to generate a help message from the Makefile itself with the "## " comments.
help:
	@echo "$$(tput bold)Available rules:$$(tput sgr0)";echo;sed -ne"/^## /{h;s/.*//;:d" -e"H;n;s/^## //;td" -e"s/:.*//;G;s/\\n## /---/;s/\\n/ /g;p;}" ${MAKEFILE_LIST}|LC_ALL='C' sort -f|awk -F --- -v n=$$(tput cols) -v i=19 -v a="$$(tput setaf 6)" -v z="$$(tput sgr0)" '{printf"%s%*s%s ",a,-i,$$1,z;m=split($$2,w," ");l=n-i;for(j=1;j<=m;j++){l-=length(w[j])+1;if(l<= 0){l=n-i-length(w[j])-1;printf"\n%*s ",-i," ";}printf"%s ",w[j];}printf"\n";}'|more $(shell test $(shell uname) == Darwin && echo '-Xr')
