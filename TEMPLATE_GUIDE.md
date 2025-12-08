# Biao Template System - Complete Guide

## What is the Template System?

The template system in biao provides a collection of pre-configured label templates that you can apply to your GitHub repositories. Instead of manually creating and configuring labels from scratch, you can use one of the built-in templates or create your own custom ones.

## Quick Start

### 1. List Available Templates

```bash
biao template list
```

This shows all available templates (built-in + any custom ones you've created).

### 2. Preview a Template

```bash
biao template show standard
```

Shows the TOML configuration for the template.

### 3. Apply a Template

```bash
# First, preview with dry-run
cd /path/to/your/repo
biao template apply standard --dry-run

# Then apply for real
biao template apply standard
```

## Built-in Templates

### 1. **standard**
Common labels used in most GitHub projects:
- bug, feature, documentation, good first issue, help wanted, invalid, question, wontfix

Perfect for general-purpose repositories.

### 2. **semantic**
Labels following semantic versioning conventions:
- breaking, feature, bugfix, docs, refactor, test, chore, ci

Ideal for projects that follow semantic versioning for releases.

### 3. **priority**
Priority-based labels for triage and planning:
- priority/critical, priority/high, priority/medium, priority/low, priority/backlog

Helps teams organize work by urgency and importance.

### 4. **type**
Type-based labels for categorizing issues:
- type/bug, type/feature, type/enhancement, type/docs, type/question, type/test, type/refactor, type/chore

Great for organizing different kinds of work.

### 5. **area**
Area-based labels for component organization:
- area/api, area/cli, area/docs, area/core, area/testing, area/ci, area/performance, area/security

Perfect for larger projects with multiple components.

## Creating Custom Templates

### Location

Save custom templates in:
```
~/.config/biao/templates/
```

Create the directory if it doesn't exist:
```bash
mkdir -p ~/.config/biao/templates
```

### Format

Templates use TOML format. Here's a minimal example:

```toml
# My Custom Template

[[labels]]
name = "urgent"
color = "ff0000"
description = "Needs immediate attention"

[[labels]]
name = "deferred"
color = "cccccc"
description = "Can be addressed later"
```

### Full Example with Advanced Features

```toml
# Rust Project Template

[[labels]]
name = "bug"
color = "d73a49"
description = "Something isn't working"
update_if_match = ["Bug", "defect"]

[[labels]]
name = "feature"
color = "a2eeef"
description = "New feature request"
update_if_match = ["Feature request", "enhancement"]

[[labels]]
name = "platform/linux"
color = "36454f"
description = "Linux-specific issue"

[[labels]]
name = "needs-review"
color = "fbca04"
description = "Waiting for code review"

[[labels]]
name = "wontfix"
color = "ffffff"
description = "Decided not to fix"
skip_if_exists = true

delete = ["deprecated", "old-label"]
```

## Advanced Features

### update_if_match
Consolidate multiple label names into one:

```toml
[[labels]]
name = "needs-help"
color = "008672"
description = "Extra attention needed"
update_if_match = ["help wanted", "help-wanted", "needs-help"]
```

This will rename all matching labels to "needs-help" with the specified color.

### Conditional Behaviors

```toml
[[labels]]
name = "newlabel"
color = "ff0000"
description = "A new label"
skip_if_exists = true        # Skip if already exists
# OR
update_if_exists = true      # Update if exists, don't fail
```

### Create vs Update Only

```toml
# With color = create new or update existing
[[labels]]
name = "labeled"
color = "ff0000"
description = "Has color, will create or update"

# Without color = update only, won't create new
[[labels]]
name = "existing-label"
description = "No color, only updates existing"
```

### Delete Labels

```toml
delete = ["old-label", "deprecated", "remove-me"]
```

## Managing Templates

### Where Templates Are Found

Biao searches for templates in order:
1. **Built-in templates** (highest priority)
2. **User templates**: `~/.config/biao/templates/`
3. **System templates**: `/usr/local/share/biao/templates/`

If a template exists in multiple locations, the first found is used.

### Organizing User Templates

Create subdirectories for organization (optional):
```bash
~/.config/biao/templates/
├── rust/
│   ├── standard-rust.toml
│   └── web-service.toml
├── python/
│   └── data-science.toml
└── general.toml
```

But note that biao searches all TOML files, not subdirectories specifically.

## Applying Templates Safely

### Always Use Dry-Run First

```bash
biao template apply <name> --dry-run
```

This shows exactly what will be created, updated, or deleted without making changes.

### Skip Existing Labels

```bash
biao template apply standard --skip-existing
```

This skips any labels that already exist instead of failing.

### Combining Flags

```bash
biao template apply my-template --dry-run --skip-existing
```

## Common Workflows

### Scenario 1: New Repository Setup

```bash
cd ~/my-new-project
biao template apply standard
```

Instantly get a standard set of labels.

### Scenario 2: Consolidating Labels

If you have multiple old label names and want to consolidate:

```toml
[[labels]]
name = "needs-review"
color = "fbca04"
description = "Waiting for review"
update_if_match = ["review", "pending-review", "code-review"]
```

```bash
biao template apply consolidate --skip-existing
```

### Scenario 3: Organization Standards

Create templates matching your organization's standards and share them:

```bash
# Share via git repository
git clone https://github.com/myorg/label-templates.git
cp label-templates/*.toml ~/.config/biao/templates/
```

### Scenario 4: Updating Existing Labels

Change colors of existing labels without creating new ones:

```toml
# Update only (no color creates new, just updates existing)
[[labels]]
name = "bug"
color = "ff0000"
description = "Updated bug label"

[[labels]]
name = "documentation"
description = "Docs - improved description"
```

```bash
biao template apply my-updates
```

## Tips and Best Practices

### 1. Use Color Schemes
Stick to a consistent color scheme within a template:
- **Red** (#d73a49): bug, urgent, breaking
- **Blue** (#0075ca): docs, feature
- **Green** (#28a745): good first issue, help
- **Yellow** (#fbca04): priority/medium, needs-review

### 2. Naming Conventions
Use prefixes for related labels:
- `priority/critical`, `priority/high`, `priority/low`
- `type/bug`, `type/feature`, `type/docs`
- `area/api`, `area/cli`, `area/docs`

### 3. Keep Descriptions Clear
Help your team understand what each label is for:
```toml
[[labels]]
name = "backlog"
color = "cccccc"
description = "Good issues, but not currently planned"
```

### 4. Start Simple
Don't create too many labels at once. Start with 10-15 essential ones.

### 5. Document Your Templates
Add comments explaining the purpose:
```toml
# Priority labels for sprint planning
# Critical - drop everything and fix
# High - include in current sprint
# Medium - next sprint or soon
# Low - backlog, no time commitment
```

### 6. Test on a Test Repo First
Before applying to important repos:
```bash
cd ~/test-repo
biao template apply my-new-template --dry-run
```

## Troubleshooting

### Template Not Found
```
Error: Template 'mytemplate' not found.
```

Check that the file exists:
```bash
ls ~/.config/biao/templates/mytemplate.toml
```

### Invalid TOML Format
```
Error: Failed to parse template
```

Validate your TOML:
```bash
# Use an online TOML validator or install toml-cli
cargo install toml-cli
toml ~/.config/biao/templates/mytemplate.toml
```

### Color Format Error
```
Error: Color must be 6 hex digits
```

Use 6 hex digits without the `#`:
- ✅ `ff0000` (red)
- ❌ `#ff0000` (includes #)
- ❌ `f00` (only 3 digits)

## Further Reading

- See `TEMPLATES.md` for technical details
- See `README.md` for general usage
- Run `biao template --help` for command help
- Run `biao template show <name>` to see any template's full content
