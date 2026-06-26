<?php
/**
 * Syntax gate for business PHP directories — must pass before next i18n batch.
 * Usage: php tools/php_lint_gate.php
 */
define('ROOT', dirname(__DIR__) . '/');

$dirs = array(
    'app/model',
    'member',
    'admin/model',
    'api/wxapp',
    'app/controller',
    'wap/member',
);

$errors = array();
$checked = 0;

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) {
        continue;
    }
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match('/vendor|PHPExcel|aliyun|install/i', $rel)) {
            continue;
        }
        $checked++;
        $out = array();
        $code = 0;
        exec('php -l ' . escapeshellarg($f->getPathname()) . ' 2>&1', $out, $code);
        if ($code !== 0) {
            $errors[] = $rel . ': ' . implode(' ', $out);
        }
    }
}

echo "Checked: $checked PHP files\n";
if (!empty($errors)) {
    echo "Parse errors: " . count($errors) . "\n";
    foreach ($errors as $e) {
        echo "  $e\n";
    }
    exit(1);
}

echo "OK — 0 Parse error\n";
exit(0);
