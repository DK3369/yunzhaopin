<?php
/**
 * One-shot fix for remaining bare Chinese in wap/member.
 */
define('ROOT', dirname(__DIR__) . '/');

$files = array(
    'wap/member/wap.controller.php',
    'wap/member/model/index.class.php',
    'wap/member/model/com.class.php',
);

function w($s) {
    $s = str_replace("'", "\\'", $s);
    return "yun_auto_t('" . $s . "')";
}

$replacements = array(
    "array('info' => '请先创建一份简历！', 'url' => 'index.php?c=addresume', 'btn' => '立即创建')" =>
        "array('info' => " . w('请先创建一份简历！') . ", 'url' => 'index.php?c=addresume', 'btn' => " . w('立即创建') . ")",
    "array('info' => '请先完善信息！', 'url' => 'index.php?c=info', 'btn' => '立即完善')" =>
        "array('info' => " . w('请先完善信息！') . ", 'url' => 'index.php?c=info', 'btn' => " . w('立即完善') . ")",
    "'请先完善个人资料'" => w('请先完善个人资料'),
    "\$headertitle='工作经历'" => "\$headertitle=" . w('工作经历'),
    "\$headertitle='教育经历'" => "\$headertitle=" . w('教育经历'),
    "\$headertitle='项目经历'" => "\$headertitle=" . w('项目经历'),
    "\$headertitle='培训经历'" => "\$headertitle=" . w('培训经历'),
    "\$headertitle='职业技能'" => "\$headertitle=" . w('职业技能'),
    "\$headertitle='其他信息'" => "\$headertitle=" . w('其他信息'),
    "\$headertitle='自我评价'" => "\$headertitle=" . w('自我评价'),
    "\$headertitle='作品案例'" => "\$headertitle=" . w('作品案例'),
    "\$headertitle='粘贴简历'" => "\$headertitle=" . w('粘贴简历'),
    "\$headertitle=\"手机认证\"" => "\$headertitle=" . w('手机认证'),
    "\$headertitle=\"邮箱认证\"" => "\$headertitle=" . w('邮箱认证'),
    '"暂未开通手机支付，请移步至电脑端充值！"' => w('暂未开通手机支付，请移步至电脑端充值！'),
    '"充值".$this->config[\'integral_pricename\']' => w('充值') . '.$this->config[\'integral_pricename\']',
    '"订单不存在！"' => w('订单不存在！'),
    "'参数错误，请重试！'" => w('参数错误，请重试！'),
    "'下单成功，请付款！'" => w('下单成功，请付款！'),
    "'提交失败，请重新提交订单！'" => w('提交失败，请重新提交订单！'),
    "'参数错误'" => w('参数错误'),
    '"下单成功，请付款！"' => w('下单成功，请付款！'),
);

foreach ($files as $rel) {
    $path = ROOT . $rel;
    $content = file_get_contents($path);
    $orig = $content;
    foreach ($replacements as $from => $to) {
        $content = str_replace($from, $to, $content);
    }
    if ($content !== $orig) {
        file_put_contents($path, $content);
        echo "FIXED: $rel\n";
    }
}
echo "Done.\n";
