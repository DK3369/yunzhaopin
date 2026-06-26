<?php
/**
 * Scan PHP for user-facing Chinese string literals not using lang keys.
 * Usage: php tools/scan_chinese_literals.php
 */
define('ROOT', dirname(__DIR__) . '/');
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
    if (strpos($rel, 'app/include/libs/') === 0) return array('common_', 'model_', 'tpl_');
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

function stripComments($code)
{
    $code = preg_replace('#/\*.*?\*/#s', '', $code);
    $code = preg_replace('#//[^\n]*#', '', $code);
    $code = preg_replace('#\#[^\n]*#', '', $code);
    return $code;
}

$dirs = array('app', 'admin', 'member', 'api/wxapp', 'wap');
$skipPattern = '/vendor|PHPExcel|install\/|data\/lang|tools\/|dbbackup|function\.queryinfo\.php|function\.querytime\.php|lib_splitword|mysql\.class|mysqli\.class/i';
$issues = array();
$noKey = array();

foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    if (!is_dir($path)) continue;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skipPattern, $rel)) continue;
        $raw = file_get_contents($f->getPathname());
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $raw)) continue;
        $code = stripComments($raw);
        preg_match_all("/(['\"])((?:\\\\.|(?!\\1)[^\\\\])*[\x{4e00}-\x{9fff}](?:\\\\.|(?!\\1)[^\\\\])*)\\1/u", $code, $m, PREG_OFFSET_CAPTURE);
        foreach ($m[2] as $item) {
            $text = stripcslashes($item[0]);
            $offset = $item[1];
            if (mb_strlen($text, 'UTF-8') < 2 || mb_strlen($text, 'UTF-8') > 200) continue;
            if (preg_match('/^\s*[\x{4e00}-\x{9fff}\s\d，。！？：、（）【】·\-\+\.\/]+$/u', $text) === 0) continue;
            if (preg_match('/\$|SELECT |INSERT |UPDATE |DELETE |function |\.php|http|www\.|yun:|smarty|namespace |class |extends |require|include/i', $text)) continue;
            $before = substr($code, max(0, $offset - 120), 120);
            $after = substr($code, $offset, 120);
            if (preg_match('/yun_at\s*\(|Yun_Lang::at\s*\(|->at\s*\(|->msg\s*\(|yun_msg\s*\(|Yun_Lang::msg\s*\(/', $before . $after)) continue;
            if (preg_match('/^[a-z]+_\d+$/', $text)) continue;
            // lang pack definition file values - skip array values in lang (not in scan dirs)
            $line = substr_count(substr($raw, 0, $offset), "\n") + 1;
            $issues[] = array('file' => $rel, 'line' => $line, 'text' => $text, 'ctx' => trim(preg_replace('/\s+/', ' ', substr($before, -60) . '>>>' . substr($after, 0, 40))));
            if (!isset($valueToKeys[$text])) {
                $noKey[$text][] = $rel;
            }
        }
    }
}

$byFile = array();
foreach ($issues as $row) {
    $byFile[$row['file']][] = $row;
}

echo "=== Chinese string literals in PHP (no key wrapper) ===\n";
echo "Total: " . count($issues) . "\n";
echo "Unique texts: " . count(array_unique(array_column($issues, 'text'))) . "\n";
echo "No lang key: " . count($noKey) . "\n\n";

$shown = 0;
foreach ($byFile as $file => $rows) {
    if ($shown++ >= 60) {
        echo "...+" . (count($byFile) - 60) . " more files\n";
        break;
    }
    echo "$file (" . count($rows) . ")\n";
    $seen = array();
    foreach ($rows as $r) {
        if (isset($seen[$r['text']])) continue;
        $seen[$r['text']] = 1;
        $key = isset($valueToKeys[$r['text']]) ? pickKey($r['text'], $valueToKeys[$r['text']], $file) : '?';
        echo "  L{$r['line']}: [{$key}] {$r['text']}\n";
        if (count($seen) >= 5 && count($rows) > 5) {
            echo "  ...+" . (count($rows) - 5) . " more in file\n";
            break;
        }
    }
}

file_put_contents(ROOT . 'tools/chinese_literals.json', json_encode(array('issues' => $issues, 'noKey' => $noKey), JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
