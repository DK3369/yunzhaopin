<?php
/**
 * Replace Chinese literals inside Smarty compile plugin $OutputStr with yun_at('key').
 * Usage: php tools/replace_smarty_compile_i18n.php [--dry-run]
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);

$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$valueToKeys = array();
foreach ($zh as $key => $value) {
    if (!is_string($value) || $value === '') continue;
    if (!isset($valueToKeys[$value])) $valueToKeys[$value] = array();
    $valueToKeys[$value][] = $key;
}

function pickKey($text, $keys)
{
    $prefs = array('common_', 'wap_com_', 'wap_', 'model_', 'tpl_');
    foreach ($prefs as $prefix) {
        foreach ($keys as $key) {
            if (strpos($key, $prefix) === 0) return $key;
        }
    }
    return $keys[0];
}

$manual = array(
    '学历 · ' => 'common_09182',
    ' · 毕业于' => 'model_00227',
    '年' => 'common_09174',
    '月' => 'common_02443',
    ' 人' => 'common_01090',
);

function resolveKey($text, $valueToKeys, $manual)
{
    if (preg_match('/^[a-z]+_\d+$/', $text)) {
        return null;
    }
    if (isset($manual[$text])) {
        return $manual[$text];
    }
    if (isset($valueToKeys[$text])) {
        return pickKey($text, $valueToKeys[$text]);
    }
    return null;
}

$dir = ROOT . 'app/include/libs/sysplugins/';
$files = glob($dir . 'smarty_internal_compile_*.php');
$replaced = 0;
$missing = array();

foreach ($files as $file) {
    $content = file_get_contents($file);
    $orig = $content;

    $content = preg_replace_callback(
        '/(=\s*|\.)([\'"])(?:(?!\2).)*[\x{4e00}-\x{9fff}](?:(?!\2).)*\2/u',
        function ($m) use ($valueToKeys, $manual, &$replaced, &$missing, $file) {
            $prefix = $m[1];
            $quote = $m[2];
            $full = $m[0];
            $text = substr($full, strlen($prefix) + 1, -1);
            $key = resolveKey($text, $valueToKeys, $manual);
            if ($key === null) {
                $missing[$text][] = basename($file);
                return $full;
            }
            $replaced++;
            if ($prefix === '.') {
                return '.yun_at(\\\'' . $key . '\\\')';
            }
            return '= yun_at(\\\'' . $key . '\\\')';
        },
        $content
    );

    $content = preg_replace_callback(
        '/stripos\(([^,]+),\s*([\'"])(?:(?!\2).)*[\x{4e00}-\x{9fff}](?:(?!\2).)*\2/u',
        function ($m) use ($valueToKeys, $manual, &$replaced, &$missing, $file) {
            $quote = $m[2];
            $text = substr($m[0], strpos($m[0], $quote) + 1);
            $text = substr($text, 0, -1);
            $key = resolveKey($text, $valueToKeys, $manual);
            if ($key === null) {
                $missing[$text][] = basename($file) . ' (stripos)';
                return $m[0];
            }
            $replaced++;
            return 'stripos(' . $m[1] . ', yun_at(\\\'' . $key . '\\\')';
        },
        $content
    );

    $content = preg_replace_callback(
        '/!=\s*([\'"])(?:(?!\1).)*[\x{4e00}-\x{9fff}](?:(?!\1).)*\1/u',
        function ($m) use ($valueToKeys, $manual, &$replaced, &$missing, $file) {
            $quote = $m[1];
            $text = substr($m[0], 3, -1);
            $key = resolveKey($text, $valueToKeys, $manual);
            if ($key === null) {
                $missing[$text][] = basename($file) . ' (!=)';
                return $m[0];
            }
            $replaced++;
            return '!= yun_at(\\\'' . $key . '\\\')';
        },
        $content
    );

    if (strpos($content, 'weekarray=array("日"') !== false) {
        $weekKeys = array('common_09175', 'common_09176', 'common_09177', 'common_09178', 'common_09179', 'common_09180', 'common_09181');
        $parts = array();
        foreach ($weekKeys as $wk) {
            $parts[] = "yun_at(\\'" . $wk . "\\')";
        }
        $content = preg_replace(
            '/\$weekarray=array\("日","一","二","三","四","五","六"\);/',
            '$weekarray=array(' . implode(',', $parts) . ');',
            $content,
            -1,
            $cnt
        );
        if ($cnt) $replaced += $cnt;
    }

    if ($content !== $orig) {
        echo ($dryRun ? '[dry] ' : '') . basename($file) . "\n";
        if (!$dryRun) {
            file_put_contents($file, $content);
        }
    }
}

echo "\nReplaced: $replaced\nMissing: " . count($missing) . "\n";
foreach ($missing as $text => $where) {
    echo "  [$text] " . implode(', ', array_unique($where)) . "\n";
}
