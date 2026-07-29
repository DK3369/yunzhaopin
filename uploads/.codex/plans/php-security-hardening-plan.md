# PHP Security Hardening Plan

## Scope

This plan covers the first security-hardening batch for:

- SQL injection prevention in the Locoy news ingestion endpoint.
- Security attributes for authentication cookies issued by `cookie_model`.

It intentionally does not change password storage, upload policy, legacy
integration protocols, or unrelated request handling.

## SQL Safety

`api/locoy/model/news.class.php` must not concatenate request values into SQL
fragments. Reads and writes must use the framework's array-based
`select_once()` and `insert_into()` methods. Those methods validate schema
fields, apply the existing input filter, and escape values through the active
database driver.

Numeric fields received by the endpoint should be normalized to integers before
storage. Text fields should retain the endpoint's existing content-processing
behavior and be passed as array values so escaping occurs at the database
boundary.

The existing Locoy enable switch and key check remain required. They are an
authorization layer, not a substitute for safe SQL construction.

## Authentication Cookie Policy

Authentication cookies (`uid`, `shell`, `usertype`, `userdid`, and `amtype`)
must be issued with:

- `HttpOnly` to prevent JavaScript access.
- `SameSite=Lax` to reduce cross-site request abuse while preserving normal
  top-level navigation.
- `Secure` whenever the current request is HTTPS.
- The existing path, domain, and expiry behavior.

Non-authentication cookies keep their current defaults because some are read by
front-end scripts.

For PHP 7.3 and newer, use the native cookie options array. For older supported
PHP versions, use the compatible `SameSite` path attribute form while still
passing the native `Secure` and `HttpOnly` flags.

Cookie deletion must use the same security attributes and scope as creation.

## Validation

After implementation:

1. Run PHP syntax checks on the changed production files.
2. Run `php tools/php_lint_gate.php`.
3. Confirm no direct `$_POST` concatenation remains in the Locoy news database
   operations.
4. Review the diff for unrelated changes.

