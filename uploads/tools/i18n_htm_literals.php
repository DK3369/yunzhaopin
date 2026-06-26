<?php
/**
 * Replace hardcoded Chinese in .htm templates with {yun:}t key="..."{/yun}
 * using existing language packs (data/lang/auto + data/lang/zh_cn.php).
 *
 * Usage:
 *   php tools/i18n_htm_literals.php [--dry-run] [--file=path]
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$singleFile = '';
foreach ($argv ?? array() as $arg) {
    if (preg_match('/^--file=(.+)$/', $arg, $m)) $singleFile = $m[1];
}

$zhAuto = include ROOT . 'data/lang/auto/zh_cn.php';
$valueToKeys = array();
foreach ($zhAuto as $key => $value) {
    if (!is_string($value) || $value === '' || $key === '_meta') continue;
    if (!isset($valueToKeys[$value])) $valueToKeys[$value] = array();
    $valueToKeys[$value][] = array('type' => 'auto', 'key' => $key);
}

function flattenStructured($arr, $prefix, &$out)
{
    foreach ($arr as $k => $v) {
        if ($k === '_meta') continue;
        $full = $prefix === '' ? $k : $prefix . '.' . $k;
        if (is_array($v)) {
            flattenStructured($v, $full, $out);
        } elseif (is_string($v) && $v !== '') {
            if (!isset($out[$v])) $out[$v] = array();
            $out[$v][] = array('type' => 'struct', 'key' => $full);
        }
    }
}
$zhStruct = include ROOT . 'data/lang/zh_cn.php';
flattenStructured($zhStruct, '', $valueToKeys);

$commentCache = is_file(__DIR__ . '/comment_translation_cache.json')
    ? json_decode(file_get_contents(__DIR__ . '/comment_translation_cache.json'), true) : array();

function pathPrefixHints($rel)
{
    $hints = array();
    if (preg_match('#^app/template/admin/user/company/#', $rel)) {
        $hints[] = 'admin_user_company_';
    }
    if (preg_match('#^app/template/wap/member/com/#', $rel)) {
        $hints[] = 'wap_com_';
    } elseif (preg_match('#^app/template/wap/member/user/#', $rel)) {
        $hints[] = 'wap_user_';
    } elseif (preg_match('#^app/template/wap/#', $rel)) {
        $hints[] = 'wap_';
    } elseif (preg_match('#^app/template/member/com/#', $rel)) {
        $hints[] = 'member_com_';
    } elseif (preg_match('#^app/template/member/user/#', $rel)) {
        $hints[] = 'member_user_';
    } elseif (preg_match('#^app/template/admin/#', $rel)) {
        $hints[] = 'admin_';
    }
    return array_merge($hints, array('common_', 'default_', 'ajax_', 'model_'));
}

function pickKey($text, $entries, $rel)
{
    if (count($entries) === 1) return $entries[0]['key'];
    foreach ($entries as $e) {
        if ($e['type'] === 'struct') return $e['key'];
    }
    foreach (pathPrefixHints($rel) as $prefix) {
        foreach ($entries as $e) {
            if ($e['type'] === 'auto' && strpos($e['key'], rtrim($prefix, '.')) === 0) return $e['key'];
        }
    }
    return $entries[0]['key'];
}

function lookupKey($text, $rel)
{
    global $valueToKeys;
    $text = html_entity_decode($text, ENT_QUOTES, 'UTF-8');
    $candidates = array($text, trim($text));
    $trimmed = trim($text);
    $normalized = preg_replace('/\s+/u', ' ', $trimmed);
    if ($normalized !== $trimmed) $candidates[] = $normalized;
    $stripped = preg_replace('/[：:，,。！!？?；;\x{3002}\x{FF01}\x{FF0C}\x{FF1A}\x{FF1F}\x{FF1B}]+$/u', '', $trimmed);
    if ($stripped !== $trimmed) $candidates[] = $stripped;
    $candidates = array_unique($candidates);
    foreach ($candidates as $c) {
        if ($c === '') continue;
        if (isset($valueToKeys[$c])) {
            return pickKey($c, $valueToKeys[$c], $rel);
        }
    }
    return null;
}

function tTag($key)
{
    return "{yun:}t key='" . $key . "'{/yun}";
}

function jsQuoteWrap($quote, $content)
{
    if (strpos($content, "{yun:}t key='") !== false && $quote === "'") {
        return '"' . $content . '"';
    }
    return $quote . $content . $quote;
}

function isAlreadyI18n($s)
{
    return strpos($s, '{yun:}t ') !== false;
}

function replaceChineseRun($text, $rel, &$stats)
{
    if ($text === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $text) || isAlreadyI18n($text)) {
        return $text;
    }
    if (preg_match('/^(\s*)(.*?)(\s*)$/su', $text, $wm) && strpos($wm[2], '{yun:}') === false && $wm[2] !== '') {
        $key = lookupKey($wm[2], $rel);
        if ($key) {
            $stats['hit']++;
            return $wm[1] . tTag($key) . $wm[3];
        }
    }
    return preg_replace_callback('/[\x{4e00}-\x{9fff}]+/u', function ($m) use ($rel, &$stats) {
        $key = lookupKey($m[0], $rel);
        if (!$key) {
            $stats['miss']++;
            return $m[0];
        }
        $stats['hit']++;
        return tTag($key);
    }, $text);
}

function translateLineComment($line)
{
    global $commentCache;
    if (!preg_match('/^(\s*\/\/)(.*)$/u', $line, $m)) return $line;
    $body = $m[2];
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $body)) return $line;
    $trim = trim($body);
    $hash = md5($trim);
    if (isset($commentCache[$hash]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $commentCache[$hash])) {
        return $m[1] . ' ' . $commentCache[$hash];
    }
    return $line;
}

function transformScript($script, $rel, &$stats)
{
    $lines = preg_split('/\r\n|\r|\n/', $script);
    foreach ($lines as &$line) {
        $line = translateLineComment($line);
    }
    unset($line);
    $script = implode("\n", $lines);

    return preg_replace_callback(
        '/([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1/u',
        function ($m) use ($rel, &$stats) {
            $inner = $m[2];
            if (isAlreadyI18n($inner)) return $m[0];
            $new = replaceChineseRun($inner, $rel, $stats);
            return jsQuoteWrap($m[1], $new);
        },
        $script
    );
}

function transformHtm($content, $rel, &$stats)
{
    // Chinese between {/yun} and {yun:} (Smarty glue text — often missed)
    $content = preg_replace_callback(
        '/\{\/yun\}((?:[^{]|<[^>]*>)*?)([\x{4e00}-\x{9fff}]+)((?:[^{]|<[^>]*>)*?)\{yun:/us',
        function ($m) use ($rel, &$stats) {
            if (isAlreadyI18n($m[0])) return $m[0];
            $mid = $m[1] . $m[2] . $m[3];
            $new = replaceChineseRun($mid, $rel, $stats);
            if ($new === $mid) return $m[0];
            return '{/yun}' . $new . '{yun:';
        },
        $content
    );

    $store = array();
    $idx = 0;
    $protect = function ($m) use (&$store, &$idx) {
        $key = '@@YUN_I18N_BLK_' . ($idx++) . '@@';
        $store[$key] = $m[0];
        return $key;
    };

    $content = preg_replace_callback('/<!--[\s\S]*?-->/u', $protect, $content);
    $content = preg_replace_callback('/\{yun:\}[\s\S]*?\{\/yun\}/u', $protect, $content);

    $content = preg_replace_callback(
        '#<script\b[^>]*>([\s\S]*?)</script>#iu',
        function ($m) use ($rel, &$stats) {
            return str_replace($m[1], transformScript($m[1], $rel, $stats), $m[0]);
        },
        $content
    );

    $content = preg_replace_callback('#<style\b[^>]*>[\s\S]*?</style>#is', $protect, $content);

    $jsFns = 'layer_del|layer_confirm|showToast|pleaselogin|layer_open|showConfirm|showLoading';
    $content = preg_replace_callback(
        '/(' . $jsFns . ')\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/iu',
        function ($m) use ($rel, &$stats) {
            $key = lookupKey($m[3], $rel);
            if (!$key) return $m[0];
            $stats['hit']++;
            return $m[1] . '(' . jsQuoteWrap($m[2], tTag($key));
        },
        $content
    );

    $content = preg_replace_callback(
        '/layer\.msg\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1/iu',
        function ($m) use ($rel, &$stats) {
            $key = lookupKey($m[2], $rel);
            if (!$key) return $m[0];
            $stats['hit']++;
            return 'layer.msg(' . jsQuoteWrap($m[1], tTag($key));
        },
        $content
    );

    $attrSimple = 'alt|title|placeholder|value|onfocus|onblur|onmousemove|onmouseout';
    $content = preg_replace_callback(
        '/\b(' . $attrSimple . ')=("|\')([^"\']*[\x{4e00}-\x{9fff}][^"\']*)\2/iu',
        function ($m) use ($rel, &$stats) {
            $new = replaceChineseRun($m[3], $rel, $stats);
            return $m[1] . '=' . $m[2] . $new . $m[2];
        },
        $content
    );

    // onclick may contain nested quotes: onclick="fn('a','中文')"
    $content = preg_replace_callback(
        '/\b(onclick|onClick)=(["\'])([\s\S]*?[\x{4e00}-\x{9fff}][\s\S]*?)\2/iu',
        function ($m) use ($rel, &$stats) {
            if (strpos($m[3], '{yun:}t ') !== false) return $m[0];
            $new = replaceChineseRun($m[3], $rel, $stats);
            return $m[1] . '=' . $m[2] . $new . $m[2];
        },
        $content
    );

    $content = preg_replace_callback(
        '/>([^<]*[\x{4e00}-\x{9fff}][^<]*)</u',
        function ($m) use ($rel, &$stats) {
            return '>' . replaceChineseRun($m[1], $rel, $stats) . '<';
        },
        $content
    );

    // Chinese text immediately after a protected {yun:} block placeholder
    $content = preg_replace_callback(
        '/@@YUN_I18N_BLK_\d+@@([^<]*[\x{4e00}-\x{9fff}][^<]*)/u',
        function ($m) use ($rel, &$stats) {
            $new = replaceChineseRun($m[1], $rel, $stats);
            return str_replace($m[1], $new, $m[0]);
        },
        $content
    );

    foreach ($store as $key => $value) {
        $content = str_replace($key, $value, $content);
    }
    return $content;
}

function iterFiles($singleFile)
{
    if ($singleFile !== '') {
        $path = ROOT . ltrim($singleFile, '/');
        if (is_file($path)) yield $path;
        return;
    }
    $base = ROOT . 'app/template';
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
    foreach ($it as $f) {
        $ext = strtolower($f->getExtension());
        if ($f->isFile() && in_array($ext, array('htm', 'vue'), true)) {
            yield $f->getPathname();
        }
    }
}

$stats = array('files' => 0, 'hit' => 0, 'miss' => 0);
foreach (iterFiles($singleFile) as $file) {
    $rel = str_replace(ROOT, '', $file);
    $orig = file_get_contents($file);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $orig)) continue;
    $fileStats = array('hit' => 0, 'miss' => 0);
    $new = transformHtm($orig, $rel, $fileStats);
    if ($new === $orig) continue;
    if ($new === '' || strlen($new) < strlen($orig) * 0.5) {
        fwrite(STDERR, "SKIP unsafe transform: $rel\n");
        continue;
    }
    $stats['files']++;
    $stats['hit'] += $fileStats['hit'];
    $stats['miss'] += $fileStats['miss'];
    echo ($dryRun ? '[dry] ' : '') . "$rel (keys: {$fileStats['hit']}, miss: {$fileStats['miss']})\n";
    if (!$dryRun) file_put_contents($file, $new);
}

echo "\nFiles: {$stats['files']}, keys applied: {$stats['hit']}, unmatched segments: {$stats['miss']}\n";
