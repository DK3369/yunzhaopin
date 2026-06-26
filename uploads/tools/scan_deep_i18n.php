<?php
/**
 * Deep scan: any clean Chinese string in source not in lang pack.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$struct = include DATA_PATH . 'lang/zh_cn.php';
$zhByValue = array_flip($zh);
$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');

$skip = '#(install/|data/lang/|tools/|vendor/|node_modules|\.git/|PHPExcel|webscan360|wangeditor|layui/lay/modules|layui\.all|\.min\.js|vue-router|phpyun_data\.sql|dbbackup|mysqli\.class|mysql\.class)#i';

function walkAll($root, $skip) {
    $files = array();
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root));
    foreach ($it as $f) {
        if (!$f->isFile()) continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;
        $ext = strtolower($f->getExtension());
        if (!in_array($ext, array('php','js','htm','html','vue','css'))) continue;
        $files[] = $rel;
    }
    return $files;
}

function extractZhStrings($content) {
    $out = array();
    preg_match_all('/["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) {
        $s = trim(html_entity_decode($s, ENT_QUOTES, 'UTF-8'));
        if (mb_strlen($s,'UTF-8') >= 2 && mb_strlen($s,'UTF-8') <= 120) $out[] = $s;
    }
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m2);
    foreach ($m2[1] as $s) {
        $s = trim($s);
        if (mb_strlen($s,'UTF-8') >= 2 && mb_strlen($s,'UTF-8') <= 120) $out[] = $s;
    }
    return array_unique($out);
}

function ignorable($s) {
    if (preg_match('/\$|yun:|\/if|foreach|function\s|var\s|return\s|=>|getAttribute|\.php|\.js|namespace|prototype|console\.|layui\.|yun_auto_t|yunAdminT|\/\//u', $s)) return true;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()<>\/\\\\|~`\[\]{}]+$/u', $s)) return true;
    if (preg_match('/^\/\*|^\*|^#/', $s)) return true;
    return false;
}

function inStruct($s, $struct) {
    return strpos(json_encode($struct, JSON_UNESCAPED_UNICODE), json_encode($s, JSON_UNESCAPED_UNICODE)) !== false;
}

$missing = array();
$untranslated = array();
$todoEn = 0;

foreach (walkAll(ROOT, $skip) as $rel) {
    $c = @file_get_contents(ROOT . $rel);
    if (!$c || !preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
    foreach (extractZhStrings($c) as $s) {
        if (ignorable($s)) continue;
        $inPack = isset($zhByValue[$s]);
        $t = $i18n->autoT($s);
        $ok = ($t !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $t));
        if ($ok || $inPack || inStruct($s, $struct)) continue;
        if (!isset($missing[$s])) $missing[$s] = $rel;
    }
}

$en = include DATA_PATH . 'lang/auto/en_us.php';
foreach ($en as $v) {
    if (is_string($v) && strpos($v, '[TODO]') === 0) $todoEn++;
}

echo "=== Deep Scan ===\n";
echo "Lang pack: " . count($zh) . "\n";
echo "Missing from pack: " . count($missing) . "\n";
echo "EN [TODO] entries: $todoEn\n\n";

$i = 0;
foreach ($missing as $s => $f) {
    if ($i++ >= 40) { echo "...+" . (count($missing)-40) . " more\n"; break; }
    $short = mb_strlen($s)>50 ? mb_substr($s,0,50,'UTF-8').'...' : $s;
    echo "  [$short] $f\n";
}

file_put_contents(ROOT . 'tools/deep_missing.json', json_encode($missing, JSON_UNESCAPED_UNICODE|JSON_PRETTY_PRINT));
