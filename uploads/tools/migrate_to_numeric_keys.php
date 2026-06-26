<?php
/**
 * 语言包 key：短模块前缀 + 五位编号（昨天规则）
 * 例：common_00001、wap_00421、admin_user_company_00012
 * 模块名最多 3 段（如 admin_user_company），禁止整页路径。
 *
 * Usage:
 *   php tools/migrate_to_numeric_keys.php [--dry-run] [--skip-templates]
 */
if (php_sapi_name() !== 'cli' || !isset($argv[0]) || realpath($argv[0]) !== realpath(__FILE__)) {
    return;
}

define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$skipTemplates = in_array('--skip-templates', $argv ?? array(), true);

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

function modulePartCount($prefix)
{
    return count(explode('_', $prefix));
}

function isAllowedModule($prefix)
{
    static $allowed = array(
        'common', 'wap', 'wap_com', 'wap_user', 'wap_js',
        'member_com', 'member_user',
        'admin', 'admin_user', 'admin_user_company', 'admin_user_weipin', 'admin_user_partuser',
        'admin_system', 'admin_tool', 'admin_yunying', 'admin_index',
        'default', 'company', 'resume', 'ask', 'ajax',
    );
    if (in_array($prefix, $allowed, true)) {
        return true;
    }
    if (preg_match('/^(model|api)_[a-z0-9_]+$/', $prefix) && modulePartCount($prefix) <= 2) {
        return true;
    }
    if (preg_match('/^admin_[a-z0-9_]+$/', $prefix) && modulePartCount($prefix) <= 3) {
        return in_array($prefix, $allowed, true);
    }
    return false;
}

function isPageNumericKey($key)
{
    if (!is_string($key) || !preg_match('/^([a-z][a-z0-9_]*)_([0-9]{5})$/', $key, $m)) {
        return false;
    }
    return isAllowedModule($m[1]);
}

function isChineseKey($key)
{
    return is_string($key) && preg_match('/[\x{4e00}-\x{9fff}]/u', $key);
}

/** 文件路径 → 短模块名（最多 3 段） */
function shortModuleFromPath($file, $root)
{
    $rel = ltrim(str_replace('\\', '/', str_replace($root, '', $file)), '/');

    if (preg_match('#^app/template/admin/user/company/#', $rel)) {
        return 'admin_user_company';
    }
    if (preg_match('#^app/template/admin/user/partuser/#', $rel)) {
        return 'admin_user_partuser';
    }
    if (preg_match('#^app/template/admin/user/weipin/#', $rel)) {
        return 'admin_user_weipin';
    }
    if (preg_match('#^app/template/admin/user/#', $rel)) {
        return 'admin_user';
    }
    if (preg_match('#^app/template/admin/system/#', $rel)) {
        return 'admin_system';
    }
    if (preg_match('#^app/template/admin/tool/#', $rel)) {
        return 'admin_tool';
    }
    if (preg_match('#^app/template/admin/yunying/#', $rel)) {
        return 'admin_yunying';
    }
    if (preg_match('#^app/template/admin/#', $rel)) {
        return 'admin';
    }
    if (preg_match('#^app/template/wap/member/com/#', $rel)) {
        return 'wap_com';
    }
    if (preg_match('#^app/template/wap/member/user/#', $rel)) {
        return 'wap_user';
    }
    if (preg_match('#^app/template/wap/js/#', $rel)) {
        return 'wap_js';
    }
    if (preg_match('#^app/template/wap/#', $rel)) {
        return 'wap';
    }
    if (preg_match('#^app/template/member/com/#', $rel)) {
        return 'member_com';
    }
    if (preg_match('#^app/template/member/user/#', $rel)) {
        return 'member_user';
    }
    if (preg_match('#^app/template/default/#', $rel)) {
        return 'default';
    }
    if (preg_match('#^app/template/company/#', $rel)) {
        return 'company';
    }
    if (preg_match('#^app/template/resume/#', $rel)) {
        return 'resume';
    }
    if (preg_match('#^app/template/ask/#', $rel)) {
        return 'ask';
    }
    if (preg_match('#^app/controller/wap/#', $rel)) {
        return 'wap';
    }
    if (preg_match('#^app/controller/ajax/#', $rel)) {
        return 'ajax';
    }
    if (preg_match('#^app/controller/([^/]+)/#', $rel, $m)) {
        return $m[1];
    }
    if (preg_match('#^admin/model/([^/]+)#', $rel, $m)) {
        return 'admin_' . $m[1];
    }
    if (preg_match('#^app/model/([^/]+)#', $rel, $m)) {
        return 'model_' . $m[1];
    }
    if (preg_match('#^api/wxapp/#', $rel)) {
        return 'api_wxapp';
    }
    if (preg_match('#^api/([^/]+)#', $rel, $m)) {
        return 'api_' . $m[1];
    }

    return 'common';
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
        return 0;
    }
    if (strpos($file, 'app/template/default/') === 0) {
        return 1;
    }
    if (strpos($file, 'app/template/member/') === 0) {
        return 2;
    }
    if (strpos($file, 'app/template/company/') === 0) {
        return 3;
    }
    if (strpos($file, 'app/controller/') === 0 || strpos($file, 'api/') === 0) {
        return 4;
    }
    if (strpos($file, 'app/template/admin/') === 0 || strpos($file, 'admin/') === 0) {
        return 10;
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
        $prefix = shortModuleFromPath(is_file($full) ? $full : ROOT . $file, ROOT);
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

function remapKeysInTemplates(array $keyMap)
{
    if (empty($keyMap)) {
        return 0;
    }
    uksort($keyMap, function ($a, $b) {
        return strlen($b) - strlen($a);
    });
    $dirs = array(ROOT . 'app/template', ROOT . 'app/controller', ROOT . 'admin', ROOT . 'api');
    $changed = 0;
    foreach ($dirs as $dir) {
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
            $path = $file->getPathname();
            $content = file_get_contents($path);
            $orig = $content;
            foreach ($keyMap as $old => $new) {
                if ($old === $new) {
                    continue;
                }
                $content = str_replace("key='" . $old . "'", "key='" . $new . "'", $content);
                $content = str_replace('key="' . $old . '"', 'key="' . $new . '"', $content);
                $content = str_replace("yun_at('" . $old . "')", "yun_at('" . $new . "')", $content);
                $content = str_replace('yun_at("' . $old . '")', 'yun_at("' . $new . '")', $content);
            }
            if ($content !== $orig) {
                file_put_contents($path, $content);
                $changed++;
            }
        }
    }
    return $changed;
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

if (!$skipTemplates && !empty($keyMap)) {
    $n = remapKeysInTemplates($keyMap);
    echo "Remapped keys in $n template/controller files\n";
}

$maxLen = 0;
foreach ($newZh as $k => $v) {
    $maxLen = max($maxLen, strlen($k));
}
echo "Max key length: $maxLen\n";
