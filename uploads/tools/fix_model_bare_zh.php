<?php
/**
 * Wrap bare Chinese in app/model, member, app/include PHP (msg/error/user-facing strings).
 * Extends fix_wap_bare_zh patterns; uses single-quoted regex for correct backreferences.
 */
define('ROOT', dirname(__DIR__) . '/');

require_once __DIR__ . '/fix_bare_zh_core.php';

$skipFiles = array(
    'app/include/wap.enum.php',
    'api/wxapp/wap.enum.php',
);

$dirs = array('app/model', 'member', 'app/include', 'app/controller', 'admin/model');

$changed = fixBareZhDirs($dirs, $skipFiles);
echo "Done. $changed files updated.\n";
