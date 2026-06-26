<?php
/**
 * Migrate auto lang keys from Chinese text to page_english_string format.
 * Usage: php tools/migrate_auto_lang_keys.php [--dry-run]
 */

$dryRun = in_array('--dry-run', $argv ?? [], true);
$root = dirname(__DIR__);
$zhFile = $root . '/data/lang/auto/zh_cn.php';
$enFile = $root . '/data/lang/auto/en_us.php';
$aliasFile = $root . '/data/lang/auto/aliases.php';

$zh = include $zhFile;
$en = include $enFile;

$searchDirs = array(
    $root . '/app/template',
    $root . '/admin',
    $root . '/app/controller',
    $root . '/app/model',
);

$fileContents = array();
$iterator = new RecursiveIteratorIterator(
    new RecursiveDirectoryIterator($root . '/app/template', FilesystemIterator::SKIP_DOTS)
);
foreach ($iterator as $file) {
  /** @var SplFileInfo $file */
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
foreach (array($root . '/admin', $root . '/app/controller', $root . '/app/model') as $dir) {
    if (!is_dir($dir)) {
        continue;
    }
    $it = new RecursiveIteratorIterator(
        new RecursiveDirectoryIterator($dir, FilesystemIterator::SKIP_DOTS)
    );
    foreach ($it as $file) {
        if (!$file->isFile()) {
            continue;
        }
        $ext = strtolower($file->getExtension());
        if (!in_array($ext, array('php', 'js', 'htm', 'html'), true)) {
            continue;
        }
        $path = str_replace('\\', '/', $file->getPathname());
        $fileContents[$path] = file_get_contents($path);
    }
}

function extractPagePrefix($file, $root)
{
    $file = str_replace('\\', '/', $file);
    $rel = str_replace($root . '/', '', $file);

    if (preg_match('#^app/template/admin/(.+)\.(html|htm|js)$#', $rel, $m)) {
        return slugPage('admin_' . str_replace('/', '_', $m[1]));
    }
    if (preg_match('#^app/template/member/([^/]+)/(.+)\.(html|htm)$#', $rel, $m)) {
        return slugPage('member_' . $m[1] . '_' . str_replace('/', '_', $m[2]));
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
        return slugPage($m[1] . '_' . str_replace('.class', '', $m[2]));
    }
    if (preg_match('#^admin/model/([^/]+)\.class\.php$#', $rel, $m)) {
        return slugPage('admin_' . str_replace('.class', '', $m[1]));
    }
    if (preg_match('#^app/model/([^/]+)\.model\.php$#', $rel, $m)) {
        return slugPage('model_' . str_replace('.model', '', $m[1]));
    }

    $base = basename($rel, '.' . pathinfo($rel, PATHINFO_EXTENSION));
    $dir = dirname($rel);
    return slugPage(str_replace('/', '_', $dir . '_' . $base));
}

function slugPage($text)
{
    $text = strtolower($text);
    $text = preg_replace('/\.(html|htm|js|php)$/', '', $text);
    $text = str_replace(array('-', '.', ' '), '_', $text);
    $text = preg_replace('/[^a-z0-9_]/', '', $text);
    $text = preg_replace('/_+/', '_', $text);
    return trim($text, '_');
}

function slugSuffix($english, $chinese)
{
    $english = html_entity_decode(strip_tags($english), ENT_QUOTES, 'UTF-8');
    $english = preg_replace('/\{[^}]+\}/', '', $english);
    $english = preg_replace('/\$[a-zA-Z0-9_\[\]\'\"]+/', '', $english);
    $english = preg_replace('/\b(if|else|endif)\b/i', '', $english);
    $english = preg_replace('/[^a-zA-Z0-9\s]/', ' ', $english);
    $english = preg_replace('/\s+/', ' ', trim($english));
    $words = array_values(array_filter(explode(' ', strtolower($english))));
    $stop = array('the', 'a', 'an', 'to', 'for', 'of', 'in', 'on', 'at', 'is', 'are', 'was', 'were', 'be', 'been', 'and', 'or', 'with', 'from', 'that', 'this', 'it', 'as', 'by', 'can', 'will', 'please', 'your', 'you', 'we', 'our');
    $filtered = array();
    foreach ($words as $w) {
        if (strlen($w) < 2 || in_array($w, $stop, true)) {
            continue;
        }
        $filtered[] = $w;
        if (count($filtered) >= 5) {
            break;
        }
    }
    if (empty($filtered)) {
        $hash = substr(md5($chinese), 0, 8);
        return 'text_' . $hash;
    }
    $suffix = implode('_', $filtered);
    if (strlen($suffix) > 48) {
        $suffix = substr($suffix, 0, 48);
        $suffix = rtrim($suffix, '_');
    }
    return $suffix;
}

function findPageForKey($key, $fileContents, $root)
{
    $hits = array();
    foreach ($fileContents as $path => $content) {
        if ($content !== false && strpos($content, $key) !== false) {
            $hits[$path] = isset($hits[$path]) ? $hits[$path] + 1 : 1;
        }
    }
    if (empty($hits)) {
        return 'common';
    }
    arsort($hits);
    $best = array_key_first($hits);
    return extractPagePrefix($best, $root);
}

function isChineseKey($key)
{
    return (bool) preg_match('/[\x{4e00}-\x{9fff}]/u', $key);
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

$aliases = array();
$newZh = array();
$newEn = array();
$usedKeys = array();
$stats = array('migrated' => 0, 'kept' => 0, 'collisions' => 0);

foreach ($zh as $oldKey => $zhValue) {
  $enValue = isset($en[$oldKey]) ? $en[$oldKey] : $zhValue;

    if (!isChineseKey($oldKey)) {
        $newZh[$oldKey] = $zhValue;
        $newEn[$oldKey] = $enValue;
        $stats['kept']++;
        continue;
    }

    $page = findPageForKey($oldKey, $fileContents, $root);
    $suffix = slugSuffix($enValue, $oldKey);
    $newKey = $page . '_' . $suffix;

    $baseKey = $newKey;
    $i = 2;
    while (isset($usedKeys[$newKey])) {
        $newKey = $baseKey . '_' . $i;
        $i++;
        $stats['collisions']++;
    }
    $usedKeys[$newKey] = true;

    $aliases[$oldKey] = $newKey;
    $newZh[$newKey] = $zhValue;
    $newEn[$newKey] = $enValue;
    $stats['migrated']++;
}

uksort($newZh, 'strcmp');
uksort($newEn, 'strcmp');
ksort($aliases);

echo "Migrated: {$stats['migrated']}\n";
echo "Kept: {$stats['kept']}\n";
echo "Collisions resolved: {$stats['collisions']}\n";
echo "Aliases: " . count($aliases) . "\n";

if ($dryRun) {
    $sample = array_slice($aliases, 0, 15, true);
    foreach ($sample as $old => $new) {
        echo "$old => $new\n";
    }
    exit(0);
}

file_put_contents($zhFile, exportPhpArray($newZh, 'Auto-generated zh_cn messages'));
file_put_contents($enFile, exportPhpArray($newEn, 'Auto-generated en_us messages'));
file_put_contents($aliasFile, exportPhpArray($aliases, 'Chinese source text => new English key aliases'));

echo "Written:\n- $zhFile\n- $enFile\n- $aliasFile\n";
