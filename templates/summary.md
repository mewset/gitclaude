# Merge Summary

**Merged branch:** {{source_branch}}
**Into:** {{target_branch}}
**Commits merged:** {{commit_count}}

## Commits

{{#each commits}}
- `{{hash}}` {{message}}
{{/each}}

## Overall Diff

```
{{diff_stat}}
```

## Instructions

Provide a summary of this merge in max 5 lines:

1. What was the main purpose of this branch?
2. What major changes were made?
3. Is there anything that should be followed up on?

Start the summary with an appropriate emoji:
- 🚀 Feature release
- 🐛 Bug fix
- 🔧 Refactoring
- 📚 Documentation
- 🔒 Security
