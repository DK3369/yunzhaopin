# PHP 8.5 Upgrade Plan

## Target

Upgrade the production runtime from PHP 7.4.33 to PHP 8.5.7 while keeping PHP 7.4 installed as a rollback runtime.

## Repository Changes

- PHP 7 and newer must load the mysqli-backed database driver.
- Legacy payment and UC entrypoints must not route PHP 8.x to PHP 5 mysql_* code paths.
- Removed PHP APIs such as each() and get_magic_quotes_gpc() must be replaced or guarded before the runtime switch.
- WAP authentication entrypoints must remain directly testable after the switch; desktop/local access to WAP login, register, and password recovery should not be forced through the PC jump helper.

## Server Changes

- Install PHP 8.5.7 side by side under the existing /www/server/php layout.
- Match required PHP 7.4 extensions where available: mysqli, pdo_mysql, curl, gd, mbstring, intl, zip, bcmath, soap, sockets, openssl, sodium, xml, dom, SimpleXML, xmlreader, and xmlwriter.
- Keep PHP 7.4 and the existing /tmp/php-cgi-74.sock Nginx include as rollback.
- Switch the site vhost to the PHP 8.5 FPM socket only after code checks pass.

## Validation

- Run PHP syntax checks for changed files after each compatibility batch.
- Run tools/php_lint_gate.php before switching web traffic.
- After PHP 8.5 is installed, repeat syntax checks using the PHP 8.5 binary and smoke test the site homepage, admin login, collection API, payment entrypoints, and UC entrypoints.
