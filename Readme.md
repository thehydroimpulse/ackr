# Ackr

A Rust implementation of the Storm acking algorithm allowing to track the state of arbitrary tuples in a DAG with a static ~20 bytes of memory needed.

## Getting Started

Install it with Cargo:

```toml
[dependencies]
ackr = "1.0.0"
```

And include the crate in your `src/lib.rs`

```rust
extern crate ackr;

use ackr::{Tuple, Source, Task, Ackr};
```

## Creating a new Source

Each Ackr can track a number of DAG's of tuples and their state.

```rust
let mut ackr = Ackr::new();
ackr.insert(Source(1), Task(1));
```

Tasks are arbitrary and are simply a wrapper around `u32`s, but they have the ability to track the origin of the acks.

Source's are simply the source tuple id, as `u64`s. If you have the following DAG:

```
A(1)
|
B(2)
```

Each tuple should be associated a random id. `A` or the `Source` would be the first tuple we insert and thus need to ack later on for it to be considered "completed".

```rust
assert!(ackr.has_completed(Source(1)));
```

Because we use the initial Source as the first tuple, `has_completed` will return false because we haven't acked it.

```rust
ackr.ack(Source(1), Tuple(1));
assert!(ackr.has_completed(Source(1)));
```

This will now return `true` because we have acked the Source/Tuple of `1`.


## Acking Tuples

Inserting and acking are basically the same thing. Acking a tuple once will act as an insert and acking it twice will remove it, effectively "completing" the tuple.

However, to make it clearer, there are two distinct APIs.

```rust
ackr.insert(Source(1), Tuple(2));
```

Going back to the previous DAG example of `A(1) -> B(2)`, we're inserting the next tuple of the DAG.

```rust
ackr.ack(Source(1), Tuple(2));
```

Now we have acked/completed the tuple.

Once all the tuples (including the source dag) have been acked, `has_completed` will return `true`.

## License

MIT


