<?php
/**
 * Convert invalid Smarty {yun:}t tags in admin .vue files to lc() bindings.
 *
 * Usage:
 *   php tools/fix_vue_yun_to_lc.php [--dry-run] --dir=app/template/admin/tool/
 *   php tools/fix_vue_yun_to_lc.php [--dry-run] --file=app/template/admin/tool/weixin/component/usertag.vue
 */
define('ROOT', dirname(__DIR__) . '/');

$dryRun = in_array('--dry-run', $argv ?? array(), true);
$singleFile = '';
$singleDir = '';
foreach ($argv ?? array() as $arg) {
    if (preg_match('/^--file=(.+)$/', $arg, $m)) {
        $singleFile = $m[1];
    }
    if (preg_match('/^--dir=(.+)$/', $arg, $m)) {
        $singleDir = rtrim($m[1], '/') . '/';
    }
}

if ($singleFile === '' && $singleDir === '') {
    fwrite(STDERR, "ERROR: --file= or --dir= is required.\n");
    exit(1);
}

define('YUN_TAG', '/\{yun:\}t\s+key=(["\'])([^"\']+)\1\s*\{\/yun\}/u');

function convertContent($content)
{
    $bindAttrs = 'label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text|inactive-text|active-text|header|description|content|confirm-button-text|cancel-button-text|submit-text';

    // attr="{yun:}t key='K'{/yun}" -> :attr="lc('K')"
    $content = preg_replace_callback(
        '/(?<![:\w-])(' . $bindAttrs . ')\s*=\s*["\']\{yun:\}t\s+key=(["\'])([^"\']+)\2\s*\{\/yun\}["\']/u',
        function ($m) {
            return ':' . $m[1] . '="lc(\'' . $m[3] . '\')"';
        },
        $content
    );

    // Quoted yun tags in JS: "{yun:}t key='K'{/yun}" -> lc('K')
    $content = preg_replace_callback(
        '/(["\'])\{yun:\}t\s+key=(["\'])([^"\']+)\2\s*\{\/yun\}\1/u',
        function ($m) {
            return "lc('" . $m[3] . "')";
        },
        $content
    );

    // Text nodes / mixed content: {yun:}t key='K'{/yun} -> {{ lc('K') }}
    $content = preg_replace_callback(
        YUN_TAG,
        function ($m) {
            return '{{ lc(\'' . $m[2] . '\') }}';
        },
        $content
    );

    return $content;
}

function processFile($path, $dryRun)
{
    $fullPath = (strpos($path, ROOT) === 0) ? $path : ROOT . ltrim($path, '/');
    if (!is_file($fullPath)) {
        fwrite(STDERR, "SKIP missing: $path\n");
        return 0;
    }
    $original = file_get_contents($fullPath);
    $converted = convertContent($original);
    if ($converted === $original) {
        return 0;
    }
    $before = preg_match_all(YUN_TAG, $original);
    $after = preg_match_all(YUN_TAG, $converted);
    if (!$dryRun) {
        file_put_contents($fullPath, $converted);
    }
    $rel = str_replace(ROOT, '', $fullPath);
    echo ($dryRun ? '[dry-run] ' : '') . "$rel: fixed " . ($before - $after) . " tags\n";
    return $before - $after;
}

$total = 0;
$files = 0;

if ($singleFile !== '') {
    $n = processFile($singleFile, $dryRun);
    if ($n > 0) {
        $files++;
        $total += $n;
    }
} else {
    $dir = ROOT . ltrim($singleDir, '/');
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($dir));
    foreach ($it as $f) {
        if (!$f->isFile() || strtolower($f->getExtension()) !== 'vue') {
            continue;
        }
        $n = processFile($f->getPathname(), $dryRun);
        if ($n > 0) {
            $files++;
            $total += $n;
        }
    }
}

echo "\nDone: $files files, $total tags converted" . ($dryRun ? ' (dry-run)' : '') . "\n";
