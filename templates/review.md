# Code Review Request

**Commit:** `{{commit_hash}}`
**Message:** {{commit_message}}
**Author:** {{author}}
**Date:** {{date}}
**Branch:** {{branch}}

{{#if affected_packages}}
## Affected Packages
{{#each affected_packages}}
- {{this}}
{{/each}}
{{/if}}

## Statistics
```
{{diff_stat}}
```

## Changes

```diff
{{diff}}
```

{{#if recent_commits}}
## Recent Context
{{#each recent_commits}}
- `{{hash}}` {{message}}
{{/each}}
{{/if}}

## Instructions

Ge en kort code review av denna commit. Fokusera på:

1. **Buggar** - Finns det uppenbara buggar eller edge cases?
2. **Förbättringar** - Kan koden förbättras utan att ändra funktionalitet?
3. **Säkerhet** - Finns det säkerhetsproblem?
4. **Best practices** - Följer koden idiomatiska mönster?

Håll svaret koncist (max 15 rader). Börja med en emoji som indikerar allvarlighetsgrad:
- ✅ Ser bra ut
- 💡 Mindre förslag
- ⚠️ Bör åtgärdas
- 🚨 Kritiskt problem
