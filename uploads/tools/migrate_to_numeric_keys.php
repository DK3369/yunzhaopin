<?php
/**
 * 语言包 key 迁移：页面前缀 + 五位编号
 * 例：wap/resume.htm → wap_resume_00001；全站通用 → common_00001
 *
 * 对照 scan report 里的 files 字段定位页面，不用 report 里的 common_XXXXX key。
 *
 * Usage:
 *   php tools/migrate_to_numeric_keys.php [--dry-run]
 */
if (php_sapi_name() !== 'cli' || !isset($argv[0]) || realpath($argv[0]) !== realpath(__FILE__)) {
    return;
}

define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);

function isUserFacingPath($file)
{
    $file = str_replace('\\', '/', $file);
    return (bool) preg_match('#^(app/template/|app/controller/|admin/|api/|member/)#', $file);
}

$reportFile = ROOT . 'tools/i18n_scan_report.json';
$textToFiles = array();
if (is_file($reportFile)) {
    $report = json_decode(file_get_contents($reportFile), true);
    foreach (array('no_autot', 'missing') as $section) {
        if (empty($report[$section]) || !is_array($report[$section])) {
            continue;
        }
        foreach ($report[$section] as $text => $info) {
            $files = array();
            if (isset($info['files']) && is_array($info['files'])) {
                $files = $info['files'];
            } elseif (is_array($info) && isset($info[0])) {
                $files = $info;
            }
            $filtered = array();
            foreach ($files as $file) {
                $file = str_replace('\\', '/', $file);
                if (isUserFacingPath($file)) {
                    $filtered[] = $file;
                }
            }
            if (!empty($filtered)) {
                if (!isset($textToFiles[$text])) {
                    $textToFiles[$text] = $filtered;
                } else {
                    $textToFiles[$text] = array_values(array_unique(array_merge($textToFiles[$text], $filtered)));
                }
            }
        }
    }
}

$zhFile = ROOT . 'data/lang/auto/zh_cn.php';
$enFile = ROOT . 'data/lang/auto/en_us.php';
$aliasFile = ROOT . 'data/lang/auto/aliases.php';
$indexFile = ROOT . 'data/lang/auto/index.json';
$keyMapFile = ROOT . 'tools/key_map.json';

$zh = include $zhFile;
$en = include $enFile;

function isPageNumericKey($key)
{
    if (!is_string($key) || !preg_match('/^[a-z][a-z0-9_]*_[0-9]{5}$/', $key)) {
        return false;
    }
    return substr_count($key, '_') >= 2;
}

function isChineseKey($key)
{
    return is_string($key) && preg_match('/[\x{4e00}-\x{9fff}]/u', $key);
}

function slugPage($text)
{
    $text = strtolower($text);
    $text = preg_replace('/\.(html|htm|js|php|vue)$/', '', $text);
    $text = str_replace(array('-', '.', ' '), '_', $text);
    $text = preg_replace('/[^a-z0-9_]/', '', $text);
    $text = preg_replace('/_+/', '_', $text);
    return trim($text, '_');
}

function extractPagePrefix($file, $root)
{
    $file = str_replace('\\', '/', $file);
    $rel = ltrim(str_replace($root, '', $file), '/');

    if (preg_match('#^app/template/admin/(.+)\.(html|htm|js|vue)$#', $rel, $m)) {
        return slugPage('admin_' . str_replace('/', '_', $m[1]));
    }
    if (preg_match('#^app/template/member/([^/]+)/(.+)\.(html|htm)$#', $rel, $m)) {
        return slugPage('member_' . $m[1] . '_' . str_replace('/', '_', $m[2]));
    }
    if (preg_match('#^app/template/wap/member/([^/]+)/(.+)\.(html|htm)$#', $rel, $m)) {
        return slugPage('wap_' . $m[1] . '_' . str_replace('/', '_', $m[2]));
    }
    if (preg_match('#^app/template/(default|wap)/(.+)\.(html|htm)$#', $rel, $m)) {
        return slugPage($m[1] . '_' . str_replace('/', '_', $m[2]));
    }
    if (preg_match('#^app/template/wap/js/(.+)\.js$#', $rel, $m)) {
        return slugPage('wap_js_' . str_replace('/', '_', $m[1]));
    }
    if (preg_match('#^app/template/admin/js/(.+)\.js$#', $rel, $m)) {
        return slugPage('admin_js_' . str_replace('/', '_', $m[1]));
    }
    if (preg_match('#^app/template/([^/]+)/(.+)\.(html|htm)$#', $rel, $m)) {
        return slugPage($m[1] . '_' . str_replace('/', '_', $m[2]));
    }
    if (preg_match('#^app/controller/([^/]+)/([^/]+)\.class\.php$#', $rel, $m)) {
        return slugPage($m[1] . '_' . preg_replace('/\.class$/', '', $m[2]));
    }
    if (preg_match('#^admin/model/([^/]+)\.class\.php$#', $rel, $m)) {
        return slugPage('admin_' . preg_replace('/\.class$/', '', $m[1]));
    }
    if (preg_match('#^app/model/([^/]+)\.model\.php$#', $rel, $m)) {
        return slugPage('model_' . preg_replace('/\.model$/', '', $m[1]));
    }
    if (preg_match('#^api/([^/]+)/#', $rel, $m)) {
        return slugPage('api_' . $m[1]);
    }

    $base = basename($rel, '.' . pathinfo($rel, PATHINFO_EXTENSION));
    $dir = dirname($rel);
    return slugPage(str_replace('/', '_', $dir . '_' . $base));
}

function nextPageKey(array &$counters, $prefix)
{
    if ($prefix === '' || !preg_match('/^[a-z][a-z0-9_]*$/', $prefix)) {
        $prefix = 'common';
    }
    if (!isset($counters[$prefix])) {
        $counters[$prefix] = 0;
    }
    $counters[$prefix]++;
    return sprintf('%s_%05d', $prefix, $counters[$prefix]);
}

function pathPriority($file)
{
    $file = str_replace('\\', '/', $file);
    if (strpos($file, 'app/template/wap/') === 0) {
        return 1;
    }
    if (strpos($file, 'app/template/default/') === 0) {
        return 2;
    }
    if (strpos($file, 'app/template/member/') === 0) {
        return 3;
    }
    if (strpos($file, 'app/template/company/') === 0) {
        return 4;
    }
    if (strpos($file, 'app/controller/') === 0 || strpos($file, 'api/') === 0) {
        return 5;
    }
    if (strpos($file, 'admin/') === 0) {
        return 6;
    }
    return 9;
}

function filterFilesByPriority(array $files)
{
    if (empty($files)) {
        return $files;
    }
    $best = 99;
    foreach ($files as $file) {
        $best = min($best, pathPriority($file));
    }
    $filtered = array();
    foreach ($files as $file) {
        if (pathPriority($file) === $best) {
            $filtered[] = $file;
        }
    }
    return $filtered;
}

function resolvePagePrefix($text, array $textToFiles, array $fileContents, $root)
{
    $prefixHits = array();
    $files = isset($textToFiles[$text]) ? $textToFiles[$text] : array();

    if (empty($files)) {
        $needle = mb_substr($text, 0, min(32, mb_strlen($text, 'UTF-8')), 'UTF-8');
        foreach ($fileContents as $path => $content) {
            if ($content === false || $needle === '' || strpos($content, $needle) === false) {
                continue;
            }
            $rel = ltrim(str_replace('\\', '/', str_replace($root, '', $path)), '/');
            if (isUserFacingPath($rel)) {
                $files[] = $rel;
            }
        }
    }

    $files = filterFilesByPriority($files);

    foreach ($files as $file) {
        $file = str_replace('\\', '/', $file);
        if (preg_match('#^(install|data/|vendor|node_modules|tools)/#', $file)) {
            continue;
        }
        if (!isUserFacingPath($file)) {
            continue;
        }
        $full = ROOT . ltrim($file, '/');
        $prefix = extractPagePrefix(is_file($full) ? $full : ROOT . $file, ROOT);
        $prefixHits[$prefix] = isset($prefixHits[$prefix]) ? $prefixHits[$prefix] + 1 : 1;
    }

    if (empty($prefixHits)) {
        return 'common';
    }
    arsort($prefixHits);
    $prefixes = array_keys($prefixHits);
    if (count($prefixes) >= 3) {
        return 'common';
    }
    return $prefixes[0];
}

function exportPhpArray($data, $headerComment = '')
{
    $out = "<?php\n\n";
    if ($headerComment !== '') {
        $out .= "// $headerComment\n";
    }
    $out .= "return array (\n";
    foreach ($data as $k => $v) {
        $out .= '  ' . var_export($k, true) . ' => ' . var_export($v, true) . ",\n";
    }
    $out .= ");\n";
    return $out;
}

echo "Indexing source files...\n";
$fileContents = array();
$scanDirs = array(ROOT . 'app/template', ROOT . 'admin', ROOT . 'app/controller', ROOT . 'app/model', ROOT . 'api');
foreach ($scanDirs as $dir) {
    if (!is_dir($dir)) {
        continue;
    }
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($dir, FilesystemIterator::SKIP_DOTS));
    foreach ($it as $file) {
        if (!$file->isFile()) {
            continue;
        }
        $ext = strtolower($file->getExtension());
        if (!in_array($ext, array('htm', 'html', 'php', 'js', 'vue'), true)) {
            continue;
        }
        $path = str_replace('\\', '/', $file->getPathname());
        $fileContents[$path] = file_get_contents($path);
    }
}

$counters = array();
foreach ($zh as $k => $v) {
    if (isPageNumericKey($k) && preg_match('/^(.+)_([0-9]{5})$/', $k, $m)) {
        $counters[$m[1]] = max(isset($counters[$m[1]]) ? $counters[$m[1]] : 0, (int) $m[2]);
    }
}

$newZh = array();
$newEn = array();
$aliases = array();
$keyMap = array();
$index = array();
$valueToKey = array();
$stats = array('kept' => 0, 'mapped' => 0, 'common' => 0, 'collisions' => 0);

foreach ($zh as $oldKey => $zhValue) {
    if (!is_string($zhValue)) {
        continue;
    }
    $enValue = isset($en[$oldKey]) ? $en[$oldKey] : $zhValue;
    $lookupText = isChineseKey($oldKey) ? $oldKey : $zhValue;

    if (isPageNumericKey($oldKey)) {
        $newKey = $oldKey;
        $stats['kept']++;
    } elseif (isset($valueToKey[$zhValue])) {
        $newKey = $valueToKey[$zhValue];
        $stats['mapped']++;
    } else {
        $pagePrefix = resolvePagePrefix($lookupText, $textToFiles, $fileContents, ROOT);
        if ($pagePrefix === 'common') {
            $stats['common']++;
        }
        $newKey = nextPageKey($counters, $pagePrefix);
        $valueToKey[$zhValue] = $newKey;
    }

    if (isset($newZh[$newKey]) && $newZh[$newKey] !== $zhValue) {
        $pagePrefix = 'common';
        $newKey = nextPageKey($counters, $pagePrefix);
        $stats['collisions']++;
    }

    $newZh[$newKey] = $zhValue;
    $newEn[$newKey] = $enValue;

    if ($oldKey !== $newKey) {
        $keyMap[$oldKey] = $newKey;
    }
    if (isChineseKey($oldKey)) {
        $aliases[$oldKey] = $newKey;
    }
    if (is_string($zhValue) && isChineseKey($zhValue) && !isset($aliases[$zhValue])) {
        $aliases[$zhValue] = $newKey;
    }

    if (!isset($index[$newKey])) {
        $module = preg_match('/^(.+)_([0-9]{5})$/', $newKey, $m) ? $m[1] : 'common';
        $index[$newKey] = array(
            'module' => $module,
            'id' => preg_match('/_([0-9]{5})$/', $newKey, $m2) ? (int) $m2[1] : 0,
            'old_key' => isChineseKey($oldKey) ? $oldKey : '',
            'zh' => $zhValue,
            'en' => $enValue,
        );
    }
}

uksort($newZh, 'strcmp');
uksort($newEn, 'strcmp');
ksort($aliases);
ksort($keyMap);
ksort($index);

$prefixStats = array();
foreach ($newZh as $k => $v) {
    if (preg_match('/^(.+)_([0-9]{5})$/', $k, $m)) {
        $prefixStats[$m[1]] = isset($prefixStats[$m[1]]) ? $prefixStats[$m[1]] + 1 : 1;
    }
}
arsort($prefixStats);

echo 'Total: ' . count($newZh) . "\n";
echo 'Kept existing page keys: ' . $stats['kept'] . "\n";
echo 'Deduped by zh value: ' . $stats['mapped'] . "\n";
echo 'Assigned to common (multi-page): ' . $stats['common'] . "\n";
echo 'Value collisions: ' . $stats['collisions'] . "\n";
echo 'Aliases: ' . count($aliases) . "\n";
echo "Top modules:\n";
$i = 0;
foreach ($prefixStats as $p => $c) {
    echo "  $p: $c\n";
    if (++$i >= 12) {
        break;
    }
}

if ($dryRun) {
    echo "\nSamples:\n";
    $n = 0;
    foreach ($keyMap as $old => $new) {
        if (!isChineseKey($old) || mb_strlen($old, 'UTF-8') > 40) {
            continue;
        }
        echo "$old => $new\n";
        if (++$n >= 15) {
            break;
        }
    }
    exit(0);
}

$ts = date('Ymd_His');
copy($zhFile, $zhFile . '.bak.' . $ts);
copy($enFile, $enFile . '.bak.' . $ts);

file_put_contents($zhFile, exportPhpArray($newZh, '页面前缀 + 五位编号'));
file_put_contents($enFile, exportPhpArray($newEn, '页面前缀 + 五位编号'));
file_put_contents($aliasFile, exportPhpArray($aliases, '中文原文 => 页面前缀编号 key'));
file_put_contents($indexFile, json_encode($index, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
file_put_contents($keyMapFile, json_encode($keyMap, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));

echo "\nWritten lang pack + aliases + index.json\n";
echo "Backup: *.bak.$ts\n";
