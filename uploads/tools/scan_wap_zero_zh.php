<?php
/**
 * Scan WAP zero-Chinese scope: app/template/wap, api/wxapp, app/controller/wap, wap/member, wap/js/vant
 * Output: tools/wap_zero_zh_report.json
 */
define('ROOT', dirname(__DIR__) . '/');

$vendorSkip = '#(/js/mui/|/js/umeditor/|/mobiscroll/|echarts\.min\.js|vant\.min\.js|vant/lib/|\.min\.js|/vendor/)#i';
$enumFile = 'app/template/wap/publichtm/wap_api_enum.htm';

$dirs = array(
    'app/template/wap' => array('htm', 'html', 'js', 'css'),
    'api/wxapp' => array('php'),
    'app/controller/wap' => array('php'),
    'wap/member' => array('php'),
    'wap/js/vant' => array('js'),
);

function classifyHit($line, $file, $enumFile) {
    $t = trim($line);
    if (strpos($file, $enumFile) !== false) {
        return 'enum_hub';
    }
    if (preg_match('/^\s*\/\//', $t) || preg_match('/^\s*\/\*/', $t) || preg_match('/^\s+\*[\s\/@]/', $line) || preg_match('/<!--/', $t)) {
        return 'comment';
    }
    if (preg_match('/微软雅黑|Microsoft YaHei/i', $t)) {
        return 'font';
    }
    if (preg_match('/zpdataTitle\s*\(|zpdataUnit\s*\(/', $t)) {
        return 'i18n_wrapped';
    }
    if (preg_match('/\/\/.*[\x{4e00}-\x{9fff}]/u', $t) && !preg_match('/yun_auto_t|WapDbEnum/', $t)) {
        return 'comment';
    }
    if (preg_match('/code_web|strstr\s*\(\s*\$config\.code_web|strpos\s*\(\s*\$config\.code_web/i', $t)) {
        return 'config';
    }
    if (preg_match('/yun_auto_t\s*\(|yun_t\s*\(|yun_at\s*\(|yun:}t\s+key=|WAP_API_ENUM|WAP_CODE_WEB|WAP_JS_I18N|WAP_PUBLIC_I18N|PICKER_I18N|CMC_I18N|PRESENT_API_VALUE|WapDbEnum/i', $t)) {
        return 'i18n_wrapped';
    }
    if (preg_match("/yun_auto_t\s*\(\s*'[^']*[\x{4e00}-\x{9fff}]/u", $line)) {
        return 'i18n_wrapped';
    }
    if (preg_match("/(===|==|!==|!=)\s*['\"][^'\"]*[\x{4e00}-\x{9fff}]/u", $t)
        || preg_match("/['\"][^'\"]*[\x{4e00}-\x{9fff}][^'\"]*['\"]\s*(===|==)/u", $t)
        || preg_match("/value:\s*['\"][^'\"]*[\x{4e00}-\x{9fff}]/u", $t)
        || preg_match("/RegExp\s*\(\s*['\"][^'\"]*[\x{4e00}-\x{9fff}]/u", $t)) {
        return 'enum';
    }
    if (preg_match('/data\/lang\//', $file)) {
        return 'langpack';
    }
    return 'ui';
}

$report = array(
    'generated' => date('c'),
    'summary' => array(),
    'files' => array(),
    'hits' => array(),
);

foreach ($dirs as $base => $exts) {
    $path = ROOT . $base;
    if (!is_dir($path)) {
        continue;
    }
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile()) {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($vendorSkip, $rel)) {
            continue;
        }
        $ext = strtolower($f->getExtension());
        if (!in_array($ext, $exts, true)) {
            continue;
        }
        $lines = file($f->getPathname());
        if (!$lines) {
            continue;
        }
        $fileHits = array();
        foreach ($lines as $num => $line) {
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $line)) {
                continue;
            }
            $cat = classifyHit($line, $rel, $enumFile);
            $fileHits[] = array(
                'line' => $num + 1,
                'category' => $cat,
                'text' => rtrim($line),
            );
        }
        if ($fileHits) {
            $report['files'][$rel] = count($fileHits);
            foreach ($fileHits as $h) {
                $report['hits'][] = array_merge(array('file' => $rel), $h);
                $report['summary'][$h['category']] = ($report['summary'][$h['category']] ?? 0) + 1;
            }
        }
    }
}

$out = ROOT . 'tools/wap_zero_zh_report.json';
file_put_contents($out, json_encode($report, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));

echo "=== WAP Zero-ZH Scan ===\n";
echo "Files with Chinese: " . count($report['files']) . "\n";
foreach ($report['summary'] as $k => $v) {
    echo "  $k: $v\n";
}
$ui = ($report['summary']['ui'] ?? 0) + ($report['summary']['enum'] ?? 0) + ($report['summary']['config'] ?? 0);
echo "Actionable (ui+enum+config): $ui\n";
echo "Report: $out\n";
