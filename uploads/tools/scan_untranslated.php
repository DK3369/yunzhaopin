<?php
/**
 * Scan project for Chinese text not covered by i18n.
 * Usage: php tools/scan_untranslated.php
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

require ROOT . 'app/include/i18n.class.php';

$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$en = include DATA_PATH . 'lang/auto/en_us.php';
$zhByValue = array();
foreach ($zh as $key => $val) {
    $zhByValue[$val] = $key;
}

$i18n = new Yun_I18n(DATA_PATH . 'lang/', 'en_us');

$skipDirs = array('node_modules', 'vendor', '.git', 'data/cache', 'data/upload', 'install');
$skipFiles = array('.min.js', 'vue-router.js', 'echarts', 'jquery', 'layui.all');

$exts = array('htm', 'html', 'vue', 'js', 'php');

function shouldSkip($path, $skipDirs, $skipFiles) {
    foreach ($skipDirs as $d) {
        if (strpos($path, '/' . $d . '/') !== false) return true;
    }
    foreach ($skipFiles as $s) {
        if (strpos($path, $s) !== false) return true;
    }
    return false;
}

function extractChinese($content) {
    $found = array();
    // HTML text nodes pattern (simplified)
    if (preg_match_all('/>([^<>]*[\x{4e00}-\x{9fff}][^<>]*)</u', $content, $m)) {
        foreach ($m[1] as $s) $found[] = trim($s);
    }
    // attributes
    if (preg_match_all('/\b(?:alt|title|placeholder|value|content|label|message|tip|text)\s*=\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/iu', $content, $m)) {
        foreach ($m[1] as $s) $found[] = trim($s);
    }
    // JS strings: '...' or "..."
    if (preg_match_all('/["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m)) {
        foreach ($m[1] as $s) {
            if (strlen($s) < 200) $found[] = trim($s);
        }
    }
    // layer.msg / alert patterns
    if (preg_match_all('/(?:layer\.msg|alert|confirm|yunAdminT|yunT|yun_auto_t)\s*\(\s*["\']([^"\']*[\x{4e00}-\x{9fff}][^"\']*)["\']/u', $content, $m)) {
        foreach ($m[1] as $s) $found[] = trim($s);
    }
    return $found;
}

function isIgnorable($s) {
    if ($s === '' || mb_strlen($s, 'UTF-8') < 2) return true;
    // Smarty/vue variables
    if (preg_match('/^\{.*\}$/u', $s)) return true;
    if (preg_match('/\$[a-zA-Z_]/', $s)) return true;
    if (preg_match('/\{yun:|yun:\}|if\s|\/if|foreach|\/foreach/u', $s)) return true;
    if (preg_match('/^[\d\s\.,;:!?\-+=%#@&*()\[\]{}<>\/\\\\|~`]+$/u', $s)) return true;
    // URLs, paths
    if (preg_match('/^https?:\/\//', $s)) return true;
    if (preg_match('/\.(png|jpg|gif|css|js|php|html)/i', $s)) return true;
    // pure comment markers
    if (preg_match('/^\/\/|^\/\*|^\*/', $s)) return true;
    return false;
}

$results = array(
    'missing' => array(),      // not in lang pack at all
    'no_autot' => array(),     // in lang pack but autoT won't translate (no alias)
    'translated' => 0,
);
$fileHits = array();

$iter = new RecursiveIteratorIterator(new RecursiveDirectoryIterator(ROOT));
foreach ($iter as $file) {
    if (!$file->isFile()) continue;
    $path = $file->getPathname();
    $rel = str_replace(ROOT, '', $path);
    if (shouldSkip($rel, $skipDirs, $skipFiles)) continue;
    $ext = strtolower(pathinfo($path, PATHINFO_EXTENSION));
    if (!in_array($ext, $exts)) continue;
    // skip backend model PHP mostly - focus on user-facing
    if ($ext === 'php' && preg_match('#/(model|include/mysql|api/uc)/#', $rel)) continue;

    $content = @file_get_contents($path);
    if ($content === false || !preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) continue;

    $strings = extractChinese($content);
    foreach ($strings as $s) {
        if (isIgnorable($s)) continue;
        // normalize whitespace
        $s = preg_replace('/\s+/u', ' ', $s);
        $s = trim($s);
        if (isIgnorable($s)) continue;

        $translated = $i18n->autoT($s);
        $inPack = isset($zhByValue[$s]);
        $hasEn = $inPack && isset($en[$zhByValue[$s]]) && $en[$zhByValue[$s]] !== $s;

        if ($translated !== $s && !preg_match('/[\x{4e00}-\x{9fff}]/u', $translated)) {
            $results['translated']++;
            continue;
        }

        if (!$inPack) {
            if (!isset($results['missing'][$s])) {
                $results['missing'][$s] = array();
            }
            if (count($results['missing'][$s]) < 3) {
                $results['missing'][$s][] = $rel;
            }
        } elseif (!$hasEn) {
            if (!isset($results['no_en'][$s])) {
                $results['no_en'][$s] = array('key' => $zhByValue[$s], 'files' => array());
            }
            if (count($results['no_en'][$s]['files']) < 2) {
                $results['no_en'][$s]['files'][] = $rel;
            }
        } else {
            // in pack with EN but autoT fails (aliases deleted)
            if (!isset($results['no_autot'][$s])) {
                $results['no_autot'][$s] = array('key' => $zhByValue[$s], 'en' => $en[$zhByValue[$s]], 'files' => array());
            }
            if (count($results['no_autot'][$s]['files']) < 2) {
                $results['no_autot'][$s]['files'][] = $rel;
            }
        }
    }
}

// Output report
echo "=== i18n Scan Report ===\n\n";
echo "Lang pack entries: " . count($zh) . "\n";
echo "Aliases file: " . (is_file(DATA_PATH . 'lang/auto/aliases.php') ? 'YES' : 'NO (autoT cannot map Chinese→key)') . "\n";
echo "autoT success (sampled): {$results['translated']}\n\n";

echo "--- Missing from lang pack: " . count($results['missing']) . " ---\n";
$i = 0;
foreach ($results['missing'] as $s => $files) {
    if ($i++ >= 40) { echo "... and " . (count($results['missing']) - 40) . " more\n"; break; }
    echo "  [$s] => " . implode(', ', $files) . "\n";
}

if (!empty($results['no_en'])) {
    echo "\n--- In pack but no English: " . count($results['no_en']) . " ---\n";
    $i = 0;
    foreach ($results['no_en'] as $s => $info) {
        if ($i++ >= 15) break;
        echo "  [{$info['key']}] $s\n";
    }
}

echo "\n--- In pack but autoT won't translate (need alias or template key): " . count($results['no_autot']) . " ---\n";
$i = 0;
foreach ($results['no_autot'] as $s => $info) {
    if ($i++ >= 30) { echo "... and " . (count($results['no_autot']) - 30) . " more\n"; break; }
    $short = mb_strlen($s) > 50 ? mb_substr($s, 0, 50, 'UTF-8') . '...' : $s;
    echo "  [{$info['key']}] $short => {$info['en']}\n";
}

// Save full report
$report = array(
    'scanned_at' => date('Y-m-d H:i:s'),
    'missing_count' => count($results['missing']),
    'no_autot_count' => count($results['no_autot']),
    'missing' => $results['missing'],
    'no_autot' => array_map(function($v) { return array('key'=>$v['key'],'en'=>$v['en'],'files'=>$v['files']); }, $results['no_autot']),
);
file_put_contents(ROOT . 'tools/i18n_scan_report.json', json_encode($report, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nFull report: tools/i18n_scan_report.json\n";
