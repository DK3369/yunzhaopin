<?php
$f = dirname(__DIR__) . '/app/template/wap/data_show_index.htm';
$c = file_get_contents($f);
$r = [
    '招聘</span>大数据' => '招聘</span>{yun:}t key=\'wap_01501\'{/yun}{yun:}t key=\'wap_01502\'{/yun}',
    '<span>年度分析报告</span>' => '<span>{yun:}t key=\'wap_01503\'{/yun}</span>',
    '{yun:}$year{/yun}求职者数据' => '{yun:}$year{/yun}{yun:}t key=\'wap_01504\'{/yun}',
    '<span>求职者画像</span>' => '<span>{yun:}t key=\'wap_01505\'{/yun}</span>',
    '地区，年龄，经验，男女，学历' => '{yun:}t key=\'wap_01506\'{/yun}',
    '<span>地区分布</span>' => '<span>{yun:}t key=\'wap_01507\'{/yun}</span>',
    '<span>年龄分布</span>' => '<span>{yun:}t key=\'wap_01508\'{/yun}</span>',
    '</span>岁' => '</span>{yun:}t key=\'wap_01425\'{/yun}',
    '<span>经验分布</span>' => '<span>{yun:}t key=\'wap_01509\'{/yun}</span>',
    '<span>男性求职者</span>' => '<span>{yun:}t key=\'wap_01510\'{/yun}</span>',
    '<b>性别占比</b>' => '<b>{yun:}t key=\'wap_01511\'{/yun}</b>',
    '<span>女性求职者</span>' => '<span>{yun:}t key=\'wap_01512\'{/yun}</span>',
    '<span>学历分布</span>' => '<span>{yun:}t key=\'wap_01513\'{/yun}</span>',
    'id="edu1" class="citys">大专' => 'id="edu1" class="citys">{yun:}t key=\'wap_01514\'{/yun}',
    'id="edu2" class="citys">本科' => 'id="edu2" class="citys">{yun:}t key=\'wap_01515\'{/yun}',
    'id="edu3" class="citys">高中' => 'id="edu3" class="citys">{yun:}t key=\'wap_01516\'{/yun}',
    '<span>求职者行为</span>' => '<span>{yun:}t key=\'wap_01517\'{/yun}</span>',
    '活跃趋势、行为趋势' => '{yun:}t key=\'wap_01518\'{/yun}',
    '<span>1-12月活跃趋势</span>' => '<span>{yun:}t key=\'wap_01519\'{/yun}</span>',
    '<span>1-12月注册趋势</span>' => '<span>{yun:}t key=\'wap_01520\'{/yun}</span>',
    '{yun:}$year{/yun}企业数据' => '{yun:}$year{/yun}{yun:}t key=\'wap_01521\'{/yun}',
    '<span>企业画像</span>' => '<span>{yun:}t key=\'wap_01522\'{/yun}</span>',
    '公司地区，公司规模，公司性质' => '{yun:}t key=\'wap_01523\'{/yun}',
    '<span>公司地区分布</span>' => '<span>{yun:}t key=\'wap_01524\'{/yun}</span>',
    '<span>公司规模分布</span>' => '<span>{yun:}t key=\'wap_01525\'{/yun}</span>',
    '<span>公司性质分布</span>' => '<span>{yun:}t key=\'wap_01526\'{/yun}</span>',
    '<span>企业行为</span>' => '<span>{yun:}t key=\'wap_01527\'{/yun}</span>',
    '登录趋势，发布岗位趋势' => '{yun:}t key=\'wap_01528\'{/yun}',
    '<span>1-12月登录趋势</span>' => '<span>{yun:}t key=\'wap_01529\'{/yun}</span>',
    '<span>1-12月发布岗位趋势</span>' => '<span>{yun:}t key=\'wap_01530\'{/yun}</span>',
    '<span>谢谢您的观看</span>' => '<span>{yun:}t key=\'wap_01531\'{/yun}</span>',
    '<span>以上数据由' => '<span>{yun:}t key=\'wap_01532\'{/yun}',
    '提供，最终解释权归我司所有</span>' => '{yun:}t key=\'wap_01533\'{/yun}</span>',
    '>点击查看好工作</a>' => '>{yun:}t key=\'wap_01534\'{/yun}</a>',
    "img_title: '年度数据'" => "img_title: '{yun:}t key=\'wap_01535\'{/yun}'",
];
foreach ($r as $a => $b) $c = str_replace($a, $b, $c);
file_put_contents($f, $c);
echo "OK data_show_index\n";

// Add nativeshare keys if missing
$keys = [
    'wap_01590' => ['邀请注册', 'Invite to register'],
    'wap_01591' => ['点击浏览器顶部或底部的', 'Tap the top or bottom of the browser'],
    'wap_01592' => ['或', ' or '],
    'wap_01593' => ['然后点击分享按钮', 'Then tap the share button'],
];
foreach (['zh_cn.php' => 0, 'en_us.php' => 1, 'aliases.php' => 2] as $file => $idx) {
    $path = dirname(__DIR__) . "/data/lang/auto/$file";
    $content = file_get_contents($path);
    $add = '';
    foreach ($keys as $k => $v) {
        if (strpos($content, "'$k'") !== false) continue;
        if ($file === 'zh_cn.php') $add .= "  '$k' => '{$v[0]}',\n";
        elseif ($file === 'en_us.php') $add .= "  '$k' => '{$v[1]}',\n";
        else $add .= "  '{$v[0]}' => '$k',\n";
    }
    if ($add) {
        $content = preg_replace('/\);\s*$/', rtrim($add) . "\n);", $content);
        file_put_contents($path, $content);
    }
}
