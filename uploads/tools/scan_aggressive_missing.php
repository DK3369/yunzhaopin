<?php
/**
 * Aggressive scan: Chinese string literals in source not in lang pack.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$zhByValue = array_flip($zh);

$skipRe = '#(install/|data/lang/|tools/|vendor/|node_modules|\.git/|PHPExcel|webscan360|wangeditor|layui/lay/modules|layui\.all|\.min\.js|vue-router|umeditor/lang/zh-cn|phpyun_data\.sql|dbbackup|mysqli\.class|mysql\.class|api/uc)#i';

function walk($root, $skipRe) {
    $files = array();
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($root));
    foreach ($it as $f) {
        if (!$f->isFile()) continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skipRe, $rel)) continue;
        $ext = strtolower($f->getExtension());
        if (!in_array($ext, array('php', 'js', 'htm', 'html', 'vue'))) continue;
        $files[] = $rel;
    }
    return $files;
}

function extractAll($content) {
    $found = array();
    // Quoted strings with Chinese
    preg_match_all('/["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = $s;
    // HTML/Vue text
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m);
    foreach ($m[1] as $s) $found[] = $s;
    // Vue/React attributes
    preg_match_all('/\b(?:title|label|placeholder|message|tip|content|alt|value|confirmButtonText|cancelButtonText)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m);
    foreach ($m[1] as $s) $found[] = $s;
  preg_match_all('/\b(?:title|label|placeholder|message|tip|content)\s*:\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = $s;
    return array_unique($found);
}

function isNoise($s) {
    if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 150) return true;
    if (preg_match('/\$[a-zA-Z_{]|yun:|\/yun|yun_auto_t|yunAdminT|yunT\(|\/if|foreach|function\s|var\s|return\s|=>|getAttribute|addEventListener|styleSheet|namespace|prototype|console\.|layui\.|\.php|\.js|\.css|\.html|preg_match|\/u\'|\\\\x\{|\\\\u\{|JSON_|array\(|class\s/i', $s)) return true;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()<>\/\\\\|~`\[\]{}]+$/u', $s)) return true;
    if (preg_match('/^\/\/|^\/\*|^\*/', trim($s))) return true;
    if (substr_count($s, ' ') > 25) return true;
    return false;
}

function area($rel) {
    if (strpos($rel, 'app/template/admin') === 0) return 'admin_vue';
    if (strpos($rel, 'app/template/') === 0) return 'template';
    if (strpos($rel, 'app/model/') === 0) return 'model';
    if (strpos($rel, 'app/controller/') === 0) return 'controller';
    if (strpos($rel, 'member/') === 0) return 'member';
    if (strpos($rel, 'admin/') === 0) return 'admin_php';
    if (strpos($rel, 'api/wxapp/') === 0) return 'wxapp';
    if (strpos($rel, 'js/') === 0 || strpos($rel, '/js/') !== false) return 'js';
    if (strpos($rel, 'app/include/') === 0) return 'include';
    return 'other';
}

$missing = array();
$byArea = array();

foreach (walk(ROOT, $skipRe) as $rel) {
    $c = @file_get_contents(ROOT . $rel);
    if (!$c || !preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
    foreach (extractAll($c) as $raw) {
        $s = trim(html_entity_decode($raw, ENT_QUOTES, 'UTF-8'));
        $s = preg_replace('/\s+/u', ' ', $s);
        if (isNoise($s)) continue;
        if (isset($zhByValue[$s])) continue;
        $a = area($rel);
        $byArea[$a] = ($byArea[$a] ?? 0) + 1;
        if (!isset($missing[$s])) {
            $missing[$s] = array('file' => $rel, 'area' => $a);
        }
    }
}

arsort($byArea);
echo "=== Aggressive Missing Scan ===\n";
echo "Total missing unique strings: " . count($missing) . "\n\nBy area:\n";
foreach ($byArea as $a => $c) echo "  $a: $c\n";

echo "\nTop 50 missing:\n";
$i = 0;
foreach ($missing as $s => $info) {
    if ($i++ >= 50) { echo "  ...+" . (count($missing) - 50) . " more\n"; break; }
    $short = mb_strlen($s) > 55 ? mb_substr($s, 0, 55, 'UTF-8') . '...' : $s;
    echo "  [$short] ({$info['area']}) {$info['file']}\n";
}

$export = array();
foreach ($missing as $s => $info) {
    $mod = 'common';
    if ($info['area'] === 'admin_vue' || $info['area'] === 'admin_php') $mod = 'admin';
    elseif ($info['area'] === 'model' || $info['area'] === 'controller') $mod = 'model';
    elseif ($info['area'] === 'member') $mod = 'member_com';
    elseif ($info['area'] === 'wxapp') $mod = 'wap';
    elseif ($info['area'] === 'template' && strpos($info['file'], 'wap') !== false) $mod = 'wap';
    $export[$s] = array('file' => $info['file'], 'module' => $mod);
}
file_put_contents(ROOT . 'tools/missing_i18n.json', json_encode($export, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nSaved: tools/missing_i18n.json (" . count($export) . " entries)\n";
