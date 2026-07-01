<?php
/**
 * DEPRECATED — DO NOT USE.
 *
 * This batch tool caused site outages (aliases.php corruption, broken Vue syntax).
 * Use manual per-file lc() binding instead. See .cursor/rules/agent-workflow.mdc
 *
 * Convert admin .html shell pages: bare Chinese → lc('key') bindings.
 *
 * Usage:
 *   php tools/i18n_admin_html.php [--dry-run] --file=app/template/admin/tool/generate/generate_cache.html
 *   php tools/i18n_admin_html.php [--dry-run] --dir=app/template/admin/tool/generate/
 *
 * Full-site batch disabled without --file= or --dir=.
 */
fwrite(STDERR, "ERROR: i18n_admin_html.php is disabled. Use manual per-file lc() binding.\n");
exit(1);

define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$singleFile = '';
$singleDir = '';
foreach ($argv ?? array() as $arg) {
    if (preg_match('/^--file=(.+)$/', $arg, $m)) {
        $singleFile = $m[1];
    }
    if (preg_match('/^--dir=(.+)$/', $arg, $m)) {
        $singleDir = rtrim($m[1], '/') . '/';
    }
}

if ($singleFile === '' && $singleDir === '') {
    fwrite(STDERR, "ERROR: --file= or --dir= is required (full-site batch disabled).\n");
    exit(1);
}

$zhAuto = include ROOT . 'data/lang/auto/zh_cn.php';
$enAuto = include ROOT . 'data/lang/auto/en_us.php';
$valueToKeys = array();
foreach ($zhAuto as $key => $value) {
    if (!is_string($value) || $value === '' || $key === '_meta') {
        continue;
    }
    if (!isset($valueToKeys[$value])) {
        $valueToKeys[$value] = array();
    }
    $valueToKeys[$value][] = $key;
}

$zhStruct = include ROOT . 'data/lang/zh_cn.php';
function flattenStructured($arr, $prefix, &$out)
{
    foreach ($arr as $k => $v) {
        if ($k === '_meta') {
            continue;
        }
        $full = $prefix === '' ? $k : $prefix . '.' . $k;
        if (is_array($v)) {
            flattenStructured($v, $full, $out);
        } elseif (is_string($v) && $v !== '') {
            if (!isset($out[$v])) {
                $out[$v] = array();
            }
            $out[$v][] = $full;
        }
    }
}
flattenStructured($zhStruct, '', $valueToKeys);

function pathPrefixHints($rel)
{
    $hints = array('admin_tool_', 'admin_system_', 'admin_user_', 'admin_', 'common_', 'wap_js_', 'wap_');
    if (preg_match('#^app/template/admin/tool/#', $rel)) {
        array_unshift($hints, 'admin_tool_');
    } elseif (preg_match('#^app/template/admin/system/#', $rel)) {
        array_unshift($hints, 'admin_system_');
    } elseif (preg_match('#^app/template/admin/user/#', $rel)) {
        array_unshift($hints, 'admin_user_');
    } elseif (preg_match('#^app/template/admin/neirong/#', $rel)) {
        array_unshift($hints, 'admin_neirong_', 'admin_');
    } elseif (preg_match('#^app/template/admin/yunying/#', $rel)) {
        array_unshift($hints, 'admin_yunying_', 'admin_');
    }
    return $hints;
}

function pickKey($text, $keys, $rel)
{
    if (count($keys) === 1) {
        return $keys[0];
    }
    foreach (pathPrefixHints($rel) as $prefix) {
        foreach ($keys as $key) {
            if (strpos($key, $prefix) === 0) {
                return $key;
            }
        }
    }
    return $keys[0];
}

function lookupKey($text, $rel)
{
    global $valueToKeys;
    $candidates = array(
        $text,
        trim($text),
        preg_replace('/\s+/u', ' ', trim($text)),
        html_entity_decode(trim($text), ENT_QUOTES, 'UTF-8'),
    );
    foreach (array_unique($candidates) as $c) {
        if ($c !== '' && isset($valueToKeys[$c])) {
            return pickKey($c, $valueToKeys[$c], $rel);
        }
    }
    return null;
}

function lcExpr($key)
{
    return "lc('" . $key . "')";
}

function isAlreadyI18n($s)
{
    return preg_match('/lc\s*\(|yunAdminT\s*\(|yun:}t\s/u', $s);
}

function nextHtmlKey(&$num)
{
    global $zhAuto;
    while (isset($zhAuto['admin_html_' . str_pad($num, 5, '0', STR_PAD_LEFT)])) {
        $num++;
    }
    $k = 'admin_html_' . str_pad($num, 5, '0', STR_PAD_LEFT);
    $num++;
    return $k;
}

function resolveKey($text, $rel, &$newKeys, &$htmlKeyNum, $maxNew)
{
    $key = lookupKey($text, $rel);
    if ($key) {
        return $key;
    }
    if (isset($newKeys[$text])) {
        return $newKeys[$text];
    }
    if (count($newKeys) >= $maxNew) {
        return null;
    }
    $k = nextHtmlKey($htmlKeyNum);
    $newKeys[$text] = $k;
    return $k;
}

function transformHtml($content, $rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats)
{
    if (isAlreadyI18n($content) && !preg_match('/[\x{4e00}-\x{9fff}]/u', preg_replace('/lc\s*\([^)]+\)/u', '', $content))) {
        return $content;
    }

    $scriptPart = '';
    if (preg_match('#<!--\s*script\s*-->[\s\S]*$#iu', $content, $sm, PREG_OFFSET_CAPTURE)) {
        $scriptPart = substr($content, $sm[0][1]);
        $content = substr($content, 0, $sm[0][1]);
    } elseif (preg_match_all('#<script(?![^>]*\bsrc=)[^>]*>([\s\S]*?)</script>#iu', $content, $blocks, PREG_OFFSET_CAPTURE)) {
        for ($i = count($blocks[0]) - 1; $i >= 0; $i--) {
            $inner = $blocks[1][$i][0];
            if (strpos($inner, 'new Vue') !== false || strpos($inner, 'Vue.extend') !== false) {
                $start = $blocks[0][$i][1];
                $scriptPart = substr($content, $start);
                $content = substr($content, 0, $start);
                break;
            }
        }
    }

    $store = array();
    $idx = 0;
    $protect = function ($m) use (&$store, &$idx) {
        $key = '@@YUN_HTML_BLK_' . ($idx++) . '@@';
        $store[$key] = $m[0];
        return $key;
    };

    $content = preg_replace_callback('/<!--[\s\S]*?-->/u', $protect, $content);

    // yunAdminT in template area only (rare)
    $content = preg_replace_callback(
        '/yunAdminT\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1\s*\)/u',
        function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
            $key = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
            if (!$key) {
                return $m[0];
            }
            $stats['hit']++;
            return lcExpr($key);
        },
        $content
    );

    $bindAttrs = 'title|description|placeholder|label|range-separator|start-placeholder|end-placeholder|empty-text|content|alt';
    $replaceAttr = function ($content, $attr) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
        return preg_replace_callback(
            '/\b' . $attr . '\s*=\s*"([^"]*[\x{4e00}-\x{9fff}][^"]*)"/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats, $attr) {
                if (isAlreadyI18n($m[1])) {
                    return $m[0];
                }
                $key = resolveKey($m[1], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return ':' . $attr . '="' . lcExpr($key) . '"';
            },
            $content
        );
    };
    foreach (explode('|', $bindAttrs) as $attr) {
        $content = $replaceAttr($content, $attr);
        $content = preg_replace_callback(
            "/\b" . $attr . "\s*=\s*'([^']*[\x{4e00}-\x{9fff}][^']*)'/u",
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats, $attr) {
                if (isAlreadyI18n($m[1])) {
                    return $m[0];
                }
                $key = resolveKey($m[1], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return ':' . $attr . '="' . lcExpr($key) . '"';
            },
            $content
        );
    }

    // Avoid double-colon when attribute already bound
    $content = str_replace('::', ':', $content);

    // el-button / span / td inner text (may span whitespace)
    $content = preg_replace_callback(
        '/(<(?:el-button|span|td|th|div|a|p|li|template)[^>]*>)\s*([^<]*[\x{4e00}-\x{9fff}][^<]*)\s*(<\/(?:el-button|span|td|th|div|a|p|li|template)>)/u',
        function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
            $text = trim($m[2]);
            if ($text === '' || isAlreadyI18n($text) || preg_match('/^\{\{/u', $text)) {
                return $m[0];
            }
            $key = resolveKey($text, $rel, $newKeys, $htmlKeyNum, $maxNew);
            if (!$key) {
                $stats['miss']++;
                return $m[0];
            }
            $stats['hit']++;
            return $m[1] . '{{ ' . lcExpr($key) . ' }}' . $m[3];
        },
        $content
    );

    // template slot append text e.g. <template slot="append">分钟</template>
    $content = preg_replace_callback(
        '/(<template[^>]*>)\s*([^<]*[\x{4e00}-\x{9fff}][^<]*)\s*(<\/template>)/u',
        function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
            $text = trim($m[2]);
            if ($text === '' || isAlreadyI18n($text)) {
                return $m[0];
            }
            $key = resolveKey($text, $rel, $newKeys, $htmlKeyNum, $maxNew);
            if (!$key) {
                return $m[0];
            }
            $stats['hit']++;
            return $m[1] . '{{ ' . lcExpr($key) . ' }}' . $m[3];
        },
        $content
    );

    // Mixed icon+text in one line: <i ...></i>预览</el-button>
    $content = preg_replace_callback(
        '/(<i[^>]*><\/i>)\s*([\x{4e00}-\x{9fff}]+)\s*(<\/el-button>)/u',
        function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
            $key = resolveKey(trim($m[2]), $rel, $newKeys, $htmlKeyNum, $maxNew);
            if (!$key) {
                return $m[0];
            }
            $stats['hit']++;
            return $m[1] . '{{ ' . lcExpr($key) . ' }}' . $m[3];
        },
        $content
    );

    $content = preg_replace_callback(
        '/>([^<{]*[\x{4e00}-\x{9fff}][^<{]*)</u',
        function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
            $inner = $m[1];
            if (isAlreadyI18n($inner) || preg_match('/^\s*$/', $inner)) {
                return $m[0];
            }
            if (preg_match('/^\s*\{\{/u', trim($inner))) {
                return $m[0];
            }
            $trimmed = trim($inner);
            if ($trimmed === '') {
                return $m[0];
            }
            $key = resolveKey($trimmed, $rel, $newKeys, $htmlKeyNum, $maxNew);
            if (!$key) {
                $stats['miss']++;
                return $m[0];
            }
            $stats['hit']++;
            $pad = '';
            if (preg_match('/^(\s+)/', $inner, $wm)) {
                $pad = $wm[1];
            }
            return '>' . $pad . '{{ ' . lcExpr($key) . ' }}<';
        },
        $content
    );

    foreach ($store as $key => $value) {
        $content = str_replace($key, $value, $content);
    }

    if ($scriptPart !== '') {
        $scriptPart = preg_replace_callback(
            '/yunAdminT\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1\s*\)/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return lcExpr($key);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(?:message\.(?:error|warning|success|info)|layer\.msg)\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return str_replace($m[1] . $m[2] . $m[1], lcExpr($key), $m[0]);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(\b(?:label|text|message|title|confirmButtonText|cancelButtonText)\s*:\s*)([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[3], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return $m[1] . lcExpr($key);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(?:confirm|alert|prompt)\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return str_replace($m[1] . $m[2] . $m[1], lcExpr($key), $m[0]);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(\[\s*[\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2(\s*,\s*[\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\4/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $k1 = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
                $k2 = resolveKey($m[4], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$k1 || !$k2) {
                    return $m[0];
                }
                $stats['hit'] += 2;
                return '[' . lcExpr($k1) . ', ' . lcExpr($k2);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/\$confirm\s*\(\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\1\s*,\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\3/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $k1 = resolveKey($m[2], $rel, $newKeys, $htmlKeyNum, $maxNew);
                $k2 = resolveKey($m[4], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$k1 || !$k2) {
                    return $m[0];
                }
                $stats['hit'] += 2;
                return '$confirm(' . lcExpr($k1) . ', ' . lcExpr($k2);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(\b(?:dataText|titleAddEdit|title\w*|emptytext|emptyText|placeholder\w*)\s*:\s*)([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[3], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return $m[1] . lcExpr($key);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            '/(_this\.\w+\s*=\s*)([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
            function ($m) use ($rel, &$newKeys, &$htmlKeyNum, $maxNew, &$stats) {
                $key = resolveKey($m[3], $rel, $newKeys, $htmlKeyNum, $maxNew);
                if (!$key) {
                    return $m[0];
                }
                $stats['hit']++;
                return $m[1] . lcExpr($key);
            },
            $scriptPart
        );
        $scriptPart = preg_replace_callback(
            "/madeall:\s*'[\x{4e00}-\x{9fff}]+'/u",
            function ($m) {
                return "madeall: '1'";
            },
            $scriptPart
        );
        $content .= $scriptPart;
    }

    return $content;
}

function appendLangKeys($newKeys)
{
    if (empty($newKeys)) {
        return;
    }
    global $enAuto;
    foreach (array('zh_cn', 'en_us') as $lang) {
        $path = ROOT . 'data/lang/auto/' . $lang . '.php';
        $content = file_get_contents($path);
        $content = rtrim($content);
        $content = preg_replace('/\);\s*$/', '', $content);
        foreach ($newKeys as $text => $k) {
            $val = $lang === 'en_us' ? ($enAuto[$k] ?? $text) : $text;
            if ($lang === 'en_us' && !isset($enAuto[$k])) {
                $val = $text;
            }
            $content .= "\n  '" . addslashes($k) . "' => '" . addslashes($val) . "',";
        }
        $content .= "\n);\n";
        file_put_contents($path, $content);
    }
    $aliasPath = ROOT . 'data/lang/auto/aliases.php';
    if (!is_file($aliasPath)) {
        return;
    }
    $content = file_get_contents($aliasPath);
    $content = rtrim($content);
    $content = preg_replace('/\);\s*$/', '', $content);
    foreach ($newKeys as $text => $k) {
        if (!is_string($text) || $text === '' || strpos($content, var_export($text, true)) !== false) {
            continue;
        }
        $content .= "\n  " . var_export($text, true) . ' => ' . var_export($k, true) . ',';
    }
    $content .= "\n);\n";
    file_put_contents($aliasPath, $content);
}

function iterFiles($singleFile, $singleDir)
{
    if ($singleFile !== '') {
        $path = ROOT . ltrim($singleFile, '/');
        if (is_file($path)) {
            yield $path;
        }
        return;
    }
    $base = ROOT . ltrim($singleDir, '/');
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
    foreach ($it as $f) {
        if ($f->isFile() && strtolower($f->getExtension()) === 'html') {
            yield $f->getPathname();
        }
    }
}

$maxNew = 20;
$htmlKeyNum = 1;
while (isset($zhAuto['admin_html_' . str_pad($htmlKeyNum, 5, '0', STR_PAD_LEFT)])) {
    $htmlKeyNum++;
}
$allNewKeys = array();
$stats = array('files' => 0, 'hit' => 0, 'miss' => 0);

foreach (iterFiles($singleFile, $singleDir) as $file) {
    $rel = str_replace(ROOT, '', $file);
    if ($rel === 'app/template/admin/index.htm') {
        continue;
    }
    $orig = file_get_contents($file);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $orig)) {
        continue;
    }
    $fileNewKeys = array();
    $fileStats = array('hit' => 0, 'miss' => 0);
    $new = transformHtml($orig, $rel, $fileNewKeys, $htmlKeyNum, $maxNew - count($allNewKeys), $fileStats);
    if ($new === $orig) {
        continue;
    }
    $stats['files']++;
    $stats['hit'] += $fileStats['hit'];
    $stats['miss'] += $fileStats['miss'];
    $allNewKeys = array_merge($allNewKeys, $fileNewKeys);
    echo ($dryRun ? '[dry] ' : '') . "$rel (keys: {$fileStats['hit']}, miss: {$fileStats['miss']}, new: " . count($fileNewKeys) . ")\n";
    if (!$dryRun) {
        file_put_contents($file, $new);
    }
    if (count($allNewKeys) >= $maxNew) {
        echo "Reached max new keys ($maxNew) for this run.\n";
        break;
    }
}

if (!$dryRun && $allNewKeys) {
    appendLangKeys($allNewKeys);
    echo "Added " . count($allNewKeys) . " new admin_html_* keys.\n";
}

echo "\nFiles: {$stats['files']}, keys applied: {$stats['hit']}, unmatched: {$stats['miss']}\n";
