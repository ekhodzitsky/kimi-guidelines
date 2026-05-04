# kimi-dotfiles Benchmark

This directory contains an A/B benchmarking framework to measure whether code generated **with** the `kimi-dotfiles` guidelines produces higher-quality Rust than code generated without them.

## Structure

```
benchmarks/
├── prompts/          # 10 realistic Rust coding prompts
├── scoring/          # `score_output.py` — evaluates .rs files
├── .gitignore        # Ignores generated artifacts
└── README.md         # This file
```

## Prompts

Each prompt in `prompts/` is a Markdown file describing a self-contained coding task:

| # | Task |
|---|------|
| 01 | Email Validator |
| 02 | Token Bucket Rate Limiter |
| 03 | TOML-like Config Parser |
| 04 | JSON Serializer/Deserializer |
| 05 | Priority Job Queue |
| 06 | HTTP URL Router |
| 07 | Streaming CSV Parser |
| 08 | Argon2 Password Hasher |
| 09 | Typed Event Bus |
| 10 | LRU Cache |

## Scoring

`scoring/score_output.py` analyzes any `.rs` file and produces a JSON report.

### Usage

```bash
# Score a single file
python benchmarks/scoring/score_output.py path/to/output.rs

# Score a file and save the report
python benchmarks/scoring/score_output.py path/to/output.rs --output report.json
```

### Scoring Criteria (0-100)

| Criterion | Max Points | Description |
|-----------|------------|-------------|
| **Hoare triples** | 30 | `/// {` doc comments (up to 3 counted) |
| **No unwraps** | 20 | Penalty of 5 pts per `unwrap()` / `expect()` / `panic!()` |
| **Newtype** | 10 | Uses tuple struct newtype pattern |
| **PhantomData** | 10 | Uses `PhantomData` for lifetime/type safety |
| **Typestate** | 10 | Uses marker structs + generics for typestate pattern |
| **Function length** | 10 | Average function length ≤ 40 lines |
| **Result handling** | 10 | Returns `Result` for fallible operations |

### Example Output

```json
{
  "file": "output.rs",
  "score": 78,
  "criteria": {
    "hoare_triples": 3,
    "unwrap_count": 1,
    "newtype_used": true,
    "phantomdata_used": false,
    "typestate_used": true,
    "avg_function_length": 25,
    "result_handling": true,
    "option_handling": true
  }
}
```

## Running an A/B Test

1. **Generate outputs**  
   Feed each prompt to Kimi K2.6 twice:
   - **Control group** — no system guidelines.
   - **Treatment group** — prepend the `kimi-dotfiles` system prompt / guidelines.

2. **Save outputs**  
   Place the generated `.rs` files under `benchmarks/results/control/` and `benchmarks/results/treatment/`.

3. **Score**  
   ```bash
   mkdir -p benchmarks/results/control benchmarks/results/treatment
   
   for f in benchmarks/results/control/*.rs; do
     python benchmarks/scoring/score_output.py "$f" --output "${f%.rs}.json"
   done
   
   for f in benchmarks/results/treatment/*.rs; do
     python benchmarks/scoring/score_output.py "$f" --output "${f%.rs}.json"
   done
   ```

4. **Compare**  
   Average the scores. The hypothesis is that the treatment group scores significantly higher.

## Ignored Files

Generated results, `.json` reports, and any build artifacts inside `benchmarks/` are ignored by `.gitignore`.
