# PHP 8.5 Upgrade Plan

## Target

Run the application exclusively on PHP 8.5.7. PHP 7.4 may remain installed as an offline reference, but its FPM service, socket, Nginx includes, CLI paths, and scheduled-task paths must not be active.

## Repository Changes

- PHP 7 and newer must load the mysqli-backed database driver.
- Legacy payment and UC entrypoints must not route PHP 8.x to PHP 5 mysql_* code paths.
- Removed PHP APIs such as each() and get_magic_quotes_gpc() must be replaced or guarded before the runtime switch.
- WAP authentication entrypoints must remain directly testable after the switch; desktop/local access to WAP login, register, and password recovery should not be forced through either WAP jump helper.
- WAP templates should load shared API/JS i18n snippets from the common header only; page templates must not duplicate the same shared include before the header because legacy Smarty can truncate nested duplicate subtemplate output under PHP 8.5.
- Optional WAP DIY template cache files must be guarded when absent so missing runtime cache does not emit noisy PHP 8.5 warnings.
- Local development hosts such as dev.test, localhost, and 127.0.0.1 should be allowed to open explicit /wap/ URLs without desktop-to-PC redirects, while production hosts keep the existing redirect behavior.
- Legacy PHP4-style parent constructor calls and count() guards must be modernized where they cause PHP 8.5 fatal errors.
- PHP 8.5 strict runtime calls must receive valid types; shared entrypoints and API actions initialize arrays and protect nullable values before strict built-ins.
- The bundled Smarty runtime declares properties it creates and uses PHP 8-compatible exception properties; generated templates are never patched directly.
- API route names are identifier-only and invalid routes return structured errors.
- Every externally reachable action is classified and receives at least one safe regression case.

## Security Changes

- Authentication cookies are HttpOnly and SameSite=Lax; Secure is conditional on HTTPS, and PHP sessions use strict mode.
- Credentialed wxapp CORS uses explicit configured origins and never combines credentials with a wildcard.
- Locoy, payment, UC, and PW URLs and payloads stay compatible while types, lengths, authentication, signatures, request sizes, and deserialization are validated before side effects.
- External request values are not concatenated into SQL; collection and payment code uses typed values and the array-based model interface.
- Uploads use server-side MIME checks, safe extensions, random names, and non-executable directories.
- Password verification is centralized; legacy salted MD5 remains accepted during transition and upgrades to password_hash() after successful login once password columns are widened to varchar(255).
- Reviewed PHP/Nginx security templates are tracked before server application.

## Server Changes

- Install PHP 8.5.7 side by side under the existing /www/server/php layout.
- Match required PHP 7.4 extensions where available: mysqli, pdo_mysql, curl, gd, mbstring, intl, zip, bcmath, soap, sockets, openssl, sodium, xml, dom, SimpleXML, xmlreader, and xmlwriter.
- Switch the site vhost to the PHP 8.5 FPM socket only after code checks pass.
- Server rollback restores the previous PHP 8.5 configuration and never automatically reactivates PHP 7.4.

## Validation

- Run PHP syntax checks for changed files after each compatibility batch.
- Run tools/php_lint_gate.php before switching web traffic.
- After PHP 8.5 is installed, repeat syntax checks using the PHP 8.5 binary and smoke test the site homepage, admin login, collection API, payment entrypoints, and UC entrypoints.
- WAP list pages must avoid PHP 8.5 fatal patterns seen during local smoke tests: initialize Smarty pagination variables before assignment, compare resume sex CSS classes by numeric `sex_id` instead of nested translated labels, and count filter arrays before `implode()`.
- Member and API base controllers must call `parent::__construct(...)` instead of legacy PHP4-style `$this->common(...)`, including WAP member, PC member, wxapp, and version API entrypoints.
- API front controllers should guard missing route parameters and missing model files, returning structured JSON errors instead of letting PHP 8.5 `require` failures become HTTP 500 responses.
- PHP 7.4 is fully disabled operationally after the PHP 8.5.7 cutover: keep rollback config files on disk, but stop and disable the PHP-FPM 7.4 service and point the default `enable-php.conf` include at `/tmp/php-cgi-85.sock`.
- The route audit must classify every discovered action; public cases execute normally and authenticated/write cases receive unauthorized or dedicated-fixture tests.
- Security tests cover invalid collection/payment/UC/PW signatures, SQL injection, traversal, CORS, cookies, uploads, and sensitive-file access.
- Each batch captures a log offset and fails on new Fatal, Uncaught, TypeError, ValueError, mysqli_sql_exception, or Deprecated messages.
- After server changes, validate FPM and Nginx, reload without rebooting, rerun HTTP checks, and prove only PHP 8.5.7 is active.

## Delivery

- Work on dev and commit and push each validated batch to origin/dev.
- Never commit secrets, uploads, compiled templates, or backups; state-changing tests clean up only their dedicated fixture IDs.
