<?php
/**
 * Strip/replace Chinese in PHP comments under api/wxapp and app/controller/wap.
 */
define('ROOT', dirname(__DIR__) . '/');

$dirs = array('api/wxapp', 'app/controller/wap', 'wap/member');
$skip = array('wap.enum.php', 'app/include/wap.enum.php');

$blockMap = array(
    '/*wxapp基本信息页面显示、创建简历页面显示*/' => '/* wxapp: profile display */',
    '/*wxapp保存基本信息*/' => '/* wxapp: save profile */',
    '/*wxapp保存头像*/' => '/* wxapp: save avatar */',
    '/*wxapp申请记录*/' => '/* wxapp: applications */',
    '/*wxapp申请记录删除*/' => '/* wxapp: delete application */',
    '/*wxapp取消申请*/' => '/* wxapp: cancel application */',
    '/*wxapp收藏记录*/' => '/* wxapp: favorites */',
    '/*wxapp收藏记录删除*/' => '/* wxapp: delete favorite */',
    '/*wxapp面试通知记录*/' => '/* wxapp: notices */',
    '/*wxapp面试邀请页面-面试通知详情页*/' => '/* wxapp: interview notice detail */',
    '/*wxapp面试通知记录删除*/' => '/* wxapp: delete interview notice */',
    '/*wxapp浏览记录*/' => '/* wxapp: browse history */',
    '/*wxapp浏览记录删除*/' => '/* wxapp: delete browse history */',
    '/*wxapp相似职位*/' => '/* wxapp: similar jobs */',
    '/*wxapp意向职位修改页面显示*/' => '/* wxapp: expect job edit */',
    '/*wxapp工作经历、培训经历。。。修改页面*/' => '/* wxapp: resume section edit */',
    '/*wxapp保创建简历保存*/' => '/* wxapp: create resume save */',
    '/**************************简历是否必填工作经历*************************************************/' => '/* resume: work exp required check */',
    '/**************************简历是否必填教育经历*************************************************/' => '/* resume: edu exp required check */',
    '/**************************简历是否必填项目经历*************************************************/' => '/* resume: project exp required check */',
    '/*wxapp简历管理页面刷新*/' => '/* wxapp: refresh resume */',
    '/*wxapp简历管理页面设置默认*/' => '/* wxapp: set default resume */',
    '/*wxapp简历管理页面设置是否公开*/' => '/* wxapp: resume privacy */',
    '/*wxapp简历管理页面删除*/' => '/* wxapp: delete resume */',
    '/*wxapp谁看过我记录*/' => '/* wxapp: who viewed me */',
    '/*wxapp谁看过我记录删除*/' => '/* wxapp: delete who viewed me */',
    '/* 删除职位 */' => '/* delete job */',
    '/* 职位推广（置顶、推荐、紧急招聘） */' => '/* job promotion */',
    '/* 取消职位推广（置顶、推荐、紧急招聘） */' => '/* cancel job promotion */',
    '/*wxapp职位管理页面上架下架*/' => '/* wxapp: job shelf toggle */',
    '/* 求职咨询 */' => '/* job inquiry */',
    '/*wxapp面试邀请页面-面试通知详情页*/' => '/* wxapp: interview invite detail */',
    '/* wxapp浏览简历记录 */' => '/* wxapp: resume browse log */',
    '/* wxapp删除浏览简历记录 */' => '/* wxapp: delete resume browse log */',
    '/* wxapp谁看过我记录 */' => '/* wxapp: who viewed company */',
    '/* wxapp删除谁看过我记录 */' => '/* wxapp: delete who viewed company */',
    '/* 注册发送手机号 */' => '/* register: send mobile code */',
    '/* 注册发送手机号-快速投递 */' => '/* register: quick apply mobile code */',
);

$inlineMap = array(
    '//分页' => '// paginate',
    '// 分页' => '// paginate',
    '//订单' => '// order',
    '//置顶' => '// top service',
    '//推荐' => '// recommend service',
    '//紧急招聘' => '// urgent service',
    '//查询会员信息' => '// member statis',
    '//二维码' => '// qrcode upload',
    '//删除申请的职位' => '',
    '//删除收藏的职位' => '',
    '//删除职位浏览记录' => '',
    '//简历浏览记录' => '',
    '//删除简历浏览记录' => '',
    '// 简历状态为未审核' => '// resume pending review',
    '// 取消 简历未备注' => '// clear resume note',
    '//  无可浏览会员' => '// no viewable members',
    '//  已暂停' => '// paused',
    '//套餐会员' => '// package members',
    '//时间会员' => '// time-based members',
    '//当前已有的服务信息' => '// current services',
    '//增值列表' => '// addon list',
    '//支付方式' => '// payment types',
    '// 展示升级套餐提醒' => '// show upgrade tip',
    '// 描述' => '// description',
    '// 添加职位，行业默认是企业行业' => '// default industry from company',
    '// 添加职位，带企业添加的福利待遇' => '// default welfare from company',
    '//删除成功' => '// deleted',
    '//招聘中职位' => '// active jobs',
    '//排序' => '// sort',
    '//会员日志' => '// member log',
    '// 不限量' => '// unlimited',
    '// 套餐数量不足' => '// quota insufficient',
    '// 套餐已被使用完' => '// quota exhausted',
    '// 计算差集' => '// delta',
    '// 近一月' => '// last month',
    '// 减少1天，包含上当天' => '// include today',
    '// 包含今天，共七天' => '// seven days incl. today',
    '// 前台用来展示时间段' => '// display date range',
    '//用做 一级-全部-\'\'' => '// level-1 all placeholder',
    '//关键字展示' => '// hot keywords',
    '// 热搜关键词最多展示12个' => '// max 12 hot keywords',
    '//关键字类别' => '// keyword groups',
    '// 短信发送端口$port : 3-小程序  4-APP' => '// SMS port: 3=mini 4=app',
    '// 短信发送端口$port : 9-小程序快速投递,10-APP快速投递' => '// SMS port: 9=mini quick apply 10=app',
    '//来自企业会员中心-应聘悬赏简历-查看简历' => '// from reward resume view',
    '//是否来自企业应聘悬赏简历的查看简历' => '// reward resume source flag',
    '//微简历列表' => '// tiny resume list',
    '//类别ID' => '// category id',
    '//关键字' => '// keyword',
    '//排除没有值的字段' => '// skip empty fields',
    '//微简历内容' => '// tiny resume detail',
    '// PC发送短信' => '// PC SMS',
    '// WAP快速投递' => '// WAP quick apply SMS',
    '// 先更新被浏览次数，再查公告信息，防止新公告首次被浏览时出现被浏览次数为0' => '// bump view count before load',
    '//公告名称' => '// title',
    '//描述' => '// description',
    '//相关文章' => '// related articles',
    '//新闻类别' => '// news category',
    '//新闻名称' => '// news title',
    '//删除回答' => '// delete answer',
    '//删除问题' => '// delete question',
    '// 不是从企业列表过来的才会直接赋值返回URL' => '// back URL when not from list',
    '//可以指定前缀' => '// optional prefix',
    '//公招名称' => '// title',
    '// 预留信息' => '// lead source',
    '// all为降权和非降权均查询' => '// depower filter all',
    '// 默认查询未降权的职位' => '// default non-depowered jobs',
    '//开始时间' => '// day start',
    '//当天总的已发布量' => '// today publish count',
    '// 更新浏览次数' => '// bump hits',
    '//关闭会员注册' => '// registration disabled',
    '//邀请注册生成' => '// invite register',
    '//简历人姓名' => '// resume name',
    '//城市' => '// city',
    '//行业' => '// job',
    '//今天开始时间' => '// today start',
    '//查询场地' => '// venue',
    '//加密的salt' => '// token salt',
    '//上传头像' => '// avatar upload',
    '//上传企业营业执照' => '// business license',
    '//个人上传身份证' => '// id card',
    '//个人上传头像' => '// user avatar',
);

function cleanLine($line, $blockMap, $inlineMap)
{
    foreach ($blockMap as $from => $to) {
        if (strpos($line, $from) !== false) {
            $line = str_replace($from, $to, $line);
        }
    }
    foreach ($inlineMap as $from => $to) {
        if (strpos($line, $from) !== false) {
            $line = str_replace($from, $to, $line);
        }
    }
    // Strip remaining Chinese in trailing // comments
    if (preg_match('/^(.*)(\s\/\/\s*)(.+)$/u', $line, $m)) {
        if (preg_match('/[\x{4e00}-\x{9fff}]/u', $m[3]) && !preg_match('/yun_auto_t|WapDbEnum/', $m[1] . $m[3])) {
            $line = rtrim($m[1]) . "\n";
        }
    }
    // Pure block comment lines with Chinese only
    if (preg_match('/^\s*\/\*.*[\x{4e00}-\x{9fff}].*\*\/\s*$/u', $line)) {
        return '';
    }
    return $line;
}

$changed = 0;
foreach ($dirs as $dir) {
    $path = ROOT . $dir;
    $it = new RecursiveIteratorIterator(new RecursiveDirectoryIterator($path));
    foreach ($it as $f) {
        if (!$f->isFile() || $f->getExtension() !== 'php') {
            continue;
        }
        $rel = str_replace(ROOT, '', $f->getPathname());
        foreach ($skip as $s) {
            if (strpos($rel, $s) !== false) {
                continue 2;
            }
        }
        $lines = file($f->getPathname());
        $out = array();
        $fileChanged = false;
        foreach ($lines as $line) {
            $new = cleanLine($line, $blockMap, $inlineMap);
            if ($new !== $line) {
                $fileChanged = true;
            }
            $out[] = $new;
        }
        if ($fileChanged) {
            file_put_contents($f->getPathname(), implode('', $out));
            echo "FIXED: $rel\n";
            $changed++;
        }
    }
}
echo "Done. $changed files.\n";
