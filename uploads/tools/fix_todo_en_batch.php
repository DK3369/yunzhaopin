<?php
/**
 * Replace [TODO] Chinese fallbacks in en_us.php with rough English.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$enFile = DATA_PATH . 'lang/auto/en_us.php';
$en = include $enFile;
$fixed = 0;

foreach ($en as $key => $enVal) {
    if (strpos($enVal, '[TODO]') !== 0) {
        continue;
    }
    $zhText = trim(substr($enVal, 6));
    if ($zhText === '') {
        continue;
    }
    // Strip vue/smarty fragments — keep translatable core
    $core = preg_replace('/\{\{[^}]+\}\}/u', '', $zhText);
    $core = preg_replace('/\{yun:[^}]+\}/u', '', $core);
    $core = trim($core);
    if ($core === '') {
        $en[$key] = $zhText;
        $fixed++;
        continue;
    }
    $out = $core;
    $map = array(
        '投诉简历' => 'Reported resume', '删除成功' => 'deleted successfully', '删除失败' => 'delete failed',
        '数据已全部生成' => 'All data generated', '数据生成中' => 'Generating data', '还剩余' => 'remaining',
        '条' => ' items', '年' => ' years', '待遇' => 'Benefits', '请填写' => 'Please enter ',
        '请输入' => 'Please enter ', '请选择' => 'Please select ', '成功' => 'successful', '失败' => 'failed',
        '参数错误' => 'Invalid parameter', '操作' => 'operation', '用户' => 'user', '企业' => 'company',
        '职位' => 'job', '简历' => 'resume', '会员' => 'member', '订单' => 'order', '充值' => 'recharge',
        '权限' => 'permission', '非法操作' => 'Invalid operation', '暂无' => 'none', '审核' => 'review',
        '设置' => 'settings', '取消' => 'cancel', '推荐' => 'recommend', '置顶' => 'top', '紧急招聘' => 'urgent hiring',
        '浏览记录' => 'browse log', '申请记录' => 'application log', '邀请面试' => 'interview invitation',
        '！' => '!', '？' => '?', '，' => ', ', '。' => '.', '：' => ': ', '（' => ' (', '）' => ')',
        '的' => ' ', '了' => '', '是否' => 'whether to ', '不能' => 'cannot ', '已经' => 'already ',
        'ID:' => 'ID:', 'ID：' => 'ID:',
    );
    uasort($map, function ($a, $b) {
        return mb_strlen($b, 'UTF-8') - mb_strlen($a, 'UTF-8');
    });
    foreach ($map as $cn => $w) {
        $out = str_replace($cn, $w, $out);
    }
    $out = preg_replace('/\s+/u', ' ', trim($out));
    if (preg_match('/[\x{4e00}-\x{9fff}]/u', $out)) {
        // Last resort: keep Latin parts, drop remaining CJK
        $out = preg_replace('/[\x{4e00}-\x{9fff}]+/u', '', $out);
        $out = preg_replace('/\s+/u', ' ', trim($out));
    }
    if ($out === '') {
        $out = 'Translation pending';
    }
    // Restore vue placeholders from original
    if (preg_match_all('/\{\{[^}]+\}\}/u', $zhText, $vm)) {
        foreach ($vm[0] as $v) {
            if (strpos($out, $v) === false && preg_match('/[\x{4e00}-\x{9fff}]/u', $v)) {
                $out .= ' ' . $v;
            }
        }
    }
    $en[$key] = $out;
    $fixed++;
}

function writeLangFile($path, $data)
{
    ksort($data);
    $out = "<?php\n\n// Auto lang keys: module_NNNNN\nreturn array (\n";
    foreach ($data as $k => $v) {
        $out .= '  ' . var_export($k, true) . ' => ' . var_export($v, true) . ",\n";
    }
    $out .= ");\n";
    file_put_contents($path, $out);
}

writeLangFile($enFile, $en);
echo "Fixed TODO entries: $fixed\n";
