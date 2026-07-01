---
name: i18n-migration
description: Safe multilingual/i18n migration workflow for this legacy PHPYun repository. Use when Codex is asked to plan, review, or implement translation work in htm/html/vue templates, JavaScript, CSS, SQL seed files, PHP language packs, currency formatting, money storage, or i18n migration/scanner tools.
---

# I18n Migration

## Required Context

Before editing files, read `.codex/plans/i18n-translation-plan.md`. For money/currency work, read `.codex/plans/currency-lcCoin-plan.md`. For normal repository commands and style, read `.codex/plans/repository-guidelines.md`.

## Workflow

1. Update or create the corresponding `.codex/plans/` document first, then edit code to match that plan.
2. Identify the exact target scope: one file, page group, module, or tool. Do not expand to unrelated directories.
3. Inspect existing patterns in nearby files before replacing text.
4. Classify every Chinese or user-facing string as template text, attribute text, JS UI text, CSS generated content, SQL seed data, comment, vendor code, or user-generated content.
5. Use the correct translation surface:
   - PHP recommended: `yun_t()` for normal keys, `lc()` for admin/component namespace, `yun_auto_t()` only for legacy Chinese fallback, and `lcCoin()` for money display.
   - PHP compatibility: keep existing `yun_at()` calls, but do not use it for new code unless maintaining nearby legacy style.
   - Admin Vue/template bindings: `lc('key')` or `{{ lc('key') }}`.
   - Browser JS recommended: `yunT()` for normal keys, `yunLc()` for admin/component namespace, `yunAutoT()` only for legacy Chinese fallback, and `lcCoin()` for money display.
   - Browser JS compatibility: keep existing `yunAt()` calls, but do not use it for new code unless maintaining nearby legacy style.
   - For alias cleanup, migrate `yunAt()` to `yunT()` one file at a time; first confirm the target pages load `js/yun-i18n.js` before the migrated file or already expose `yunT()`. Only rename the function and never change keys, arguments, ordering, or business logic.
   - SQL seeds: store stable display keys or add matching language-pack entries; do not call runtime functions in SQL.
   - CSS: move visible `content` text into HTML/JS where possible; do not translate selectors or class names.
6. Keep language-pack values pure text with placeholders. Do not store HTML, Vue expressions, JavaScript expressions, or Smarty blocks as translations.
7. Preserve Smarty `{yun:}` syntax, Vue `{{ }}` bindings, JS behavior, SQL quoting, and CSS selectors.
8. Run the smallest relevant validation command set before handoff.

## Validation Gates

Run these after PHP or language-pack changes:

```bash
php tools/php_lint_gate.php
php tools/scan_i18n_status.php
```

Run targeted scanners when relevant:

```bash
php tools/scan_hardcoded_php.php
php tools/scan_wap_zero_zh.php
php tools/scan_htm_missed_chinese.php
php tools/scan_vue_remaining.php
```

For SQL seed changes, import into a local test database or clearly report that SQL import was not run.

## Hard Rules

- For i18n, currency, money storage, SQL seed strategy, migration helper, shared formatter, or configuration-default changes, update the matching `.codex/plans/` document before editing code.
- Never run disabled bulk tools such as `php tools/i18n_admin_html.php`.
- Do not edit third-party vendor libraries unless the user explicitly scopes that work.
- Do not translate identifiers: variables, functions, classes, DB fields, URL params, enum values, CSS classes, IDs, or storage keys.
- Do not split sentences into translated fragments when placeholders can preserve grammar.
- Stop and fix syntax or language-pack corruption before continuing migration.
