<?php
/**
 * Comprehensive scan for hardcoded Chinese not in lang pack.
 * Usage: php tools/scan_hardcoded_zh.php
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$structZh = include DATA_PATH . 'lang/zh_cn.php';
$zhByValue = array_flip($zh);
$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');

$skipDirs = array(
    'node_modules', 'vendor', '.git', 'data/cache', 'data/upload',
    'install', 'data/lang', 'tools', 'api/uc', 'api/uc_php7',
    'app/include/libs/PHPExcel', 'app/include/webscan360',
    'app/template/admin/js/wangeditor', 'js/layui/lay/modules',
);
$skipFilePat = array(
    '/\.min\.js$/i', '/vue-router/i', '/echarts/i', '/jquery/i',
    '/layui\.all/i', '/layui\.js$/i', '/phpyun_data\.sql$/i',
);

function shouldSkip($rel, $skipDirs, $skipFilePat) {
    foreach ($skipDirs as $d) {
        if (strpos($rel, $d . '/') === 0 || strpos($rel, '/' . $d . '/') !== false) return true;
    }
    foreach ($skipFilePat as $p) {
        if (preg_match($p, $rel)) return true;
    }
    return false;
}

function extractStrings($content, $ext) {
    $found = array();
    // HTML/Vue text nodes
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m);
    foreach ($m[1] as $s) $found[] = array('text' => trim($s), 'type' => 'html');

    // Attributes
    preg_match_all('/\b(?:alt|title|placeholder|value|content|label|message|tip)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m);
    foreach ($m[1] as $s) $found[] = array('text' => trim($s), 'type' => 'attr');

    // JS/PHP string literals with Chinese
    preg_match_all('/["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) {
        if (mb_strlen($s, 'UTF-8') > 150) continue;
        $found[] = array('text' => trim($s), 'type' => 'string');
    }

    // PHP echo/heredoc Chinese
    if ($ext === 'php') {
        preg_match_all('/(?:echo|die|exit)\s*[\(\s]*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
        foreach ($m[1] as $s) $found[] = array('text' => trim($s), 'type' => 'php_echo');
    }

    // Vue :title / message: etc
    preg_match_all('/(?:message|title|tip|text|label|placeholder)\s*:\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = array('text' => trim($s), 'type' => 'vue_prop');

    return $found;
}

function isIgnorable($s, $type) {
    if ($s === '' || mb_strlen($s, 'UTF-8') < 2) return true;
    if (mb_strlen($s, 'UTF-8') > 120) return true;
    // Already i18n wrapped
    if (preg_match('/yun_auto_t|yunAdminT|yunT|yun_t\(|lc\(/u', $s)) return true;
    // Smarty/vue vars
    if (preg_match('/\{yun:|yun:\}|\$[a-zA-Z_]|if\s|\/if|foreach|v-if|v-for|\{\{/u', $s)) return true;
    // Code fragments
    if (preg_match('/function\s|var\s|return\s|=>|\.js|getAttribute|addEventListener|styleSheet|namespace|prototype|console\./u', $s)) return true;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()<>\/\\\\|~`\[\]{}]+$/u', $s)) return true;
    if (preg_match('/^\/\/|^\/\*|^\*/', $s)) return true;
    // URLs paths
    if (preg_match('/^https?:\/\/|\.(png|jpg|gif|css|php|html|htm)/i', $s)) return true;
    // Comments in PHP
    if (preg_match('/^[\s\*\/]+/', $s) && $type === 'string') return true;
    return false;
}

function guessModule($file) {
    static $map = array(
        'app/template/admin/' => 'admin', 'app/template/member/com/' => 'member_com',
        'app/template/member/user/' => 'member_user', 'app/template/wap/member/com/' => 'wap_com',
        'app/template/wap/member/user/' => 'wap_user', 'app/template/wap/' => 'wap',
        'app/controller/wap/' => 'wap', 'app/controller/' => 'model',
        'member/com/' => 'member_com', 'member/user/' => 'member_user', 'member/' => 'member_com',
        'admin/' => 'admin', 'js/' => 'common',
    );
    foreach ($map as $p => $m) {
        if (strpos($file, $p) === 0) return $m;
    }
    return 'common';
}

function inStructLang($text, $structZh) {
    $flat = json_encode($structZh, JSON_UNESCAPED_UNICODE);
    return strpos($flat, $text) !== false;
}

$exts = array('htm', 'html', 'vue', 'js', 'php');
$missing = array();
$untranslatable = array(); // in pack but autoT fails
$byArea = array();

$iter = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT));
foreach ($iter as $file) {
    if (!$file->isFile()) continue;
    $rel = str_replace(ROOT, '', $file->getPathname());
    if (shouldSkip($rel, $skipDirs, $skipFilePat)) continue;
    $ext = strtolower($file->getExtension());
    if (!in_array($ext, $exts)) continue;

    $content = @file_get_contents($file->getPathname());
    if (!$content || !preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) continue;

    $area = 'other';
    if (strpos($rel, 'app/template/') === 0) $area = 'template';
    elseif (strpos($rel, 'js/') === 0 || strpos($rel, '/js/') !== false) $area = 'js';
    elseif (strpos($rel, 'app/controller/') === 0 || strpos($rel, 'member/') === 0 || strpos($rel, 'admin/') === 0) $area = 'php';
    elseif (strpos($rel, 'app/include/') === 0) $area = 'include';

    foreach (extractStrings($content, $ext) as $item) {
        $s = preg_replace('/\s+/u', ' ', $item['text']);
        $s = trim(html_entity_decode($s, ENT_QUOTES, 'UTF-8'));
        if (isIgnorable($s, $item['type'])) continue;

        $inPack = isset($zhByValue[$s]);
        $translated = $i18n->autoT($s);
        $ok = ($translated !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $translated));

        if ($ok) continue;

        if (!$inPack && !inStructLang($s, $structZh)) {
            $key = $s;
            if (!isset($missing[$key])) {
                $missing[$key] = array('module' => guessModule($rel), 'files' => array(), 'type' => $item['type']);
            }
            if (count($missing[$key]['files']) < 3) $missing[$key]['files'][] = $rel;
            $byArea[$area] = ($byArea[$area] ?? 0) + 1;
        }
    }
}

// Dedupe by area count
arsort($byArea);
echo "=== Hardcoded Chinese Scan ===\n";
echo "Lang pack: " . count($zh) . " entries\n";
echo "Missing from pack: " . count($missing) . "\n\n";
echo "By area:\n";
foreach ($byArea as $a => $c) echo "  $a: $c\n";

echo "\n--- Top 40 missing ---\n";
$i = 0;
foreach ($missing as $s => $info) {
    if ($i++ >= 40) { echo "  ...+" . (count($missing) - 40) . " more\n"; break; }
    $short = mb_strlen($s) > 55 ? mb_substr($s, 0, 55, 'UTF-8') . '...' : $s;
    echo "  [$short] ({$info['type']}) " . $info['files'][0] . "\n";
}

file_put_contents(ROOT . 'tools/hardcoded_missing.json', json_encode($missing, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nSaved: tools/hardcoded_missing.json\n";
