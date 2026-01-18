# Changelog Generation

Based on the following commits since last push, generate a changelog entry.

## Commits

{{#each commits}}
### `{{hash}}`
**Message:** {{message}}
**Author:** {{author}}
**Date:** {{date}}

```diff
{{diff}}
```

---
{{/each}}

## Instructions

Generate a changelog entry in Keep a Changelog format (https://keepachangelog.com/).

Categorize changes under the appropriate heading:
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Features that will be removed soon
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security updates

Format:
```markdown
## [Unreleased]

### Added
- Brief description of new feature

### Fixed
- Brief description of fix
```

Keep descriptions concise and user-focused (not developer-focused).
