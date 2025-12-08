# Template System Implementation Summary

## Overview

A complete template system has been implemented for biao, allowing users to quickly apply pre-built label configurations to their GitHub repositories.

## Components Added

### 1. Template Module (`src/templates.rs`)

- **TemplateManager**: Main class for managing template discovery and loading
- **TemplateInfo**: Metadata structure for templates
- Template search paths:
  - Built-in templates (compiled into binary)
  - User config: `~/.config/biao/templates/`
  - System-wide: `/usr/local/share/biao/templates/`

**Key Methods:**
- `list()`: Discover all available templates
- `get(name)`: Load template by name
- Built-in template getters for each default template

**Built-in Templates:**
1. `standard` - Common GitHub labels
2. `semantic` - Semantic versioning labels
3. `priority` - Priority-based triage labels
4. `type` - Type categorization labels
5. `area` - Area/component-based labels

### 2. CLI Integration (`src/cli.rs`)

**New Commands:**
- `biao template list` - List all available templates
- `biao template show <name>` - Display template content
- `biao template apply <name>` - Apply template to repository

**Features:**
- Template commands don't require git repository (unlike other commands)
- Supports `--dry-run` and `--skip-existing` flags for safe application
- Temporary file handling for applying templates via existing `cmd_apply` function

### 3. Template Files (`templates/` directory)

Five TOML template files included with the distribution:
- `standard.toml` - 8 common labels with cleanup
- `semantic.toml` - 8 semantic versioning labels
- `priority.toml` - 5 priority-based labels with renaming rules
- `type.toml` - 8 type-categorized labels with consolidation
- `area.toml` - 8 area-based labels for component organization

### 4. Documentation

**Main README.md Updates:**
- Template overview and quick start
- Template list with descriptions
- Commands and examples

**New TEMPLATES.md:**
- Comprehensive guide to using templates
- User template creation instructions
- Template file format specification
- Template resolution order
- Best practices and examples
- Tips for distributing templates

## Features

### Built-in Template Management
- Templates compiled into binary (no external files needed)
- Automatic discovery of user and system templates
- Duplicate handling (first found wins)

### User Template Support
- Custom templates in `~/.config/biao/templates/`
- System-wide templates in `/usr/local/share/biao/templates/`
- TOML format compatible with regular label configs

### Template Features
- `update_if_match` array for consolidating multiple label names
- Conflict handling with `skip_if_exists` and `update_if_exists`
- Optional color field (create vs update only)
- Label deletion support

### Safety Features
- `--dry-run` mode to preview changes
- `--skip-existing` flag to avoid overwriting
- Temporary file cleanup
- Error messages guide users to correct template usage

## Testing

All new functionality covered by tests:
- `test_template_manager_creation` - Manager initialization
- `test_get_builtin_template` - Built-in template retrieval
- `test_get_template` - Template loading
- `test_template_not_found` - Error handling

**Test Results:** ✅ 14 tests passing

## Usage Examples

### List templates
```bash
$ biao template list
Available Templates:
  area - Area-based labels (area/api, area/cli, area/docs, etc.)
  priority - Priority-based labels (critical, high, medium, low)
  semantic - Semantic labels (breaking, feature, bugfix, docs, etc.)
  standard - Standard GitHub labels (bug, feature, documentation, etc.)
  type - Type-based labels (type/bug, type/feature, type/docs, etc.)
```

### Show template
```bash
$ biao template show priority
Template: priority

# Priority-Based Labels Template
# Use priority levels to triage and prioritize work
...
```

### Apply template
```bash
$ biao template apply standard --dry-run
Repository: auto-detected
Template: standard

▶ Processing 8 label(s):
  ✓ Creating 'bug'... [DRY RUN]
  ✓ Creating 'feature'... [DRY RUN]
  ...

=== Summary ===
  Success: 8
  This was a dry run. No actual changes were made.
```

## File Structure

```
biao/
├── src/
│   ├── templates.rs         # Template manager and built-in templates
│   ├── cli.rs               # Template CLI commands (updated)
│   ├── main.rs              # Module declaration (updated)
│   └── ...
├── templates/               # Shipped template files
│   ├── standard.toml
│   ├── semantic.toml
│   ├── priority.toml
│   ├── type.toml
│   └── area.toml
├── TEMPLATES.md             # Template system documentation
├── README.md                # Updated with template info
└── ...
```

## Future Enhancements

Potential improvements for future versions:
1. Template validation/linting
2. Template composition (combining multiple templates)
3. Template versioning
4. Interactive template selector
5. Template publishing/sharing system
6. Template variables/customization
7. Web-based template gallery

## Notes

- Templates are embedded in the binary for easy distribution
- User templates can override built-in ones with the same name
- All template operations use existing label infrastructure
- Backward compatible with all existing features
