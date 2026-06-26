<?php
define('ROOT', dirname(__DIR__) . '/');
$base = ROOT . 'app/template/wap';
$skip = array('/js/mui/', '/js/umeditor/', '/mobiscroll/', 'demo.css', 'demo.html');
$reps = array(
    '微软雅黑' => 'Microsoft YaHei',
    '宋体' => 'sans-serif',
    'microsoft yahei,宋体' => 'Microsoft YaHei, sans-serif',
    'microsoft yahei' => 'Microsoft YaHei',
);
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
$n = 0;
foreach ($it as $f) {
    if (!$f->isFile() || $f->getExtension() !== 'css') {
        continue;
    }
    $path = $f->getPathname();
    foreach ($skip as $s) {
        if (strpos($path, $s) !== false) {
            continue 2;
        }
    }
    $c = file_get_contents($path);
    $orig = $c;
    foreach ($reps as $from => $to) {
        $c = str_replace($from, $to, $c);
    }
    if ($c !== $orig) {
        file_put_contents($path, $c);
        echo str_replace(ROOT, '', $path) . "\n";
        $n++;
    }
}
echo "Updated $n CSS files.\n";
