<?php
define('ROOT', dirname(__DIR__) . '/');
$en = include ROOT . 'data/lang/auto/en_us.php';
$zh = include ROOT . 'data/lang/auto/zh_cn.php';
$dict = [
    '备注' => 'Remarks', '绑定用户' => 'Bound user', '绑定时间' => 'Bind time', '排序' => 'Sort',
    '请填写登录用户名' => 'Please enter login username', 'LOGO预览地址：' => 'LOGO preview URL:',
    '请填写域名备注' => 'Please enter domain note', '请选择要删除的管理员权限组' => 'Select admin groups to delete',
    '确定现在执行该任务？' => 'Execute this task now?', '请先设置Web端Key' => 'Please set Web API key first',
    '请填写链接标题' => 'Please enter link title', '请选择站点下使用范围' => 'Select site scope',
    '积分不能为空' => 'Points cannot be empty', '折扣不能为空' => 'Discount cannot be empty',
    '你确定要删除该条数据？' => 'Delete this record?', '请填写模板路径' => 'Please enter template path',
    '确定要删除该类别？' => 'Delete this category?', '请输入新闻内容' => 'Please enter news content',
    '请输入回答内容' => 'Please enter answer content', '请选择问答类别' => 'Select Q&A category',
    '请输入评论内容' => 'Please enter comment content', '请输入公告关键字' => 'Please enter announcement keywords',
    '结束时间必须大于开始时间' => 'End time must be after start time', '请输入公告描述' => 'Please enter announcement description',
    '请填写招聘会名称' => 'Please enter job fair name', '请填写场地名称' => 'Please enter venue name',
    '请填写图片名称' => 'Please enter image name', '请选择参会企业！' => 'Select participating companies!',
    '请选择展位！' => 'Select booth!', '请选择用户名称' => 'Select username',
    '确定要取消名企吗？' => 'Remove featured company?', '确定要设为名企吗？' => 'Set as featured company?',
    '确定导出所有参会企业吗？' => 'Export all participating companies?', '确定导出选择的参会企业吗？' => 'Export selected participating companies?',
    '请填写商品名称！' => 'Please enter product name!', '请选择商品类别！' => 'Select product category!',
    '请上传商品图片！' => 'Please upload product image!', '请填写限购数量！' => 'Please enter purchase limit!',
    '请填写库存数量！' => 'Please enter stock quantity!', '请填写问答标题！' => 'Please enter Q&A title!',
    '请选择广告位置！' => 'Select ad placement!', '请选择广告类型！' => 'Select ad type!',
    '请选择消费模式！' => 'Select billing mode!', '请输入购买金额！' => 'Please enter purchase amount!',
    '请选输入天数！' => 'Please enter number of days!', '广告名称不能为空！' => 'Ad name cannot be empty!',
    '请选择一种广告类型！' => 'Select an ad type!', '请填写文字信息！' => 'Please enter text content!',
    '复制失败' => 'Copy failed', '确定删除该海报？' => 'Delete this poster?', '网站名称不能大于12个字符' => 'Site name cannot exceed 12 characters',
    '确定更新数据？' => 'Update data?', '请选择用户类型' => 'Select user type', '请输入自定义邮箱' => 'Please enter custom email',
    '请输入邮件内容' => 'Please enter email content', '请选择发送信息的用户' => 'Select users to send to',
    '请完善评语管理！' => 'Please complete review management!', '消息' => 'Message', '最多选择' => 'Select at most ',
];
$fixed = 0;
foreach ($en as $k => $v) {
    if (strpos($k, 'admin_vue_') !== 0) continue;
    if (!preg_match('/[\x{4e00}-\x{9fff}]/u', $v)) continue;
    $zhVal = $zh[$k] ?? $v;
    if (isset($dict[$zhVal])) {
        $en[$k] = $dict[$zhVal];
        $fixed++;
    } elseif (isset($dict[trim($zhVal)])) {
        $en[$k] = $dict[trim($zhVal)];
        $fixed++;
    }
}
$en['admin_vue_00124'] = 'Message';
$en['admin_vue_00125'] = 'Select at most ';
$out = "<?php\nreturn array(\n";
foreach ($en as $k => $v) {
    $out .= "  '" . addslashes($k) . "' => '" . addslashes($v) . "',\n";
}
$out .= ");\n";
file_put_contents(ROOT . 'data/lang/auto/en_us.php', $out);
echo "Fixed $fixed admin_vue en entries\n";
