# Pre-commit Validation

**Branch:** {{branch}}
**Staged files:** {{staged_count}}

## Staged Changes

```diff
{{staged_diff}}
```

## Instructions

Validera dessa ändringar innan commit. Kontrollera:

1. **Syntaxfel** - Finns det uppenbara syntaxfel?
2. **Debug-kod** - Finns det kvarglömd debug-kod (console.log, print, dbg!, etc.)?
3. **Känslig data** - Exponeras API-nycklar, lösenord eller annan känslig data?
4. **TODO/FIXME** - Finns det oavslutade TODOs som borde åtgärdas?
5. **Stora filer** - Committas onödigt stora filer?

Svara med endast ett av följande:
- `PASS` - Inga problem hittades
- `WARN: <anledning>` - Varning men kan committas
- `FAIL: <anledning>` - Bör inte committas

Exempel:
```
WARN: console.log på rad 45 i src/api.ts
```
```
FAIL: API_KEY exponerad i config.json
```
