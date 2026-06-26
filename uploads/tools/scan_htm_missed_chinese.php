<?php
/** Scan .htm for Chinese the i18n tool may miss. */
define('ROOT', dirname(__DIR__) . '/');
$types = array('between_yun' => 0, 'text_node' => 0, 'attr' => 0, 'script' => 0, 'comment' => 0, 'other' => 0);
$samples = array();
foreach (new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT . 'app/template')) as $f) {
    if (!$f->isFile() || strtolower($f->getExtension()) !== 'htm') continue;
    $rel = str_replace(ROOT, '', $f->getPathname());
    $c = file_get_contents($f->getPathname());
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
    if (preg_match_all('/\{\/yun\}((?:[^{]|<[^>]*>)*?)([\x{4e00}-\x{9fff}]+)((?:[^{]|<[^>]*>)*?)\{yun:/us', $c, $m)) {
        foreach ($m[2] as $t) {
            $types['between_yun']++;
            $samples['between:' . $t] = ($samples['between:' . $t] ?? 0) + 1;
        }
    }
    if (preg_match_all('/<!--([\s\S]*?)-->/u', $c, $m)) {
        foreach ($m[1] as $inner) {
            if (preg_match('/[\x{4e00}-\x{9fff}]/u', trim($inner))) $types['comment']++;
        }
    }
    if (preg_match_all('#<script[^>]*>([\s\S]*?)</script>#iu', $c, $sm)) {
        foreach ($sm[1] as $js) {
            $types['script'] += preg_match_all('/[\x{4e00}-\x{9fff}]+/u', $js);
            $types['script_comment'] = ($types['script_comment'] ?? 0) + preg_match_all('~//.*[\x{4e00}-\x{9fff}].*$~mu', $js);
        }
    }
    $s = $c;
    $s = preg_replace('/<!--[\s\S]*?-->/u', '', $s);
    $s = preg_replace('/\{yun:\}[\s\S]*?\{\/yun\}/u', '', $s);
    $s = preg_replace('#<(script|style)[^>]*>.*?</\1>#is', '', $s);
    if (preg_match_all('/[\x{4e00}-\x{9fff}]+/u', $s, $m2)) {
        $types['other'] += count($m2[0]);
        foreach ($m2[0] as $t) {
            if (mb_strlen($t) <= 20) $samples['other:' . $t] = ($samples['other:' . $t] ?? 0) + 1;
        }
    }
}
arsort($samples);
echo "Between yun tags: {$types['between_yun']}\n";
echo "HTML comments: {$types['comment']}\n";
echo "Script Chinese chars: {$types['script']}\n";
echo "Script // comments: " . ($types['script_comment'] ?? 0) . "\n";
echo "Other visible segments: {$types['other']}\n\nTop samples:\n";
$i = 0;
foreach ($samples as $k => $n) {
    echo "  [$n] $k\n";
    if (++$i >= 40) break;
}
