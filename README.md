<div align="center">

# Bumper
![Rust](https://img.shields.io/badge/Rust-1.78-000000?style=flat-square&logo=rust&logoColor=white)
<br/>
A bump allocator (arena/region allocator) built from scratch in Rust.

Pre-allocates a fixed memory block upfront and hands out slices via pointer bumping — zero per-object overhead, no garbage collector, automatic cleanup on drop.

</div>

---

## What is a Bump Allocator?

Normal heap allocation (`malloc`, `Box::new`) asks the OS for memory on every single allocation. The allocator searches a free list, finds a suitable block, marks it used, and returns a pointer. This happens thousands of times per second in typical programs — and every one of those calls has overhead.

A bump allocator takes a different approach:

```
Normal allocation:
[alloc A] -> OS    [alloc B] -> OS    [alloc C] -> OS    (N separate OS calls)

Bump allocation:
[grab big block from OS once]
[A][B][C][D]...                                          (1 OS call + pointer math)
```

One large block is reserved upfront. Every allocation just moves a pointer forward by the requested size. Freeing is instant — reset the pointer to zero. Everything is freed at once.

---

## Benchmarks

Measured on Apple M-series. Each benchmark runs 100 samples via [criterion](https://github.com/bheisler/criterion.rs).

| Benchmark           | Bump Allocator | Heap (Box) | Speedup         |
| ------------------- | -------------- | ---------- | --------------- |
| 1000 x u32          | 2.12 µs        | 17.68 µs   | 8.3x faster     |
| 1000 x Point struct | 2.94 µs        | 21.33 µs   | 7.2x faster     |
| reset + reuse x1000 | 809 ns         | —          | sub-microsecond |

The gap comes from eliminating per-object allocator overhead entirely. Each allocation in the bump allocator is a bounds check and a pointer increment — nothing more.

Run benchmarks yourself:

```bash
cargo bench
```

HTML reports are generated at `target/criterion/report/index.html`.

---

## Usage

```rust
use bumper::BumpAllocator;

// Create an arena with 64KB of memory
let mut arena = BumpAllocator::new(1024 * 64);

// Allocate raw bytes
let bytes = arena.alloc(4, 4).expect("out of memory");
bytes[0] = 42;

// Allocate a typed value
let x: &mut u32 = arena.alloc_val(100u32).expect("out of memory");
assert_eq!(*x, 100);

// Allocate a struct
#[derive(Debug)]
struct Point { x: f32, y: f32 }

let p = arena.alloc_val(Point { x: 1.0, y: 2.0 }).expect("out of memory");
println!("{:?}", p); // Point { x: 1.0, y: 2.0 }

// Inspect usage
println!("used: {} bytes", arena.used());
println!("remaining: {} bytes", arena.remaining());

// Reset — free everything at once, reuse the arena
arena.reset();
```

Memory is automatically freed when the arena goes out of scope. No manual `free()`, no garbage collector.

---

## API

| Method                     | Description                                      |
| -------------------------- | ------------------------------------------------ |
| `BumpAllocator::new(size)` | Create a new arena with `size` bytes             |
| `alloc(size, align)`       | Allocate raw bytes, returns `Option<&mut [u8]>`  |
| `alloc_val::<T>(val)`      | Allocate a typed value, returns `Option<&mut T>` |
| `reset()`                  | Free all allocations, reuse the arena            |
| `used()`                   | Bytes allocated so far                           |
| `remaining()`              | Bytes still available                            |
| `capacity()`               | Total arena size                                 |

---

## How it works

```
BumpAllocator::new(1024)
[ 0 0 0 0 0 0 0 0 ... 0 0 0 0 ]   1024 bytes
  ^ offset = 0

alloc(4, 4)  ->  returns bytes 0..4
[ X X X X 0 0 0 0 ... 0 0 0 0 ]
              ^ offset = 4

alloc(8, 4)  ->  returns bytes 4..12
[ X X X X Y Y Y Y Y Y Y Y 0 0 ]
                            ^ offset = 12

reset()      ->  offset = 0, memory reused on next allocation
```

Alignment is handled by rounding the current offset up to the nearest multiple of the requested alignment using bitwise math:

```rust
let aligned = (current + align - 1) & !(align - 1);
```

This is the same technique used in production allocators like `jemalloc` and `mimalloc`.

---

## Rust concepts demonstrated

- Structs and impl blocks — separating data from behavior
- Generics — `alloc_val<T>` works for any type T
- Lifetimes — allocated references cannot outlive the arena
- `unsafe` — raw pointer writes via `ptr::write`
- `Option` — explicit failure handling, no nulls
- Drop trait — automatic cleanup via RAII
- `#[cfg(test)]` — test-only compilation

---

## Running tests

```bash
cargo test
```

---

## Prior art

This is a simplified version of [bumpalo](https://github.com/fitzgen/bumpalo), a production bump allocator with 30M+ downloads on crates.io. Building this from scratch is a useful exercise in understanding what bumpalo does under the hood.

---

_Built as a learning project to explore Rust memory management fundamentals._
