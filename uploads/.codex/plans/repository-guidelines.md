# Repository Guidelines

## Project Structure & Module Organization

This repository is a PHP recruitment system located under `uploads/`. Entry points include `index.php`, `admin/index.php`, `member/index.php`, `wap/index.php`, and API routes under `api/`. Core business code is split across `app/model`, `app/controller`, `admin/model`, `member`, `wap/member`, and `api/wxapp`. Templates and Vue-style admin components live in `app/template`; static JavaScript and vendor assets live in `js/` and `wap/js/`. Language packs and generated i18n data are in `data/lang` and `data/i18n_build`. Maintenance and migration scripts are in `tools/`.

## Build, Test, and Development Commands

There is no package-manager build step in this tree. Run PHP directly from `uploads/`:

- `php -S 127.0.0.1:8000 -t .` starts a local PHP server.
- `php tools/php_lint_gate.php` checks syntax for the main business PHP directories.
- `php tools/scan_i18n_status.php` runs the combined i18n status gate.
- `php tools/scan_wap_zero_zh.php` and `php tools/scan_hardcoded_php.php` inspect remaining untranslated or hardcoded text.

Run targeted scripts after changing related templates, language packs, or migration helpers.

## Coding Style & Naming Conventions

Follow the existing legacy PHP style. Controller and model files commonly use names like `index.class.php`, `resume.class.php`, and classes such as `index_controller`. Keep array syntax as `array(...)` unless editing a file that already uses short arrays. Preserve existing spacing around framework calls such as `$this -> MODEL(...)` when working nearby. Use i18n helpers (`yun_t`, `yun_at`, `lc`, `yun_auto_t`) instead of adding new user-facing hardcoded strings.

## Testing Guidelines

No formal unit test suite is present. Treat syntax and migration scanners as required checks before handoff. At minimum, run `php tools/php_lint_gate.php` for PHP changes. For template, WAP, API, or language-pack work, also run the relevant i18n scanner and manually exercise the changed page or endpoint in a local browser/server.

## Commit & Pull Request Guidelines

Recent commits use scoped prefixes such as `fix(i18n): ...`, `i18n(d16): ...`, and `tools(d15c): ...`. Keep messages short, scoped, and outcome-focused. Pull requests should describe changed areas, list validation commands run, call out language-pack or generated-data updates, and include screenshots for visible admin/WAP/template changes.

## Security & Configuration Tips

Do not commit local secrets from `config/db.config.php`, generated uploads, backups, or cache output. Review changes under `data/backup`, `data/upload`, and `data/templates_c` carefully; these are usually runtime artifacts rather than source changes.

## Agent-Specific Instructions

For i18n work, read `.codex/plans/i18n-translation-plan.md` and `.codex/skills/i18n-migration/SKILL.md` before editing `htm/html/vue`, `js`, `css`, `sql`, language packs, or migration tools. Keep each migration batch narrow, run the relevant scanners, and never use disabled bulk scripts such as `tools/i18n_admin_html.php`.

## Currency & Money Formatting

For money display, cents-based database storage, currency JSON, and `lcCoin` rules, read `.codex/plans/currency-lcCoin-plan.md` before changing finance, package, payment, order, or price display code.

## Documentation-First Change Rule

For cross-cutting changes, update `.codex/plans/` before code. This includes i18n, currency symbols, cents-based money storage, SQL seed strategy, migration scripts, shared helpers, configuration defaults, and agent rules. Code changes must follow the documented plan, and handoff notes should name the plan file that was updated.
