<?php
// Fix key conflicts: Tier1 used wap_01284-01305 and wap_01548+
$fixes = [
    // wrong => new key => zh => en
    'wap_01284' => ['wap_01570', '提问', 'Ask'],
    'wap_01285' => ['wap_01571', '关注企业', 'Follow company'],
    'wap_01286' => ['wap_01572', '生成海报', 'Generate poster'],
    'wap_01287' => ['wap_01573', '距离', 'Distance'],
    'wap_01288' => ['wap_01574', '注册地址：', 'Registered address:'],
    'wap_01289' => ['wap_01575', '已安全认证', 'Verified'],
    'wap_01290' => ['wap_01576', '优质名企', 'Premium employer'],
    'wap_01291' => ['wap_01577', '正在招聘职位', 'Open positions'],
    'wap_01292' => ['wap_01578', '阅读', 'views'],
    'wap_01293' => ['wap_01579', '我的主页', 'My page'],
    'wap_01294' => ['wap_01580', '搜一搜感兴趣的问题', 'Search questions you are interested in'],
    'wap_01295' => ['wap_01581', '达人推荐', 'Expert picks'],
    'wap_01299' => ['wap_01582', '来自', 'From'],
    'wap_01300' => ['wap_01583', '的提问', "'s question"],
    'wap_01548' => ['wap_01584', '之后你可以在', 'You can switch in'],
    'wap_01549' => ['wap_01585', '中切换', ' later'],
    'wap_01550' => ['wap_01586', '密码长度应为6-20', 'Password must be 6-20'],
    'wap_01551' => ['wap_01587', '！', '!'],
    'wap_01552' => ['wap_01588', '刷信誉、淘宝刷钻、YY网络兼职、加YY联系的职位都是骗子！收取费用或押金的都有欺诈嫌疑，请警惕！', 'Beware of scam jobs asking for fees or deposits via credit farming, Taobao brushing, YY part-time, etc.'],
    'wap_01553' => ['wap_01589', '我们将在第一时间及时回复您的反馈，如您的问题比较紧急，请致电服务热线!', 'We will reply to your feedback as soon as possible. For urgent issues, please call our hotline!'],
];

$base = dirname(__DIR__) . '/app/template/wap/';
$it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($base));
foreach ($it as $f) {
    if (!$f->isFile() || $f->getExtension() !== 'htm') continue;
    $path = $f->getPathname();
    $c = file_get_contents($path);
    $orig = $c;
    foreach ($fixes as $old => [$new, $zh, $en]) {
        $c = str_replace("key='{$old}'", "key='{$new}'", $c);
        $c = str_replace('key="'.$old.'"', 'key="'.$new.'"', $c);
    }
    if ($c !== $orig) file_put_contents($path, $c);
}

foreach (['zh_cn.php', 'en_us.php', 'aliases.php'] as $lf) {
    $path = dirname(__DIR__) . "/data/lang/auto/$lf";
    $content = file_get_contents($path);
    $add = '';
    foreach ($fixes as $old => [$new, $zh, $en]) {
        if (strpos($content, "'$new'") !== false) continue;
        $zhE = addcslashes($zh, "'\\");
        if ($lf === 'zh_cn.php') $add .= "  '$new' => '$zhE',\n";
        elseif ($lf === 'en_us.php') {
            $enVal = $fixes[$old][2];
            $enE = addcslashes($enVal, "'\\");
            $add .= "  '$new' => '$enE',\n";
        } else {
            $add .= "  '$zhE' => '$new',\n";
        }
    }
    if ($add) {
        $content = preg_replace('/\);\s*$/', rtrim($add) . "\n);", $content);
        file_put_contents($path, $content);
    }
}
echo "Fixed " . count($fixes) . " key conflicts (wap_01570+)\n";
