<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$en = include DATA_PATH . 'lang/auto/en_us.php';
$zhByValue = array_flip($zh);
$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');

function walk($pattern) {
    $files = glob(ROOT . $pattern, GLOB_BRACE);
    if (!$files) {
        // glob recursive fallback
        $files = array();
        $base = ROOT . dirname($pattern);
        $name = basename($pattern);
        if (!is_dir($base)) return array();
        $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
        foreach ($it as $f) {
            if ($f->isFile() && fnmatch($name, $f->getFilename())) {
                $files[] = $f->getPathname();
            }
        }
    }
    return array_map(function($f) { return str_replace(ROOT, '', $f); }, $files);
}

$groups = array(
    '前台.htm' => walk('app/template/{default,company,member,wap}/**/*.htm'),
    '后台.vue' => walk('app/template/admin/**/*.vue'),
    '业务JS'   => array_merge(walk('js/*.js'), walk('app/template/wap/js/*.js')),
);

function extractClean($content) {
    $found = array();
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m);
    foreach ($m[1] as $s) {
        $s = trim(preg_replace('/\s+/u', ' ', html_entity_decode($s)));
        if ($s && !preg_match('/\$|yun:|\/if|foreach/u', $s)) $found[] = $s;
    }
    preg_match_all('/(?:placeholder|title|alt|label)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    // Vue template text in .vue
    preg_match_all('/>([^<>{}\n]*[\x{4e00}-\x{9fff}][^<>{}\n]*)</u', $content, $m);
    foreach ($m[1] as $s) {
        $s = trim($s);
        if ($s && !preg_match('/\{\{|v-|@click|\$|\/\//u', $s)) $found[] = $s;
    }
    preg_match_all('/(?:message|title|tip|text|label|placeholder)\s*:\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m);
    foreach ($m[1] as $s) $found[] = trim($s);
    return array_unique($found);
}

echo "=== 精准扫描（排除第三方库）===\n\n";
$allMissing = array();
$allNoAutot = array();

foreach ($groups as $name => $files) {
    $missing = array();
    $noAutot = array();
    $ok = 0; $n = 0;
    foreach ($files as $rel) {
        if (preg_match('/wangeditor|layui\.all|\.min\.js|vue-router/i', $rel)) continue;
        $c = file_get_contents(ROOT . $rel);
        foreach (extractClean($c) as $s) {
            if (mb_strlen($s,'UTF-8') < 2) continue;
            $n++;
            $t = $i18n->autoT($s);
            if ($t !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $t)) { $ok++; continue; }
            if (!isset($zhByValue[$s])) {
                $missing[$s] = $rel;
                $allMissing[$s] = $rel;
            } else {
                $noAutot[$s] = $zhByValue[$s];
                $allNoAutot[$s] = $zhByValue[$s];
            }
        }
    }
    $rate = $n ? round($ok/$n*100,1) : 100;
    echo "$name: 文件".count($files)." 中文{$n}条 | 可译{$ok} | 缺包".count($missing)." | 有包未译".count($noAutot)." | 覆盖率{$rate}%\n";
}

echo "\n--- 真正缺语言包（前台+后台，前30条）---\n";
$i=0;
foreach ($allMissing as $s=>$f) {
    if ($i++>=30) { echo "...共".count($allMissing)."条\n"; break; }
    echo "  $s  [$f]\n";
}

echo "\n--- 关键结论 ---\n";
echo "语言包有英文但无法自动翻译: ".count($allNoAutot)." 条（因 aliases.php 已删，模板中文无法映射到编号key）\n";
echo "语言包完全缺失: ".count($allMissing)." 条\n";
