# biao - GitHub Label Manager CLI

A fast, efficient Rust CLI for managing GitHub repository labels using the `gh` CLI.

## Features

- ✅ **List** all labels in a repository
- ✅ **Get** details of a specific label
- ✅ **Create** new labels with color and description
- ✅ **Update** existing labels (name, color, description)
- ✅ **Delete** labels with confirmation prompts
- ✅ **Batch operations** from TOML config files with dry-run mode
- ✅ Auto-detects repository from git remote (zero config)
- ✅ Leverages `gh` for authentication (no manual token handling)
- ✅ Colored terminal output with progress indicators
- ✅ Comprehensive error handling and validation

## Requirements

- [GitHub CLI (`gh`)](https://cli.github.com/) installed and authenticated
- Rust 1.56+ (for building from source)

## Installation

Build from source:

```bash
cargo build --release
./target/release/biao --help
```

## Quick Start

```bash
# 1. Authenticate with GitHub
biao auth login

# 2. Navigate to your repository
cd ~/my-project

# 3. List labels
biao list

# 4. Create a label
biao create "bug" "d73a49" --description "Something is broken"

# 5. Batch operations from a config file
cat > labels.toml << 'EOF'
[[new]]
name = "priority-high"
color = "d73a49"
EOF

biao apply labels.toml --dry-run  # Preview changes
biao apply labels.toml             # Apply changes
```

## Configuration

No configuration needed! `biao` automatically detects your repository from the git remote:

1. Navigate to any directory within your git repository
2. Run any `biao` command
3. It will auto-detect the repository owner and name from your `origin` remote

### Supported Remote URLs

- HTTPS: `https://github.com/owner/repo.git`
- SSH: `git@github.com:owner/repo.git`
- Both with and without `.git` suffix

**Authentication Setup:**

Option 1: Use the built-in auth command (recommended)

```bash
biao auth login
# Follow prompts to authenticate with GitHub
```

Option 2: Use `gh` CLI directly

```bash
gh auth login
```

Both commands handle authentication with GitHub automatically using keychain/credential storage.

**Missing GitHub CLI:**

If `gh` is not installed, biao will provide a helpful error message with installation instructions:

```
Error: gh CLI not found: github.com/cli/cli

Please install GitHub CLI: https://cli.github.com/
```

Install it via:
- **macOS**: `brew install gh`
- **Linux**: See [GitHub CLI docs](https://github.com/cli/cli/blob/trunk/docs/install_linux.md)
- **Windows**: `choco install gh` or `scoop install gh`

## Error Handling

**Not in a git repository:**
```
Error: Invalid input: Not a git repository. Run this command from within a git repository.
```

**Missing GitHub origin:**
```
Error: Invalid input: Could not find remote.origin.url. Make sure your repository has an origin remote pointing to GitHub.
```

**Missing gh CLI:**
```
Error: gh CLI not found: github.com/cli/cli

Please install GitHub CLI: https://cli.github.com/
```

## Why This Approach?

- **Zero Config**: No need to set environment variables or CLI flags
- **Less Error-Prone**: Can't accidentally work on the wrong repository
- **Faster Workflow**: Just `cd` into a repo and start managing labels
- **Integrates Naturally**: Works with your existing git setup

## Usage Examples

### Working with Labels

All commands automatically use the repository from your current git directory.

#### Authentication

Login to GitHub:

```bash
biao auth login
```

Check authentication status:

```bash
biao auth status
```

Logout:

```bash
biao auth logout
```

### Using Templates

`biao` comes with 5 built-in label templates covering common scenarios:

#### List available templates

```bash
biao template list
```

Output:
```
Available Templates:
  standard - Standard GitHub labels (bug, feature, documentation, etc.)
  semantic - Semantic labels (breaking, feature, bugfix, docs, etc.)
  priority - Priority-based labels (critical, high, medium, low)
  type - Type-based labels (type/bug, type/feature, type/docs, etc.)
  area - Area-based labels (area/api, area/cli, area/docs, etc.)
```

#### View a template

```bash
biao template show standard
```

#### Apply a template to your repository

```bash
# Preview changes (dry-run)
biao template apply standard --dry-run

# Apply the template
biao template apply standard

# Apply with skip if labels exist
biao template apply standard --skip-existing
```

#### Available Templates

1. **standard** - Common GitHub labels (bug, feature, documentation, good first issue, help wanted, etc.)
2. **semantic** - Semantic versioning labels (breaking, feature, bugfix, docs, refactor, test, chore, ci)
3. **priority** - Priority-based triage (critical, high, medium, low, backlog)
4. **type** - Type categorization (type/bug, type/feature, type/docs, type/test, type/chore, etc.)
5. **area** - Area-based organization (area/api, area/cli, area/docs, area/core, area/testing, etc.)

### List all labels

```bash
biao list
```

### Get a specific label

```bash
biao get "bug"
```

### Create a new label

```bash
biao create "feature" "00ff00" --description "New feature"
```

Colors should be 6-digit hex without the `#`:
- Red: `ff0000`
- Green: `00ff00`
- Blue: `0000ff`

### Update a label

```bash
# Change name
biao update "bug" --new-name "bug-report"

# Change color and description
biao update "feature" --color "00aa00" --description "Feature requests"
```

### Delete a label

```bash
# Interactive confirmation
biao delete "wontfix"

# Skip confirmation
biao delete "wontfix" -f
```

### Batch operations with TOML config

Create a `labels.toml` file:

```toml
# Delete labels (put this first due to TOML syntax)
delete = ["wontfix", "invalid"]

# Create new labels
[[new]]
name = "priority-high"
color = "d73a49"
description = "High priority issue"

[[new]]
name = "priority-low"
color = "0075ca"

# Skip if already exists (per-label control)
[[new]]
name = "bug"
color = "d73a49"
skip_if_exists = true

# Update if already exists (per-label control)
[[new]]
name = "feature"
color = "a2eeef"
description = "New feature"
update_if_exists = true

# Update existing labels
[[update]]
name = "bug"
description = "Updated description"

[[update]]
name = "enhancement"
new_name = "feature"
color = "a2eeef"
```

Apply the config:

```bash
# Dry run first (recommended)
biao apply labels.toml --dry-run

# Apply the changes
biao apply labels.toml

# Skip labels that already exist instead of failing
biao apply labels.toml --skip-existing

# Or with a different file
biao apply my-labels.toml
```

**Conflict Handling:**

Control what happens when a label in `[[new]]` already exists:

**Global flag:**
- `--skip-existing`: Skip all existing labels

**Per-label control in TOML:**
- `skip_if_exists = true`: Skip this label if it exists
- `update_if_exists = true`: Update this label if it exists (preserves name)

**Priority:** Per-label flags override the global `--skip-existing` flag.

**Behavior without any flags:** Operation fails with an error.

Example output:
```
▶ Creating 4 new label(s):
  ✓ Creating 'priority-high'... OK
  ✓ Creating 'bug'... SKIPPED (already exists)
  ✓ Creating 'feature'... EXISTS (updating)... UPDATED
  ✓ Creating 'priority-low'... OK

=== Summary ===
  Success: 3
  Skipped: 1
```

## Architecture

```
src/
├── main.rs      # Entry point
├── cli.rs       # Command parsing and handlers
├── client.rs    # GitHub CLI wrapper
├── models.rs    # Data structures
├── error.rs     # Error types
└── lib.rs       # Library exports
```

### Key Modules

**`client.rs`** - `GithubClient`
- Wraps `gh` CLI calls
- Methods: `list_labels()`, `get_label()`, `create_label()`, `update_label()`, `delete_label()`
- Parses JSON responses from `gh api`

**`cli.rs`** - Command parsing
- Uses `clap` for argument parsing
- Color-coded output with `colored` crate
- Input validation and confirmation prompts

**`models.rs`** - Data structures
- `GithubLabel` - Response model from GitHub API
- `CreateLabelRequest` - Request for creation
- `UpdateLabelRequest` - Request for updates

**`error.rs`** - Error handling
- Custom `BiaoError` type with `thiserror`
- Specific error variants for gh CLI, parsing, and env errors

## Dependencies

- **tokio** - Async runtime
- **serde** - JSON serialization
- **clap** - CLI argument parsing
- **colored** - Terminal colors
- **thiserror** - Error types

### Workflow Example

```bash
# Navigate to your repository
cd ~/projects/my-awesome-repo

# Setup authentication (one time)
biao auth login

# List existing labels (automatically detects owner/repo from git remote)
biao list

# Create labels for a workflow
biao create "priority-high" "d73a49"
biao create "priority-medium" "f0883e"
biao create "status-in-progress" "0075ca"

# Update a label
biao update "priority-high" --description "High priority - resolve ASAP"

# View the final state
biao list
```

## Why Zero-Config Auto-Detection?

`biao` automatically discovers your repository context without any setup:

- **Git Integration**: Uses your `.git/config` to find the repository owner and name
- **Smart Detection**: Searches up from current directory for git repository root
- **No Environment Variables**: No need to export `GITHUB_OWNER` or `GITHUB_REPO`
- **No CLI Flags**: No `--owner` or `--repo` flags to remember

Just navigate to your git repository and run `biao` - it figures out everything automatically!

**Error Messages**

If something goes wrong, you'll get helpful messages:

## Future Enhancements

Potential features:
- Batch operations from JSON/YAML files
- Label syncing between repos
- Template library (common label sets)
- Dry-run mode
- Export labels to file
- Import from existing labels


