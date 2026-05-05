# Python Agent Guidelines

> Version: 1.6.0 | Source: kimi-dotfiles/languages/python/

## Type Safety

- **Every function** must have type hints for arguments and return values (PEP 484).
- Use `mypy --strict` or `pyright` in CI. No untyped code in production paths.
- Prefer `dataclasses` or `pydantic.BaseModel` over raw `dict` for structured data.
- Use `NewType` and `Literal` to encode invariants at the type level.

## Validation

- **External input** (HTTP, CLI, file) must pass through `pydantic` validation before business logic.
- Never trust `request.json()` or `json.load()` without a schema.
- Use `pydantic.v1` or `pydantic.v2` consistently across the project.

## Error Handling

- No bare `except:`. Always catch specific exceptions.
- Use `Result`-like patterns where appropriate (`returns.result` or custom `Result[T, E]`).
- Log exceptions with context, never swallow silently.
- Functions that can fail must document failure modes in the docstring.

## Documentation (Hoare Triples for Python)

Every public function must have a docstring with preconditions and postconditions:

```python
# { amount >= 0 and 0 <= rate <= 1.0 }
# def calculate_total(amount: Decimal, rate: Decimal) -> Decimal
# { result >= amount }
def calculate_total(amount: Decimal, rate: Decimal) -> Decimal:
    ...
```

## Property-Based Testing

- Use `hypothesis` for property-based tests on all pure functions.
- Test invariants, not just examples: `reverse(reverse(x)) == x`.
- Run hypothesis tests in CI with `hypothesis.settings(max_examples=1000)` for nightly builds.

## Linting & Formatting

- `ruff` for linting and import sorting (replaces flake8, isort, pydocstyle).
- `black` for formatting with line length 88.
- `mypy --strict` or `pyright` for type checking.
- Pre-commit hook must block commits with type errors or lint violations.

## Security

- No `eval()`, `exec()`, or `compile()` on untrusted input.
- Use `ast.literal_eval` if you must evaluate literals.
- SQL queries must use parameterized statements (psycopg2, sqlalchemy).
- Path operations must validate against directory traversal (`..`, absolute paths).
- Never log secrets (API keys, tokens, passwords).

## Performance

- Use `functools.lru_cache` or `functools.cache` for deterministic pure functions.
- Avoid premature optimization, but profile slow paths with `cProfile`.
- Async code must use `async`/`await` consistently; no mixing sync and async in the same path.

## Dependencies

- Pin all dependencies in `requirements.txt` or `pyproject.toml` with exact versions.
- Use `pip-audit` or `safety` in CI to catch known CVEs.
- Minimize dependencies. Prefer standard library where possible.
