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

Provide a brief code review of this commit. Focus on:

1. **Bugs** - Are there obvious bugs or edge cases?
2. **Improvements** - Can the code be improved without changing functionality?
3. **Security** - Are there any security concerns?
4. **Best practices** - Does the code follow idiomatic patterns?

Keep the response concise (max 15 lines). Start with an emoji indicating severity:
- ✅ Looks good
- 💡 Minor suggestions
- ⚠️ Should be addressed
- 🚨 Critical issue
