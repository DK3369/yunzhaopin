<?php
/**
 * Convert autoText('中文') / yun_auto_t('中文') to at('key') / yun_at('key').
 * Usage: php tools/convert_autotext_to_keys.php [--dry-run]
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);

$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$valueToKeys = array();
foreach ($zh as $key => $value) {
    if (!is_string($value) || $value === '') {
        continue;
    }
    if (!isset($valueToKeys[$value])) {
        $valueToKeys[$value] = array();
    }
    $valueToKeys[$value][] = $key;
}

function pathPrefixHints($rel)
{
    if (strpos($rel, 'api/wxapp/') === 0) {
        return array('wap_', 'wap_com_', 'wap_user_', 'common_');
    }
    if (strpos($rel, 'admin/') === 0) {
        return array('admin_', 'admin_sys_', 'admin_tool_', 'admin_model_', 'admin_com_', 'admin_user_', 'admin_news_', 'admin_ops_', 'common_');
    }
    if (strpos($rel, 'app/model/') === 0) {
        return array('model_', 'common_');
    }
    if (strpos($rel, 'member/com/') === 0) {
        return array('member_com_', 'common_');
    }
    if (strpos($rel, 'member/user/') === 0) {
        return array('member_user_', 'common_');
    }
    if (strpos($rel, 'member/') === 0) {
        return array('member_com_', 'member_user_', 'common_');
    }
    if (strpos($rel, 'app/controller/wap/') === 0) {
        return array('wap_', 'common_');
    }
    if (strpos($rel, 'app/controller/') === 0) {
        return array('controller_', 'common_', 'wap_');
    }
    return array('common_', 'model_', 'wap_', 'admin_');
}

function pickKey($text, $keys, $rel)
{
    if (count($keys) === 1) {
        return $keys[0];
    }
    foreach (pathPrefixHints($rel) as $prefix) {
        foreach ($keys as $key) {
            if (strpos($key, $prefix) === 0) {
                return $key;
            }
        }
    }
    return $keys[0];
}

function lookupKey($text, $rel, $valueToKeys)
{
    if (isset($valueToKeys[$text])) {
        return pickKey($text, $valueToKeys[$text], $rel);
    }
    $normalized = preg_replace('/\s+/u', ' ', trim(html_entity_decode($text, ENT_QUOTES, 'UTF-8')));
    if ($normalized !== $text && isset($valueToKeys[$normalized])) {
        return pickKey($normalized, $valueToKeys[$normalized], $rel);
    }
    return null;
}

function convertStringArg($text, $rel, $valueToKeys, &$converted, &$missing)
{
    if (preg_match('/^[a-z]+_\d+$/', $text)) {
        return $text;
    }
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $text)) {
        return null;
    }
    $key = lookupKey($text, $rel, $valueToKeys);
    if ($key === null) {
        $missing[$text][] = $rel;
        return null;
    }
    $converted++;
    return $key;
}

$dirs = array('app', 'admin', 'member', 'api/wxapp');
$converted = 0;
$missing = array();
$filesChanged = 0;

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
        if (!preg_match('/autoText\s*\(|yun_auto_t\s*\(/', $c)) {
            continue;
        }
        $orig = $c;

        $c = preg_replace_callback(
            '/(\$this->)autoText\s*\(\s*([\'"])(.*?)(?<!\\\\)\2(\s*\))/s',
            function ($m) use ($rel, $valueToKeys, &$converted, &$missing) {
                $text = stripcslashes($m[3]);
                $key = convertStringArg($text, $rel, $valueToKeys, $converted, $missing);
                if ($key === null) {
                    return $m[0];
                }
                return '$this->at(\'' . $key . '\')';
            },
            $c
        );

        $c = preg_replace_callback(
            '/\byun_auto_t\s*\(\s*([\'"])(.*?)(?<!\\\\)\1(\s*\))/s',
            function ($m) use ($rel, $valueToKeys, &$converted, &$missing) {
                $text = stripcslashes($m[2]);
                $key = convertStringArg($text, $rel, $valueToKeys, $converted, $missing);
                if ($key === null) {
                    return $m[0];
                }
                return "yun_at('$key')";
            },
            $c
        );

        if ($c !== $orig) {
            $filesChanged++;
            if (!$dryRun) {
                file_put_contents($f->getPathname(), $c);
            }
            echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        }
    }
}

echo "\nConverted: $converted\n";
echo "Files: $filesChanged\n";
echo "Missing: " . count($missing) . "\n";
if (!empty($missing)) {
    $i = 0;
    foreach ($missing as $text => $files) {
        if ($i++ >= 40) {
            echo "...+" . (count($missing) - 40) . " more\n";
            break;
        }
        echo "  [$text] " . implode(', ', array_unique($files)) . "\n";
    }
    file_put_contents(ROOT . 'tools/autotext_missing_keys.json', json_encode($missing, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
}
