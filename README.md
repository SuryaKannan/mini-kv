# mini-kv

A small key-value store DB which allows you to associate a unique key with a value and retrieve that value later.

For now we'll store in memory, then move to persistent in stage two. Later, we will also extend to a network service.

Initial functional requirements

- Storage: In-memory, ephemeral
- Keys: Non-empty, restricted ASCII, case-insensitive
- Values: Strings, may be empty, no explicit size limit
- SET: Insert or replace
- GET: Retrieve; error if missing
- DELETE: Remove; error if missing
- EXIT: Gracefully terminate
- Commands: Uppercase only
- Arguments: Quoted strings and basic escaping supported
- Invalid input: Error without terminating
- Execution: One command at a time
- Architecture: Storage engine independent of CLI

## TODO

- Make parser indexing safe for UTF-8. `chars().enumerate()` counts characters, but string slicing requires byte offsets. Input such as `SET name é x` currently panics. Use `char_indices()` when calculating slice boundaries.
- Exit gracefully at end of input (EOF, including Ctrl+D). Check whether `read_line()` returns `0` bytes to avoid repeatedly processing empty input.
- Handle repeated spaces without creating empty arguments. `SET  Alice` currently stores an empty key. Preserve intentionally empty quoted values when improving whitespace handling.
- Support quoted arguments and basic escaping. `SET name "Alice Smith"` should store a value containing a space, and `SET empty ""` should store an empty string rather than literal quote characters.
- Validate keys as non-empty and restricted ASCII, and normalize their case consistently across SET, GET, and DELETE. `SET Name Alice` followed by `GET name` should retrieve the stored value.
