<?php
/**
 * Unified i18n status gate: WAP scan, hardcoded PHP, template gaps, lang pack alignment.
 * Usage: php tools/scan_i18n_status.php [--save-baseline]
 */
define('ROOT', dirname(__DIR__) . '/');

$saveBaseline = in_array('--save-baseline', $argv ?? array(), true);

function runCapture($script)
{
    $path = ROOT . 'tools/' . $script;
    if (!is_file($path)) {
        return '';
    }
    return shell_exec('php ' . escapeshellarg($path) . ' 2>&1');
}

function parseWapActionable($out)
{
    if (preg_match('/Actionable \(ui\+enum\+config\):\s*(\d+)/', $out, $m)) {
        return (int) $m[1];
    }
    return null;
}

function parseHardcoded($out)
{
    $r = array('total' => 0, 'hardcoded' => 0, 'missing_pack' => 0, 'echo' => 0);
    if (preg_match('/Total assignments:\s*(\d+)/', $out, $m)) {
        $r['total'] = (int) $m[1];
    }
    if (preg_match('/In pack but NOT yun_auto_t:\s*(\d+)/', $out, $m)) {
        $r['hardcoded'] = (int) $m[1];
    }
    if (preg_match('/NOT in pack:\s*(\d+)/', $out, $m)) {
        $r['missing_pack'] = (int) $m[1];
    }
    if (preg_match('/echo\/die:\s*(\d+)/', $out, $m)) {
        $r['echo'] = (int) $m[1];
    }
    return $r;
}

function parseTemplateMissing($out)
{
    if (preg_match('/语言包完全缺失:\s*(\d+)\s*条/u', $out, $m)) {
        return (int) $m[1];
    }
    return null;
}

function langPackStats()
{
    $zh = include ROOT . 'data/lang/auto/zh_cn.php';
    $en = include ROOT . 'data/lang/auto/en_us.php';
    $zk = array_keys($zh);
    $ek = array_keys($en);
    $sameOrEmpty = 0;
    $enHasZh = 0;
    $keepSame = array('APPKEY', 'APPSECRET', 'XML', 'General Worker Resumes', 'Registration Settings',
        '0 means unlimited', 'Store Recruitment', 'Membership Level', 'Personal Tags', 'Apply Resume', 'Post Resume');
    $sameNeedFix = 0;
    foreach ($zh as $k => $v) {
        if (!isset($en[$k])) {
            continue;
        }
        if ($en[$k] === '' || $en[$k] === $v) {
            $sameOrEmpty++;
            if (!in_array($v, $keepSame, true) && preg_match('/[\x{4e00}-\x{9fff}]/u', $v)) {
                $sameNeedFix++;
            }
        }
        if (preg_match('/[\x{4e00}-\x{9fff}]/u', $en[$k])) {
            $enHasZh++;
        }
    }
    return array(
        'zh_keys' => count($zk),
        'en_keys' => count($ek),
        'only_zh' => count(array_diff($zk, $ek)),
        'only_en' => count(array_diff($ek, $zk)),
        'en_same_or_empty' => $sameOrEmpty,
        'en_same_zh_need_fix' => $sameNeedFix,
        'en_contains_chinese' => $enHasZh,
    );
}

function hardcodedByDir()
{
    $file = ROOT . 'tools/hardcoded_php.json';
    if (!is_file($file)) {
        return array();
    }
    $j = json_decode(file_get_contents($file), true);
    $by = array();
    foreach (array('hardcoded', 'missing_pack', 'echo') as $type) {
        foreach ($j[$type] ?? array() as $s => $rel) {
            $parts = explode('/', $rel);
            $bucket = $parts[0] . (isset($parts[1]) ? '/' . $parts[1] : '');
            if (!isset($by[$bucket])) {
                $by[$bucket] = array('hardcoded' => 0, 'missing_pack' => 0, 'echo' => 0);
            }
            $by[$bucket][$type]++;
        }
    }
    arsort($by);
    return $by;
}

$wapOut = runCapture('scan_wap_zero_zh.php');
$hardOut = runCapture('scan_hardcoded_php.php');
$tplOut = runCapture('scan_untranslated_clean.php');
$pack = langPackStats();
$byDir = hardcodedByDir();

$status = array(
    'generated' => date('c'),
    'wap_actionable' => parseWapActionable($wapOut),
    'hardcoded_php' => parseHardcoded($hardOut),
    'hardcoded_by_dir' => $byDir,
    'template_missing_pack' => parseTemplateMissing($tplOut),
    'lang_pack' => $pack,
);

echo "=== i18n Status Gate ===\n";
echo "WAP actionable (ui+enum+config): " . ($status['wap_actionable'] ?? '?') . "\n";
echo "PHP hardcoded (in pack, no wrap): " . $status['hardcoded_php']['hardcoded'] . "\n";
echo "PHP missing lang key: " . $status['hardcoded_php']['missing_pack'] . "\n";
echo "PHP echo/die Chinese: " . $status['hardcoded_php']['echo'] . "\n";
echo "Template missing pack: " . ($status['template_missing_pack'] ?? '?') . "\n";
echo "Lang zh/en keys: {$pack['zh_keys']} / {$pack['en_keys']}\n";
echo "en same as zh or empty: {$pack['en_same_or_empty']} (need fix: {$pack['en_same_zh_need_fix']})\n";
echo "en contains Chinese chars: {$pack['en_contains_chinese']}\n";
echo "\nHardcoded by directory:\n";
foreach ($byDir as $dir => $counts) {
    $sum = $counts['hardcoded'] + $counts['missing_pack'] + $counts['echo'];
    if ($sum > 0) {
        echo "  $dir: hardcoded={$counts['hardcoded']} missing={$counts['missing_pack']} echo={$counts['echo']}\n";
    }
}

$outFile = ROOT . 'tools/i18n_status.json';
file_put_contents($outFile, json_encode($status, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nReport: $outFile\n";

if ($saveBaseline) {
    copy($outFile, ROOT . 'tools/i18n_baseline.json');
    echo "Baseline saved: tools/i18n_baseline.json\n";
}
