# Plonky2 benchmarks

This crate benchmarks Plonky2 1.1 circuits through the repository's shared
benchmark harness. The circuits use the Goldilocks field, Plonkish
arithmetization, and FRI for both the IOP and PCS. The benchmark metadata
reports 97 security bits, post-quantum soundness, zero-knowledge mode, and an
audited but unmaintained proving system.

The SHA-256 circuit is derived from
[polymerdao/plonky2-sha256](https://github.com/polymerdao/plonky2-sha256).
The ECDSA circuit uses the pinned
[`AnInsaneJimJam/plonky2-ecdsa@0bf1a54`](https://github.com/AnInsaneJimJam/plonky2-ecdsa/commit/0bf1a54c5d97a64917861596c08fd4fc0d4366b6)
fork.

> [!NOTE]
> SHA-256 and Keccak use the pinned
> [`alxkzmn/plonky2-u32`](https://github.com/alxkzmn/plonky2-u32) revision
> `fcabb02`. Its serializers are required to measure the serialized circuit
> and prover data. The ECDSA fork aligns with Plonky2 1.1 and serializes its
> custom gates and witness generators.

## Prerequisites

Install Rust with rustup. Rustup automatically selects the repository's
canonical toolchain from [`../rust-toolchain.toml`](../rust-toolchain.toml).
The first build may need network access to fetch the pinned Git dependencies.

## Benchmarking

Run all four targets with the reduced profile while iterating:

```bash
BENCH_INPUT_PROFILE=reduced cargo bench -p plonky2_circuits
```

Run one target:

```bash
BENCH_INPUT_PROFILE=reduced cargo bench -p plonky2_circuits --bench sha256
BENCH_INPUT_PROFILE=reduced cargo bench -p plonky2_circuits --bench keccak
BENCH_INPUT_PROFILE=reduced cargo bench -p plonky2_circuits --bench poseidon
BENCH_INPUT_PROFILE=reduced cargo bench -p plonky2_circuits --bench ecdsa
```

Use `BENCH_INPUT_PROFILE=full` for the complete variable-size sweep. The
shared harness writes the standardized metrics, Criterion reports, and memory
reports; see [`CONTRIBUTING.md`](../CONTRIBUTING.md) for the workflow and
output names. 

## Circuit details

- **SHA-256** — accepts byte messages of 128, 256, 512, 1024, or 2048 bytes
  (the reduced profile uses 128 and 256). The message is a private witness;
  the expected digest is constrained in the circuit. This is an explicit
  circuit, so its acceleration metadata is `None`.
- **Keccak-256** — uses the in-tree bit-level Keccak circuit with the same byte
  sizes and reduced profile as SHA-256. The message is private and the expected
  digest is constrained in the circuit. Its acceleration metadata is `None`.
- **Poseidon** — hashes 2, 4, 8, 12, or 16 private Goldilocks field-element
  targets (the reduced profile uses 2 and 8). The four-field-element hash
  output is public. The benchmark records `precompile` acceleration for this
  built-in Plonky2 Poseidon operation.
- **secp256k1 ECDSA** — verifies the generated fixed fixture for a 32-byte
  prehash. The public inputs are ordered `digest`, `public_key_x`,
  `public_key_y`, `r`, and `s`; each 32-byte value is represented by eight
  little-endian 32-bit limbs. The circuit enables Plonky2's zero-knowledge
  blinding, so the proof mode is marked ZK. This does not make the fixture's
  statement private: all of those values are public, and no secret key or
  private message is supplied.

The ECDSA gadget uses incomplete affine addition and compares `r` directly
with the computed x-coordinate instead of reducing that coordinate modulo the
secp256k1 scalar order. Treat its result as a measurement of the deterministic
fixture, not as evidence of a complete production ECDSA verifier.

The in-tree Keccak implementation includes its upstream MIT license at
[`src/keccak256/MIT-LICENSE.txt`](src/keccak256/MIT-LICENSE.txt).

## Reported metrics

- `num_constraints` — the circuit builder's gate count.
- `preprocessing_size` — serialized common circuit data plus serialized
  prover-only data.
- `proof_size` — the serialized proof core, excluding public inputs.
- Prove and verify timings, plus peak memory measured by each target's memory
  binary.

The memory binaries perform only circuit preparation and proving, including
witness generation. All targets use the shared harness; see
[`CONTRIBUTING.md`](../CONTRIBUTING.md) for the repository-wide benchmark
requirements.
