# Changelog Generation

Baserat på följande commits sedan senaste push, generera en changelog-entry.

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

Generera en changelog-entry i Keep a Changelog-format (https://keepachangelog.com/).

Kategorisera ändringar under rätt rubrik:
- **Added** - Ny funktionalitet
- **Changed** - Ändringar i befintlig funktionalitet
- **Deprecated** - Funktionalitet som snart tas bort
- **Removed** - Borttagen funktionalitet
- **Fixed** - Buggfixar
- **Security** - Säkerhetsuppdateringar

Format:
```markdown
## [Unreleased]

### Added
- Kort beskrivning av ny feature

### Fixed
- Kort beskrivning av fix
```

Håll beskrivningarna koncisa och användarfokuserade (inte utvecklarfokuserade).
