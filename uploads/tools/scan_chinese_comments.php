<?php
/**
 * List PHP comment tokens containing Chinese.
 * Usage: php tools/scan_chinese_comments.php [--json]
 */
define('ROOT', dirname(__DIR__) . '/');
$asJson = in_array('--json', $argv ?? array(), true);

$skip = '/vendor|PHPExcel|install\/|data\/lang|tools\/i18n_scan_report|dbbackup|PHPWord|tcpdf|ueditor|lib_splitword/i';
$dirs = array('app', 'admin', 'member', 'api', 'wap');
$items = array();

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;
        $content = file_get_contents($f->getPathname());
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) continue;
        $tokens = token_get_all($content);
        foreach ($tokens as $token) {
            if (!is_array($token)) continue;
            if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) continue;
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $token[1])) continue;
            $text = trim($token[1]);
            $items[] = array('file' => $rel, 'line' => $token[2], 'text' => $text);
        }
    }
}

if ($asJson) {
    echo json_encode($items, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT);
    exit;
}

$unique = array();
foreach ($items as $it) {
    $unique[$it['text']] = ($unique[$it['text']] ?? 0) + 1;
}
arsort($unique);
echo 'Files with Chinese comments: ' . count(array_unique(array_column($items, 'file'))) . "\n";
echo 'Comment tokens: ' . count($items) . "\n";
echo 'Unique comment texts: ' . count($unique) . "\n\nTop 40:\n";
$i = 0;
foreach ($unique as $text => $cnt) {
    if ($i++ >= 40) break;
    $one = str_replace("\n", ' ', $text);
    if (mb_strlen($one) > 120) $one = mb_substr($one, 0, 117) . '...';
    echo "[$cnt] $one\n";
}
