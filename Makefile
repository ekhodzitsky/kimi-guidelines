# Makefile for Python projects using kimi-dotfiles
# Provides a unified entrypoint analogous to `cargo kimi check` for Rust.

.PHONY: check test lint format audit clean

PYTHON_FILES := $(shell find . -name '*.py' -not -path './.git/*' -not -path './.*')

## check: Run all Python checks (ruff, mypy, pytest, pip-audit)
check: lint test audit
	@echo "✅ All Python checks passed"

## lint: Run ruff and mypy
lint:
	@echo "Running ruff..."
	ruff check .
	@echo "Running mypy..."
	mypy --strict .

## test: Run pytest with coverage
test:
	@echo "Running pytest..."
	pytest -q --tb=short

## format: Auto-format code with black and ruff
format:
	@echo "Formatting code..."
	black .
	ruff format .

## audit: Run pip-audit for known vulnerabilities
audit:
	@echo "Running pip-audit..."
	pip-audit

## clean: Remove cache and temp files
clean:
	find . -type d -name '__pycache__' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '.mypy_cache' -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name '.pytest_cache' -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name '*.pyc' -delete 2>/dev/null || true

## help: Show this help message
help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-10s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
