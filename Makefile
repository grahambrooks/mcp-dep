# Standard targets. `make check` is the gate: exactly what the ci.yml check job runs;
# prek runs it before every push. Releases are tag-based (see CLAUDE.md), not make.

.DEFAULT_GOAL := help

.PHONY: help check fmt fmt-check lint test hooks

help: ## List targets
	@grep -hE '^[a-zA-Z0-9_-]+:.*## ' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*## "}; {printf "  %-16s %s\n", $$1, $$2}'

check: fmt-check lint test ## The gate: fmt-check + clippy + test, as CI runs them
	@echo "All checks passed."

fmt: ## Format the code
	cargo fmt --all

fmt-check: ## Check formatting without modifying
	cargo fmt --all --check

lint: ## Clippy with warnings denied
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

test: ## Run the tests
	cargo test --workspace --all-features --locked

hooks: ## Install the git hooks (pre-commit and pre-push) with prek
	prek install
