<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$zhByValue = array_flip($zh);

function walkDir($dir, $exts) {
    $files = array();
    $full = ROOT . $dir;
    if (!is_dir($full)) return $files;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($full));
    foreach ($it as $f) {
        if (!$f->isFile()) continue;
        $ext = strtolower($f->getExtension());
        if (!in_array($ext, $exts)) continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match('/wangeditor|layui\/lay\/modules|layui\.all|\.min\.js|vue-router|install\/|data\/lang\/|tools\/|PHPExcel|webscan360|umeditor\/lang/i', $rel)) continue;
        $files[] = $rel;
    }
    return $files;
}

function extractClean($content, $ext) {
    $found = array();
    // HTML text
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m);
    foreach ($m[1] as $s) {
        $s = trim(preg_replace('/\s+/u', ' ', html_entity_decode($s, ENT_QUOTES, 'UTF-8')));
        if ($s) $found[] = $s;
    }
    preg_match_all('/(?:placeholder|title|alt|label|value|content)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    // Vue props
    preg_match_all('/(?:message|title|tip|text|label|placeholder)\s*:\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    // JS/PHP quoted strings (layer.msg, echo, showToast, etc.)
    preg_match_all('/(?:layer\.(?:msg|alert|load|confirm)|showToast|yunAdminT|yunT|alert)\s*\(\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    if ($ext === 'php') {
        preg_match_all('/(?:echo|die)\s+["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
        foreach ($m[1] as $s) $found[] = trim($s);
        preg_match_all('/render_json\s*\([^,]+,\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
        foreach ($m[1] as $s) $found[] = trim($s);
        // Smarty plugin generated strings: "中文"
        preg_match_all('/["\']([^"\']{2,80}[\x{4e00}-\x{9fff}][^"\']{0,80})["\']/u', $content, $m);
        foreach ($m[1] as $s) {
            if (preg_match('/^[\x{4e00}-\x{9fff}\s\d，。！？：、（）【】·\-\+]+$/u', $s)) $found[] = trim($s);
        }
    }
    if (in_array($ext, array('js', 'htm', 'html'))) {
        preg_match_all('/["\']([^"\']{2,60}[\x{4e00}-\x{9fff}][^"\']{0,40})["\']/u', $content, $m);
        foreach ($m[1] as $s) {
            if (preg_match('/^[\x{4e00}-\x{9fff}\s\d，。！？：、（）【】·\-\.]+$/u', $s)) $found[] = trim($s);
        }
    }
    return array_unique($found);
}

function isClean($s) {
    if (mb_strlen($s, 'UTF-8') < 2 || mb_strlen($s, 'UTF-8') > 100) return false;
    if (preg_match('/\$|yun:|\/if|foreach|function |var |return |=>|getAttribute|addEventListener|styleSheet|\.php|\.js|namespace|prototype|console\.|layui\.use/i', $s)) return false;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()<>\/\\\\|~`\[\]{}]+$/u', $s)) return false;
    if (preg_match('/\{yun:|yun_auto_t|yunAdminT/i', $s)) return false;
    return true;
}

function guessModule($file) {
    static $map = array(
        'app/template/admin/' => 'admin', 'app/template/member/com/' => 'member_com',
        'app/template/member/user/' => 'member_user', 'app/template/wap/member/com/' => 'wap_com',
        'app/template/wap/member/user/' => 'wap_user', 'app/template/wap/' => 'wap',
        'app/template/resume/' => 'resume', 'app/template/company/' => 'common',
        'app/template/default/' => 'common', 'app/controller/wap/' => 'wap',
        'app/controller/' => 'model', 'member/com/' => 'member_com',
        'member/user/' => 'member_user', 'member/' => 'member_com',
        'admin/' => 'admin', 'app/include/libs/plugins/' => 'common',
        'app/include/libs/sysplugins/' => 'model', 'app/include/' => 'common',
        'js/' => 'common', 'app/template/wap/js/' => 'wap', 'wap/js/' => 'wap',
        'api/wxapp/' => 'wap',
    );
    foreach ($map as $p => $m) {
        if (strpos($file, $p) === 0) return $m;
    }
    return 'common';
}

$scanDirs = array(
    array('app/template', array('htm', 'html', 'vue')),
    array('js', array('js')),
    array('app/template/wap/js', array('js')),
    array('app/controller', array('php')),
    array('member', array('php')),
    array('app/include/libs/plugins', array('php')),
    array('app/include/libs/sysplugins', array('php')),
    array('app/include', array('php')),
    array('app/model', array('php')),
    array('admin', array('php')),
    array('api/wxapp', array('php')),
    array('wap/js', array('js')),
);

$missing = array();
foreach ($scanDirs as $item) {
    list($dir, $exts) = $item;
    foreach (walkDir($dir, $exts) as $rel) {
        if (preg_match('/dbbackup|mysqli\.class|mysql\.class|datacall\.class/i', $rel)) continue;
        $c = @file_get_contents(ROOT . $rel);
        if (!$c || !preg_match('/[\x{4e00}-\x{9fff}]/u', $c)) continue;
        $ext = pathinfo($rel, PATHINFO_EXTENSION);
        foreach (extractClean($c, $ext) as $s) {
            if (!isClean($s)) continue;
            if (!isset($zhByValue[$s]) && !isset($missing[$s])) {
                $missing[$s] = array('file' => $rel, 'module' => guessModule($rel));
            }
        }
    }
}

file_put_contents(ROOT . 'tools/missing_i18n.json', json_encode($missing, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "Missing: " . count($missing) . "\n";
