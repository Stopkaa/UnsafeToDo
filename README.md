# CLI Todo Application

A simple and efficient command-line todo application that helps you manage your tasks directly from the terminal. Your todos are stored in JSON format and can be synchronized across devices using Git repositories.

## Features

- ✅ **Add todos** - Create new tasks with ease
- ❌ **Remove todos** - Delete completed or unwanted tasks
- 📋 **Display todos** - View all your tasks in a clean format
- ✔️ **Complete todos** - Mark tasks as finished
- 🔄 **Sort todos** - Organize your tasks by different criteria
- 💾 **JSON format** - All todos are saved in a JSON format
- 🔄 **Git synchronization** - Sync your todos across multiple devices using Git

## Installation

### With cargo

```bash
# Clone the repository
git clone https://github.com/Stopkaa/UnsafeToDo.git
# Navigate to the project directory
cd UnsafeToDo

cargo install --path .

```

## Usage

### Basic Commands

```bash
# Add a new todo
unsafetodo add -t "make cup of tea☕"

# List all todos
unsafetodo show

# Complete a todo
unsafetodo complete 0

# Remove a todo
unsafetodo remove 0

```
## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
