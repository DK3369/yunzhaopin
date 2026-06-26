<?php
define('ROOT', dirname(__DIR__) . '/');
$dirs = array('api/wxapp', 'app/controller/wap', 'wap/member');
foreach ($dirs as $dir) {
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT . $dir));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (strpos($rel, 'wap.enum') !== false) {
            continue;
        }
        foreach (file($f->getPathname()) as $n => $line) {
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $line)) {
                continue;
            }
            $t = trim($line);
            if (preg_match('/^\s*(\*|\/\/|#)/', $t)) {
                continue;
            }
            if (preg_match('/zpdataTitle\s*\(/', $line)) {
                continue;
            }
            if (preg_match('/yun_auto_t\s*\(/', $line)) {
                continue;
            }
            if (preg_match('/WapDbEnum::/', $line)) {
                continue;
            }
            // skip pure end-of-line comments without string literals
            if (preg_match('/\/\//', $line) && !preg_match('/=\s*["\'][^"\']*[\x{4e00}-\x{9fff}]/u', $line)
                && !preg_match('/\?\s*["\']/', $line) && !preg_match('/render_json|member_log|layer_msg|ACT_msg/', $line)) {
                continue;
            }
            echo $rel . ':' . ($n + 1) . ': ' . trim($line) . "\n";
        }
    }
}
