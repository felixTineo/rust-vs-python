# Rust vs Python: Speed Benchmark

This repository contains two implementations of the same project in **Rust** and **Python** to compare their performance in calculating prime numbers.

## Project Structure

```javascript
├── python
│   ├── main.py
│   └── requirements.txt
├── README.md
└── rust
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    └── target
```

## Getting Started

### Python Implementation

1. Navigate to the `python` folder:

```bash
cd python
```

2. Create and activate environment

```bash
python -m venv .env
source .env/bin/activate    # Use `.env\Scripts\activate` on Windows
```

3. Install any required dependencies:

```bash
pip install -r requirements.txt
```

4. Run the program

```bash
python main.py
```

### Rust implementation

1. Navigate to the `rust` folder:

```bash
cd rust
```

2. Build the project

```bash
cargo run build
```

3. Run the project

```bash
cargo run --release
```

## Comparing Performance

Each implementation calculates all prime numbers up to 10 million. The time taken for execution will be printed to the console after running the programs.

### Benchmarking Tools

- Python: Execution time is measured using the built-in time module.
- Rust: Use std::time for basic measurement, or integrate criterion for advanced benchmarking.

## Contributions

Feel free to fork the repository, open issues, or submit pull requests to improve the implementations or add new comparisons.
