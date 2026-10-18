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