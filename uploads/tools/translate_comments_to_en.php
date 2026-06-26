<?php
/**
 * Translate Chinese PHP/JS comments to English (API-only, safe).
 *
 *   php tools/translate_comments_to_en.php --build-cache   # build API cache
 *   php tools/translate_comments_to_en.php                   # apply cache
 *   php tools/translate_comments_to_en.php --finish          # build cache + apply
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$buildCache = in_array('--build-cache', $argv ?? array(), true);
$finish = in_array('--finish', $argv ?? array(), true);
$phpOnly = in_array('--php-only', $argv ?? array(), true);
$jsOnly = in_array('--js-only', $argv ?? array(), true);

$cacheFile = __DIR__ . '/comment_translation_cache.json';
$cache = is_file($cacheFile) ? json_decode(file_get_contents($cacheFile), true) : array();
if (!is_array($cache)) $cache = array();

$skip = '/vendor|PHPExcel|install\/|data\/lang|tools\/i18n_scan_report|dbbackup|PHPWord|tcpdf|ueditor|lib_splitword|umeditor|wangeditor/i';

function cacheKey($body) { return md5(preg_replace('/\s+$/s', '', $body)); }

function googleTranslateOne($text)
{
    $url = 'https://translate.googleapis.com/translate_a/single?client=gtx&sl=zh-CN&tl=en&dt=t&q=' . rawurlencode(mb_substr(trim($text), 0, 450));
    $ch = curl_init($url);
    curl_setopt_array($ch, array(CURLOPT_RETURNTRANSFER => true, CURLOPT_TIMEOUT => 25, CURLOPT_CONNECTTIMEOUT => 10));
    $body = curl_exec($ch);
    curl_close($ch);
    if (!$body) return null;
    $json = json_decode($body, true);
    return (is_array($json) && isset($json[0][0][0])) ? $json[0][0][0] : null;
}

function googleTranslateBatch(array $texts)
{
    if (empty($texts)) return array();
    $mh = curl_multi_init();
    $handles = array();
    foreach ($texts as $i => $text) {
        $url = 'https://translate.googleapis.com/translate_a/single?client=gtx&sl=zh-CN&tl=en&dt=t&q=' . rawurlencode(mb_substr(trim($text), 0, 450));
        $ch = curl_init($url);
        curl_setopt_array($ch, array(CURLOPT_RETURNTRANSFER => true, CURLOPT_TIMEOUT => 25, CURLOPT_CONNECTTIMEOUT => 10));
        curl_multi_add_handle($mh, $ch);
        $handles[$i] = $ch;
    }
    $running = null;
    do { curl_multi_exec($mh, $running); curl_multi_select($mh, 1.0); } while ($running > 0);
    $out = array();
    foreach ($handles as $i => $ch) {
        $body = curl_multi_getcontent($ch);
        $json = json_decode($body, true);
        $out[$i] = (is_array($json) && isset($json[0][0][0])) ? $json[0][0][0] : null;
        curl_multi_remove_handle($mh, $ch);
        curl_close($ch);
    }
    curl_multi_close($mh);
    usleep(60000);
    return $out;
}

function saveCache($cacheFile, $cache)
{
    file_put_contents($cacheFile, json_encode($cache, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
}

function splitCommentContent($content)
{
    $suffix = '';
    if (preg_match('/(\s+)$/s', $content, $m)) {
        $suffix = $m[1];
        $content = substr($content, 0, -strlen($suffix));
    }
    return array($content, $suffix);
}

/** For // body: translate only the comment label, not code merged on the same line. */
function splitLineCommentBody($content)
{
    if (preg_match('~^(.+?)(\s+)((?:\$[a-zA-Z_][\w]*\s*(?:=|\-\>|::)|if\s*\(|foreach\s*\(|while\s*\(|for\s*\(|switch\s*\(|function\s+\w|return\s|else\b).*)$~s', $content, $m)) {
        $label = $m[1];
        if (!preg_match('/[;{}=]/', $label) && mb_strlen($label) <= 80) {
            return array($label, $m[2] . $m[3]);
        }
    }
    return splitCommentContent($content);
}

function translateBody($body, &$cache, $allowApi, $singleLine = false)
{
    $body = $body;
    if ($body === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $body)) return $body;
    $key = cacheKey($body);
    if (isset($cache[$key]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$key])) {
        $out = $cache[$key];
        return $singleLine ? preg_replace('/\s+/u', ' ', trim($out)) : $out;
    }
    if ($allowApi) {
        $out = googleTranslateOne($body);
        if ($out && !preg_match('/[\x{4e00}-\x{9fff}]/u', $out)) {
            if ($singleLine) $out = preg_replace('/\s+/u', ' ', trim($out));
            $cache[$key] = $out;
            return $out;
        }
    }
    return $body;
}

function translateCommentToken($raw, &$cache, $allowApi)
{
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $raw)) return $raw;

    if (preg_match('~^(/\*\*?)([\s\S]*?)(\*/)?$~', $raw, $m)) {
        $open = $m[1]; $body = $m[2]; $close = $m[3] ?? '';
        $lines = preg_split('/\r\n|\r|\n/', $body);
        $newLines = array();
        foreach ($lines as $line) {
            if (preg_match('/^(\s*\*?\s*)(.*)$/u', $line, $lm)) {
                $content = $lm[2];
                if (preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) {
                    $content = translateBody($content, $cache, $allowApi);
                }
                $newLines[] = $lm[1] . $content;
            } else {
                $newLines[] = $line;
            }
        }
        return $open . implode("\n", $newLines) . $close;
    }

    if (preg_match('~^//(.*)$~s', $raw, $m)) {
        list($content, $suffix) = splitLineCommentBody($m[1]);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return $raw;
        return '//' . translateBody($content, $cache, $allowApi, true) . $suffix;
    }

    if (preg_match('~^#(.*)$~s', $raw, $m)) {
        list($content, $suffix) = splitCommentContent($m[1]);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return $raw;
        return '#' . translateBody($content, $cache, $allowApi, true) . $suffix;
    }

    return translateBody($raw, $cache, $allowApi);
}

function collectPhpComments($path, &$bodies)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return;
    foreach (token_get_all($content) as $token) {
        if (!is_array($token)) continue;
        if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) continue;
        extractBodiesFromRaw($token[1], $bodies);
    }
}

function collectJsComments($path, &$bodies)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return;
    if (preg_match_all('~/\*[\s\S]*?\*/~u', $content, $m)) {
        foreach ($m[0] as $c) extractBodiesFromRaw($c, $bodies);
    }
    if (preg_match_all('~//[^\n\r]*~u', $content, $m)) {
        foreach ($m[0] as $c) extractBodiesFromRaw($c, $bodies);
    }
}

function extractBodiesFromRaw($raw, &$bodies)
{
    if (preg_match('~^/\*[\s\S]*\*/$~', $raw)) {
        if (preg_match('~^/\*\*?([\s\S]*?)\*/$~', $raw, $m)) {
            foreach (preg_split('/\r\n|\r|\n/', $m[1]) as $line) {
                if (preg_match('/^(\s*\*?\s*)(.*)$/u', $line, $lm) && preg_match('/[\x{4e00}-\x{9fff}]/u', $lm[2])) {
                    $bodies[cacheKey($lm[2])] = $lm[2];
                }
            }
            return;
        }
    }
    if (preg_match('~^//(.*)$~s', $raw, $m) && preg_match('/[\x{4e00}-\x{9fff}]/u', $m[1])) {
        list($content,) = splitLineCommentBody($m[1]);
        if (preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) {
            $bodies[cacheKey($content)] = $content;
        }
    }
}

function transformPhpFile($path, &$cache, $allowApi, &$stats)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return $content;
    $tokens = token_get_all($content);
    $changed = false;
    foreach ($tokens as &$token) {
        if (!is_array($token)) continue;
        if (!in_array($token[0], array(T_COMMENT, T_DOC_COMMENT), true)) continue;
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $token[1])) continue;
        $new = translateCommentToken($token[1], $cache, $allowApi);
        if ($new !== $token[1]) {
            $token[1] = $new;
            $changed = true;
            $stats['comments']++;
            if (preg_match('/[\x{4e00}-\x{9fff}]/u', $new)) $stats['remaining']++;
        }
    }
    unset($token);
    if (!$changed) return $content;
    $stats['files']++;
    $out = '';
    foreach ($tokens as $token) $out .= is_array($token) ? $token[1] : $token;
    return $out;
}

function transformJsFile($path, &$cache, $allowApi, &$stats)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return $content;
    $replace = function ($m) use (&$cache, $allowApi, &$stats) {
        $new = translateCommentToken($m[0], $cache, $allowApi);
        if ($new !== $m[0]) {
            $stats['comments']++;
            if (preg_match('/[\x{4e00}-\x{9fff}]/u', $new)) $stats['remaining']++;
        }
        return $new;
    };
    $new = preg_replace_callback('~/\*[\s\S]*?\*/~u', $replace, $content);
    $new = preg_replace_callback('~//[^\n\r]*~u', $replace, $new);
    if ($new !== $content) $stats['files']++;
    return $new;
}

function iterFiles($phpOnly, $jsOnly)
{
    global $skip;
    foreach (array('app', 'admin', 'member', 'api', 'wap') as $dir) {
        $path = ROOT . $dir;
        if (!is_dir($path)) continue;
        $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
        foreach ($it as $f) {
            if (!$f->isFile()) continue;
            $rel = str_replace(ROOT, '', $f->getPathname());
            if (preg_match($skip, $rel)) continue;
            $ext = $f->getExtension();
            if (!$jsOnly && $ext === 'php') yield $f->getPathname();
            if (!$phpOnly && $ext === 'js') yield $f->getPathname();
        }
    }
}

if ($buildCache || $finish) {
    $bodies = array();
    foreach (iterFiles($phpOnly, $jsOnly) as $file) {
        if (substr($file, -3) === '.js') collectJsComments($file, $bodies);
        else collectPhpComments($file, $bodies);
    }
    $pending = array();
    foreach ($bodies as $key => $body) {
        if (isset($cache[$key]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$key])) continue;
        $pending[$key] = $body;
    }
    echo 'Unique bodies: ' . count($bodies) . ', need API: ' . count($pending) . "\n";
    $keys = array_keys($pending);
    for ($i = 0; $i < count($keys); $i += 25) {
        $sliceKeys = array_slice($keys, $i, 25);
        $sliceTexts = array();
        foreach ($sliceKeys as $k) $sliceTexts[] = $pending[$k];
        $translated = googleTranslateBatch($sliceTexts);
        foreach ($sliceKeys as $j => $k) {
            $t = $translated[$j] ?? null;
            if ($t && !preg_match('/[\x{4e00}-\x{9fff}]/u', $t)) $cache[$k] = $t;
        }
        if ((($i / 25) + 1) % 20 === 0 || $i + 25 >= count($keys)) {
            saveCache($cacheFile, $cache);
            echo 'Cached ' . min($i + 25, count($keys)) . ' / ' . count($pending) . "\n";
        }
    }
    saveCache($cacheFile, $cache);
    echo 'Cache entries: ' . count($cache) . "\n";
    if ($buildCache && !$finish) exit;
}

$stats = array('files' => 0, 'comments' => 0, 'remaining' => 0);
foreach (iterFiles($phpOnly, $jsOnly) as $file) {
    $rel = str_replace(ROOT, '', $file);
    $orig = file_get_contents($file);
    $new = (substr($file, -3) === '.js')
        ? transformJsFile($file, $cache, false, $stats)
        : transformPhpFile($file, $cache, false, $stats);
    if ($new !== $orig) {
        echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        if (!$dryRun) file_put_contents($file, $new);
    }
}
saveCache($cacheFile, $cache);
echo "\nFiles: {$stats['files']}, comments: {$stats['comments']}, still Chinese: {$stats['remaining']}\n";

// Repair inline comment corruption from translation (multi-line fix disabled: too many false positives)
passthru('php ' . escapeshellarg(__DIR__ . '/fix_corrupted_inline_comments.php'), $fix1);
