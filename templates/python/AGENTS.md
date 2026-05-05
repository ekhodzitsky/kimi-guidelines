# Python Project Guidelines

> Generated from kimi-guidelines/templates/python
> Version: 1.0.0 | Strictness: standard

## Project Setup

```bash
pip install mypy ruff black hypothesis pydantic pip-audit
```

## Pre-commit Hook

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/astral-sh/ruff-pre-commit
    rev: v0.9.0
    hooks:
      - id: ruff
        args: [--fix]
      - id: ruff-format
  - repo: https://github.com/pre-commit/mirrors-mypy
    rev: v1.14.0
    hooks:
      - id: mypy
        args: [--strict]
```

## Type Hints

- All functions must have type annotations.
- Use `mypy --strict` in CI.
- No `Any` in production code without `# type: ignore[reason]`.

## Validation

- All external input validated through Pydantic models.
- Use `pydantic.BaseModel` for request/response schemas.

## Error Handling

```python
# { price >= 0 and 0 <= tax_rate <= 1.0 }
# def calculate_total(price: Decimal, tax_rate: Decimal) -> Decimal
# { result >= price }
def calculate_total(price: Decimal, tax_rate: Decimal) -> Decimal:
    if price < 0:
        raise ValueError("price must be non-negative")
    if not 0 <= tax_rate <= 1.0:
        raise ValueError("tax_rate must be between 0 and 1")
    return price * (1 + tax_rate)
```

## Property-Based Testing

```python
from hypothesis import given, strategies as st
from decimal import Decimal

@given(st.decimals(min_value=0, max_value=10000),
       st.decimals(min_value=0, max_value=1))
def test_calculate_total_monotonic(price, tax_rate):
    result = calculate_total(price, tax_rate)
    assert result >= price
```

## Security

- No `eval()` / `exec()` on untrusted input.
- Parameterized SQL queries only.
- Validate file paths against traversal (`..`, absolute paths).

## CI Checklist

```bash
ruff check .
mypy --strict .
pytest
pip-audit
```
