<?php
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');
require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$en = include DATA_PATH . 'lang/auto/en_us.php';
$zhByValue = array_flip($zh);
$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');
$hasAlias = is_file(DATA_PATH . 'lang/auto/aliases.php');

$scanRoots = array(
    '前台模板' => array('app/template/default', 'app/template/company', 'app/template/member', 'app/template/wap'),
    '后台Vue'  => array('app/template/admin'),
    '前台JS'   => array('js', 'app/template/wap/js'),
    '控制器PHP'=> array('app/controller', 'member'),
);

function walkFiles($dirs) {
    $files = array();
    foreach ($dirs as $dir) {
        $full = ROOT . $dir;
        if (!is_dir($full)) continue;
        $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($full));
        foreach ($it as $f) {
            if (!$f->isFile()) continue;
            $ext = strtolower($f->getExtension());
            if (!in_array($ext, array('htm','html','vue','js','php'))) continue;
            if (preg_match('/\.min\.js$|vue-router|echarts|jquery\.|layui\.all/i', $f->getPathname())) continue;
            $files[] = str_replace(ROOT, '', $f->getPathname());
        }
    }
    return $files;
}

function extractStrings($content) {
    $found = array();
    preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m1);
    preg_match_all('/\b(?:alt|title|placeholder|value|content|label)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m2);
    preg_match_all('/(?:layer\.msg|layer\.alert|layer\.confirm|yunAdminT|yunT)\s*\(\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m3);
    foreach (array_merge($m1[1], $m2[1], $m3[1]) as $s) {
        $s = trim(preg_replace('/\s+/u', ' ', $s));
        if ($s !== '') $found[] = $s;
    }
    return $found;
}

function ignorable($s) {
    if (mb_strlen($s,'UTF-8') < 2) return true;
    if (preg_match('/\$[a-zA-Z_{]|yun:|\/yun|if\s|foreach|\/if/u', $s)) return true;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()<>\/\\\\]+$/u', $s)) return true;
    return false;
}

$stats = array();
$missingByArea = array();
$noAutotByArea = array();

foreach ($scanRoots as $area => $dirs) {
    $files = walkFiles($dirs);
    $missing = array();
    $noAutot = array();
    $ok = 0;
    $total = 0;

    foreach ($files as $rel) {
        $content = file_get_contents(ROOT . $rel);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) continue;
        foreach (extractStrings($content) as $s) {
            if (ignorable($s)) continue;
            $total++;
            $translated = $i18n->autoT($s);
            $inPack = isset($zhByValue[$s]);

            if ($translated !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $translated)) {
                $ok++;
            } elseif (!$inPack) {
                if (!isset($missing[$s])) $missing[$s] = $rel;
            } else {
                if (!isset($noAutot[$s])) $noAutot[$s] = array('key' => $zhByValue[$s], 'en' => $en[$zhByValue[$s]], 'file' => $rel);
            }
        }
    }

    $stats[$area] = array('files'=>count($files), 'strings'=>$total, 'ok'=>$ok, 'missing'=>count($missing), 'no_autot'=>count($noAutot));
    $missingByArea[$area] = $missing;
    $noAutotByArea[$area] = $noAutot;
}

echo "aliases.php: " . ($hasAlias ? '存在' : '已删除（中文无法自动映射到编号key）') . "\n\n";
echo str_pad('区域', 12) . str_pad('文件', 8) . str_pad('中文条', 8) . str_pad('可翻译', 8) . str_pad('缺语言包', 10) . str_pad('有包未译', 10) . "覆盖率\n";
echo str_repeat('-', 72) . "\n";
foreach ($stats as $area => $s) {
    $rate = $s['strings'] > 0 ? round(($s['ok'] / $s['strings']) * 100, 1) : 100;
    echo str_pad($area, 14) . str_pad($s['files'], 8) . str_pad($s['strings'], 8) . str_pad($s['ok'], 8) . str_pad($s['missing'], 10) . str_pad($s['no_autot'], 10) . "{$rate}%\n";
}

foreach ($missingByArea as $area => $missing) {
    if (empty($missing)) continue;
    echo "\n=== {$area}：语言包缺失（前20条）===\n";
    $i = 0;
    foreach ($missing as $s => $f) {
        if ($i++ >= 20) { echo "  ...共 " . count($missing) . " 条\n"; break; }
        $short = mb_strlen($s) > 60 ? mb_substr($s, 0, 60, 'UTF-8') . '...' : $s;
        echo "  $short  [$f]\n";
    }
}

// Top no_autot samples across all areas
$allNoAutot = array();
foreach ($noAutotByArea as $area => $items) {
    foreach ($items as $s => $info) {
        $allNoAutot[$s] = $info;
    }
}
echo "\n=== 语言包有但当前无法翻译（缺aliases，前25条高频）===\n";
$i = 0;
foreach ($allNoAutot as $s => $info) {
    if ($i++ >= 25) { echo "  ...共 " . count($allNoAutot) . " 条\n"; break; }
    echo "  [{$info['key']}] $s => {$info['en']}\n";
}
