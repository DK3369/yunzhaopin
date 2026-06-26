<?php
/**
 * Translate Chinese HTML comments in .htm templates to English.
 *
 *   php tools/translate_htm_comments_to_en.php --build-cache
 *   php tools/translate_htm_comments_to_en.php
 *   php tools/translate_htm_comments_to_en.php --finish
 *   php tools/translate_htm_comments_to_en.php --limit=50   # process at most N files per run
 */
define('ROOT', dirname(__DIR__) . '/');
$dryRun = in_array('--dry-run', $argv ?? array(), true);
$buildCache = in_array('--build-cache', $argv ?? array(), true);
$finish = in_array('--finish', $argv ?? array(), true);
$limit = 0;
foreach ($argv ?? array() as $arg) {
    if (preg_match('/^--limit=(\d+)$/', $arg, $m)) $limit = (int) $m[1];
}

$cacheFile = __DIR__ . '/comment_translation_cache.json';
$cache = is_file($cacheFile) ? json_decode(file_get_contents($cacheFile), true) : array();
if (!is_array($cache)) $cache = array();

$skip = '/data\/lang|tools\/|install\/|ueditor|umeditor|wangeditor/i';

function cacheKey($body) { return md5(preg_replace('/\s+$/s', '', trim($body))); }

function googleTranslateBatch(array $texts)
{
    if (empty($texts)) return array();
    $mh = curl_multi_init();
    $handles = array();
    foreach ($texts as $i => $text) {
        $url = 'https://translate.googleapis.com/translate_a/single?client=gtx&sl=zh-CN&tl=en&dt=t&q='
            . rawurlencode(mb_substr(trim($text), 0, 450));
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
        $t = (is_array($json) && isset($json[0][0][0])) ? $json[0][0][0] : null;
        $out[$i] = $t ? preg_replace('/\s+/u', ' ', trim($t)) : null;
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

/** Common HTM comment phrases (exact match after trim). */
function htmCommentGlossary()
{
    static $map = null;
    if ($map !== null) return $map;
    $pairs = array(
        '职位列表 start' => 'Job list start',
        '职位列表 end' => 'Job list end',
        '职位列表 end--' => 'Job list end',
        '天眼查 Start' => 'Tianyancha Start',
        '天眼查 End' => 'Tianyancha End',
        '普通模式' => 'Normal mode',
        '扫码查看联系方式' => 'Scan QR code to view contact',
        '内容结束' => 'End of content',
        '底部footer' => 'Footer',
        '头部结束' => 'Header end',
        '未搜索到' => 'No results found',
        '基本信息' => 'Basic information',
        '自我评价' => 'Self evaluation',
        '粘贴简历' => 'Paste resume',
        '举报弹窗' => 'Report dialog',
        '作品案例' => 'Portfolio',
        '右侧开始' => 'Right sidebar start',
        '右侧结束' => 'Right sidebar end',
        '下面为自动推送功能' => 'Auto push feature below',
        '请不要删除我' => 'Do not remove',
        '导航开关按钮预留' => 'Nav toggle button placeholder',
        '有提示时显示' => 'Show when there is a tip',
        '无提示时显示' => 'Show when there is no tip',
        '路由渲染组件' => 'Route render component',
        '弹窗' => 'Dialog',
        '广告' => 'Advertisement',
        '未完善情况提示' => 'Incomplete profile prompt',
        '备用简历' => 'Backup resume',
        '绑定手机弹出框' => 'Bind mobile dialog',
        '绑定邮箱弹出框' => 'Bind email dialog',
        '弹出框 end' => 'Dialog end',
        '不强制但是第一次认证' => 'Optional first-time verification',
        '提示部分  end' => 'Prompt section end',
        '必填' => 'Required',
        '购买-------------------------------------------------end' => 'Purchase end',
        '图片验证码弹框  End' => 'Image captcha dialog End',
        '其他登录方式' => 'Other login methods',
        '滚动' => 'Scroll',
        '广告位 Start' => 'Ad slot Start',
        '广告位 End' => 'Ad slot End',
        '投诉顾问弹出框' => 'Complaint advisor dialog',
        '刷新职位提示弹出框' => 'Refresh job prompt dialog',
        '年度报告提示' => 'Annual report prompt',
        '提示弹出框 end' => 'Prompt dialog end',
        '引导完善信息' => 'Guide to complete profile',
        '页面头部返回按钮' => 'Page header back button',
        'Substation 分站地区' => 'Substation region',
        'Sortpr 企业性质弹出框' => 'Company type dialog',
        '简历更多筛选' => 'Resume more filters',
        '企业规模选择器' => 'Company size picker',
        '兼职类型选择器' => 'Part-time type picker',
        'form表单区域' => 'Form area',
        '求职意向弹出框 end' => 'Job intention dialog end',
        '基本信息出框 end' => 'Basic info dialog end',
        '工作经历弹出框' => 'Work experience dialog',
        '工作经历弹出框 end' => 'Work experience dialog end',
        '教育经历弹出框' => 'Education dialog',
        '教育经历弹出框 end' => 'Education dialog end',
        '培训经历弹出框' => 'Training dialog',
        '培训经历弹出框 end' => 'Training dialog end',
    );
    $map = $pairs;
    return $map;
}

function translateBody($body, &$cache, $allowApi)
{
    $body = trim($body);
    if ($body === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $body)) return $body;
    $glossary = htmCommentGlossary();
    if (isset($glossary[$body])) {
        $cache[cacheKey($body)] = $glossary[$body];
        return $glossary[$body];
    }
    $key = cacheKey($body);
    if (isset($cache[$key]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$key])) {
        return preg_replace('/\s+/u', ' ', trim($cache[$key]));
    }
    if ($allowApi) {
        $batch = googleTranslateBatch(array($body));
        $out = $batch[0] ?? null;
        if ($out && !preg_match('/[\x{4e00}-\x{9fff}]/u', $out)) {
            $cache[$key] = $out;
            return $out;
        }
    }
    return $body;
}

function iterHtmFiles()
{
    global $skip;
    $base = ROOT . 'app/template';
    if (!is_dir($base)) return;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
    foreach ($it as $f) {
        if (!$f->isFile() || strtolower($f->getExtension()) !== 'htm') continue;
        $rel = str_replace(ROOT, '', $f->getPathname());
        if (preg_match($skip, $rel)) continue;
        yield $f->getPathname();
    }
}

function collectHtmCommentBodies($path, &$bodies)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return;
    if (!preg_match_all('/<!--([\s\S]*?)-->/u', $content, $m)) return;
    foreach ($m[1] as $inner) {
        $inner = trim($inner);
        if ($inner === '' || !preg_match('/[\x{4e00}-\x{9fff}]/u', $inner)) continue;
        $bodies[cacheKey($inner)] = $inner;
    }
}

function transformHtmFile($path, &$cache, $allowApi, &$stats)
{
    $content = file_get_contents($path);
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $content)) return $content;
    $changed = false;
    $new = preg_replace_callback('/<!--([\s\S]*?)-->/u', function ($m) use (&$cache, $allowApi, &$stats, &$changed) {
        $inner = $m[1];
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $inner)) return $m[0];
        $translated = translateBody($inner, $cache, $allowApi);
        if ($translated === $inner) return $m[0];
        $changed = true;
        $stats['comments']++;
        if (preg_match('/[\x{4e00}-\x{9fff}]/u', $translated)) $stats['remaining']++;
        return '<!--' . $translated . '-->';
    }, $content);
    if ($changed) $stats['files']++;
    return $new;
}

if ($buildCache || $finish) {
    $bodies = array();
    foreach (iterHtmFiles() as $file) collectHtmCommentBodies($file, $bodies);
    $pending = array();
    foreach ($bodies as $key => $body) {
        if (isset($cache[$key]) && !preg_match('/[\x{4e00}-\x{9fff}]/u', $cache[$key])) continue;
        $pending[$key] = $body;
    }
    echo 'HTM unique comments: ' . count($bodies) . ', need API: ' . count($pending) . "\n";
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
        if ((($i / 25) + 1) % 10 === 0 || $i + 25 >= count($keys)) {
            saveCache($cacheFile, $cache);
            echo 'Cached ' . min($i + 25, count($keys)) . ' / ' . count($pending) . "\n";
        }
    }
    saveCache($cacheFile, $cache);
    echo 'Cache entries: ' . count($cache) . "\n";
    if ($buildCache && !$finish) exit;
}

$stats = array('files' => 0, 'comments' => 0, 'remaining' => 0);
$allowApiOnApply = $finish;
$processed = 0;
foreach (iterHtmFiles() as $file) {
    if ($limit > 0 && $processed >= $limit) break;
    $rel = str_replace(ROOT, '', $file);
    $orig = file_get_contents($file);
    if (!preg_match('/<!--[\s\S]*?[\x{4e00}-\x{9fff}][\s\S]*?-->/u', $orig)) continue;
    $new = transformHtmFile($file, $cache, $allowApiOnApply, $stats);
    if ($new !== $orig) {
        echo ($dryRun ? '[dry] ' : '') . "$rel\n";
        if (!$dryRun) file_put_contents($file, $new);
        $processed++;
    }
}
saveCache($cacheFile, $cache);
echo "\nFiles: {$stats['files']}, comments: {$stats['comments']}, still Chinese: {$stats['remaining']}\n";
