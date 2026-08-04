#!/usr/bin/env python3
"""Verify stable error keys are translated in every supported locale."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LOCALES = ("en", "zh-CN", "zh-TW")
TAG_RE = re.compile(r"=>\s*\"([a-z][a-z0-9_]+)\"\.into\(\)")
KEY_RE = re.compile(r"(?:param_invalid|InvalidParam|param_missing)\(\s*\"([a-z][a-z0-9_]*)\"")


def main() -> int:
    locale_data = {}
    for locale in LOCALES:
        path = ROOT / "locales" / f"{locale}.json"
        try:
            doc = json.loads(path.read_text(encoding="utf-8"))
            locale_data[locale] = doc["errors"]
        except (OSError, json.JSONDecodeError, KeyError) as exc:
            print(f"ERROR {path}: {exc}")
            return 2

    keys = set()
    for path in (ROOT / "crates").rglob("*.rs"):
        source = path.read_text(encoding="utf-8", errors="replace")
        keys.update(KEY_RE.findall(source))
        # ApiError implementations use stable string tags in their tag()
        # matches. Include those tags so domain errors cannot fall back to a
        # raw machine key when a locale entry is missing.
        keys.update(TAG_RE.findall(source))

    missing = []
    for key in sorted(keys):
        for locale, errors in locale_data.items():
            value = errors.get(key)
            if not isinstance(value, str) or not value.strip():
                missing.append((key, locale, "missing/empty"))
            elif value == f"errors.{key}":
                missing.append((key, locale, "untranslated key"))

    sets = {locale: set(errors) for locale, errors in locale_data.items()}
    union = set().union(*sets.values())
    for key in sorted(union):
        present = [locale for locale, errors in sets.items() if key in errors]
        if len(present) != len(LOCALES):
            missing.append((key, ",".join(present), "locale set mismatch"))

    print(f"stable source keys: {len(keys)}")
    print("locale error keys: " + ", ".join(f"{k}={len(v)}" for k, v in sets.items()))
    if missing:
        for key, locale, reason in missing:
            print(f"MISSING {key} [{locale}] ({reason})")
        return 1
    print("OK: all stable error keys are non-empty and translated in all locales")
    return 0


if __name__ == "__main__":
    sys.exit(main())
