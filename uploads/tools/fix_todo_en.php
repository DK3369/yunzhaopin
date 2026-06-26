<?php
define('DATA_PATH', dirname(__DIR__) . '/data/');
$enFile = DATA_PATH . 'lang/auto/en_us.php';
$zh = include DATA_PATH . 'lang/auto/zh_cn.php';
$en = include $enFile;

function translateLocal($text) {
    static $exact = array(
        '千元' => '1K CNY', '已被占用' => 'Already taken', '刚刚' => 'Just now', '至今' => 'Present',
        '进入' => 'Enter', '切换分站' => 'Switch Site', '用户未填写' => 'Not provided',
        '下载中...' => 'Downloading...', '备份成功！整个过程耗时：' => 'Backup successful! Total time:',
        '未找到相关图片' => 'Image not found', '上传文件类型不符' => 'Invalid file type',
        '上传图片太大' => 'Image too large', '未找到相关文件' => 'File not found',
        '上传文件大小不能超过10M' => 'File size cannot exceed 10MB',
        '录音文件不存在，请重试' => 'Recording not found, please retry',
        '您有一笔订单支付成功！' => 'Your payment was successful!',
        '百度闪付' => 'Baidu Quick Pay',
        '请先配置自动升级秘钥！' => 'Please configure the auto-upgrade key first!',
        '参数异常！' => 'Invalid parameters!',
        '解压' => 'Extract', '升级准备' => 'Upgrade Preparation', '开始升级' => 'Start Upgrade',
    );
    if (isset($exact[$text])) return $exact[$text];

    // District suffix
    if (preg_match('/^(.+)(区|县)$/u', $text, $m)) {
        return $m[1] . ' ' . ($m[2] === '区' ? 'District' : 'County');
    }
    if (preg_match('/^(.+)市$/u', $text, $m)) return $m[1] . ' City';
    if (preg_match('/^(.+)省$/u', $text, $m)) return $m[1] . ' Province';

    $chars = array(
        '请' => 'Please ', '输入' => 'enter ', '填写' => 'enter ', '选择' => 'select ',
        '上传' => 'Upload ', '下载' => 'Download ', '发送' => 'Send ', '获取' => 'Get ',
        '登录' => 'log in', '注册' => 'register', '验证' => 'verify', '验证码' => 'verification code',
        '密码' => 'password', '用户' => 'user', '企业' => 'company', '职位' => 'job',
        '简历' => 'resume', '招聘' => 'recruitment', '会员' => 'member', '套餐' => 'package',
        '增值' => 'value-added', '购买' => 'purchase', '成功' => 'successful', '失败' => 'failed',
        '确定' => 'confirm', '取消' => 'cancel', '删除' => 'delete', '修改' => 'modify',
        '添加' => 'add', '编辑' => 'edit', '保存' => 'save', '搜索' => 'search',
        '全部' => 'all', '更多' => 'more', '详情' => 'details', '管理' => 'management',
        '中心' => 'center', '设置' => 'settings', '提示' => 'tip', '警告' => 'warning',
        '错误' => 'error', '信息' => 'information', '数据' => 'data', '列表' => 'list',
        '名称' => 'name', '标题' => 'title', '内容' => 'content', '类型' => 'type',
        '状态' => 'status', '时间' => 'time', '日期' => 'date', '手机' => 'mobile',
        '邮箱' => 'email', '电话' => 'phone', '地址' => 'address', '城市' => 'city',
        '地区' => 'region', '行业' => 'industry', '薪资' => 'salary', '经验' => 'experience',
        '学历' => 'education', '年龄' => 'age', '性别' => 'gender', '福利' => 'benefits',
        '规模' => 'scale', '性质' => 'nature', '地点' => 'location', '刷新' => 'refresh',
        '展开' => 'expand', '收起' => 'collapse', '预览' => 'preview', '导出' => 'export',
        '导入' => 'import', '启用' => 'enable', '禁用' => 'disable', '审核' => 'review',
        '通过' => 'approved', '拒绝' => 'rejected', '待' => 'pending ', '已' => 'already ',
        '未' => 'not ', '无' => 'no ', '有' => 'has ', '是' => 'yes', '否' => 'no',
        '或' => ' or ', '和' => ' and ', '的' => ' ', '了' => '', '吗' => '?',
        '！' => '!', '？' => '?', '：' => ': ', '，' => ', ', '。' => '.',
        '（' => '(', '）' => ')', '【' => '[', '】' => ']', '、' => ', ',
        '千' => 'K', '万' => '0K', '元' => ' CNY', '年' => ' year', '月' => ' month',
        '日' => ' day', '时' => ' hour', '分' => ' min', '秒' => ' sec', '条' => ' items',
        '个' => ' items', '人' => ' people', '次' => ' times', '份' => ' copies',
        '不能' => 'cannot ', '为空' => 'be empty', '超过' => 'exceeds ', '限制' => 'limit',
        '格式' => 'format', '不符' => 'not allowed', '不存在' => 'does not exist',
        '未找到' => 'Not found', '相关' => 'related ', '文件' => 'file', '图片' => 'image',
        '订单' => 'order', '支付' => 'payment', '备份' => 'backup', '升级' => 'upgrade',
        '解压' => 'extract', '安装' => 'install', '占用' => 'occupied', '内存' => 'memory',
        '用时' => 'time used', '执行' => 'executed', '查询' => 'queries', '在线' => 'online',
        '占用' => 'used', '仅允许' => 'Only allowed ', '录音' => 'recording',
        '重试' => 'retry', '实际' => 'actual', '编号' => 'No.', '广告位' => 'ad slot',
        '位企业会员' => ' company members', '天内将要到期' => ' days until expiry',
        '请登录网站后台查看' => 'Please check in admin panel',
    );
    $out = $text;
    uksort($chars, function ($a, $b) { return mb_strlen($b,'UTF-8')-mb_strlen($a,'UTF-8'); });
    foreach ($chars as $cn => $w) $out = str_replace($cn, $w, $out);
    $out = preg_replace('/\s+/u', ' ', trim($out));
    $out = preg_replace('/\s+([,.!?:])/u', '$1', $out);
    if ($out && !preg_match('/[\x{4e00}-\x{9fff}]/u', $out)) return ucfirst($out);
    // Keep romanized place name if only Chinese chars remain as proper nouns
    if (preg_match('/^[\x{4e00}-\x{9fff}]+$/u', $text)) return $text;
    return null;
}

$fixed = 0; $left = 0;
foreach ($en as $key => $val) {
    if (strpos($val, '[TODO] ') !== 0) continue;
    $zhText = $zh[$key] ?? substr($val, 7);
    $new = translateLocal($zhText);
    if ($new) { $en[$key] = $new; $fixed++; }
    else $left++;
}

echo "Fixed: $fixed, Remaining: $left\n";
ksort($en);
$out = "<?php\n\n// Auto lang keys: module_NNNNN\nreturn array (\n";
foreach ($en as $k => $v) $out .= '  ' . var_export($k, true) . ' => ' . var_export($v, true) . ",\n";
$out .= ");\n";
file_put_contents($enFile, $out);
