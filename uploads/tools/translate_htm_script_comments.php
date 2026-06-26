<?php
/**
 * Translate // Chinese comments inside <script> blocks in .htm files.
 * Uses tools/comment_translation_cache.json
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$cacheFile = __DIR__ . '/comment_translation_cache.json';
$cache = is_file($cacheFile) ? json_decode(file_get_contents($cacheFile), true) : array();
if (!is_array($cache)) $cache = array();

function translateCommentBody($body, &$cache, $allowApi = false)
{
    $body = trim($body);
    if ($body === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $body)) return $body;
    $key = md5($body);
    if (isset($cache[$key]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$key])) {
        return preg_replace('/\s+/u', ' ', trim($cache[$key]));
    }
    if ($allowApi) {
        $url = 'https://translate.googleapis.com/translate_a/single?client=gtx&sl=zh-CN&tl=en&dt=t&q=' . rawurlencode(mb_substr($body, 0, 450));
        $ch = curl_init($url);
        curl_setopt_array($ch, array(CURLOPT_RETURNTRANSFER => true, CURLOPT_TIMEOUT => 20));
        $resp = curl_exec($ch);
        curl_close($ch);
        $json = json_decode($resp, true);
        $en = (is_array($json) && isset($json[0][0][0])) ? preg_replace('/\s+/u', ' ', trim($json[0][0][0])) : null;
        if ($en && !preg_match('/[\x{4e00}-\x{9fff}]/u', $en)) {
            $cache[$key] = $en;
            return $en;
        }
    }
    return $body;
}

$buildCache = in_array('--build-cache', $argv ?? array(), true);
$finish = in_array('--finish', $argv ?? array(), true);

$files = 0;
$lines = 0;
$bodies = array();
$base = ROOT . 'app/template';
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
foreach ($it as $f) {
    if (!$f->isFile() || strtolower($f->getExtension()) !== 'htm') continue;
    $content = file_get_contents($f->getPathname());
    if (!preg_match_all('#<script\b[^>]*>([\s\S]*?)</script>#iu', $content, $sm)) continue;
    foreach ($sm[1] as $js) {
        if (preg_match_all('/^(\s*\/\/)(.*)$/mu', $js, $lm)) {
            foreach ($lm[2] as $body) {
                if (preg_match('/[\x{4e00}-\x{9fff}]/u', $body)) $bodies[md5(trim($body))] = trim($body);
            }
        }
    }
}
if ($buildCache || $finish) {
    $pending = 0;
    foreach ($bodies as $k => $body) {
        if (isset($cache[$k]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$k])) continue;
        translateCommentBody($body, $cache, true);
        $pending++;
        if ($pending % 25 === 0) file_put_contents($cacheFile, json_encode($cache, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
        usleep(50000);
    }
    file_put_contents($cacheFile, json_encode($cache, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
    echo "Cached script comment bodies: $pending\n";
    if ($buildCache && !$finish) exit;
}

$files = 0;
$lines = 0;
$base = ROOT . 'app/template';
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
foreach ($it as $f) {
    if (!$f->isFile() || strtolower($f->getExtension()) !== 'htm') continue;
    $path = $f->getPathname();
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) continue;
    $changed = false;
    $new = preg_replace_callback(
        '#<script\b[^>]*>([\s\S]*?)</script>#iu',
        function ($m) use (&$cache, &$lines, &$changed) {
            $js = $m[1];
            $out = preg_replace_callback('/^(\s*\/\/)(.*)$/mu', function ($lm) use (&$cache, &$lines, &$changed) {
                if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $lm[2])) return $lm[0];
                $en = translateCommentBody($lm[2], $cache, false);
                if ($en === trim($lm[2])) return $lm[0];
                $changed = true;
                $lines++;
                return $lm[1] . ' ' . $en;
            }, $js);
            return str_replace($m[1], $out, $m[0]);
        },
        $content
    );
    if (!$changed) continue;
    $files++;
    $rel = str_replace(ROOT, '', $path);
    echo ($dryRun ? '[dry] ' : '') . "$rel ($lines lines)\n";
    if (!$dryRun) file_put_contents($path, $new);
}

file_put_contents($cacheFile, json_encode($cache, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
echo "\nFiles: $files, comment lines: $lines\n";
