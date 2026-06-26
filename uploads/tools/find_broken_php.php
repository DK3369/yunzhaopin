<?php
define('ROOT', dirname(__DIR__) . '/');
$skip = '/vendor|tecentcode|aliyunemail|PHPExcel|install\/|dbbackup/i';
$broken = array();
foreach (array('app', 'admin', 'member', 'api', 'wap') as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;
        exec('php -l ' . escapeshellarg($f->getPathname()) . ' 2>&1', $out, $code);
        if ($code !== 0) $broken[] = $rel;
    }
}
sort($broken);
file_put_contents(__DIR__ . '/broken_php_files.txt', implode("\n", $broken) . "\n");
echo count($broken) . " broken files\n";
foreach ($broken as $f) echo "$f\n";
