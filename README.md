# Segment Tree
Implements a basic sum segment tree that is safe on any system bit-width. ie 16, 32, 64

## State
Overall, the code is organized and safe
This code would need some cleanup for any production use
- Code documentation and comments
- Results as `Result<T, CustomError>` rather than `Result<T, &'static str>`
- Most of the unit tests are missing
- `Sync` and `Send` implementation for use with async
