<?php
define('ROOT', dirname(__DIR__) . '/');
$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$flip = array_flip($zh);
$missing = array();

$dirs = array('app/include', 'app/controller', 'admin/model', 'api', 'member');
$skip = array('/vendor/', '/cache/', '/data/plus/');

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        $skipIt = false;
        foreach ($skip as $s) { if (strpos($rel, $s) !== false) { $skipIt = true; break; } }
        if ($skipIt) continue;
        $c = file_get_contents($f->getPathname());
        // 用户可见：echo/return/msg/HTML片段
        preg_match_all("/(?:echo\s+|return\s+['\"]|['\"](?:msg|error|title|content)['\"]\s*=>\s*['\"]|prestr|nextstr|value=)[^'\"]*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m);
        preg_match_all("/>([\x{4e00}-\x{9fff}][^<]{1,40})</u", $c, $m2);
        preg_match_all("/['\"]([^'\"]{2,60}[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m3);
        foreach (array_merge($m[1] ?? array(), $m2[1] ?? array(), $m3[1] ?? array()) as $s) {
            $s = trim(html_entity_decode($s, ENT_QUOTES, 'UTF-8'));
            if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 80) continue;
            if (preg_match('/\$|=>|function |\\\\|\.php|SELECT |INSERT |UPDATE |DELETE |\/\//i', $s)) continue;
            if (preg_match('/^[\x{4e00}-\x{9fff}]{1}$/u', $s)) continue; // single char weekday ok in pack
            if (!isset($flip[$s])) $missing[$s] = $rel;
        }
    }
}

ksort($missing);
echo "PHP runtime missing: " . count($missing) . "\n";
$export = array();
foreach ($missing as $s => $f) {
    $export[$s] = array('file' => $f, 'module' => 'php');
}
file_put_contents(ROOT . 'tools/missing_i18n.json', json_encode($export, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
$i = 0;
foreach ($missing as $s => $f) {
    if ($i++ >= 80) { echo "...+" . (count($missing) - 80) . " more\n"; break; }
    echo "  $s [$f]\n";
}
