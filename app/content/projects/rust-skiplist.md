---
github: JP-Ellis/rust-skiplist
slug: rust-skiplist
tagline: A skip list data structure for Rust
title: rust-skiplist
---

A skip list is a probabilistic data structure that provides O(log n) average search, insertion, and deletion. It hasthe same asymptotic complexity as a balanced binary search tree, but with simpler implementation and cache-friendly access patterns for sequential reads.

This crate provides `SkipList<T>` and `OrderedSkipList<T>`, both implementing the standard Rust collection traits (`IntoIterator`, `FromIterator`, `Extend`).

## Quick Start

```rust
use skiplist::OrderedSkipList;

let mut list = OrderedSkipList::new();
list.insert(3);
list.insert(1);
list.insert(4);

assert_eq!(list.front(), Some(&1));
assert_eq!(list.len(), 3);
```

The crate is available on [crates.io](https://crates.io/crates/skiplist).
