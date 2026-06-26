<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$flip = array_flip($zh);
$missing = array();

$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT . 'app/model'));
foreach ($it as $f) {
    if (!$f->isFile() || $f->getExtension() !== 'php') continue;
    $rel = str_replace(ROOT, '', $f->getPathname());
    $c = file_get_contents($f->getPathname());
    // msg/error/title/content 字段
    preg_match_all("/['\"](?:msg|error|errmsg|title|content|message|statusbody|sbody)['\"]\s*=>\s*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m);
    preg_match_all("/return\s+array\s*\(\s*['\"](?:msg|error)['\"]\s*=>\s*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m2);
    preg_match_all("/>\s*([\x{4e00}-\x{9fff}][^<]{0,30})\s*<\//u", $c, $m3);
    foreach (array_merge($m[1], $m2[1], $m3[1]) as $s) {
        $s = trim($s);
        if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 80) continue;
        if (preg_match('/\$|=>|function /', $s)) continue;
        if (!isset($flip[$s])) $missing[$s] = $rel;
    }
}

echo "Model UI/msg missing: " . count($missing) . "\n";
$export = array();
foreach ($missing as $s => $f) {
    $export[$s] = array('file' => $f, 'module' => 'model');
}
file_put_contents(ROOT . 'tools/missing_i18n.json', json_encode($export, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
$i = 0;
foreach ($missing as $s => $f) {
    if ($i++ >= 50) { echo "...+" . (count($missing) - 50) . "\n"; break; }
    echo "  $s [$f]\n";
}
