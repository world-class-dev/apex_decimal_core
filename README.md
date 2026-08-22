# Apex-Decimal-Core

**Deterministic, Constant-Time Fixed-Point Engine for Military-Grade Financial Clearing & High-Frequency Trading.**

`apex_decimal_core` is a ultra-low latency, zero-floating-point error arithmetic framework engineered in Rust. It eliminates CPython/IEEE-754 precision inaccuracies by utilizing 128-bit integer scaling primitives, reverse-stream evaluation, and obfuscated payload masking to protect high-frequency transactions against timing attacks and payload tampering.

---

## ⚡ Key Architectural Features

* **Zero Floating-Point Inaccuracy:** Guarantees absolute precision up to 8 decimal places ($1.00000000$) using native `i128` fixed-point scaling.
* **Obfuscated Reverse-Stream Processing:** Ingests transaction payloads in reverse chronological sequence, interleaving authentic data with bitwise mask decoys to defeat dynamic memory dumps.
* **Timing-Attack Resistance:** Constant-time cryptographic state validation prevents side-channel analysis during execution.
* **Sub-Microsecond Settlement:** Zero-allocation memory structure optimized for high-throughput FFI binding with Python.

---

## 🌀 Payload Obfuscation & Execution Flow

```text
  [ Incoming Raw Payload: $1,250.50000000 ]
                     │
                     ▼
  ┌─────────────────────────────────────────────────────────┐
  │ 1. Fixed-Point Scaling (i128) -> 125,050,000,000        │
  │ 2. Bitwise Masking & Decoy Injection                    │
  └──────────────────────────┬──────────────────────────────┘
                             │
                             ▼
  [ Encoded Network Stream / Memory Allocation ]
  ┌──────────────────┬──────────────────┬──────────────────┐
  │ Decoy Payload B  │ Authentic Value  │ Decoy Payload A  │
  │ (0x7B2F...9A)    │ (125050000000)   │ (0x5A5A...A5)    │
  └──────────────────┴─────────┬────────┴──────────────────┘
                               │
            ◄──────────────────┴───────────────────
            REVERSE STREAM TRAVERSAL (Index N -> 0)
                               │
                               ▼
  ┌─────────────────────────────────────────────────────────┐
  │ 3. Fast Bitwise Decoy Stripping                         │
  │ 4. Fixed-Point Reconstruction (8 Decimals)              │
  └──────────────────────────┬──────────────────────────────┘
                             │
                             ▼
  [ Verified Output: 1250 . 50000000 ] -> Pure Precision

  apex-decimal-core/
├── Cargo.toml                  # Rust compilation & optimization profile (LTO=true)
├── pyproject.toml              # Maturin build framework configuration
├── README.md                   # System specifications & benchmarks
├── src/
│   ├── lib.rs                  # PyO3 FFI Gateway & PyModule Bindings
│   ├── core/
│   │   ├── mod.rs              # Core engine module router
│   │   ├── decimal.rs          # Fixed-Point Arithmetic (8 Decimals)
│   │   └── obfuscator.rs       # Reverse Traversal & Decoy Injection Primitives
│   └── security/
│       ├── mod.rs              # Security definitions
│       └── constant_time.rs    # Anti-timing side-channel guards
└── tests/
    └── integration_test.rs     # Performance benchmarks & reverse stream validation