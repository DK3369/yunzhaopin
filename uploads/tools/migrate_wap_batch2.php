<?php
$base = dirname(__DIR__) . '/app/template/wap/';
$all = [
    'once_show.htm' => [
        "<font color='red'>登录后查看联系电话</font>" => "<font color='red'>{yun:}t key='wap_01350'{/yun}</font>",
        '<div class="tiny_tag">只有发布者本人才可以操作</div>' => '<div class="tiny_tag">{yun:}t key=\'wap_01351\'{/yun}</div>',
        '<div class="tiny_show_tckbox_cont_p">招聘密码：</div>' => '<div class="tiny_show_tckbox_cont_p">{yun:}t key=\'wap_01352\'{/yun}</div>',
        'placeholder="请输入添加时的密码"' => 'placeholder="{yun:}t key=\'wap_01353\'{/yun}"',
    ],
    'once_pay.htm' => [
        '<span class="once_pay_list_name">所需金额</span>' => '<span class="once_pay_list_name">{yun:}t key=\'wap_01354\'{/yun}</span>',
        '> 支付宝支付' => '> {yun:}t key=\'wap_01355\'{/yun}',
        '<span>网站已关闭支付功能，' => '<span>{yun:}t key=\'wap_01043\'{/yun}',
    ],
    'claim.htm' => [
        'placeholder="请输入新的用户名"' => 'placeholder="{yun:}t key=\'wap_01451\'{/yun}"',
        'placeholder="请输入新的用户密码"' => 'placeholder="{yun:}t key=\'wap_01452\'{/yun}"',
        '<div class="selecttip">确认新密码：</div>' => '<div class="selecttip">{yun:}t key=\'wap_01450\'{/yun}</div>',
        'placeholder="请输入确认新的用户密码"' => 'placeholder="{yun:}t key=\'wap_01453\'{/yun}"',
        "showToast('输入新的用户名！')" => "showToast('{yun:}t key=\'wap_01454\'{/yun}')",
        'showToast("密码长度应为6-20{yun:}t key=\'common_02053\'{/yun}！")' => 'showToast("{yun:}t key=\'wap_01550\'{/yun}{yun:}t key=\'common_02053\'{/yun}{yun:}t key=\'wap_01551\'{/yun}")',
    ],
    'company_vue.htm' => [
        '<div class="ptyhy"><i class="ptyhy_icon"></i>实地核验 </div>' => '<div class="ptyhy"><i class="ptyhy_icon"></i>{yun:}t key=\'wap_00274\'{/yun} </div>',
        '<em class="com_list_box_jobncor">{{item.jobnum}}</em>个在招职位</span>' => '<em class="com_list_box_jobncor">{{item.jobnum}}</em>{yun:}t key=\'wap_com_00094\'{/yun}</span>',
    ],
    'part_show.htm' => [
        '<span class="part_hot">优选</span>' => '<span class="part_hot">{yun:}t key=\'wap_01392\'{/yun}</span>',
        '{yun:}if $job.edate{/yun}短期{yun:}else{/yun}' => '{yun:}if $job.edate{/yun}{yun:}t key=\'wap_01393\'{/yun}{yun:}else{/yun}',
        '<span class="user_contnet_info_n">有效期至： </span>' => '<span class="user_contnet_info_n">{yun:}t key=\'wap_01394\'{/yun} </span>',
        '{yun:}if $job.link_tip==1 || $job.link_tip==2 || $job.link_tip==3{/yun}暂未开放联系方式' => '{yun:}if $job.link_tip==1 || $job.link_tip==2 || $job.link_tip==3{/yun}{yun:}t key=\'wap_01395\'{/yun}',
        '：刷信誉、淘宝刷钻、YY网络兼职、加YY联系的职位都是骗子！收取费用或押金的都有欺诈嫌疑，请警惕！</div>' => '：{yun:}t key=\'wap_01552\'{/yun}</div>',
        "showToast('暂未开放联系方式')" => "showToast('{yun:}t key=\'wap_01395\'{/yun}')",
        "onclick=\"showToast('报名兼职，{yun:}t key='wap_00447'{/yun}')\"" => "onclick=\"showToast('{yun:}t key='wap_00371'{/yun}')\"",
        "showToast('只有个人用户才能申请报名', 2)" => "showToast('{yun:}t key=\'wap_01396\'{/yun}', 2)",
    ],
];

foreach ($all as $f => $pairs) {
    $path = $base . $f;
    $c = file_get_contents($path);
    foreach ($pairs as $from => $to) {
        $c = str_replace($from, $to, $c);
    }
    file_put_contents($path, $c);
    echo "OK $f\n";
}

// uploadimg
$f = 'uploadimg.htm';
$c = file_get_contents($base.$f);
$u = [
    ['<div class="header_h1">手机上传</div>', '<div class="header_h1">{yun:}t key=\'wap_00542\'{/yun}</div>'],
    ['>您的姓名</span>', '>{yun:}t key=\'wap_01397\'{/yun}</span>'],
    ['>证件号码</span>', '>{yun:}t key=\'wap_01398\'{/yun}</span>'],
    ['>上传图片</div>', '>{yun:}t key=\'wap_01399\'{/yun}</div>'],
    ['（ 文字清晰，四角齐全 )', '{yun:}t key=\'wap_01400\'{/yun}'],
    ['>格式为', '>{yun:}t key=\'wap_01401\'{/yun}'],
    ['不得超过', '{yun:}t key=\'wap_01402\'{/yun}'],
    ['>公司名称</span>', '>{yun:}t key=\'wap_01403\'{/yun}</span>'],
    ['>信用代码</span>', '>{yun:}t key=\'wap_01404\'{/yun}</span>'],
    ['>上传营业执照/组织机构代码证</div>', '>{yun:}t key=\'wap_01405\'{/yun}</div>'],
    ['>选择上传图片</span>', '>{yun:}t key=\'wap_01406\'{/yun}</span>'],
    ['执照中的文字、图片、章印等需清晰可辨别，否则不能通过认证。', '{yun:}t key=\'wap_01407\'{/yun}'],
    ['>上传经办人身份证</div>', '>{yun:}t key=\'wap_01408\'{/yun}</div>'],
    ['图片和文字需清晰可辨别，否则不能通过认证。', '{yun:}t key=\'wap_01409\'{/yun}'],
    ['>上传委托书/承诺函</div>', '>{yun:}t key=\'wap_01410\'{/yun}</div>'],
    ['>上传其他材料（选填）</div>', '>{yun:}t key=\'wap_01411\'{/yun}</div>'],
    ['value="保存"', 'value="{yun:}t key=\'wap_user_00101\'{/yun}"'],
    ["showToast('请上传图片')", "showToast('{yun:}t key=\'wap_01412\'{/yun}')"],
    ["showToast('请填写您的姓名')", "showToast('{yun:}t key=\'wap_01413\'{/yun}')"],
    ["showToast('请填写证件号码')", "showToast('{yun:}t key=\'wap_01414\'{/yun}')"],
    ["showToast('请填写正确证件号码！')", "showToast('{yun:}t key=\'wap_01415\'{/yun}')"],
    ["showToast('请填写公司名称')", "showToast('{yun:}t key=\'wap_01416\'{/yun}')"],
    ["showLoading('上传中')", "showLoading('{yun:}t key=\'wap_01417\'{/yun}')"],
];
foreach ($u as [$a,$b]) $c = str_replace($a,$b,$c);
file_put_contents($base.$f,$c);
echo "OK uploadimg.htm\n";

// advice
$f = 'advice.htm';
$c = file_get_contents($base.$f);
$a = [
    ['placeholder="留下您的意见或反馈，我们会不断改进~"', 'placeholder="{yun:}t key=\'wap_01455\'{/yun}"'],
    ['>短信验证</span>', '>{yun:}t key=\'wap_01456\'{/yun}</span>'],
    ['placeholder="输入短信验证码"', 'placeholder="{yun:}t key=\'wap_01457\'{/yun}"'],
    ['我们将在第一时间及时回复您的反馈，如您的问题比较紧急，请致电服务热线!', '{yun:}t key=\'wap_01553\'{/yun}'],
    ['showToast("请选择意见类型", 2)', 'showToast("{yun:}t key=\'wap_01458\'{/yun}", 2)'],
    ["showToast('联系人不能空!', 2)", "showToast('{yun:}t key=\'wap_01459\'{/yun}', 2)"],
    ["showToast('联系手机不能为空!', 2)", "showToast('{yun:}t key=\'wap_01460\'{/yun}', 2)"],
    ["showToast('手机格式错误!', 2)", "showToast('{yun:}t key=\'wap_01461\'{/yun}', 2)"],
    ["showToast('反馈内容不能为空!', 2)", "showToast('{yun:}t key=\'wap_01462\'{/yun}', 2)"],
];
foreach ($a as [$x,$y]) $c = str_replace($x,$y,$c);
file_put_contents($base.$f,$c);
echo "OK advice.htm\n";

echo "done batch2\n";
