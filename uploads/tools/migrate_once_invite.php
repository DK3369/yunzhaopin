<?php
$f = dirname(__DIR__) . '/app/template/wap/once_add.htm';
$c = file_get_contents($f);
$r = [
    '<span class="yun_newwap_text_name">我想招聘</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01356\'{/yun}</span>',
    'placeholder="请填写招聘名称,如厨师"' => 'placeholder="{yun:}t key=\'wap_01357\'{/yun}"',
    '<span class="yun_newwap_text_name">工作薪资</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01358\'{/yun}</span>',
    'placeholder="请填写工资"' => 'placeholder="{yun:}t key=\'wap_01359\'{/yun}"',
    '<span class="yun_newwap_text_name">工作地区</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01360\'{/yun}</span>',
    "{{city  ? city : '请填写工作地区'}}" => "{{city  ? city : i18n.fillWorkArea}}",
    '<span class="yun_newwap_text_name">详细地址</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01362\'{/yun}</span>',
    'placeholder="请填写详细地址"' => 'placeholder="{yun:}t key=\'wap_01363\'{/yun}"',
    '<span class="yun_newwap_text_name">招聘要求</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01364\'{/yun}</span>',
    "{{require  ? require : '请填写'}}" => "{{require  ? require : i18n.fillIn}}",
    '<span class="yun_newwap_text_name">店面名称</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01366\'{/yun}</span>',
    'placeholder="请填写店铺名称"' => 'placeholder="{yun:}t key=\'wap_01367\'{/yun}"',
    'placeholder="请填写联系人"' => 'placeholder="{yun:}t key=\'wap_01368\'{/yun}"',
    '<span class="yun_newwap_text_name">联系电话</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01432\'{/yun}</span>',
    'placeholder="请填写联系电话"' => 'placeholder="{yun:}t key=\'wap_01369\'{/yun}"',
    '<span class="yun_newwap_text_name">验证码</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01370\'{/yun}</span>',
    'placeholder="请输入图片验证码"' => 'placeholder="{yun:}t key=\'wap_user_00141\'{/yun}"',
    '<span class="yun_newwap_text_name">短信验证码</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01371\'{/yun}</span>',
    'placeholder="请填写短信验证码"' => 'placeholder="{yun:}t key=\'wap_01372\'{/yun}"',
    '>获取验证码</a>' => '>{yun:}t key=\'wap_01373\'{/yun}</a>',
    '<span class="yun_newwap_text_name">招聘时长</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01374\'{/yun}</span>',
    '{yun:}$row.day_n{/yun} 天</span>' => '{yun:}$row.day_n{/yun} {yun:}t key=\'wap_01375\'{/yun}</span>',
    "{{ oncepricegearStr ? oncepricegearStr : '请选择招聘时长'}}" => "{{ oncepricegearStr ? oncepricegearStr : i18n.selectDuration}}",
    '<span class="yun_newwap_text_name">店面营业执照</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01377\'{/yun}</span>',
    '<span class="yun_newwap_text_name">店面形象</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01378\'{/yun}</span>',
    '<span class="yun_newwap_text_name">设置密码</span>' => '<span class="yun_newwap_text_name">{yun:}t key=\'wap_01379\'{/yun}</span>',
    'placeholder="请输入密码"' => 'placeholder="{yun:}t key=\'wap_01380\'{/yun}"',
    '<div class="yun_newwap_box_ts">提示：密码可用于刷新/修改/删除此信息</div>' => '<div class="yun_newwap_box_ts">{yun:}t key=\'wap_01381\'{/yun}</div>',
    '<button id="oncesubmit" type="button" onclick="oncesubmit()">提交操作</button>' => '<button id="oncesubmit" type="button" onclick="oncesubmit()">{yun:}t key=\'wap_00354\'{/yun}</button>',
    'title="请选择地区"' => 'title="{yun:}t key=\'wap_01382\'{/yun}"',
    '<h3>招聘时长</h3>' => '<h3>{yun:}t key=\'wap_01374\'{/yun}</h3>',
    '}} 天</span>' => '}} {yun:}t key=\'wap_01375\'{/yun}</span>',
    '}} 元</span>' => '}} {yun:}t key=\'common_02056\'{/yun}</span>',
    '<button>确定</button>' => '<button>{yun:}t key=\'wap_01384\'{/yun}</button>',
    '<div class="yun_wap_info_brief_tit"> 招聘要求 </div>' => '<div class="yun_wap_info_brief_tit"> {yun:}t key=\'wap_01364\'{/yun} </div>',
    'placeholder="请填写招聘的具体要求，如性别、学历、年龄、工作经验和工作待遇等"' => 'placeholder="{yun:}t key=\'wap_01383\'{/yun}"',
    '<a class="yun_wap_info_brief_tit_bc mui-action-back">确定</a>' => '<a class="yun_wap_info_brief_tit_bc mui-action-back">{yun:}t key=\'wap_01384\'{/yun}</a>',
    "showConfirm('{yun:}t key=\'wap_01544\'{/yun}{yun:}$num{/yun}{yun:}t key=\'wap_01545\'{/yun}'" => "showConfirm('{yun:}t key=\'wap_01544\'{/yun}{yun:}$num{/yun}{yun:}t key=\'wap_01545\'{/yun}'",
    "}, '继续发布', '去付款');" => "}, '{yun:}t key=\'wap_01385\'{/yun}', '{yun:}t key=\'wap_01386\'{/yun}');",
    "+ ' 天/' + this.oncepricegearOpt.price + ' 元'" => "+ ' ' + this.i18n.days + '/' + this.oncepricegearOpt.price + ' ' + this.i18n.yuan",
    'showToast("请输入手机号码！")' => 'showToast("{yun:}t key=\'wap_01389\'{/yun}")',
    "showToast('手机格式错误！')" => "showToast('{yun:}t key=\'wap_01390\'{/yun}')",
    "showToast('请填写图片验证码！')" => "showToast('{yun:}t key=\'wap_js_00116\'{/yun}')",
    "showToast('请勿重复发送！', 2)" => "showToast('{yun:}t key=\'wap_01391\'{/yun}', 2)",
    "$(obj).html('重新发送(' + smsTimer_flag + 's)')" => "$(obj).html('{yun:}t key=\'wap_01388\'{/yun}' + smsTimer_flag + 's)')",
    "$(obj).html('重新发送')" => "$(obj).html('{yun:}t key=\'wap_01387\'{/yun}')",
];
foreach ($r as $a => $b) $c = str_replace($a, $b, $c);
// add i18n to vue data
$c = str_replace(
    "data:{\n            showArea: false,",
    "data:{\n            i18n: {\n                fillWorkArea: '{yun:}t key=\'wap_01361\'{/yun}',\n                fillIn: '{yun:}t key=\'wap_01365\'{/yun}',\n                selectDuration: '{yun:}t key=\'wap_01376\'{/yun}',\n                days: '{yun:}t key=\'wap_01375\'{/yun}',\n                yuan: '{yun:}t key=\'common_02056\'{/yun}',\n            },\n            showArea: false,",
    $c
);
file_put_contents($f, $c);
echo "OK once_add.htm\n";

// invite.htm
$f = dirname(__DIR__) . '/app/template/wap/invite.htm';
$c = file_get_contents($f);
$r = [
    '{{user.user_exp}}经验</li>' => '{{user.user_exp}}{yun:}t key=\'wap_01424\'{/yun}</li>',
    '` · ${user.age}岁`' => '` · ${user.age}{yun:}t key=\'wap_01425\'{/yun}`',
    '<div class="job_resume_left">面试时间</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01426\'{/yun}</div>',
    "{{intertime == '' ? '请选择面试时间' :" => "{{intertime == '' ? i18n.selectInterviewTime :",
    'title="选择面试时间"' => 'title="{yun:}t key=\'wap_01428\'{/yun}"',
    '<div class="job_resume_left">面试职位</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01429\'{/yun}</div>',
    '<div class="job_resume_left">邀请模板</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01430\'{/yun}</div>',
    '<div class="job_resume_left">联系人</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01431\'{/yun}</div>',
    '<div class="job_resume_left">联系方式</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01432\'{/yun}</div>',
    '<div class="job_resume_left">面试地址</div>' => '<div class="job_resume_left">{yun:}t key=\'wap_01433\'{/yun}</div>',
    'placeholder="请填写面试地址"' => 'placeholder="{yun:}t key=\'wap_01434\'{/yun}"',
    '<div class="remark_name">备注信息</div>' => '<div class="remark_name">{yun:}t key=\'wap_01435\'{/yun}</div>',
    'placeholder="可告知求职者面试时所需材料,面试前的相关注意事项"' => 'placeholder="{yun:}t key=\'wap_01436\'{/yun}"',
    "{{ymedit?'更新':'保存'}}邀请模板</div>" => "{{ymedit?i18n.update:i18n.save}}{yun:}t key=\'wap_01430\'{/yun}</div>",
    'value="发送面试邀请"' => 'value="{yun:}t key=\'wap_01438\'{/yun}"',
    "showModal('请选择面试职位')" => "showModal('{yun:}t key=\'wap_01439\'{/yun}')",
    "showModal('请填写联系人')" => "showModal('{yun:}t key=\'wap_01368\'{/yun}')",
    "showModal('请填写联系电话')" => "showModal('{yun:}t key=\'wap_01369\'{/yun}')",
    "showModal('联系电话格式错误')" => "showModal('{yun:}t key=\'wap_01440\'{/yun}')",
    "showModal('请选择面试时间')" => "showModal('{yun:}t key=\'wap_01427\'{/yun}')",
    "showModal('请填写面试地址')" => "showModal('{yun:}t key=\'wap_01434\'{/yun}')",
    "showLoading('邀请中')" => "showLoading('{yun:}t key=\'wap_01441\'{/yun}')",
    "showToast('邀请成功', 2" => "showToast('{yun:}t key=\'wap_01442\'{/yun}', 2",
];
foreach ($r as $a => $b) $c = str_replace($a, $b, $c);
// add i18n to yunvue data - find data: { in invite
$c = preg_replace(
    '/(var yunvue = new Vue\(\{[\s\S]*?data:\s*\{)/',
    "$1\n                i18n: {\n                    selectInterviewTime: '{yun:}t key='wap_01427'{/yun}',\n                    update: '{yun:}t key='wap_01437'{/yun}',\n                    save: '{yun:}t key='wap_user_00101'{/yun}',\n                },",
    $c,
    1
);
file_put_contents($f, $c);
echo "OK invite.htm\n";
