# Rust Buddy Memory Allocator
An implementation of the [buddy allocation algorithm](https://en.wikipedia.org/wiki/Buddy_memory_allocation) in rust.

## Usage
```
extern crate rustbuddy;

// Create a new instance w/ n levels
let mut buddy = rustbuddy::BuddyAllocator::new(4);

// Allocate a single block (returns the index offset of the block)
let offset = buddy.allocate(1);

// Dump the tree for debugging
println!("{}", buddy.dump());
```
