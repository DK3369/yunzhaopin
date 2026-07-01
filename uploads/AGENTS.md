# Repository Guidelines

This is the Codex entrypoint for the repository. Keep detailed project guidance under `.codex/` so plans, rules, and skills stay organized.

Before making changes, read `.codex/plans/repository-guidelines.md` for structure, commands, style, testing, commits, and security notes.

## Documentation-First Change Rule

Before changing code for i18n, currency, money storage, configuration strategy, migration scripts, shared helpers, SQL seed strategy, or other cross-cutting project behavior, update or create the corresponding document under `.codex/plans/` first. Then implement code to match the documented plan. If no matching plan exists, create one before editing production files.

For multilingual/i18n work, also read `.codex/plans/i18n-translation-plan.md` and `.codex/skills/i18n-migration/SKILL.md` before editing `htm/html/vue`, `js`, `css`, `sql`, language packs, or migration tools.

For currency symbols, cents-based money storage, or `lcCoin` work, read `.codex/plans/currency-lcCoin-plan.md` first.

Project Codex files:

- `.codex/config.toml` stores project-scoped Codex settings.
- `.codex/rules/` stores command safety rules.
- `.codex/plans/` stores implementation plans and repository guidance.
- `.codex/skills/` stores reusable task workflows.
- `.codex/prompts/` stores reusable prompts when needed.
- `.codex/references/` stores supporting reference material when needed.
