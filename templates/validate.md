# Pre-commit Validation

**Branch:** {{branch}}
**Staged files:** {{staged_count}}

## Staged Changes

```diff
{{staged_diff}}
```

## Instructions

Validate these changes before commit. Check for:

1. **Syntax errors** - Are there obvious syntax errors?
2. **Debug code** - Is there leftover debug code (console.log, print, dbg!, etc.)?
3. **Sensitive data** - Are API keys, passwords, or other sensitive data exposed?
4. **TODO/FIXME** - Are there unfinished TODOs that should be addressed?
5. **Large files** - Are unnecessarily large files being committed?

Respond with only one of the following:
- `PASS` - No issues found
- `WARN: <reason>` - Warning but can be committed
- `FAIL: <reason>` - Should not be committed

Examples:
```
WARN: console.log on line 45 in src/api.ts
```
```
FAIL: API_KEY exposed in config.json
```
