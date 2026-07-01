# Codex Rules

These files define project command safety rules. They complement, but do not replace, the documentation-first workflow.

Before changing code for i18n, currency, money storage, SQL seed strategy, shared helpers, configuration defaults, or migration scripts, update the corresponding plan in `.codex/plans/` first. If no plan exists, create one before touching production code.

Use `.rules` files for enforceable command decisions such as forbidding destructive Git commands or disabled migration scripts.
