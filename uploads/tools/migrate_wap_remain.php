<?php
// Safe targeted replacements for reverted WAP files
$base = dirname(__DIR__) . '/app/template/wap/';
$reps = [
    'ident.htm' => [
        '之后你可以在"' => '之后你可以在"', // skip - use full line
        '<div class="regok_tit_p">之后你可以在"' => '<div class="regok_tit_p">{yun:}t key=\'wap_01548\'{/yun}"',
        '<p>我要找工作</p>' => '<p>{yun:}t key=\'wap_01446\'{/yun}</p>',
        '<span>我是求职者，我要找工作</span>' => '<span>{yun:}t key=\'wap_01447\'{/yun}</span>',
        '<p>我要招人</p>' => '<p>{yun:}t key=\'wap_01448\'{/yun}</p>',
        '<span>我是企业，我要招人</span>' => '<span>{yun:}t key=\'wap_01449\'{/yun}</span>',
    ],
];
// Run manual file by file via shell sed for speed
$files = [
    ['maplist.htm', [
        ['>列表</a>', '>{yun:}t key=\'wap_01418\'{/yun}</a>'],
        ['>地图</a>', '>{yun:}t key=\'wap_01419\'{/yun}</a>'],
        ['>换一批<', '>{yun:}t key=\'wap_01420\'{/yun}<'],
        ["'查看详情>>'", "'{yun:}t key='wap_01421'{/yun}'"],
        ["showToast('您的附近没有相关职位！', 2)", "showToast('{yun:}t key='wap_01422'{/yun}', 2)"],
        ["showConfirm('您确定查找该地区附近的职位吗？'", "showConfirm('{yun:}t key='wap_01423'{/yun}'"],
    ]],
    ['evaluateshow.htm', [
        ['题</span>', '{yun:}t key=\'wap_01479\'{/yun}</span>'],
        ['人访问过</div>', '{yun:}t key=\'wap_00612\'{/yun}</div>'],
        ['>开始测试</a>', '>{yun:}t key=\'wap_01480\'{/yun}</a>'],
    ]],
    ['article_channels.htm', [
        ['>我的频道<', '>{yun:}t key=\'wap_01463\'{/yun}<'],
        ['>推荐频道</div>', '>{yun:}t key=\'wap_01464\'{/yun}</div>'],
        ["showToast('执行中')", "showToast('{yun:}t key='wap_01537'{/yun}')"],
        ["showToast('我的频道最少要有一个', 2)", "showToast('{yun:}t key='wap_01538'{/yun}', 2)"],
    ]],
    ['spe_show.htm', [
        ['%Y年%m月%d日', '%Y-%m-%d'],
    ]],
    ['job/index.htm', [
        ['全力助您梦想起航', '{yun:}t key=\'wap_01483\'{/yun}'],
        [' 月薪：<b>', ' {yun:}t key=\'wap_01484\'{/yun}<b>'],
        ['无福利待遇', '{yun:}t key=\'wap_01486\'{/yun}'],
        ['查看该公司其他职位', '{yun:}t key=\'wap_01487\'{/yun}'],
        ['<span>向</span>', '<span>{yun:}t key=\'wap_01539\'{/yun}</span>'],
        ['<span>左</span>', '<span>{yun:}t key=\'wap_01540\'{/yun}</span>'],
        ['<span>滑</span>', '<span>{yun:}t key=\'wap_01541\'{/yun}</span>'],
        ['<span>动</span>', '<span>{yun:}t key=\'wap_01542\'{/yun}</span>'],
    ]],
    ['company/index.htm', [
        ['其他联系方式', '{yun:}t key=\'wap_01488\'{/yun}'],
        ['查看详细信息', '{yun:}t key=\'wap_01489\'{/yun}'],
        ['查看招聘职位', '{yun:}t key=\'wap_01490\'{/yun}'],
    ]],
    ['hb/whb.htm', [
        ['>生成</span>', '>{yun:}t key=\'wap_01500\'{/yun}</span>'],
        ['选择海报展示职位信息', '{yun:}t key=\'wap_01499\'{/yun}'],
        ['>生成海报</van-button>', '>{yun:}t key=\'wap_01286\'{/yun}</van-button>'],
        ['长按图片保存', '{yun:}t key=\'wap_01497\'{/yun}'],
        ["showLoading('生成中...')", "showLoading('{yun:}t key='wap_01498'{/yun}')"],
    ]],
];
foreach ($files as [$f, $pairs]) {
    $path = $base . $f;
    $c = file_get_contents($path);
    foreach ($pairs as [$from, $to]) {
        $c = str_replace($from, $to, $c);
    }
    file_put_contents($path, $c);
    echo "OK $f\n";
}
echo "done\n";
