<?php
/**
 * Scan PHP model/controller msg strings not in auto lang pack.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$zhByValue = array_flip($zh);

function walkPhp($dir) {
    $files = array();
    $full = ROOT . $dir;
    if (!is_dir($full)) return $files;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($full));
    foreach ($it as $f) {
        if ($f->isFile() && $f->getExtension() === 'php') {
            $rel = str_replace(ROOT, '', $f->getPathname());
            if (preg_match('/vendor|install|data\/lang|tools\//', $rel)) continue;
            $files[] = $rel;
        }
    }
    return $files;
}

$missing = array();
foreach (walkPhp('app/model') as $rel) {
    $c = file_get_contents(ROOT . $rel);
    preg_match_all("/['\"]msg['\"]\s*=>\s*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m);
    preg_match_all("/['\"]message['\"]\s*=>\s*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $c, $m2);
    foreach (array_merge($m[1], $m2[1]) as $s) {
        $s = trim($s);
        if (mb_strlen($s,'UTF-8') < 2 || mb_strlen($s,'UTF-8') > 120) continue;
        if (!isset($zhByValue[$s])) $missing[$s] = $rel;
    }
}

echo "Model msg missing: " . count($missing) . "\n";
$i = 0;
foreach ($missing as $s => $f) {
    if ($i++ >= 30) { echo "...+" . (count($missing)-30) . "\n"; break; }
    echo "  $s [$f]\n";
}
file_put_contents(ROOT . 'tools/missing_model_msg.json', json_encode($missing, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
