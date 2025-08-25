# CLI Todo Application

A simple and efficient command-line todo application that helps you manage your tasks directly from the terminal. Your todos are stored in JSON format and can be synchronized across devices using Git repositories.

## Features

- ✅ **Add todos** - Create new tasks with ease
- ❌ **Remove todos** - Delete completed or unwanted tasks
- 📋 **Display todos** - View all your tasks in a clean format
- ✔️ **Complete todos** - Mark tasks as finished
- 🔄 **Sort todos** - Organize your tasks by priority, date, or status
- 💾 **JSON storage** - All data stored in human-readable JSON format
- 🔄 **Git synchronization** - Sync your todos across multiple devices using Git

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/cli-todo-app.git

# Navigate to the project directory
cd cli-todo-app

# Install dependencies (if any)
npm install

# Make the script executable (Unix/Linux/macOS)
chmod +x todo
```

## Usage

### Basic Commands

```bash
# Add a new todo
./todo add "Buy groceries"
./todo add "Finish project documentation" --priority high

# List all todos
./todo list
./todo ls

# Complete a todo
./todo complete 1
./todo done 1

# Remove a todo
./todo remove 1
./todo rm 1

# Sort todos
./todo sort --by priority
./todo sort --by date
./todo sort --by status
```

### Advanced Usage

```bash
# Add todo with due date
./todo add "Submit report" --due "2024-01-15"

# Filter todos by status
./todo list --status pending
./todo list --status completed

# Search todos
./todo search "project"

# Clear all completed todos
./todo clear-completed
```

## Git Synchronization

The application supports synchronizing your todos across multiple devices using Git repositories.

### Setup Git Sync

```bash
# Initialize git sync with your repository
./todo sync init https://github.com/yourusername/my-todos.git

# Push current todos to remote repository
./todo sync push

# Pull todos from remote repository
./todo sync pull

# Automatic sync (pushes and pulls)
./todo sync
```

### Sync Configuration

Create a `.todoconfig` file in your home directory or project root:

```json
{
  "git": {
    "repository": "https://github.com/yourusername/my-todos.git",
    "auto_sync": true,
    "sync_on_change": false
  },
  "display": {
    "show_completed": true,
    "date_format": "YYYY-MM-DD",
    "sort_default": "priority"
  }
}
```

## File Structure

```
~/.todos/
├── todos.json          # Main todo storage file
├── config.json         # Application configuration
└── .git/              # Git repository (if sync enabled)
```

### Todo JSON Format

```json
{
  "todos": [
    {
      "id": 1,
      "text": "Buy groceries",
      "status": "pending",
      "priority": "medium",
      "created": "2024-01-10T10:00:00Z",
      "due": "2024-01-12T18:00:00Z",
      "completed": null
    }
  ],
  "lastId": 1,
  "version": "1.0.0"
}
```

## Command Reference

| Command | Alias | Description | Example |
|---------|-------|-------------|---------|
| `add` | `a` | Add a new todo | `todo add "Task description"` |
| `list` | `ls`, `l` | List all todos | `todo list` |
| `complete` | `done`, `c` | Mark todo as complete | `todo complete 1` |
| `remove` | `rm`, `delete` | Remove a todo | `todo remove 1` |
| `sort` | `s` | Sort todos | `todo sort --by priority` |
| `sync` | - | Git synchronization | `todo sync` |
| `search` | `find` | Search todos | `todo search "keyword"` |
| `help` | `h` | Show help | `todo help` |

## Options and Flags

- `--priority, -p` - Set priority (low, medium, high)
- `--due, -d` - Set due date (YYYY-MM-DD format)
- `--status, -s` - Filter by status (pending, completed)
- `--sort-by` - Sort criteria (priority, date, status, text)
- `--reverse, -r` - Reverse sort order
- `--json` - Output in JSON format
- `--help, -h` - Show command help

## Configuration

### Global Configuration

Edit `~/.todos/config.json`:

```json
{
  "display": {
    "show_ids": true,
    "show_dates": true,
    "show_priority": true,
    "compact_mode": false
  },
  "behavior": {
    "confirm_delete": true,
    "auto_save": true,
    "case_sensitive_search": false
  },
  "sync": {
    "auto_push": false,
    "auto_pull": true,
    "conflict_resolution": "merge"
  }
}
```

## Examples

### Daily Workflow

```bash
# Morning: Check your todos
todo list

# Add urgent task
todo add "Call client about meeting" --priority high --due today

# Complete a task
todo done 2

# End of day: sync with remote
todo sync
```

### Team Collaboration

```bash
# Setup shared repository
todo sync init https://github.com/team/shared-todos.git

# Pull latest updates
todo sync pull

# Add team task
todo add "Review pull request #123" --priority high

# Share updates
todo sync push
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

```bash
# Run tests
npm test

# Run linter
npm run lint

# Build for distribution
npm run build
```

## Troubleshooting

### Common Issues

**Git sync fails:**
```bash
# Check git configuration
git config --list

# Verify repository access
git ls-remote https://github.com/yourusername/my-todos.git
```

**Todos not saving:**
- Check file permissions in `~/.todos/` directory
- Verify JSON format is valid

**Command not found:**
- Ensure the script is executable: `chmod +x todo`
- Add to PATH or use full path: `./todo`

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Changelog

### v1.0.0
- Initial release
- Basic CRUD operations
- JSON storage
- Git synchronization
- Sorting and filtering

---

**Happy task managing! 🚀**
