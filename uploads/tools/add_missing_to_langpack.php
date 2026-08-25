<?php
/**
 * Add missing i18n entries to zh_cn.php / en_us.php
 * Usage: php tools/add_missing_to_langpack.php [--dry-run] [--allow-full]
 *
 * Full batch write disabled unless --allow-full is passed.
 */
define('ROOT', dirname(__DIR__) . '/');
define('DATA_PATH', ROOT . 'data/');

$dryRun = in_array('--dry-run', $argv ?? array(), true);
$allowFull = in_array('--allow-full', $argv ?? array(), true);

if (!$dryRun && !$allowFull) {
    fwrite(STDERR, "ERROR: Full batch add disabled. Use --dry-run to preview or --allow-full to override.\n");
    exit(1);
}

$zhFile = DATA_PATH . 'lang/auto/zh_cn.php';
$enFile = DATA_PATH . 'lang/auto/en_us.php';
$missingFile = ROOT . 'tools/missing_i18n.json';

if (!is_file($missingFile)) {
    fwrite(STDERR, "Run export_missing_i18n.php first.\n");
    exit(1);
}

$zh = include $zhFile;
$en = include $enFile;
$zhByValue = array_flip($zh);
$missing = json_decode(file_get_contents($missingFile), true);

// Pre-sort zh phrases by length for fast longest-match
$zhPhrases = array_keys($zhByValue);
usort($zhPhrases, function ($a, $b) {
    return mb_strlen($b, 'UTF-8') - mb_strlen($a, 'UTF-8');
});

// Max ID per module
$modMax = array();
foreach (array_keys($zh) as $key) {
    if (preg_match('/^([a-z_]+)_(\d+)$/', $key, $m)) {
        $modMax[$m[1]] = max($modMax[$m[1]] ?? 0, (int)$m[2]);
    }
}

function nextKey($module, &$modMax) {
    $modMax[$module] = ($modMax[$module] ?? 0) + 1;
    return $module . '_' . str_pad($modMax[$module], 5, '0', STR_PAD_LEFT);
}

function translateToEn($zh, $zhPack, $enPack, $zhByValue, $zhPhrases) {
    // Colon / label suffix variants: 用户名： -> Username:
    if (preg_match('/^(.+?)([：:])$/u', $zh, $m)) {
        $base = $m[1];
        if (isset($zhByValue[$base])) {
            $en = $enPack[$zhByValue[$base]];
            return rtrim($en) . ':';
        }
    }

    // Prefix patterns
    $patterns = array(
        '/^请填写(.+)$/u' => 'Please enter $1',
        '/^请输入(.+)$/u' => 'Please enter $1',
        '/^请选择(.+)$/u' => 'Select $1',
        '/^请上传(.+)$/u' => 'Please upload $1',
        '/^请先(.+)$/u' => 'Please $1 first',
        '/^是否(.+)$/u' => '$1?',
        '/^您确定要(.+)吗？$/u' => 'Are you sure you want to $1?',
        '/^你确定要(.+)吗？$/u' => 'Are you sure you want to $1?',
        '/^(.+)成功！$/u' => '$1 successful!',
        '/^(.+)成功$/u' => '$1 successful',
        '/^(.+)失败！$/u' => '$1 failed!',
        '/^(.+)失败$/u' => '$1 failed',
    );
    foreach ($patterns as $pat => $tpl) {
        if (preg_match($pat, $zh, $m)) {
            $inner = translateToEn($m[1], $zhPack, $enPack, $zhByValue, $zhPhrases);
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $inner)) {
                return preg_replace('/\$(\d+)/', $inner, $tpl);
            }
        }
    }

    // Word dictionary (common UI)
    static $dict = null;
    if ($dict === null) {
        $dict = array(
            '欢迎登录' => 'Welcome, please log in',
            '忘记密码' => 'Forgot Password',
            'QQ登录' => 'QQ Login',
            '微信登录' => 'WeChat Login',
            '微信扫一扫登录' => 'Scan with WeChat to log in',
            '普通登录' => 'Standard Login',
            '短信登录' => 'SMS Login',
            '扫码登录' => 'QR Code Login',
            '安全验证' => 'Security Verification',
            '激活邮箱' => 'Activate Email',
            '发送动态码' => 'Send Verification Code',
            '发送激活邮件' => 'Send Activation Email',
            '请填写短信动态码' => 'Please enter SMS verification code',
            '请填写验证码' => 'Please enter verification code',
            '二维码失效点击刷新' => 'QR code expired. Click to refresh',
            '进入管理中心' => 'Go to Dashboard',
            '购买增值包' => 'Purchase Value-added Package',
            '下载增值包' => 'Download Value-added Package',
            '职位增值包' => 'Job Value-added Package',
            '综合增值包' => 'Comprehensive Value-added Package',
            '兼职增值包' => 'Part-time Value-added Package',
            '简历超值套餐' => 'Resume Premium Package',
            '智能推广' => 'Smart Promotion',
            '收到简历' => 'Resumes Received',
            '生成海报' => 'Generate Poster',
            '下载海报' => 'Download Poster',
            '生成招聘海报' => 'Generate Recruitment Poster',
            '请先发布职位' => 'Please post a job first',
            '请选择海报职位' => 'Select a job for the poster',
            '用户名' => 'Username',
            '密码' => 'Password',
            '地区' => 'Region',
            '地点' => 'Location',
            '行业' => 'Industry',
            '性质' => 'Nature',
            '规模' => 'Scale',
            '福利' => 'Benefits',
            '薪资' => 'Salary',
            '立即注册' => 'Register Now',
            '手机验证码' => 'SMS Verification Code',
            '名称' => 'Name',
            '状态' => 'Status',
            '说明' => 'Description',
            '域名备注' => 'Domain Note',
            '分站形式' => 'Sub-site Type',
            '绑定域名' => 'Bound Domain',
            '分站目录' => 'Sub-site Directory',
            '请输入内容' => 'Please enter content',
            '请选择' => 'Select',
            '下一步' => 'Next',
            '上一步' => 'Previous',
            '提交' => 'Submit',
            '取消' => 'Cancel',
            '确定' => 'Confirm',
            '删除' => 'Delete',
            '编辑' => 'Edit',
            '添加' => 'Add',
            '保存' => 'Save',
            '搜索' => 'Search',
            '返回' => 'Back',
            '更多' => 'More',
            '全部' => 'All',
            '不限' => 'Unlimited',
            '加载更多' => 'Load More',
            '选择地区' => 'Select Region',
            '请选择县/区' => 'Select county/district',
            '请输入关键字进行过滤' => 'Enter keywords to filter',
            '紧急招聘' => 'Urgent Hiring',
            '应聘简历' => 'Application Resume',
            '用户登录' => 'User Login',
            '查看简历详细信息' => 'View Resume Details',
            '招聘人数' => 'Number of Openings',
            '语言要求' => 'Language Requirements',
            '手机也能找工作' => 'Find jobs on your phone',
            '海量职位 让求职更简单' => 'Massive job listings make job hunting easier',
            '您的建议让我们每天变的更好' => 'Your feedback helps us improve every day',
            '请输入您的姓名' => 'Please enter your name',
            '1. 选择海报上展示的职位' => '1. Select the job to display on the poster',
            '2. 在喜欢的图片下方点击生成海报' => '2. Click Generate Poster below your preferred image',
            '刚刚' => 'Just now',
            '至今' => 'Present',
            '进入' => 'Enter',
            '切换分站' => 'Switch Site',
            '用户未填写' => 'Not provided',
            '下载中...' => 'Downloading...',
            '下载中' => 'Downloading',
            '您已成功退出！' => 'You have logged out successfully!',
            '退出失败！' => 'Logout failed!',
            '面试时间不能为空！' => 'Interview time cannot be empty!',
            '电话格式错误！' => 'Invalid phone number format!',
            '面试地点不能为空！' => 'Interview location cannot be empty!',
            '您已成功邀请！' => 'Invitation sent successfully!',
            '发布职位才可以邀请面试！' => 'Post a job before inviting to interview!',
            '完成相关认证才能发布职位！' => 'Complete verification before posting jobs!',
            '套餐不足，请先购买会员！' => 'Insufficient package. Please purchase membership first!',
            '评论留言内容不能为空！' => 'Comment cannot be empty!',
            '验证码不能为空！' => 'Verification code cannot be empty!',
            '咨询内容不能为空！' => 'Inquiry content cannot be empty!',
            '您最多只能选择五项！' => 'You can select up to 5 items!',
            '您最多只能选择五个城市！' => 'You can select up to 5 cities!',
            '请选择职位类别！' => 'Select a job category!',
            '你还没有登录！' => 'You are not logged in!',
            '请选择举报理由！' => 'Select a report reason!',
            '验证码不正确！' => 'Incorrect verification code!',
            '您已经举报过该用户！' => 'You have already reported this user!',
            '举报成功！' => 'Report submitted successfully!',
            '举报失败！' => 'Report failed!',
            '您还没有登录，请先登录！' => 'Please log in first!',
            '请正确填写兑换数量！' => 'Please enter a valid redemption quantity!',
            '超出限购数量,请正确填写！' => 'Exceeds purchase limit. Please enter a valid quantity!',
            '超出库存数量,请正确填写！' => 'Exceeds stock. Please enter a valid quantity!',
            '联系人或联系电话不能为空！' => 'Contact name or phone cannot be empty!',
            '联系电话格式不正确请正确填写！' => 'Invalid contact phone format!',
            '请填写收货地址信息！' => 'Please enter shipping address!',
            '请输入密码！' => 'Please enter password!',
            '最多只能选择' => 'You can select up to ',
            '个类别哦' => ' categories',
            '参加过' => 'Participated in ',
            '份工作' => ' jobs',
            '涉及' => ' involving ',
            '等岗位' => ' and other positions',
            '学历 · ' => 'Education · ',
            ' · 毕业于' => ' · Graduated in ',
            '年' => ' year',
        );
    }
    if (isset($dict[$zh])) {
        return $dict[$zh];
    }

    // Longest-match phrase replacement from existing pack
    $best = '';
    $bestLen = 0;
    foreach ($zhPhrases as $val) {
        if (mb_strlen($val, 'UTF-8') < 2) continue;
        if (mb_strpos($zh, $val, 0, 'UTF-8') !== false) {
            $best = $val;
            $bestLen = mb_strlen($val, 'UTF-8');
            break;
        }
    }
    if ($bestLen > 0) {
        $enPart = $enPack[$zhByValue[$best]];
        $result = str_replace($best, $enPart, $zh);
        if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $result)) {
            return $result;
        }
        // partial: replace known part only
        $rest = str_replace($best, '', $zh);
        if ($rest !== $zh) {
            $restEn = translateToEn(trim($rest), $zhPack, $enPack, $zhByValue, $zhPhrases);
            if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $restEn)) {
                return trim($enPart . ' ' . $restEn);
            }
        }
    }

    // Character-level dict for remaining fragments
    static $chars = null;
    if ($chars === null) {
        $chars = array(
            '请' => 'Please ', '输入' => 'enter ', '填写' => 'enter ', '选择' => 'select ',
            '上传' => 'upload ', '下载' => 'download ', '发送' => 'send ', '获取' => 'get ',
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
        );
    }
    $out = $zh;
    uksort($chars, function ($a, $b) {
        return mb_strlen($b, 'UTF-8') - mb_strlen($a, 'UTF-8');
    });
    foreach ($chars as $cn => $enWord) {
        $out = str_replace($cn, $enWord, $out);
    }
    $out = preg_replace('/\s+/u', ' ', trim($out));
    $out = preg_replace('/\s+([,.!?:])/u', '$1', $out);
    if ($out !== '' && !preg_match('/[\x{4e00}-\x{9fff}]/u', $out)) {
        return ucfirst($out);
    }

    return null;
}

function apiTranslate($zh) {
    return null; // disabled for batch speed; char-dict fallback used instead
}

$added = 0;
$skipped = 0;
$apiUsed = 0;
$stillChinese = array();

foreach ($missing as $text => $info) {
    if (isset($zhByValue[$text])) {
        $skipped++;
        continue;
    }

    $module = $info['module'] ?? 'common';
    if (!isset($modMax[$module])) {
        $module = 'common';
    }

    $enText = translateToEn($text, $zh, $en, $zhByValue, $zhPhrases);
    if ($enText === null || preg_match('/[\x{4e00}-\x{9fff}]/u', $enText)) {
        $enText = apiTranslate($text);
        if ($enText) $apiUsed++;
    }
    if ($enText === null || preg_match('/[\x{4e00}-\x{9fff}]/u', $enText)) {
        $stillChinese[] = $text;
        $enText = '[TODO] ' . $text; // fallback marker
    }

    $key = nextKey($module, $modMax);
    $zh[$key] = $text;
    $en[$key] = $enText;
    $zhByValue[$text] = $key;
    $added++;
}

echo "Added: $added, Skipped: $skipped, API calls: $apiUsed, TODO: " . count($stillChinese) . "\n";

if ($dryRun) {
    echo "Dry run - no files written.\n";
    exit(0);
}

function writeLangFile($path, $data) {
    ksort($data);
    $out = "<?php\n\n// Auto lang keys: module_NNNNN\nreturn array (\n";
    foreach ($data as $k => $v) {
        $out .= '  ' . var_export($k, true) . ' => ' . var_export($v, true) . ",\n";
    }
    $out .= ");\n";
    file_put_contents($path, $out);
}

writeLangFile($zhFile, $zh);
writeLangFile($enFile, $en);

if (!empty($stillChinese)) {
    file_put_contents(ROOT . 'tools/i18n_todo.json', json_encode($stillChinese, JSON_UNESCAPED_UNICODE | JSON_PRETTY_PRINT));
    echo "TODO list: tools/i18n_todo.json\n";
}

echo "Done. Total keys: " . count($zh) . "\n";
