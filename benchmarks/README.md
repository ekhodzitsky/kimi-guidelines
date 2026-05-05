# kimi-guidelines Benchmark

This directory contains an A/B benchmarking framework to measure whether code generated **with** the `kimi-guidelines` guidelines produces higher-quality Rust than code generated without them.

## Structure

```
benchmarks/
├── prompts/          # 10 realistic Rust coding prompts
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

Use `cargo kimi check` (from the [`cargo-kimi`](https://github.com/ekhodzitsky/cargo-kimi) CLI) to evaluate generated `.rs` files. It produces a JSON report with a 0–100 quality score based on Hoare triples, unwrap/expect/panic usage, unsafe blocks, SAFETY comments, and more.

### Usage

```bash
# Score a single file
cargo kimi check --file path/to/output.rs

# Score a whole directory
cargo kimi check path/to/project/
```

## Running an A/B Test

1. **Generate outputs**  
   Feed each prompt to Kimi K2.6 twice:
   - **Control group** — no system guidelines.
   - **Treatment group** — prepend the `kimi-guidelines` system prompt / guidelines.

2. **Save outputs**  
   Place the generated `.rs` files under `benchmarks/results/control/` and `benchmarks/results/treatment/`.

3. **Score**  
   ```bash
   mkdir -p benchmarks/results/control benchmarks/results/treatment
   
   for f in benchmarks/results/control/*.rs; do
     cargo kimi check --file "$f"
   done
   
   for f in benchmarks/results/treatment/*.rs; do
     cargo kimi check --file "$f"
   done
   ```

4. **Compare**  
   Average the scores. The hypothesis is that the treatment group scores significantly higher.

## Ignored Files

Generated results, `.json` reports, and any build artifacts inside `benchmarks/` are ignored by `.gitignore`.
