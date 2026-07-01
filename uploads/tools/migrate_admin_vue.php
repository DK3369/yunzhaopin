<?php
/**
 * Migrate admin .vue Chinese strings to i18n keys.
 * Usage: php tools/migrate_admin_vue.php <relative-dir> [--key-start=N] [--max-keys=N] [--dry-run]
 */
define('ROOT', dirname(__DIR__) . '/');

function loadZh() { static $z; if (!$z) $z = include ROOT . 'data/lang/auto/zh_cn.php'; return $z; }
function loadEn() { static $e; if (!$e) $e = include ROOT . 'data/lang/auto/en_us.php'; return $e; }

function buildValueMap() {
    static $m;
    if (!$m) {
        $m = [];
        foreach (loadZh() as $k => $v) {
            if (is_string($v) && $v !== '') {
                if (!isset($m[$v])) $m[$v] = [];
                $m[$v][] = $k;
            }
        }
    }
    return $m;
}

function pickKey($text, $keys, $file) {
    $prefs = ['admin_user_', 'admin_company_', 'admin_tool_', 'admin_system_', 'admin_yunying_', 'admin_', 'common_', 'wap_', 'member_'];
    if (strpos($file, 'user/users') !== false) array_unshift($prefs, 'admin_user_');
    if (strpos($file, 'user/member') !== false) array_unshift($prefs, 'admin_user_');
    if (strpos($file, 'user/weipin') !== false) array_unshift($prefs, 'admin_user_weipin_');
    if (strpos($file, 'admin/component') !== false) array_unshift($prefs, 'admin_');
    if (strpos($file, 'admin/tool') !== false) array_unshift($prefs, 'admin_tool_');
    if (strpos($file, 'admin/system') !== false) array_unshift($prefs, 'admin_system_');
    if (strpos($file, 'admin/neirong') !== false) array_unshift($prefs, 'admin_');
    if (strpos($file, 'admin/yunying') !== false) array_unshift($prefs, 'admin_yunying_');
    $prefs = array_unique($prefs);
    foreach ($prefs as $p) foreach ($keys as $k) if (strpos($k, $p) === 0) return $k;
    return $keys[0];
}

function isCommentLine($line) {
    $t = ltrim($line);
    return $t === '' || strpos($t, '//') === 0 || strpos($t, '*') === 0 || strpos($t, '/*') === 0
        || strpos($t, '<!--') === 0 || preg_match('/^\s*<!--/', $line);
}

function inHtmlComment($lines, $idx) {
    for ($i = $idx; $i >= 0; $i--) {
        if (strpos($lines[$i], '<!--') !== false) {
            if (strpos($lines[$i], '-->') === false || strpos($lines[$i], '<!--') > strrpos($lines[$i], '-->')) return true;
        }
        if (strpos($lines[$i], '-->') !== false) return false;
    }
    return false;
}

function sectionAt($lines, $idx) {
    $sec = 'other';
    $tmplDepth = 0;
    $inScript = false;
    $inStyle = false;
    for ($i = 0; $i <= $idx; $i++) {
        if (preg_match('/<script[\s>]/', $lines[$i])) {
            $inScript = true;
            $sec = 'script';
        }
        if (preg_match('/<\/script>/', $lines[$i])) {
            $inScript = false;
            if ($tmplDepth <= 0) $sec = 'other';
        }
        if (preg_match('/<style[\s>]/', $lines[$i])) {
            $inStyle = true;
            $sec = 'style';
        }
        if (preg_match('/<\/style>/', $lines[$i])) {
            $inStyle = false;
            if ($tmplDepth <= 0 && !$inScript) $sec = 'other';
        }
        if (!$inScript && !$inStyle) {
            if (preg_match('/<template[\s>]/', $lines[$i])) {
                $tmplDepth++;
                $sec = 'template';
            }
            if (preg_match('/<\/template>/', $lines[$i])) {
                $tmplDepth = max(0, $tmplDepth - 1);
                if ($tmplDepth > 0) {
                    $sec = 'template';
                } elseif (!$inScript && !$inStyle) {
                    $sec = 'other';
                }
            }
        }
    }
    return $sec;
}

function skipBackendCompare($line) {
    return preg_match('/[=!]{1,2}=\s*[\'"][\x{4e00}-\x{9fff}]/u', $line)
        || preg_match('/[\'"][\x{4e00}-\x{9fff}][^\'"]*[\'"]\s*[=!]{1,2}=/u', $line)
        || preg_match('/\.(status|state|type)\s*[=!]==?\s*[\'"][\x{4e00}-\x{9fff}]/u', $line);
}

function lcExpr($key) { return "lc('" . $key . "')"; }
function lcTemplate($key) { return "{{ lc('" . $key . "') }}"; }
function lcAttr($attr, $key) { return ':' . $attr . '="lc(\'' . $key . '\')"'; }
function lcQuoted($key) { return '"' . lcExpr($key) . '"'; }

function migrateLine($line, $sec, $vmap, $file, &$newKeys, &$keyNum, $maxNew) {
    if (isCommentLine($line) || inHtmlComment(explode("\n", $line), 0)) return $line;
    if ($sec === 'style') return $line;
    if (skipBackendCompare($line)) return $line;
    // skip JS string concatenation expressions
    if (preg_match('/[\'"][\x{4e00}-\x{9fff}][^\'"]*[\'"]\s*\+/u', $line)) return $line;
    if (preg_match('/\+\s*[\'"][\x{4e00}-\x{9fff}]/u', $line)) return $line;

    $replacements = [];

    // Template attrs: label="中文"
    if ($sec === 'template') {
        $line = preg_replace_callback(
            '/\b(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text|inactive-text|active-text)\s*=\s*"([^"]*[\x{4e00}-\x{9fff}][^"]*)"/u',
            function ($m) use ($vmap, $file, &$newKeys, &$keyNum, $maxNew) {
                $text = $m[2];
                if (preg_match('/\{yun:\}t/u', $text)) return $m[0];
                $key = resolveKey($text, $vmap, $file, $newKeys, $keyNum, $maxNew);
                return lcAttr($m[1], $key);
            }, $line
        );
        $line = preg_replace_callback(
            "/\b(label|placeholder|title|range-separator|start-placeholder|end-placeholder|empty-text|inactive-text|active-text)\s*=\s*'([^']*[\x{4e00}-\x{9fff}][^']*)'/u",
            function ($m) use ($vmap, $file, &$newKeys, &$keyNum, $maxNew) {
                $text = $m[2];
                if (preg_match('/\{yun:\}t/u', $text)) return $m[0];
                $key = resolveKey($text, $vmap, $file, $newKeys, $keyNum, $maxNew);
                return lcAttr($m[1], $key);
            }, $line
        );
    }

    // Quoted strings with Chinese
    $line = preg_replace_callback(
        '/(["\'])((?:\\\\.|(?!\1)[^\\\\])*[\x{4e00}-\x{9fff}](?:\\\\.|(?!\1)[^\\\\])*)\1/u',
        function ($m) use ($line, $sec, $vmap, $file, &$newKeys, &$keyNum, $maxNew) {
            $text = stripcslashes($m[2]);
            $full = $m[0];
            $pos = strpos($line, $full);
            if ($pos === false) return $full;
            // skip if part of yun key already
            $before = substr($line, 0, $pos);
            if (preg_match('/\{yun:\}t\s+key=/u', $before . $full)) return $full;
            if (preg_match('/lc\s*\(\s*$/u', rtrim($before))) return $full;
            $key = resolveKey($text, $vmap, $file, $newKeys, $keyNum, $maxNew);
            if ($sec === 'script') {
                return lcExpr($key);
            }
            if ($sec === 'template') {
                return lcQuoted($key);
            }
            return lcQuoted($key);
        }, $line
    );

    return $line;
}

function resolveKey($text, $vmap, $file, &$newKeys, &$keyNum, $maxNew) {
    if (isset($newKeys[$text])) return $newKeys[$text];
    if (isset($vmap[$text])) return pickKey($text, $vmap[$text], $file);
    if (count(array_filter($newKeys, fn($k) => strpos($k, 'admin_vue_') === 0)) >= $maxNew && !isset($newKeys[$text])) {
        // over limit - still need key, use text as fallback key name
        fwrite(STDERR, "WARN: max keys reached, skipping new: $text\n");
        return 'admin_vue_overflow';
    }
    while (isset(loadZh()['admin_vue_' . str_pad($keyNum, 5, '0', STR_PAD_LEFT)])) $keyNum++;
    $k = 'admin_vue_' . str_pad($keyNum, 5, '0', STR_PAD_LEFT);
    $newKeys[$text] = $k;
    $keyNum++;
    return $k;
}

function migrateFile($path, $vmap, &$newKeys, &$keyNum, $maxNew, $dryRun) {
    $fullPath = (strpos($path, ROOT) === 0) ? $path : ROOT . ltrim($path, '/');
    $content = file_get_contents($fullPath);
    $lines = explode("\n", $content);
    $out = [];
    $changed = false;
    for ($i = 0; $i < count($lines); $i++) {
        $sec = sectionAt($lines, $i);
        $newLine = migrateLine($lines[$i], $sec, $vmap, $path, $newKeys, $keyNum, $maxNew);
        if ($newLine !== $lines[$i]) $changed = true;
        $out[] = $newLine;
    }
    $result = implode("\n", $out);
    if ($changed && !$dryRun) file_put_contents($fullPath, $result);
    return $changed;
}

function appendKeys($newKeys) {
    if (!$newKeys) return;
    $en = loadEn();
    foreach (['zh_cn', 'en_us'] as $lang) {
        $path = ROOT . "data/lang/auto/{$lang}.php";
        $c = file_get_contents($path);
        $c = rtrim($c);
        $c = preg_replace('/\);\s*$/', '', $c);
        foreach ($newKeys as $text => $key) {
            if ($lang === 'zh_cn') {
                $val = $text;
            } else {
                $val = $en[$key] ?? translateToEn($text);
            }
            $c .= "\n  '" . addslashes($key) . "' => '" . addslashes($val) . "',";
        }
        $c .= "\n);\n";
        file_put_contents($path, $c);
    }
}

function translateToEn($zh) {
    $en = loadEn();
    $vmap = buildValueMap();
    // if same text exists in another key with en translation
    if (isset($vmap[$zh])) {
        foreach ($vmap[$zh] as $k) {
            if (isset($en[$k]) && $en[$k] !== $zh) return $en[$k];
        }
    }
    static $d = [
        '用户名'=>'Username','用户ID'=>'User ID','姓名'=>'Name','个人姓名'=>'Personal name',
        '内容'=>'Content','编号'=>'No.','至'=>'to','时间'=>'Time','操作'=>'Actions','状态'=>'Status',
        '已审核'=>'Approved','未审核'=>'Pending review','未通过'=>'Rejected','待审核'=>'Awaiting review',
        '你确定要删除选中项吗？'=>'Are you sure you want to delete the selected items?',
        '确定要清空用户解绑日志？'=>'Are you sure you want to clear user unbind logs?',
        '头像'=>'Avatar','身份证号'=>'ID number','认证资料'=>'Verification documents',
        '申请时间'=>'Application time','作品'=>'Portfolio','期望职位'=>'Expected position',
        '简历ID'=>'Resume ID','手机号'=>'Mobile','教育经历'=>'Education','工作经历'=>'Work experience',
        '项目经历'=>'Project experience','培训经历'=>'Training','职业技能'=>'Professional skills',
        '创建时间'=>'Created at','更新时间'=>'Updated at','基本信息'=>'Basic info',
        '完整度/状态'=>'Completeness/Status','投递岗位'=>'Applications','简历状态'=>'Resume status',
        '推荐'=>'Recommend','置顶'=>'Pin to top','更新/创建时间'=>'Updated/Created',
        '来源/IP/归属地'=>'Source/IP/Location','公司名'=>'Company name','职位名'=>'Job title',
        '职位编号'=>'Job No.','公司名称'=>'Company name','职位名称'=>'Job title','工作地区'=>'Work location',
        '职位类别'=>'Job category','待遇'=>'Salary','确定要推荐吗？'=>'Are you sure you want to recommend?',
        '发送完成'=>'Send complete','价格'=>'Price','匹配岗位'=>'Matching jobs',
        '登录账户不能为空'=>'Login account cannot be empty','密码不能为空'=>'Password cannot be empty',
        '密码长度不能小6位字符'=>'Password must be at least 6 characters',
        '请再次输入密码'=>'Please enter password again','两次密码不一致'=>'Passwords do not match',
        '用户姓名不能为空'=>'User name cannot be empty','性别不能为空'=>'Gender cannot be empty',
        '现居住地不能为空'=>'Current residence cannot be empty','出生年月不能为空'=>'Date of birth cannot be empty',
        '自我评价不能为空'=>'Self evaluation cannot be empty',
        '今天'=>'Today','最近三天'=>'Last 3 days','最近七天'=>'Last 7 days','最近半月'=>'Last 15 days',
        '最近一个月'=>'Last 30 days',
    ];
    return $d[$zh] ?? $zh;
}

if (realpath($_SERVER['SCRIPT_FILENAME'] ?? '') === __FILE__) {
// CLI
$dir = $argv[1] ?? '';
$opts = array_slice($argv, 2);
$keyStart = 1; $maxKeys = 9999; $dryRun = false;
foreach ($opts as $o) {
    if (preg_match('/--key-start=(\d+)/', $o, $m)) $keyStart = (int)$m[1];
    if (preg_match('/--max-keys=(\d+)/', $o, $m)) $maxKeys = (int)$m[1];
    if ($o === '--dry-run') $dryRun = true;
}
if (!$dir) { echo "Usage: php migrate_admin_vue.php <dir> [--key-start=N] [--max-keys=N] [--dry-run]\n"; exit(1); }

$target = ROOT . ltrim($dir, '/');
$files = [];
if (is_file($target) && substr($target, -4) === '.vue') {
    $files = [$target];
} else {
    $fullDir = rtrim($target, '/') . '/';
    $files = glob($fullDir . '*.vue') ?: [];
}
$vmap = buildValueMap();
$newKeys = [];
$keyNum = $keyStart;
$updated = 0;
foreach ($files as $f) {
    if (migrateFile(str_replace(ROOT, '', $f), $vmap, $newKeys, $keyNum, $maxKeys, $dryRun)) {
        echo "Updated: " . str_replace(ROOT, '', $f) . "\n";
        $updated++;
    }
}
echo "Updated $updated files, " . count($newKeys) . " new keys\n";
foreach ($newKeys as $t => $k) echo "  $k => $t\n";
if (!$dryRun && $newKeys) appendKeys($newKeys);
echo "Next key: admin_vue_" . str_pad($keyNum, 5, '0', STR_PAD_LEFT) . "\n";
}
