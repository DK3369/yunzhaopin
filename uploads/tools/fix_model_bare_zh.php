<?php
/**
 * Wrap bare Chinese in business PHP (msg/error/user-facing strings).
 * Full-site batch disabled — pass --dir= explicitly.
 *
 * Usage: php tools/fix_model_bare_zh.php --dir=app/model
 */
define('ROOT', dirname(__DIR__) . '/');

require_once __DIR__ . '/fix_bare_zh_core.php';

$dirArg = '';
foreach ($argv ?? array() as $arg) {
    if (preg_match('/^--dir=(.+)$/', $arg, $m)) {
        $dirArg = trim($m[1], '/');
    }
}

if ($dirArg === '') {
    fwrite(STDERR, "ERROR: --dir= is required (full-site batch disabled).\n");
    fwrite(STDERR, "Example: php tools/fix_model_bare_zh.php --dir=app/model\n");
    exit(1);
}

$skipFiles = array(
    'app/include/wap.enum.php',
    'api/wxapp/wap.enum.php',
);

$changed = fixBareZhDirs(array($dirArg), $skipFiles);
echo "Done. $changed files updated.\n";
