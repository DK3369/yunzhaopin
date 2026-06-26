<?php
/**
 * Replace Chinese string literals with lang pack keys in PHP source.
 * Usage: php tools/replace_chinese_literals.php [--dry-run]
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);

$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$valueToKeys = array();
foreach ($zh as $key => $value) {
    if (!is_string($value) || $value === '') continue;
    if (!isset($valueToKeys[$value])) $valueToKeys[$value] = array();
    $valueToKeys[$value][] = $key;
}

function pathPrefixHints($rel)
{
    if (strpos($rel, 'api/wxapp/') === 0) return array('wap_', 'wap_com_', 'wap_user_', 'common_');
    if (strpos($rel, 'admin/') === 0) return array('admin_', 'admin_sys_', 'admin_tool_', 'admin_model_', 'admin_com_', 'admin_user_', 'common_');
    if (strpos($rel, 'app/model/') === 0) return array('model_', 'common_');
    if (strpos($rel, 'member/com/') === 0) return array('member_com_', 'common_');
    if (strpos($rel, 'member/user/') === 0) return array('member_user_', 'common_');
    if (strpos($rel, 'app/controller/wap/') === 0) return array('wap_', 'common_');
    if (strpos($rel, 'app/controller/') === 0) return array('controller_', 'common_', 'wap_');
    if (strpos($rel, 'app/include/') === 0) return array('common_', 'model_', 'wap_', 'admin_');
    return array('common_', 'model_', 'wap_', 'admin_');
}

function pickKey($text, $keys, $rel)
{
    if (count($keys) === 1) return $keys[0];
    foreach (pathPrefixHints($rel) as $prefix) {
        foreach ($keys as $key) {
            if (strpos($key, $prefix) === 0) return $key;
        }
    }
    return $keys[0];
}

function lookupKey($text, $rel)
{
    global $valueToKeys;
    if (isset($valueToKeys[$text])) {
        return pickKey($text, $valueToKeys[$text], $rel);
    }
    $normalized = preg_replace('/\s+/u', ' ', trim(html_entity_decode($text, ENT_QUOTES, 'UTF-8')));
    if ($normalized !== $text && isset($valueToKeys[$normalized])) {
        return pickKey($normalized, $valueToKeys[$normalized], $rel);
    }
    return null;
}

function isUserFacingString($text)
{
    if (mb_strlen($text, 'UTF-8') < 2 || mb_strlen($text, 'UTF-8') > 120) return false;
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $text)) return false;
    if (preg_match('/\$|SELECT |INSERT |UPDATE |DELETE |function |\.php|http|www\.|namespace |class |extends /i', $text)) return false;
    return true;
}

function prevSigToken($tokens, $index)
{
    for ($i = $index - 1; $i >= 0; $i--) {
        if (is_array($tokens[$i]) && in_array($tokens[$i][0], array(T_WHITESPACE, T_COMMENT, T_DOC_COMMENT), true)) {
            continue;
        }
        return is_array($tokens[$i]) ? $tokens[$i] : $tokens[$i];
    }
    return null;
}

function prev2SigToken($tokens, $index)
{
    $seen = 0;
    for ($i = $index - 1; $i >= 0; $i--) {
        if (is_array($tokens[$i]) && in_array($tokens[$i][0], array(T_WHITESPACE, T_COMMENT, T_DOC_COMMENT), true)) {
            continue;
        }
        $seen++;
        if ($seen === 2) {
            return is_array($tokens[$i]) ? $tokens[$i] : $tokens[$i];
        }
    }
    return null;
}

function needsYunAt($tokens, $index)
{
    $prev = prevSigToken($tokens, $index);
    if ($prev === 'echo' || $prev === 'die' || $prev === 'exit' || $prev === 'print') {
        return true;
    }
    if (is_array($prev) && $prev[0] === T_STRING && in_array($prev[1], array('echo', 'die', 'exit', 'print'), true)) {
        return true;
    }
    if (is_array($prev) && $prev[0] === T_EXIT) {
        return true;
    }
    return false;
}

function transformFile($rel, $content, &$replaced, &$missing)
{
    $tokens = token_get_all($content);
    $changed = false;
    foreach ($tokens as $i => &$token) {
        if (!is_array($token) || $token[0] !== T_CONSTANT_ENCAPSED_STRING) {
            continue;
        }
        $raw = $token[1];
        $quote = $raw[0];
        $text = stripcslashes(substr($raw, 1, -1));
        if (!isUserFacingString($text)) {
            continue;
        }
        if (preg_match('/^[a-z]+_\d+$/', $text)) {
            continue;
        }
        $key = lookupKey($text, $rel);
        if ($key === null) {
            $missing[$text][] = $rel;
            continue;
        }
        if (needsYunAt($tokens, $i)) {
            $token[1] = "Yun_Lang::at('$key')";
        } else {
            $token[1] = "'$key'";
        }
        $replaced++;
        $changed = true;
    }
    unset($token);
    if (!$changed) {
        return $content;
    }
    $out = '';
    foreach ($tokens as $token) {
        $out .= is_array($token) ? $token[1] : $token;
    }
    return $out;
}

$dirs = array('app', 'admin', 'member', 'api/wxapp', 'wap');
$skipPattern = '/vendor|PHPExcel|install\/|data\/lang|tools\/|dbbackup|function\.queryinfo\.php|function\.querytime\.php|aliyun|webscan|umeditor|wangeditor|PHPWord|tcpdf|ueditor|lib_splitword|mysql\.class|mysqli\.class/i';
$replaced = 0;
$missing = array();
$filesChanged = 0;

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skipPattern, $rel)) continue;
        $orig = file_get_contents($f->getPathname());
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $orig)) continue;
        $new = transformFile($rel, $orig, $replaced, $missing);
        if ($new !== $orig) {
            $filesChanged++;
            if (!$dryRun) {
                file_put_contents($f->getPathname(), $new);
            }
            echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        }
    }
}

echo "\nReplaced: $replaced\nFiles: $filesChanged\nMissing keys: " . count($missing) . "\n";
if (!empty($missing)) {
    $i = 0;
    foreach ($missing as $text => $files) {
        if ($i++ >= 30) {
            echo "...+" . (count($missing) - 30) . " more\n";
            break;
        }
        echo "  [$text] " . implode(', ', array_unique($files)) . "\n";
    }
}
