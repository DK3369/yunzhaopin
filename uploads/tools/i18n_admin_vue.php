<?php
/**
 * i18n admin .vue files: scan or apply migrations.
 * Usage:
 *   php tools/i18n_admin_vue.php scan app/template/admin/user/users/component/
 *   php tools/i18n_admin_vue.php apply app/template/admin/user/users/component/ --key-start=1 --max-keys=20
 */
define('ROOT', dirname(__DIR__) . '/');

function loadLang()
{
    static $zh = null;
    if ($zh === null) {
        $zh = include ROOT . 'data/lang/auto/zh_cn.php';
    }
    return $zh;
}

function valueToKeyMap()
{
    static $map = null;
    if ($map === null) {
        $map = [];
        foreach (loadLang() as $k => $v) {
            if (!is_string($v) || $v === '') continue;
            if (!isset($map[$v])) $map[$v] = [];
            $map[$v][] = $k;
        }
    }
    return $map;
}

function pickKey($text, $keys, $rel)
{
    $prefixes = ['admin_user_', 'admin_company_', 'admin_', 'common_', 'wap_', 'member_'];
    if (strpos($rel, 'admin/user/users') !== false) $prefixes = array_merge(['admin_user_'], $prefixes);
    if (strpos($rel, 'admin/user/member') !== false) $prefixes = array_merge(['admin_user_'], $prefixes);
    if (strpos($rel, 'admin/user/weipin') !== false) $prefixes = array_merge(['admin_user_weipin_'], $prefixes);
    if (strpos($rel, 'admin/component') !== false) $prefixes = array_merge(['admin_'], $prefixes);
    if (strpos($rel, 'admin/tool') !== false) $prefixes = array_merge(['admin_tool_'], $prefixes);
    if (strpos($rel, 'admin/system') !== false) $prefixes = array_merge(['admin_system_'], $prefixes);
    if (strpos($rel, 'admin/neirong') !== false) $prefixes = array_merge(['admin_'], $prefixes);
    if (strpos($rel, 'admin/yunying') !== false) $prefixes = array_merge(['admin_yunying_'], $prefixes);
    foreach ($prefixes as $p) {
        foreach ($keys as $k) {
            if (strpos($k, $p) === 0) return $k;
        }
    }
    return $keys[0];
}

function stripVueComments($code)
{
    // HTML comments
    $code = preg_replace('/<!--[\s\S]*?-->/u', '', $code);
    // JS block comments
    $code = preg_replace('#/\*[\s\S]*?\*/#u', '', $code);
    // JS line comments (but not URLs)
    $code = preg_replace('#(?<!:)//[^\n]*#u', '', $code);
    // CSS comments in style
    return $code;
}

function hasChinese($s)
{
    return is_string($s) && preg_match('/[\x{4e00}-\x{9fff}]/u', $s);
}

function findChineseStrings($content, $rel)
{
    $stripped = stripVueComments($content);
    $found = [];
    // Match quoted strings with Chinese
    preg_match_all('/(["\'])((?:\\\\.|(?!\1)[^\\\\])*[\x{4e00}-\x{9fff}](?:\\\\.|(?!\1)[^\\\\])*)\1/u', $stripped, $m, PREG_OFFSET_CAPTURE);
    foreach ($m[2] as $item) {
        $text = stripcslashes($item[0]);
        $offset = $item[1];
        if (mb_strlen($text) < 1 || mb_strlen($text) > 300) continue;
        // Skip if already i18n
        $ctx = substr($content, max(0, $offset - 80), 160);
        if (preg_match('/\{yun:\}t\s+key=/u', $ctx)) continue;
        if (preg_match('/lc\s*\(/u', $ctx)) continue;
        // Skip backend comparison patterns in == or ===
        $before = substr($stripped, max(0, $offset - 30), 30);
        if (preg_match('/[=!<>]+\s*$/u', $before)) continue;
        $found[$text] = true;
    }
    // Template attributes without quotes issues - label="中文"
    preg_match_all('/(?:label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text)\s*=\s*"([^"]*[\x{4e00}-\x{9fff}][^"]*)"/u', $stripped, $m2);
    foreach ($m2[1] as $text) {
        if (!preg_match('/\{yun:\}t/u', $text)) $found[$text] = true;
    }
    preg_match_all("/(?:label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text)\s*=\s*'([^']*[\x{4e00}-\x{9fff}][^']*)'/u", $stripped, $m3);
    foreach ($m3[1] as $text) {
        if (!preg_match('/\{yun:\}t/u', $text)) $found[$text] = true;
    }
    return array_keys($found);
}

function isInScript($content, $offset)
{
    $before = substr($content, 0, $offset);
    $scriptStart = strrpos($before, '<script');
    $scriptEnd = strrpos($before, '</script>');
    return $scriptStart !== false && ($scriptEnd === false || $scriptEnd < $scriptStart);
}

function isInTemplate($content, $offset)
{
    $before = substr($content, 0, $offset);
    $tmplStart = strrpos($before, '<template');
    $tmplEnd = strrpos($before, '</template>');
    return $tmplStart !== false && ($tmplEnd === false || $tmplEnd < $tmplStart);
}

function escapeRegex($s)
{
    return preg_quote($s, '/');
}

function applyReplacements($content, $mapping, $rel)
{
    $v2k = valueToKeyMap();
    foreach ($mapping as $text => $key) {
        $escaped = escapeRegex($text);
        // Template double-quoted attributes
        $content = preg_replace(
            '/(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text)\s*=\s*"' . $escaped . '"/u',
            '$1="{yun:}t key=\'' . $key . '\'{/yun}"',
            $content
        );
        $content = preg_replace(
            "/(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text)\s*=\s*'" . $escaped . "'/u",
            '$1="{yun:}t key=\'' . $key . '\'{/yun}"',
            $content
        );
        // Script/template string literals - context aware
        $pattern = '/(["\'])' . $escaped . '\1/u';
        $content = preg_replace_callback($pattern, function ($m) use ($key, $content, $rel) {
            $full = $m[0];
            $pos = strpos($content, $full);
            if ($pos === false) return $full;
            if (isInScript($content, $pos)) {
                // data() initial values use yun template syntax in quotes
                $lineStart = strrpos(substr($content, 0, $pos), "\n");
                $line = substr($content, $lineStart !== false ? $lineStart : 0, 200);
                if (preg_match('/:\s*["\']|=\s*["\']|msg\s*=|emptytext|dataText|placeholder/u', $line)) {
                    return '"{yun:}t key=\'' . $key . '\'{/yun}"';
                }
                return "lc('" . $key . "')";
            }
            return '"{yun:}t key=\'' . $key . '\'{/yun}"';
        }, $content, -1, $count);
    }
    return $content;
}

function addLangKeys($newKeys, $keyStart)
{
    $zhPath = ROOT . 'data/lang/auto/zh_cn.php';
    $enPath = ROOT . 'data/lang/auto/en_us.php';
    $zh = include $zhPath;
    $en = include $enPath;
    $num = $keyStart;
    $added = [];
    foreach ($newKeys as $text => $enText) {
        while (isset($zh['admin_vue_' . str_pad($num, 5, '0', STR_PAD_LEFT)])) $num++;
        $k = 'admin_vue_' . str_pad($num, 5, '0', STR_PAD_LEFT);
        $zh[$k] = $text;
        $en[$k] = $enText !== null ? $enText : $text; // placeholder, fix manually
        $added[$text] = $k;
        $num++;
    }
    writeLangFile($zhPath, $zh);
    writeLangFile($enPath, $en);
    return $added;
}

function writeLangFile($path, $data)
{
    $out = "<?php\nreturn array(\n";
    foreach ($data as $k => $v) {
        $out .= "  '" . addslashes($k) . "' => '" . addslashes($v) . "',\n";
    }
    $out .= ");\n";
    file_put_contents($path, $out);
}

function loadEnLang()
{
    return include ROOT . 'data/lang/auto/en_us.php';
}

function getEnTranslation($key)
{
    $en = loadEnLang();
    return isset($en[$key]) ? $en[$key] : null;
}

function processDir($dir, $keyStart, $maxKeys, $dryRun = false)
{
    $dir = rtrim($dir, '/') . '/';
    $fullDir = ROOT . $dir;
    if (!is_dir($fullDir)) {
        fwrite(STDERR, "Dir not found: $dir\n");
        exit(1);
    }
    $v2k = valueToKeyMap();
    $allNew = [];
    $mapping = [];
    $files = glob($fullDir . '*.vue');
    foreach ($files as $f) {
        $rel = str_replace(ROOT, '', $f);
        $content = file_get_contents($f);
        $strings = findChineseStrings($content, $rel);
        foreach ($strings as $text) {
            if (isset($mapping[$text])) continue;
            if (isset($v2k[$text])) {
                $mapping[$text] = pickKey($text, $v2k[$text], $rel);
            } else {
                $allNew[$text] = true;
            }
        }
    }
    $newTexts = array_keys($allNew);
    $newCount = count($newTexts);
    if ($newCount > $maxKeys) {
        $newTexts = array_slice($newTexts, 0, $maxKeys);
    }
    $en = loadEnLang();
    $newKeyMap = [];
    $num = $keyStart;
    foreach ($newTexts as $text) {
        while (isset($en['admin_vue_' . str_pad($num, 5, '0', STR_PAD_LEFT)])) $num++;
        $k = 'admin_vue_' . str_pad($num, 5, '0', STR_PAD_LEFT);
        $newKeyMap[$text] = $k;
        $mapping[$text] = $k;
        $num++;
    }
    echo "Files: " . count($files) . "\n";
    echo "Existing keys reused: " . (count($mapping) - count($newKeyMap)) . "\n";
    echo "New keys: " . count($newKeyMap) . "\n";
    foreach ($newKeyMap as $t => $k) echo "  $k => $t\n";
    if ($dryRun) return ['mapping' => $mapping, 'new' => $newKeyMap];
    // Apply to files
    foreach ($files as $f) {
        $rel = str_replace(ROOT, '', $f);
        $content = file_get_contents($f);
        $newContent = applyReplacements($content, $mapping, $rel);
        if ($newContent !== $content) {
            file_put_contents($f, $newContent);
            echo "Updated: $rel\n";
        }
    }
    if ($newKeyMap) {
        $zhPath = ROOT . 'data/lang/auto/zh_cn.php';
        $enPath = ROOT . 'data/lang/auto/en_us.php';
        $zh = include $zhPath;
        $en = include $enPath;
        foreach ($newKeyMap as $text => $k) {
            $zh[$k] = $text;
            $en[$k] = $text; // will translate below
        }
        // Don't rewrite entire lang file - append keys only
        appendLangKeys($zhPath, $enPath, $newKeyMap);
    }
    return ['mapping' => $mapping, 'new' => $newKeyMap, 'next' => $num];
}

function appendLangKeys($zhPath, $enPath, $newKeyMap)
{
    foreach ([$zhPath, $enPath] as $path) {
        $content = file_get_contents($path);
        $content = rtrim($content);
        $content = preg_replace('/\);\s*$/', '', $content);
        $isEn = strpos($path, 'en_us') !== false;
        foreach ($newKeyMap as $text => $k) {
            $val = $isEn ? translateEn($text, $k) : $text;
            $content .= "\n  '" . addslashes($k) . "' => '" . addslashes($val) . "',";
        }
        $content .= "\n);\n";
        file_put_contents($path, $content);
    }
}

function translateEn($zh, $key)
{
    // Simple common translations - for production use existing en if key exists elsewhere
    static $dict = [
        '用户名' => 'Username', '用户ID' => 'User ID', '姓名' => 'Name', '内容' => 'Content',
        '编号' => 'No.', '至' => 'to', '时间' => 'Time', '操作' => 'Actions', 'IP' => 'IP',
        '你确定要删除选中项吗？' => 'Are you sure you want to delete the selected items?',
        '确定要清空用户解绑日志？' => 'Are you sure you want to clear user unbind logs?',
        '搜索筛选项' => 'Search filters', '多选值存储' => 'Multi-select storage',
        '批量操作' => 'Batch actions', '单个删除' => 'Delete single item',
    ];
    return isset($dict[$zh]) ? $dict[$zh] : $zh;
}

$cmd = $argv[1] ?? 'scan';
$dir = $argv[2] ?? '';
$keyStart = (int)($argv[3] ?? 1);
$maxKeys = (int)($argv[4] ?? 20);

if (!$dir) {
    echo "Usage: php i18n_admin_vue.php scan|apply <dir> [keyStart] [maxKeys]\n";
    exit(1);
}

if ($cmd === 'scan') {
    processDir($dir, $keyStart, $maxKeys, true);
} elseif ($cmd === 'apply') {
    processDir($dir, $keyStart, $maxKeys, false);
} else {
    echo "Unknown command: $cmd\n";
    exit(1);
}
