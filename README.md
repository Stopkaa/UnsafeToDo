# UnsafeTodo

UnsafeTodo is a cli todo-tool written in Rust.

# Main Features

- save todos between invocations of commands (needs some kind of file type, toml, json, binary)
- add new Todo
- delete a Todo
- mark Todo as completed
- show all todos
- show all tods of prio 1/2/3/...
- show all tods of due today/this week/this month

# Optional Features

- some kind of synchronization with a next cloud (Hennes nextcloud?, rest api exists)

# Todo data structure

- name, desciption, prio (1, 2, 3), date of creation, due date, start date, completed flag, (internal?) id
