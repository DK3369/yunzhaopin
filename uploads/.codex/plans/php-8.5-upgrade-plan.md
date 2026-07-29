# PHP 8.5 Upgrade Plan

## Target

Upgrade the production runtime from PHP 7.4.33 to PHP 8.5.7 while keeping PHP 7.4 installed as a rollback runtime.

## Repository Changes

- PHP 7 and newer must load the mysqli-backed database driver.
- Legacy payment and UC entrypoints must not route PHP 8.x to PHP 5 mysql_* code paths.
- Removed PHP APIs such as each() and get_magic_quotes_gpc() must be replaced or guarded before the runtime switch.
- WAP authentication entrypoints must remain directly testable after the switch; desktop/local access to WAP login, register, and password recovery should not be forced through either WAP jump helper.
- WAP templates should load shared API/JS i18n snippets from the common header only; page templates must not duplicate the same shared include before the header because legacy Smarty can truncate nested duplicate subtemplate output under PHP 8.5.
- Optional WAP DIY template cache files must be guarded when absent so missing runtime cache does not emit noisy PHP 8.5 warnings.
- Local development hosts such as dev.test, localhost, and 127.0.0.1 should be allowed to open explicit /wap/ URLs without desktop-to-PC redirects, while production hosts keep the existing redirect behavior.
- Legacy PHP4-style parent constructor calls and count() guards must be modernized where they cause PHP 8.5 fatal errors.

## Server Changes

- Install PHP 8.5.7 side by side under the existing /www/server/php layout.
- Match required PHP 7.4 extensions where available: mysqli, pdo_mysql, curl, gd, mbstring, intl, zip, bcmath, soap, sockets, openssl, sodium, xml, dom, SimpleXML, xmlreader, and xmlwriter.
- Keep PHP 7.4 and the existing /tmp/php-cgi-74.sock Nginx include as rollback.
- Switch the site vhost to the PHP 8.5 FPM socket only after code checks pass.

## Validation

- Run PHP syntax checks for changed files after each compatibility batch.
- Run tools/php_lint_gate.php before switching web traffic.
- After PHP 8.5 is installed, repeat syntax checks using the PHP 8.5 binary and smoke test the site homepage, admin login, collection API, payment entrypoints, and UC entrypoints.
- WAP list pages must avoid PHP 8.5 fatal patterns seen during local smoke tests: initialize Smarty pagination variables before assignment, compare resume sex CSS classes by numeric `sex_id` instead of nested translated labels, and count filter arrays before `implode()`.
- Member and API base controllers must call `parent::__construct(...)` instead of legacy PHP4-style `$this->common(...)`, including WAP member, PC member, wxapp, and version API entrypoints.
- API front controllers should guard missing route parameters and missing model files, returning structured JSON errors instead of letting PHP 8.5 `require` failures become HTTP 500 responses.
- PHP 7.4 is fully disabled operationally after the PHP 8.5.7 cutover: keep rollback config files on disk, but stop and disable the PHP-FPM 7.4 service and point the default `enable-php.conf` include at `/tmp/php-cgi-85.sock`.
