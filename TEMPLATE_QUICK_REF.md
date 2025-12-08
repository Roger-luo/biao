# Biao Template System - Quick Reference

## Commands

```bash
# List all available templates
biao template list

# Show template content
biao template show <name>

# Apply a template (with options)
biao template apply <name> [OPTIONS]
```

## Template Options

```bash
biao template apply standard
biao template apply standard --dry-run              # Preview changes
biao template apply standard --skip-existing        # Don't fail on existing labels
biao template apply standard -n                     # Short form: --dry-run
biao template apply standard -s                     # Short form: --skip-existing
```

## Built-in Templates

| Template | Purpose | Labels Count |
|----------|---------|--------------|
| **standard** | General GitHub projects | 8 + cleanup |
| **semantic** | Semantic versioning | 8 |
| **priority** | Sprint planning & triage | 5 |
| **type** | Issue/PR categorization | 8 |
| **area** | Component organization | 8 |

## Template File Locations

- **Built-in**: Compiled into binary (always available)
- **User**: `~/.config/biao/templates/` (TOML files)
- **System**: `/usr/local/share/biao/templates/` (TOML files)

## Template File Format

```toml
[[labels]]
name = "bug"
color = "d73a49"
description = "Something isn't working"
update_if_match = ["Bug", "defect"]
skip_if_exists = false
update_if_exists = false

delete = ["old-label"]
```

### Label Field Reference

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Label name |
| `color` | string (6 hex) | No | Color (create/update only if present) |
| `description` | string | No | Label description |
| `update_if_match` | array | No | Label names to rename to this name |
| `skip_if_exists` | bool | No | Skip if label already exists |
| `update_if_exists` | bool | No | Update if exists instead of failing |

## Common Workflows

### Apply Built-in Template
```bash
cd /path/to/repo
biao template apply standard --dry-run
biao template apply standard
```

### Create Custom Template
```bash
mkdir -p ~/.config/biao/templates
cat > ~/.config/biao/templates/my-template.toml << 'EOF'
[[labels]]
name = "custom"
color = "ff0000"
description = "My custom label"
EOF
biao template apply my-template
```

### Preview Before Applying
```bash
biao template apply semantic --dry-run
```

### Apply Safely to Existing Repo
```bash
biao template apply standard --skip-existing
```

### Consolidate Label Names
```toml
# my-template.toml
[[labels]]
name = "needs-help"
color = "008672"
update_if_match = ["help wanted", "help-wanted"]
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Template not found | Check `~/.config/biao/templates/` for TOML files |
| TOML parse error | Validate TOML format (check quotes, commas) |
| Invalid color | Use 6 hex digits without `#` (e.g., `ff0000`) |
| Labels won't apply | Use `--dry-run` first to see what's happening |
| Need built-in template | Run `biao template show <name>` to see full content |

## Color Reference

Common colors (without #):
- Red: `d73a49` or `ff0000`
- Green: `28a745` or `00ff00`
- Blue: `0075ca` or `0000ff`
- Yellow: `fbca04` or `ffff00`
- Gray: `cccccc` or `ffffff`

## Examples

### Standard Labels
```bash
biao template apply standard
```
Creates: bug, feature, documentation, good first issue, help wanted, invalid, question, wontfix

### Semantic Versioning
```bash
biao template apply semantic
```
Creates: breaking, feature, bugfix, docs, refactor, test, chore, ci

### Priority-Based Triage
```bash
biao template apply priority
```
Creates: priority/critical, priority/high, priority/medium, priority/low, priority/backlog

### Type-Based Organization
```bash
biao template apply type
```
Creates: type/bug, type/feature, type/enhancement, type/docs, type/question, type/test, type/refactor, type/chore

### Area-Based Components
```bash
biao template apply area
```
Creates: area/api, area/cli, area/docs, area/core, area/testing, area/ci, area/performance, area/security

## See Also

- `TEMPLATE_GUIDE.md` - Comprehensive guide with examples
- `TEMPLATES.md` - Technical implementation details
- `README.md` - General biao documentation
