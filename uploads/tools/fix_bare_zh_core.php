<?php
/**
 * Shared bare-Chinese wrapping for PHP user-facing strings.
 */

if (!defined('ROOT')) {
    define('ROOT', dirname(__DIR__) . '/');
}

function shouldSkipLine($line)
{
    if (preg_match_all('/[\x{4e00}-\x{9fff}]+/u', $line, $m)) {
        $bare = preg_replace('/yun_auto_t\s*\([^)]*\)|yun_at\s*\([^)]*\)|WapDbEnum::[A-Z_]+/u', '', $line);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $bare)) {
            return true;
        }
    }
    if (preg_match('/^\s*(\*|\/\/|#)/', trim($line))) {
        return true;
    }
    // Skip lines with string concat — partial wrap breaks syntax
    if (preg_match('/yun_auto_t\s*\([^)]+\)\s*\./', $line) || preg_match('/\.\s*yun_auto_t\s*\(/', $line)) {
        return true;
    }
    if (preg_match('/[\'"][^\'"]*[\x{4e00}-\x{9fff}][^\'"]*[\'"]\s*\./u', $line) && !preg_match('/yun_auto_t\s*\(\s*[\'"][^\'"]*[\'"]\s*\)\s*;/u', $line)) {
        return true;
    }
    return false;
}

function wrapChineseString($str)
{
    $str = str_replace("'", "\\'", $str);
    return "yun_auto_t('" . $str . "')";
}

function bareZhReplaceField($line, $field)
{
    return preg_replace_callback(
        '/([\'"]' . preg_quote($field, '/') . '[\'"]\s*=>\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );
}

function fixLine($line)
{
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $line)) {
        return $line;
    }
    if (shouldSkipLine($line)) {
        return $line;
    }

    foreach (array('info', 'btn', 'title', 'msg', 'error', 'errmsg', 'linkMsg', 'content', 'message', 'statusbody') as $field) {
        $line = bareZhReplaceField($line, $field);
    }

    $line = preg_replace_callback(
        '/(->\s*yunset\s*\(\s*["\'][^"\']+["\']\s*,\s*)(?!yun_auto_t\s*\()(["\'])([^"\']*[\x{4e00}-\x{9fff}][^"\']*)\2(\s*\))/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]) . $m[4];
        },
        $line
    );

    $line = preg_replace_callback(
        '/(->(?:actMsg|ACT_layer_msg)\s*\([^,]+,\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(->ACT_msg_wap\s*\([^,]+,\s*)\$msg\s*=\s*([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(\$_SESSION\[[^\]]+\]\s*=\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/((?:->addMemberLog|member_log)\s*\(\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(->(?:layer_msg|ACT_msg_wap|ACT_msg)\s*\([^,]+,\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(\$[a-zA-Z_][\w]*(?:\[[^\]]+\])+\s*(?:\.=|=\s*))(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(->render_json\s*\([^,]+,\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/(\?\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2(\s*:\s*)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\5/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]) . $m[4] . wrapChineseString($m[6]);
        },
        $line
    );

    $line = preg_replace_callback(
        '/((?:echo|die)\s+)(?!yun_auto_t\s*\()([\'"])([^\'"]*[\x{4e00}-\x{9fff}][^\'"]*)\2/u',
        function ($m) {
            return $m[1] . wrapChineseString($m[3]);
        },
        $line
    );

    $line = preg_replace(
        '/:\s*\'不限\'/u',
        ': WapDbEnum::UNLIMITED',
        $line
    );

    return $line;
}

function fixBareZhDirs(array $dirs, array $skipFiles = array())
{
    $changed = 0;
    foreach ($dirs as $dir) {
        $path = ROOT . $dir;
        if (!is_dir($path)) {
            continue;
        }
        $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
        foreach ($it as $f) {
            if (!$f->isFile() || $f->getExtension() !== 'php') {
                continue;
            }
            $rel = str_replace(ROOT, '', $f->getPathname());
            if (in_array($rel, $skipFiles, true)) {
                continue;
            }
            if (preg_match('/vendor|PHPExcel|aliyun|install/i', $rel)) {
                continue;
            }
            $lines = file($f->getPathname());
            $out = array();
            $fileChanged = false;
            foreach ($lines as $line) {
                $new = fixLine($line);
                if ($new !== $line) {
                    $fileChanged = true;
                }
                $out[] = $new;
            }
            if ($fileChanged) {
                file_put_contents($f->getPathname(), implode('', $out));
                echo "FIXED: $rel\n";
                $changed++;
            }
        }
    }
    return $changed;
}
