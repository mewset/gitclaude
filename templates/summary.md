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

Ge en sammanfattning av denna merge på max 5 rader:

1. Vad var huvudsyftet med denna branch?
2. Vilka större ändringar gjordes?
3. Finns det något som bör följas upp?

Börja sammanfattningen med en passande emoji:
- 🚀 Feature release
- 🐛 Bug fix
- 🔧 Refactoring
- 📚 Documentation
- 🔒 Security
