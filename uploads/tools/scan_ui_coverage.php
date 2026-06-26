<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$zhByValue = array_flip($zh);
$zhNorm = array();
foreach ($zh as $k => $v) {
    $n = norm($v);
    if ($n !== '') $zhNorm[$n] = $v;
}
$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');

function norm($s) {
    $s = trim(html_entity_decode($s, ENT_QUOTES, 'UTF-8'));
    $s = preg_replace('/\s+/u', '', $s);
    $s = str_replace(array('：', ':', '，', ',', '。', '.', '！', '!', '？', '?'), '', $s);
    return $s;
}

function walk($dir, $exts) {
    $files = array();
    $full = ROOT . $dir;
    if (!is_dir($full)) return $files;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($full));
    foreach ($it as $f) {
        if (!$f->isFile() || !in_array(strtolower($f->getExtension()), $exts, true)) continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match('/wangeditor|layui\/lay\/modules|\.min\.js|vue-router|umeditor\/lang/i', $rel)) continue;
        $files[] = $rel;
    }
    return $files;
}

function uiStrings($content) {
    $found = array();
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m);
    foreach ($m[1] as $s) {
        $s = trim(preg_replace('/\s+/u', ' ', html_entity_decode($s, ENT_QUOTES, 'UTF-8')));
        if ($s && !preg_match('/\{yun:|yun:\}|\$|\/if|foreach/i', $s)) $found[] = $s;
    }
    preg_match_all('/\b(?:placeholder|title|alt|label|value|content|message|tip)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    preg_match_all('/(?:message|title|tip|text|label|placeholder)\s*:\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    preg_match_all('/(?:layer\.(?:msg|alert|load)|showToast|yunAdminT)\s*\(\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    return array_unique($found);
}

function ok($s, $zhByValue, $zhNorm, $i18n) {
    if (isset($zhByValue[$s])) return 'pack';
    if (isset($zhNorm[norm($s)])) return 'norm';
    $t = $i18n->autoT($s);
    if ($t !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $t)) return 'autot';
    return false;
}

$dirs = array(
    'app/template/default' => array('htm'),
    'app/template/member' => array('htm'),
    'app/template/wap' => array('htm'),
    'app/template/company' => array('htm'),
    'app/template/admin' => array('vue'),
    'app/template/resume' => array('htm'),
    'js' => array('js'),
    'app/template/wap/js' => array('js'),
    'wap/js' => array('js'),
);

$missing = array();
$stats = array('total' => 0, 'pack' => 0, 'norm' => 0, 'autot' => 0, 'miss' => 0);

foreach ($dirs as $dir => $exts) {
    foreach (walk($dir, $exts) as $rel) {
        $c = file_get_contents(ROOT . $rel);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
        foreach (uiStrings($c) as $s) {
            if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 100) continue;
            if (preg_match('/function |var |return |=>|getAttribute|layui\.use/i', $s)) continue;
            $stats['total']++;
            $r = ok($s, $zhByValue, $zhNorm, $i18n);
            if ($r) { $stats[$r]++; continue; }
            $stats['miss']++;
            if (!isset($missing[$s])) $missing[$s] = $rel;
        }
    }
}

echo "=== UI String Coverage ===\n";
echo "Total UI strings: {$stats['total']}\n";
echo "In pack (exact): {$stats['pack']}\n";
echo "In pack (normalized): {$stats['norm']}\n";
echo "autoT works: {$stats['autot']}\n";
echo "TRULY MISSING: {$stats['miss']}\n";
echo "Coverage: " . round((1 - $stats['miss'] / max($stats['total'], 1)) * 100, 2) . "%\n\n";

$i = 0;
foreach ($missing as $s => $f) {
    if ($i++ >= 60) { echo "...+" . (count($missing) - 60) . " more\n"; break; }
    echo "  $s [$f]\n";
}

$export = array();
foreach ($missing as $s => $f) {
    $mod = 'common';
    if (strpos($f, 'admin') !== false) $mod = 'admin';
    elseif (strpos($f, 'wap') !== false) $mod = 'wap';
  $export[$s] = array('file' => $f, 'module' => $mod);
}
file_put_contents(ROOT . 'tools/missing_i18n.json', json_encode($export, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nExport: " . count($export) . " for lang pack\n";
