<?php
/**
 * Fix yun_auto_t('module_NNNNN') -> yun_at('module_NNNNN') when arg is a lang key.
 */
define('ROOT', dirname(__DIR__) . '/');

$dirs = array('app', 'admin', 'member', 'api/wxapp', 'wap');
$changed = 0;
$fixed = 0;

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
        if (preg_match('/vendor|PHPExcel|install|tools\//i', $rel)) {
            continue;
        }
        $c = file_get_contents($f->getPathname());
        $orig = $c;
        $c = preg_replace_callback(
            "/yun_auto_t\s*\(\s*['\"]([a-z][a-z0-9_]*)_(\d{5})['\"]\s*\)/",
            function ($m) use (&$fixed) {
                $fixed++;
                return "yun_at('" . $m[1] . '_' . $m[2] . "')";
            },
            $c
        );
        if ($c !== $orig) {
            file_put_contents($f->getPathname(), $c);
            echo "FIXED: $rel\n";
            $changed++;
        }
    }
}
echo "Files: $changed, replacements: $fixed\n";
