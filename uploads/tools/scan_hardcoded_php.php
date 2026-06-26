<?php
/**
 * Scan PHP files for user-facing Chinese strings NOT wrapped with yun_auto_t.
 */
define('ROOT', dirname(__DIR__) . '/');
$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$flip = array_flip($zh);

$dirs = array('app/model', 'app/controller', 'member', 'admin/model', 'api/wxapp', 'wap/member', 'app/include');
$issues = array();
$total = 0;

function scanPhpFile($rel, $content, &$issues, &$total, $flip) {
    // Skip security-disabled dead code blocks
    $content = preg_replace('/\/\*\s*SECURITY DISABLED[\s\S]*?\*\//u', '', $content);
    // Skip comments-only lines for msg assignments
    preg_match_all(
        "/(?:\['(?:msg|error|errmsg|linkMsg|title|content|message|statusbody)'\]\s*=|(?:'msg'|\"msg\"|'error'|\"error\"|'linkMsg'|\"linkMsg\")\s*=>\s*|return\s+array\s*\([^)]*'msg'\s*=>\s*)['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u",
        $content,
        $m,
        PREG_OFFSET_CAPTURE
    );
    foreach ($m[1] as $item) {
        $s = trim($item[0]);
        $offset = $item[1];
        if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 120) continue;
        if (preg_match('/\$|function |=>|\.php|SELECT /i', $s)) continue;
        $total++;
        // Check if wrapped with yun_auto_t nearby
        $before = substr($content, max(0, $offset - 80), 80);
        if (preg_match('/yun_auto_t\s*\(/', $before)) continue;
        if (!isset($flip[$s])) {
            $issues['missing_pack'][$s] = $rel;
        } else {
            $issues['hardcoded'][$s] = $rel;
        }
    }
    // echo/die with Chinese
    preg_match_all("/(?:echo|die)\s*\(?\s*['\"]([^'\"]*[\x{4e00}-\x{9fff}][^'\"]*)['\"]/u", $content, $m2);
    foreach ($m2[1] as $s) {
        $s = trim($s);
        if (mb_strlen($s, 'UTF-8') < 2) continue;
        $total++;
        $issues['echo'][$s] = $rel;
    }
}

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match('/vendor|PHPExcel|aliyun|install/i', $rel)) continue;
        $c = file_get_contents($f->getPathname());
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
        scanPhpFile($rel, $c, $issues, $total, $flip);
    }
}

echo "=== PHP hardcoded Chinese (msg/linkMsg) ===\n";
echo "Total assignments: $total\n";
echo "In pack but NOT yun_auto_t: " . count($issues['hardcoded'] ?? array()) . "\n";
echo "NOT in pack: " . count($issues['missing_pack'] ?? array()) . "\n";
echo "echo/die: " . count($issues['echo'] ?? array()) . "\n\n";

$byDir = array();
foreach (array('hardcoded', 'missing_pack', 'echo') as $type) {
    foreach (($issues[$type] ?? array()) as $s => $rel) {
        $parts = explode('/', $rel);
        $bucket = $parts[0] . (isset($parts[1]) ? '/' . $parts[1] : '');
        if (!isset($byDir[$bucket])) {
            $byDir[$bucket] = array('hardcoded' => 0, 'missing_pack' => 0, 'echo' => 0);
        }
        $byDir[$bucket][$type]++;
    }
}
arsort($byDir);
echo "By directory:\n";
foreach ($byDir as $dir => $counts) {
    $sum = $counts['hardcoded'] + $counts['missing_pack'] + $counts['echo'];
    if ($sum > 0) {
        echo "  $dir: hardcoded={$counts['hardcoded']} missing={$counts['missing_pack']} echo={$counts['echo']}\n";
    }
}
echo "\n";

$i = 0;
foreach (($issues['hardcoded'] ?? array()) as $s => $f) {
    if ($i++ >= 40) { echo "...+" . (count($issues['hardcoded']) - 40) . " more\n"; break; }
    echo "  [hardcoded] $s [$f]\n";
}
$i = 0;
foreach (($issues['missing_pack'] ?? array()) as $s => $f) {
    if ($i++ >= 20) break;
    echo "  [missing] $s [$f]\n";
}

file_put_contents(ROOT . 'tools/hardcoded_php.json', json_encode($issues, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
