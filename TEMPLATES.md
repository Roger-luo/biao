# Biao Template System

The template system in biao provides pre-built label configurations for common scenarios, making it easy to standardize labels across your repositories.

## Built-in Templates

Biao comes with 5 built-in templates that are compiled into the binary:

1. **standard** - Common GitHub labels used in most projects
2. **semantic** - Labels following semantic versioning conventions
3. **priority** - Priority-based labels for triage and planning
4. **type** - Type-based labels for categorizing issues/PRs
5. **area** - Area-based labels for organizing by codebase sections

## Using Built-in Templates

List all available templates:
```bash
biao template list
```

View the content of a template:
```bash
biao template show standard
```

Apply a template to your repository:
```bash
# Preview changes
biao template apply standard --dry-run

# Apply the template
biao template apply standard

# Skip labels that already exist
biao template apply standard --skip-existing
```

## User Templates

You can create custom templates in two locations:

### 1. User Config Directory (Recommended)

Place template files in `~/.config/biao/templates/`:

```bash
mkdir -p ~/.config/biao/templates
cat > ~/.config/biao/templates/my-custom.toml << 'EOF'
[[labels]]
name = "my-label"
color = "ff0000"
description = "My custom label"
EOF
```

Then use it:
```bash
biao template apply my-custom
```

### 2. System-wide Installation

For package managers and system installations, templates can be placed in:
```
/usr/local/share/biao/templates/
```

## Template File Format

Templates use the same TOML format as label configuration files:

```toml
# Template description (optional comments)

[[labels]]
name = "bug"
color = "d73a49"
description = "Something isn't working"
update_if_match = ["Bug", "bug-report"]

[[labels]]
name = "feature"
color = "a2eeef"
description = "New feature or request"
update_if_match = ["Feature request"]

# Optional: delete labels
delete = ["old-label", "deprecated"]
```

### Label Fields

- **name** (required): The label name
- **color** (optional): 6-digit hex color (without #)
  - If provided: creates new label or updates existing
  - If omitted: only updates existing labels
- **description** (optional): Label description
- **update_if_match** (optional): Array of existing label names to rename/consolidate to this label
- **skip_if_exists** (optional): Skip if label already exists (boolean)
- **update_if_exists** (optional): Update if label exists instead of skipping (boolean)

## Template Resolution Order

When you request a template, biao searches in this order:

1. Built-in templates (highest priority)
2. User templates in `~/.config/biao/templates/`
3. System templates in `/usr/local/share/biao/templates/`

If a template exists in multiple locations, the first found is used.

## Creating Custom Templates

Here's an example of creating a custom template for a Rust project:

```bash
cat > ~/.config/biao/templates/rust-project.toml << 'EOF'
# Rust Project Labels

[[labels]]
name = "bug"
color = "d73a49"
description = "A bug in the code"

[[labels]]
name = "feature"
color = "a2eeef"
description = "A new feature request"

[[labels]]
name = "documentation"
color = "0075ca"
description = "Improvements or additions to documentation"

[[labels]]
name = "platform/linux"
color = "36454f"
description = "Specific to Linux"

[[labels]]
name = "platform/macos"
color = "36454f"
description = "Specific to macOS"

[[labels]]
name = "platform/windows"
color = "36454f"
description = "Specific to Windows"

[[labels]]
name = "performance"
color = "fbca04"
description = "Performance improvements or concerns"

[[labels]]
name = "unsafe-code"
color = "ff0000"
description = "Relates to unsafe Rust code"
EOF
```

Then apply it to your repository:
```bash
biao template apply rust-project
```

## Distributing Templates

To distribute templates with your organization or project:

1. Create a templates directory in your repository
2. Store template TOML files there
3. Include instructions to copy them to `~/.config/biao/templates/`

Or, install them system-wide if packaging for distribution:
```bash
sudo mkdir -p /usr/local/share/biao/templates
sudo cp my-template.toml /usr/local/share/biao/templates/
```

## Tips and Best Practices

1. **Start with built-in templates**: Use `biao template show <name>` to see examples
2. **Keep it consistent**: Use the same color scheme across related labels
3. **Use naming conventions**: Prefix-based names (e.g., `type/`, `area/`) are easier to organize
4. **Document your templates**: Add comments explaining the purpose of labels
5. **Test before applying**: Always use `--dry-run` first
6. **Iterate**: Update templates as your project's needs evolve
