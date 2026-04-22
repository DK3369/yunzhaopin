<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:19:32
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/company/default/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:181996477669e8a0b4017e82-28840456%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '8f0b332ac79477d96a655742916a7b2eb7576d1b' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/company/default/index.htm',
      1 => 1706496289,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '181996477669e8a0b4017e82-28840456',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'title' => 0,
    'keywords' => 0,
    'description' => 0,
    'style' => 0,
    'config' => 0,
    'com_style' => 0,
    'com' => 0,
    'comrat' => 0,
    'num' => 0,
    'invite_resume' => 0,
    'pre' => 0,
    'usertype' => 0,
    'isatn' => 0,
    'uid' => 0,
    'ComMember' => 0,
    'hbNum' => 0,
    'job_cnt' => 0,
    'wlist' => 0,
    'shows' => 0,
    'row' => 0,
    'ProductList' => 0,
    'departmentNames' => 0,
    'v' => 0,
    'jlist' => 0,
    'pagenav' => 0,
    'pgarr' => 0,
    'k' => 0,
    'pages' => 0,
    'city' => 0,
    'city_type' => 0,
    'city_name' => 0,
    'city_index' => 0,
    'industry_index' => 0,
    'industry_name' => 0,
    'keylist' => 0,
    'msgList' => 0,
    'msglist' => 0,
    'NewsList' => 0,
    'Info' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0b4059c22_43969121',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0b4059c22_43969121')) {function content_69e8a0b4059c22_43969121($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_modifier_date_format')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/modifier.date_format.php';
if (!is_callable('smarty_function_listurl')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.listurl.php';
?><!DOCTYPE HTML PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN""http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">

<head>
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8"/>
    <title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>
    <meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
"/>
    <meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
"/>
    <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css"/>
    <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['com_style']->value;?>
/images/comapply.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css"/>
    
</head>

<body class="companyshowbg">
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="clear"></div>
<div class="com_details_top">
    <div class="w1200">
        <div class="com_details_current">您当前的位置：<a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
">首页</a> > <a href="<?php echo smarty_function_url(array('m'=>'company'),$_smarty_tpl);?>
">企业列表</a> > <span><a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>$_smarty_tpl->tpl_vars['com']->value['uid']),$_smarty_tpl);?>
">企业详情</a> </span></div>
        <div class="com_details_top_c">
            <?php if ($_smarty_tpl->tpl_vars['com']->value['hotlogo']==1) {?><span class="com_details_name_mq" title="名企"> </span><?php }?>
            <div class="com_details_info_box">
                <div class="com_details_logo">
                    <a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>$_smarty_tpl->tpl_vars['com']->value['uid']),$_smarty_tpl);?>
"><img src="<?php echo $_smarty_tpl->tpl_vars['com']->value['logo'];?>
" width="140" height="140" onerror="showImgDelay(this,'<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_unit_icon'];?>
',2);" alt="<?php echo $_smarty_tpl->tpl_vars['com']->value['name'];?>
"/></a>
                </div>
                <h1 class="com_details_name" style="overflow: hidden; position: relative; max-width: 660px;">
                    <a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>$_smarty_tpl->tpl_vars['com']->value['uid']),$_smarty_tpl);?>
"><?php echo $_smarty_tpl->tpl_vars['com']->value['name'];?>
 </a>
                    <?php if ($_smarty_tpl->tpl_vars['comrat']->value['com_pic']!="0"&&$_smarty_tpl->tpl_vars['comrat']->value['com_pic']!='') {?><img src="<?php echo $_smarty_tpl->tpl_vars['comrat']->value['com_pic'];?>
" width="18" height="18" title="会员等级"/><?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['yyzz_status']==1) {?><i class="job_details_cominfo_rz job_details_cominfo_rz_zz" title="执照认证"></i><?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['moblie_status']==1) {?><i class="job_details_cominfo_rz job_details_cominfo_rz_sj" title="手机认证"></i><?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['email_status']==1) {?><i class="job_details_cominfo_rz job_details_cominfo_rz_yx" title="邮箱认证"></i><?php }?>
                </h1>
                <div class="com_details_info">
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['provinceid']) {?>
                    <?php echo $_smarty_tpl->tpl_vars['com']->value['provinceid'];
if ($_smarty_tpl->tpl_vars['com']->value['cityid']) {?> - <?php echo $_smarty_tpl->tpl_vars['com']->value['cityid'];
}?>
                    <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['hy_info']) {?><span class="com_details_line">|</span><?php echo $_smarty_tpl->tpl_vars['com']->value['hy_info'];?>
 <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['pr_info']) {?><span class="com_details_line">|</span><?php echo $_smarty_tpl->tpl_vars['com']->value['pr_info'];?>
 <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['mun_info']) {?><span class="com_details_line">|</span><?php echo $_smarty_tpl->tpl_vars['com']->value['mun_info'];?>
 <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['money']) {?><span class="com_details_line">|</span>注资<?php echo $_smarty_tpl->tpl_vars['com']->value['money'];?>
 <?php if ($_smarty_tpl->tpl_vars['com']->value['moneytype']==1) {?>万元<?php } else { ?>万美元<?php }?> <?php }?>
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['sdate']) {?><span class="com_details_line">|</span><?php echo $_smarty_tpl->tpl_vars['com']->value['sdate'];?>
年创办 <?php }?>
                </div>

                <div class="com_details_data_box">
                    <div class="com_details_data_box_c">
                        <div class="com_details_data">
                            <div class="com_details_data_n"><?php echo $_smarty_tpl->tpl_vars['num']->value;?>
</div>
                            <div class="com_details_dataname">在招职位</div>
                            <i class="com_details_data_line"></i>
                        </div>
                        <div class="com_details_data">
                            <div class="com_details_data_n"><?php echo $_smarty_tpl->tpl_vars['invite_resume']->value;?>
</div>
                            <div class="com_details_dataname">共邀面试</div>
                            <i class="com_details_data_line"></i>
                        </div>
                        <div class="com_details_data">
                            <div class="com_details_data_n"><?php echo $_smarty_tpl->tpl_vars['pre']->value;?>
%</div>
                            <div class="com_details_dataname"> 简历处理率</div>
                            <i class="com_details_data_line"></i>
                        </div>
                        <div class="com_details_data">
                            <div class="com_details_data_n">
                                <?php if ($_smarty_tpl->tpl_vars['com']->value['login_date']) {?>
                                <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['com']->value['login_date'],"%Y-%m-%d");?>

                                <?php } else { ?> 未登录 <?php }?>
                            </div>
                            <div class="com_details_dataname"> 最近登录时间</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="com_details_opt">
                <div class="com_details_opt_fxbox">
                    <?php if ($_smarty_tpl->tpl_vars['usertype']->value=='1') {?>
                        <?php if ($_smarty_tpl->tpl_vars['isatn']->value['id']) {?>
                        <a href="javascript:void(0)" onclick="atn('<?php echo $_smarty_tpl->tpl_vars['com']->value['uid'];?>
','<?php echo smarty_function_url(array('m'=>'ajax','c'=>'atncompany'),$_smarty_tpl);?>
')" id='atn_<?php echo $_smarty_tpl->tpl_vars['com']->value['uid'];?>
' class="com_details_opt_gz company_att">取消关注</a>
                        <?php } else { ?>
                        <a href="javascript:void(0)" onclick="atn('<?php echo $_smarty_tpl->tpl_vars['com']->value['uid'];?>
','<?php echo smarty_function_url(array('m'=>'ajax','c'=>'atncompany'),$_smarty_tpl);?>
')" id='atn_<?php echo $_smarty_tpl->tpl_vars['com']->value['uid'];?>
' class="com_details_opt_gz">关注</a>
                        <?php }?>
                    <?php } else { ?>
                        <?php if ($_smarty_tpl->tpl_vars['uid']->value) {?>
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>
                            <a href="javascript:void(0);" onclick="layer.msg('请先申请个人用户才能关注', 2, 8)" class="com_details_opt_gz">+ 关注</a>
                            <?php } else { ?>
                            <a href="javascript:void(0);" onclick="layer.msg('只有个人用户才能关注', 2, 8)" class="com_details_opt_gz">+ 关注</a>
                            <?php }?>
                        <?php } else { ?>
                            <a href="javascript:void(0)" onclick="showlogin('1');" class="com_details_opt_gz">+ 关注</a>
                        <?php }?>
                    <?php }?>
                </div>
                <?php if ($_smarty_tpl->tpl_vars['ComMember']->value['source']==6&&$_smarty_tpl->tpl_vars['ComMember']->value['claim']==0&&$_smarty_tpl->tpl_vars['ComMember']->value['email']!='') {?>
                <a href="javascript:claim('<?php echo smarty_function_url(array('m'=>'ajax','c'=>'claim','uid'=>$_smarty_tpl->tpl_vars['com']->value['uid']),$_smarty_tpl);?>
');" class="com_show_comgz">认领</a>
                <?php }?>
                <div id="status_div" style="display:none; width:350px;">
                    <div id="claimmsg"></div>
                    <div class="admin_qx_bth" style="text-align:center;padding:10px;border-radius:20px;">
                        <input type="button" onClick="layer.closeAll();" class="admin_examine_bth_qx" style="width:70px;height:29px;color:#fff;border:none;font-size:14px;border-radius:3px;background:#f60;" value='关闭' />
                    </div>
                </div>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_haibao_isopen']==1&&$_smarty_tpl->tpl_vars['hbNum']->value>0) {?>
                <div class="com_details_opt_fxbox"><a href="javascript:void(0);" onclick="selectHb('<?php echo $_smarty_tpl->tpl_vars['job_cnt']->value;?>
')" class="com_details_opt_hb" rel="nofollow">海报</a></div>
                <?php }?>

                <div class="com_details_opt_fxbox">
                    <a href="javascript:void(0);" onmousemove="$('#getwapurl').show();" onmouseout="$('#getwapurl').hide();" rel="nofollow" class="com_details_opt_fx">分享</a>
                    <div class="comapply_sq_r_cy none" id="getwapurl">
                        <div class="comapply_sq_r_cont">
                            <div class="comapply_sq_r_tipa"> 微信扫一扫：分享</div>
                            <img src="<?php echo smarty_function_url(array('m'=>'ajax','c'=>'pubqrcode','toc'=>'company','toa'=>'show','toid'=>$_smarty_tpl->tpl_vars['com']->value['uid']),$_smarty_tpl);?>
" width="130" height="130"/>
                            <div class="comapply_sq_r_tipsm"> ↑微信扫上方二维码↑<br>便可将本文分享至朋友圈</div>
                        </div>
                    </div>
                </div>
                
            </div>
        </div>
    </div>
</div>

<div class="clear"></div>

<div class="w1200">
    <div class="com_details_left">
        <?php if ($_smarty_tpl->tpl_vars['com']->value['welfare_n']) {?>
        <div class="com_show_leftbox">
            <div class="com_details_tit"><span class="com_details_tit_s">企业福利</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <div class="com_welfare ">
                <?php  $_smarty_tpl->tpl_vars['wlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['wlist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['com']->value['welfare_n']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['wlist']->key => $_smarty_tpl->tpl_vars['wlist']->value) {
$_smarty_tpl->tpl_vars['wlist']->_loop = true;
?>
                <span class="com_welfare_s "><?php echo $_smarty_tpl->tpl_vars['wlist']->value;?>
</span>
                <?php } ?>
            </div>
        </div>
        <?php }?>

        <div class="com_show_leftbox none" id="companybusiness" data-slide='1'>
            <div class="com_details_tit"><span class="com_details_tit_s">工商信息</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <div class="com_show_leftcont">
                <!-- 天眼查 start-->
                <div class="business none" id="businessInfo">
                    <div class="businessInfo">
                        <ul class="basicMsgList clearfix">
                            <li><div class="clearfix"><span>统一社会信用代码：</span><em id="creditCode"></em></div></li>
                            <li><div class="clearfix"><span>成立日期：</span><em id="estiblishTime"></em></div></li>
                            <li><div class="clearfix"><span>组织机构代码：</span><em id="orgNumber"></em></div></li>
                            <li><div class="clearfix"><span>经营期限：</span><em id="Time"></em></div></li>
                            <li><div class="clearfix"><span>企业类型：</span><em id="companyOrgType"></em></div></li>
                            <li><div class="clearfix"><span>登记机关：</span><em id="regInstitute"></em></div></li>
                            <li><div class="clearfix"><span>经营状态：</span><em id="regStatus"></em></div></li>
                            <li><div class="clearfix"><span>注册资本：</span><em id="regCapital"></em></div></li>
                        </ul>
                        <dl class="basicMsgList">
                            <dt>注册地址：</dt>
                            <dd id="regLocation"></dd>
                        </dl>
                        <dl class="basicMsgList mt20">
                            <dt>经营范围：</dt>
                            <dd id="businessScope"></dd>
                        </dl>
                        <div class="qxb clearfix">
                            <div class="qxb_tg">
                                <span>以上内容由</span>
                                <div class="hxb"><a href="" class="tianyancha" id="tianyancha" target='_blank'></a></div>
                                <span>提供</span>
                            </div>
                        </div>
                    </div>
                </div>
                <!-- 天眼查 end-->
            </div>
        </div>

        <div class="com_show_leftbox">
            <div class="com_details_tit"><span class="com_details_tit_s">公司简介</span><i class="com_details_tit_line yun_bg_color"></i></div>

            <div class="clear"></div>

            <div class="com_show_leftcont">
                <div class="con_show_introduction">
                    <?php if ($_smarty_tpl->tpl_vars['com']->value['content']) {?>
                    <div style="width:100%;height:auto; overflow:hidden" id="com_content" class="company_img_auto"><?php echo $_smarty_tpl->tpl_vars['com']->value['content'];?>
</div>
                    <div class="company_show_more none"><a href="javascript:;" onclick="showcc()">查看更多</a></div>
                    <?php } else { ?>
                    <div class="firm_ment"><div class="firm_tips_no"> 该企业还没有填写公司简介!</div></div>
                    <?php }?>
                </div>
            </div>
        </div>

        <?php if (!empty($_smarty_tpl->tpl_vars['shows']->value)) {?>
        <div class="com_show_leftbox">
            <div class="com_details_tit"><span class="com_details_tit_s">公司环境</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <div class="com_show_leftcont">
                <div class="com_show_l_box">
                    <div class="com_show_image_box">
                        <div class="com_show_image" id="layer-pic">
                            <?php  $_smarty_tpl->tpl_vars['row'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['row']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['shows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['row']->key => $_smarty_tpl->tpl_vars['row']->value) {
$_smarty_tpl->tpl_vars['row']->_loop = true;
?>
                            <div class="com_show_image_list">
                                <a href="javascript:void(0);"><img src="<?php echo $_smarty_tpl->tpl_vars['row']->value['picurl'];?>
" lay-src="<?php echo $_smarty_tpl->tpl_vars['row']->value['picurl'];?>
" width="260" height="160" alt="<?php echo $_smarty_tpl->tpl_vars['row']->value['title'];?>
"/> </a>
                            </div>
                            <?php } ?>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <?php }?>

        <?php if (!empty($_smarty_tpl->tpl_vars['ProductList']->value)) {?>
        <div class="com_show_leftbox">
            <div class="com_details_tit"><span class="com_details_tit_s">公司产品</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <div class="com_show_leftcont">
                <div class="com_show_l_box">
                    <div class="com_show_cp_box">
                        <ul class="com_show_cp">
                            <?php  $_smarty_tpl->tpl_vars['row'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['row']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['ProductList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['row']->key => $_smarty_tpl->tpl_vars['row']->value) {
$_smarty_tpl->tpl_vars['row']->_loop = true;
?>
                            <li>
                                <a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'productshow','id'=>$_smarty_tpl->tpl_vars['com']->value['uid'],'pid'=>$_smarty_tpl->tpl_vars['row']->value['id']),$_smarty_tpl);?>
" target="_blank">
                                    <img src="<?php echo $_smarty_tpl->tpl_vars['row']->value['pic'];?>
" width="260" height="145"/>
                                    <div class="com_show_cp_name"><?php echo $_smarty_tpl->tpl_vars['row']->value['title'];?>
</div>
                                </a>
                            </li>
                            <?php } ?>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
        <?php }?>

        
        <!-- 公司职位 start -->
        <div class="com_show_leftbox" id="job">
            <div class="com_details_tit"><span class="com_details_tit_s">招聘职位</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <?php if ($_smarty_tpl->tpl_vars['departmentNames']->value) {?>
            <div class="department_box">
                <span class="department_box_tit">部门分类：</span>
                <a href="javascript:void(0)" class="department_a department_a_cur" data-uid="<?php echo $_GET['id'];?>
" data-style="<?php echo $_GET['style'];?>
">全部</a>
                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['departmentNames']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                <a href="javascript:void(0)" class="department_a" data-name="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" data-uid="<?php echo $_GET['id'];?>
" data-style="<?php echo $_GET['style'];?>
"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a>
                <?php } ?>
            </div>
            <?php }?>
            <div class="com_show_leftcont">
                <div class="comshow_job" id="company_job_list">
                    <?php if ($_smarty_tpl->tpl_vars['num']->value=='0') {?>
                    <div class="firm_tips_no"> 该企业还没有发布职位信息!</div>
                    <?php } else { ?>

                    <?php  $_smarty_tpl->tpl_vars['jlist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['jlist']->_loop = false;
global $db,$db_config,$config;
		$time = time();
		
		
		//可以做缓存
        $paramer=array("ispage"=>"1","limit"=>"5","item"=>"“jlist“","com_id"=>"“auto.id“","nocache"=>"")
;
		$ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
        $Purl =  $ParamerArr[purl];
        global $ModuleName;
        if(!$Purl["m"]){
            $Purl["m"]=$ModuleName;
        }
		include_once  PLUS_PATH."/comrating.cache.php";
		include(CONFIG_PATH."db.data.php"); 
        $cache_array = $db->cacheget();
        $comclass_name  = $cache_array["comclass_name"];
        $comdata        = $cache_array["comdata"];
        $city_name      = $cache_array["city_name"];
        $job_name       = $cache_array["job_name"];
		$industry_name	= $cache_array["industry_name"];

		if($config[sy_web_site]=="1"){
			if($config[province]>0 && $config[province]!=""){
				$paramer[provinceid] = $config[province];
			}
			if($config[cityid]>0 && $config[cityid]!=""){
				$paramer[cityid] = $config[cityid];
			}
			if($config[three_cityid]>0 && $config[three_cityid]!=""){
				$paramer[three_cityid] = $config[three_cityid];
			}
			if($config[hyclass]>0 && $config[hyclass]!=""){
				$paramer[hy]=$config[hyclass];
			}
		}

		
		if($paramer[sdate]){
			$where = "`sdate`>".strtotime("-".intval($paramer[sdate])." day",time())." and `state`=1";
		}else{
			$where = "`state`=1";
		}
		
		//按照UID来查询（按公司地址查询可用GET[id]获取当前公司ID）
		if($paramer[com_id]){
			$where .= " AND `uid` = '$paramer[com_id]'";
			// 单查某企业下职位，排除城市、行业类别（排除分站）参数
            if(isset($paramer[provinceid])){unset($paramer[provinceid]);}
            if(isset($paramer[cityid])){unset($paramer[cityid]);}
            if(isset($paramer[three_cityid])){unset($paramer[three_cityid]);}
            if(isset($paramer[hy])){unset($paramer[hy]);}
		}
		
		if (!empty($paramer[depower])) {
		    trim($paramer[depower]) != 'all' && $where .= " AND `is_depower` = $paramer[depower]"; // all为降权和非降权均查询
		} else {
		    $where .= " AND `is_depower` = 2"; // 默认查询未降权的职位
		}

		//是否推荐职位
		if($paramer[rec]){
			
			$where.=" AND `rec_time`>=".time();
			
		}
		//企业认证条件
		if($paramer['cert']){
			
			$where.=" and `yyzz_status`=1";
		}
		//取不包含当前企业的职位
		if($paramer[nouid]){
			$where.= " and `uid`<>$paramer[nouid]";
		}
		//取不包含当前id的职位
		if($paramer[noid]){
			$where.= " and `id`<>$paramer[noid]";
		}
		//是否被锁定
		if($paramer[r_status]){
			$where.= " and `r_status`=2";
		}else{
			$where.= " and `r_status`=1";
		}
		//是否下架职位
		if($paramer[status]){
			$where.= " and `status`='1'";
		}else{
			$where.= " and `status`='0'";
		}
		//公司体制
		if($paramer[pr]){
			$where .= " AND `pr` =$paramer[pr]";
		}
		//公司行业分类
		if($paramer['hy']){
			$where .= " AND `hy` = $paramer[hy]";
		} 
		//职位大类
		if($paramer[job1]){
			$where .= " AND `job1` = $paramer[job1]";
		}
		//职位子类
		if($paramer[job1_son]){
			$where .= " AND `job1_son` = $paramer[job1_son]";
		}
		if($paramer[job1son]){
			$where .= " AND `job1_son` = $paramer[job1son]";
		}
		//职位三级分类
		if($paramer[job_post]){
			$where .= " AND (`job_post` IN ($paramer[job_post]))";
		}
		if($paramer[jobpost]){
			$where .= " AND (`job_post` IN ($paramer[jobpost]))";
		}
		//您可能感兴趣的职位--个人会员中心
		if($paramer['jobwhere']){
			$where .=" and ".$paramer['jobwhere'];
		}
		//职位分类综合查询
		if($paramer['jobids']){
			$where.= " AND (`job1` = '$paramer[jobids]' OR `job1_son`= '$paramer[jobids]' OR `job_post`='$paramer[jobids]')";
		}
		//职位分类区间,不建议执行该查询
		if($paramer['jobin']){
			$where .= " AND (`job1` IN ($paramer[jobin]) OR `job1_son` IN ($paramer[jobin]) OR `job_post` IN ($paramer[jobin]))";
		}
		//多选职位
		if($paramer["job"]){
			$where.=" AND `job_post` in ($paramer[job])";
		}
		//城市大类
		if($paramer[provinceid]){
			$where .= " AND `provinceid` = $paramer[provinceid]";
		}
		//城市子类
		if($paramer['cityid']){
			$where .= " AND (`cityid` IN ($paramer[cityid]))";
		}
		//城市三级子类
		if($paramer['three_cityid']){
			$where .= " AND (`three_cityid` IN ($paramer[three_cityid]))";
		}
		if($paramer['threecityid']){
			$where .= " AND (`three_cityid` IN ($paramer[threecityid]))";
		}
		if($paramer['cityin']){
			$where .= " AND `three_cityid` IN ($paramer[cityin])";
		}
		//学历
		if($paramer[edu]){
            $eduArr  = $comdata['job_edu'];
			$eduSort = 0;
			$eduIds  = array();
			// 职位搜索，排序比搜索小的都符合条件。如搜“硕士”，类别排序小于等于“硕士”排序的（要排除不限）都符合
			foreach ($eduArr as $k => $v) {
			    if ($v == $paramer[edu] && $comclass_name[$v] != "不限"){
			        $eduSort = $k;
                    break;
			    }
			}
			foreach ($eduArr as $k => $v) {
			    if ($k <= $eduSort && $comclass_name[$v] != "不限"){
			        $eduIds[] = $v;
			    }
			}
            if (!empty($eduIds)) {
            	$where .= " AND `edu` in (".@implode(",",$eduIds).")";
            }
		}
		//工作经验
		if($paramer[exp]){
            $expArr  = $comdata['job_exp'];
			$expSort = 0;
			$expIds  = array();
			// 职位搜索，排序比搜索小的都符合条件。如搜“五年”，类别排序小于等于“五年”排序的（要排除不限）都符合
            foreach ($expArr as $k => $v) {
                if ($v == $paramer[exp] && $comclass_name[$v] != "不限"){
                    $expSort = $k;
                    break;
                }
            }
            foreach ($expArr as $k => $v) {
                if ($k <= $expSort && $comclass_name[$v] != "不限"){
                    $expIds[] = $v;
                }
            }
            if (!empty($expIds)) {
            	$where .= " AND `exp` in (".@implode(",",$expIds).")";
            }
		}
		//到岗时间
		if($paramer[report]){
			$where .= " AND `report` = $paramer[report]";
		}
		//职位性质
		if($paramer[type]){
			$where .= " AND `type` = $paramer[type]";
		}
		//性别
		if($paramer[sex]){
			$where .= " AND `sex` = $paramer[sex]";
		}
		//应届生
		if($paramer[is_graduate]){
			$where .= " AND `is_graduate` = $paramer[is_graduate]";
		}
		//公司规模
		if($paramer[mun]){
			$where .= " AND `mun` = $paramer[mun]";
		}
		 
		if($paramer[minsalary] && $paramer[maxsalary]){
			$where.= " AND (`minsalary`>=".intval($paramer[minsalary])." and `minsalary`<=".intval($paramer[maxsalary])." and `maxsalary`<=".intval($paramer[maxsalary]).") ";

		}elseif($paramer[minsalary]&&!$paramer[maxsalary]){
			$where.= " AND (`minsalary`>=".intval($paramer[minsalary]).") ";

		}elseif(!$paramer[minsalary]&&$paramer[maxsalary]){
			$where.= " AND (`minsalary`<=".intval($paramer[maxsalary])." and `maxsalary`<=".intval($paramer[maxsalary]).") ";
		}
	    //福利待遇
		if($paramer[welfare]){
			$welfarename = $comclass_name[$paramer[welfare]];
            $where .=" AND `welfare` LIKE '%".$welfarename."%' ";
		}
		
		//城市区间,不建议执行该查询
		if($paramer[cityin]){
			$where .= " AND (`provinceid` IN ($paramer[cityin]) OR `cityid` IN ($paramer[cityin]) OR `three_cityid` IN ($paramer[cityin]))";
		}
		//紧急招聘urgent
		if($paramer[urgent]){
			$where.=" AND `urgent_time`>".time();
		}
		//更新时间区间
		if($paramer[uptime]){
			if($paramer[uptime]==1){
				$beginToday = strtotime('today');
				$where.=" AND lastupdate>$beginToday";
			}else{
				$time=time();
				$uptime = $time-($paramer[uptime]*86400);
				$where.=" AND lastupdate>$uptime";
			}
		}else{
		    if($config[sy_datacycle_job]>0){	
                // 后台-页面设置-数据周期	        
				$uptime = strtotime('-'.$config[sy_datacycle_job].' day');
				$where.=" AND lastupdate>$uptime";
		    }
		}		
		//按类似公司名称,不建议进行大数据量操作
		if($paramer[comname]){
			$where.=" AND `com_name` LIKE '%".$paramer[comname]."%'";
		}
		//按公司归属地,只适合查询一级城市分类
		if($paramer[com_pro]){
			$where.=" AND `com_provinceid` ='".$paramer[com_pro]."'";
		}
		// 关键字匹配
		if($paramer[keyword]){
		    $comuids    =   $db->select_all("company","`name` LIKE '%".$paramer['keyword']."%' OR `shortname` LIKE '%".$paramer['keyword']."%'","`uid`");
		    $cuidArr    =   array();
		    foreach($comuids as $v){
				$cuidArr[]=$v['uid'];
			}
            $where1     =   array();
			$where1[]   =   "`name` LIKE '%".$paramer[keyword]."%'";
			if($config['job_full_text_search'] == 1){
			    $where1[]   =   "`description` LIKE '%".$paramer[keyword]."%'";
			}
			if ($cuidArr) {
			    $where1[]   =   "`uid` in (".@implode(",",$cuidArr).")";
			}
            $cityid     =   array();
			foreach($city_name as $k=>$v){
				if(strpos($v,$paramer[keyword])!==false){
					$cityid[]=$k;
				}
			}
			if(!empty($cityid)){
                $class = array();
				foreach($cityid as $value){
					$class[]= "(provinceid = '".$value."' or cityid = '".$value."' or three_cityid = '".$value."')";
				}
				$where1[]=@implode(" or ",$class);
			}
			if($config['job_full_text_search'] == 1){
                $jobClassId =   array();
                foreach($job_name as $k=>$v){
                    if(strpos($v,$paramer[keyword])!==false){
                        $jobClassId[]=$k;
                    }
                }
                if(!empty($jobClassId)){
                    $class = array();
                    foreach($jobClassId as $value){
                    
                        $class[]= "(job1_son = '".$value."' or job_post = '".$value."')";
                    }
                    $where1[]=@implode(" or ",$class);
                }
			}
			$where.=" AND (".@implode(" or ",$where1).")";
		}

		//置顶招聘
		if($paramer[bid]){
		    $isZhiding = true;
			if($config[joblist_top]==0){
				//随机20条
				$paramer[limit] = 20;
			}elseif($config[joblist_top]==2){
			    //搜索置顶（职位分类|关键字）
			    $isZhiding = ($paramer[job1] || $paramer[job1_son] || $paramer[job1son] || $paramer[job_post] || $paramer[jobpost] || $paramer['jobwhere'] || $paramer['jobids'] || $paramer['jobin'] || $paramer["job"] || $paramer[keyword]) ? true : false;
			}
			
			if($isZhiding){
			    $where.="  and `xsdate`>'".time()."'";			
			}else{
			    $where.=" AND false";
			}
		} 
		//首页置顶
        if($paramer[istop]){
            $isIndexZhiding = true;
            if($config[joblist_top_index]==2){
                $paramer[limit] = 5;
            }elseif($config[joblist_top_index]==0){
                $isIndexZhiding = false;
            }
            if($isIndexZhiding){
			    $where.="  and `xsdate`>'".time()."'";			
			}else{
			    $where.=" AND false";
			}
        }
		//自定义查询条件，默认取代上面任何参数直接使用该语句
		if($paramer[where]){
			$where = $paramer[where];
		}

		//查询条数
		$limit = '';
		if($paramer[limit]){

			$limit = " limit ".$paramer[limit];
		}
		if($paramer[ispage]){
			$limit = PageNav($paramer,$_GET,"company_job",$where,$Purl,"",$paramer[islt]?$paramer[islt]:"6",$_smarty_tpl);        
		}

		//排序字段默认为更新时间
		//置顶设置为随机20条时，随机查询
		if($paramer[bid] && $config[joblist_top]==0){
			$order = " ORDER BY rand() ";
		}elseif($paramer[istop] && $config[joblist_top_index]==2){
		    $order = " ORDER BY rand() ";
		}else{
			if($paramer[order] && $paramer[order]!="lastdate"){
				$order = " ORDER BY ".str_replace("'","",$paramer[order])."  ";
			}else{
				$order = " ORDER BY `lastupdate` ";
			}
		}
		//排序规则 默认为倒序
		if($paramer[sort]){
			$sort = $paramer[sort];
		}else{
			$sort = " DESC";
		} 
		$where.=$order.$sort;
		
		$jlist = $db->select_all("company_job",$where.$limit);

		if(is_array($jlist) && !empty($jlist)){
			$comuid=$jobid=array();
			foreach($jlist as $key=>$value){
				if(in_array($value['uid'],$comuid)==false){$comuid[] = $value['uid'];}
				if(in_array($value['id'],$jobid)==false){$jobid[] = $value['id'];} 
			}
			$comuids = @implode(',',$comuid);
			$jobids = @implode(',',$jobid);
			//减少曝光量统计维度 只有列表才统计
			if($paramer[ispage]){
				$db->update_all("company_job", "`jobexpoure` = `jobexpoure` + 1", "`id` in ($jobids)");
			}
			

			if($comuids){
				$r_uids=$db->select_all("company","`uid` IN (".$comuids.")","`uid`,`hy`,`shortname`,`welfare`,`hotstart`,`hottime`,`fact_status`");
				if(is_array($r_uids)){
					foreach($r_uids as $key=>$value){
						if($value[shortname]){
    						$value['shortname_n'] = $value[shortname];
    					}
						if($value['hotstart']<=time() && $value['hottime']>=time()){
							$value['hotlogo'] = 1;
						}
                        $value['hy_n'] = $industry_name[$value[hy]];
						$r_uid[$value['uid']] = $value;
					}
				}
			}
			
 			if($paramer[bid]){
				$noids=array();
			}	
			if ($_COOKIE['uid'] && $_COOKIE['usertype']==1){
			    $lookJob =   $db->select_all("look_job","`uid` = ".$_COOKIE['uid'], "`jobid`");
			    if (!empty($lookJob)){
			        foreach($lookJob as $key=>$value){
						$lookJobIdArr[] = $value['jobid'];
					}
			    }
			}
			foreach($jlist as $key=>$value){

				if($paramer[bid]){
					$noids[] = $value[id];
				}
				if($paramer[istop]){
				    $noids[] = $value[id];
				}
				//筛除重复
				if($paramer[noids]==1 && !empty($noids) && in_array($value['id'],$noids)){
					unset($jlist[$key]);
					continue;
				}else{
					$jlist[$key] = $db->array_action($value,$cache_array);
					$jlist[$key][stime] = date("Y-m-d",$value[sdate]);
					$jlist[$key][etime] = date("Y-m-d",$value[edate]);
					if($arr_data['sex'][$value['sex']]){
						$jlist[$key][sex_n]=$arr_data['sex'][$value['sex']];
					}
					$jlist[$key][lastupdate] =lastupdateStyle($value[lastupdate]);
					$jlist[$key][job_salary] = salaryUnit($value[minsalary], $value[maxsalary]);
					
					if($r_uid[$value['uid']][shortname]){
						$jlist[$key][com_name] =$r_uid[$value['uid']][shortname];
					}
					if(!empty($value[zp_minage]) && !empty($value[zp_maxage])){					   
					    if($value[zp_minage]==$value[zp_maxage]){
					        $jlist[$key][job_age] = $value[zp_minage]."周岁以上";
					    }else{
					        $jlist[$key][job_age] = $value[zp_minage]."-".$value[zp_maxage]."周岁";
					    }
					}else if(!empty($value[zp_minage]) && empty($value[zp_maxage])){
					    $jlist[$key][job_age] = $value[zp_minage]."周岁以上";
					}else{
					     $jlist[$key][job_age] = 0;
					}
					if($value[zp_num]==0){
					    $jlist[$key][job_number] = "";
					}else{
					    $jlist[$key][job_number] = $value[zp_num]." 人";
					}			
                    $jlist[$key][hotlogo] = $r_uid[$value['uid']][hotlogo];
                    $jlist[$key][hy_n] = $r_uid[$value['uid']][hy_n];
                    $jlist[$key][fact_status] = $r_uid[$value['uid']][fact_status];
					$jlist[$key][logo] = checkpic($value['com_logo'],$config['sy_unit_icon']);
					$jlist[$key][pr_n] = $comclass_name[$value[pr]];
					$jlist[$key][mun_n] = $comclass_name[$value[mun]];
					$time=$value['lastupdate'];
					//今天开始时间戳
					$beginToday=mktime(0,0,0,date('m'),date('d'),date('Y'));
					//昨天开始时间戳
					$beginYesterday=mktime(0,0,0,date('m'),date('d')-1,date('Y'));
					
					if($time>$beginYesterday && $time<$beginToday){
						$jlist[$key]['time'] ="昨天";
					}elseif($time>$beginToday){	
						$jlist[$key]['time'] = $jlist[$key]['lastupdate'];
						$jlist[$key]['redtime'] =1;
					}else{
						$jlist[$key]['time'] = date("Y-m-d",$value['lastupdate']);
					}
    
                     // 前天
    				$beforeYesterday=mktime(0,0,0,date('m'),date('d')-2,date('Y'));

					if($value['sdate']>$beforeYesterday){
						$jlist[$key]['newtime'] =1;
					}
					//获得福利待遇名称
					if($value[welfare]){
					    $value[welfare] = str_replace(' ', '',$value[welfare]);
						$welfareList = @explode(',',trim($value[welfare]));

						if(!empty($welfareList)){
							$jlist[$key][welfarename] =array_filter($welfareList);
						}
					}elseif($r_uid[$value['uid']][welfare]){
						$welfareList = @explode(',',trim($r_uid[$value['uid']][welfare]));
						$jlist[$key][welfarename] =$welfareList;
					}
					//截取公司名称
					if($paramer[comlen]){
						if($r_uid[$value['uid']][shortname]){
							$jlist[$key][com_n] = mb_substr($r_uid[$value['uid']][shortname],0,$paramer[comlen],"utf-8");
						}else{
							$jlist[$key][com_n] = mb_substr($value['com_name'],0,$paramer[comlen],"utf-8");
						}
					}
					//截取职位名称
					if($paramer[namelen]){
						if($value['rec_time']>time()){
							$jlist[$key][name_n] = "<font color='red'>".mb_substr($value['name'],0,$paramer[namelen],"utf-8")."</font>";
						}else{
							$jlist[$key][name_n] = mb_substr($value['name'],0,$paramer[namelen],"utf-8");
						}
					}else{
						if($value['rec_time']>time()){
							$jlist[$key]['name_n'] = "<font color='red'>".$value['name']."</font>";
						}else{
							$jlist[$key][name_n] = $value['name'];
						}
					}
					//构建职位伪静态URL
					$jlist[$key][job_url] = Url("job",array("c"=>"comapply","id"=>$value[id]),"1");
					//构建企业伪静态URL
					$jlist[$key][com_url] = Url("company",array("c"=>"show","id"=>$value[uid]));
					
					foreach($comrat as $k=>$v){
						if($value[rating]==$v[id]){
							$jlist[$key][color] = str_replace("#","",$v[com_color]);
							if($v[com_pic]){
								$jlist[$key][ratlogo] = checkpic($v[com_pic]);
							}
							$jlist[$key][ratname] = $v[name];
						}
					}
					if($paramer[keyword]){
						$jlist[$key][name_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$jlist[$key][name_n]);
						$jlist[$key][com_n]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$jlist[$key][com_n]);
						$jlist[$key][job_city_one]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[provinceid]]);
						$jlist[$key][job_city_two]=str_replace($paramer[keyword],"<font color=#FF6600 >".$paramer[keyword]."</font>",$city_name[$value[cityid]]);
					}
					//  是否浏览过
                    $jlist[$key]['isLookEd'] = 0;
                    if(in_array($value['id'], $lookJobIdArr)){
                        $jlist[$key]['isLookEd'] = 1;
                    }
				}
			}
			if(is_array($jlist)){
				if($paramer[keyword]!=""&&!empty($jlist)){
					addkeywords('3',$paramer[keyword]);
				}
			}
		}$jlist = $jlist; if (!is_array($jlist) && !is_object($jlist)) { settype($jlist, 'array');}
foreach ($jlist as $_smarty_tpl->tpl_vars['jlist']->key => $_smarty_tpl->tpl_vars['jlist']->value) {
$_smarty_tpl->tpl_vars['jlist']->_loop = true;
?>
                    <div class="firm_post">
                        <div class="com_details_com_otherjob_l">
                            <div class="com_details_com_otherjob_name"><a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>$_smarty_tpl->tpl_vars['jlist']->value['id']),$_smarty_tpl);?>
" target="_blank" class=""><?php echo $_smarty_tpl->tpl_vars['jlist']->value['name'];?>
</a></div>
                            <div class="com_details_com_otherjob_info">
                                <?php if ($_smarty_tpl->tpl_vars['jlist']->value['job_exp']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['jlist']->value['job_exp'];?>
经验
                                <?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['jlist']->value['job_exp']&&$_smarty_tpl->tpl_vars['jlist']->value['job_edu']) {?>
                                <span class="com_details_line">|</span>
                                <?php }?>
                                <?php if ($_smarty_tpl->tpl_vars['jlist']->value['job_edu']) {?>
                                    <?php echo $_smarty_tpl->tpl_vars['jlist']->value['job_edu'];?>
学历
                                <?php }?>
                            </div>
                        </div>
                        <div class="com_details_com_otherjob_c">
                            <div class="com_details_com_otherjob_xz"><?php echo $_smarty_tpl->tpl_vars['jlist']->value['job_salary'];?>
</div>
                            <div class="com_details_com_otherjob_city"><?php echo $_smarty_tpl->tpl_vars['jlist']->value['job_city_two'];?>
</div>
                        </div>
                        <div class="com_details_com_otherjob_r">
                            <div class="com_details_com_otherjob_time"><?php echo $_smarty_tpl->tpl_vars['jlist']->value['lastupdate'];?>
</div>
                            <a href="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','id'=>$_smarty_tpl->tpl_vars['jlist']->value['id']),$_smarty_tpl);?>
" target="_blank" class="com_details_com_otherjob_sq">申请</a>
                        </div>
                    </div>
                    <?php } ?>

                    <?php if ($_smarty_tpl->tpl_vars['pagenav']->value) {?>
                    <div class="pages" style="margin-top:20px">
                        <a href="javascript:void(0);" onclick="page('<?php echo $_GET['id'];?>
','1','5','1');">上一页</a>
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['k'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['pgarr']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['k']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
                        <?php if ($_smarty_tpl->tpl_vars['k']->value==0) {?>
                        <a href="javascript:void(0);" class="selected" onclick="page('<?php echo $_GET['id'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value-1;?>
',5,'<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
')"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a>
                        <?php } else { ?>
                        <a href="javascript:void(0);" onclick="page('<?php echo $_GET['id'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value-1;?>
',5,'<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
')"><?php echo $_smarty_tpl->tpl_vars['v']->value;?>
</a>
                        <?php }?>
                        <?php } ?>
                        <a href="javascript:void(0);" onclick="page('<?php echo $_GET['id'];?>
','1','5','2');">下一页</a>
                        <a href="javascript:void(0);">总页码 <?php echo $_smarty_tpl->tpl_vars['pages']->value;?>
</a>
                    </div>
                    <?php }?>

                    <?php }?>
                </div>
            </div>
        </div>
        <div class="maincenters mt20">
            <div class="com_show_leftbox" style="margin-top:0px;">
                <div id="sortBoxs">
                    <div class="search_menuBoxs">
                        <ul>
                            <li class="search_curs" id="typezb" onmousemove="searchtype('zb');">周边招聘<i class="search_curs_line"></i></li>
                            <li id="typezp" onmousemove="searchtype('zp');">招聘频道<i class="search_curs_line"></i></li>
                            <li id="typerm" onmousemove="searchtype('rm');">热门搜索<i class="search_curs_line"></i></li>
                        </ul>
                    </div>

                    <div class="contentBoxs">
                        <!-- 周边招聘 -->
                        <div class="contentBox_conts " id="type_zb">
                            <div class="Industry_lists">
                                <?php if ($_smarty_tpl->tpl_vars['city']->value['three_cityid']) {?>

                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['city']->value['cityid']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <a href="<?php echo smarty_function_listurl(array('provinceid'=>$_smarty_tpl->tpl_vars['city']->value['provinceid'],'cityid'=>$_smarty_tpl->tpl_vars['city']->value['cityid'],'type'=>'three_cityid','v'=>$_smarty_tpl->tpl_vars['v']->value,'page'=>1),$_smarty_tpl);?>
"><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
招聘</a>
                                <?php } ?>

                                <?php } elseif ($_smarty_tpl->tpl_vars['city']->value['cityid']) {?>

                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['city']->value['provinceid']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <a href="<?php echo smarty_function_listurl(array('provinceid'=>$_smarty_tpl->tpl_vars['city']->value['provinceid'],'type'=>'cityid','v'=>$_smarty_tpl->tpl_vars['v']->value,'page'=>1),$_smarty_tpl);?>
"><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
招聘</a>
                                <?php } ?>

                                <?php } else { ?>

                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['city_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <a href="<?php echo smarty_function_listurl(array('type'=>'provinceid','v'=>$_smarty_tpl->tpl_vars['v']->value,'page'=>1),$_smarty_tpl);?>
"><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
招聘</a>
                                <?php } ?>

                                <?php }?>
                            </div>
                        </div>

                        <!-- 招聘频道 -->
                        <div class="contentBox_conts none" id="type_zp">
                            <div class="Industry_lists">
                                <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['industry_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                                <a href="<?php echo smarty_function_listurl(array('type'=>'hy','v'=>$_smarty_tpl->tpl_vars['v']->value),$_smarty_tpl);?>
"><?php echo $_smarty_tpl->tpl_vars['industry_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
招聘</a>
                                <?php } ?>
                            </div>
                        </div>

                        <!-- 热门搜索 -->
                        <div class="contentBox_conts none" id="type_rm">
                            <div class="Industry_lists">
                                <?php  $_smarty_tpl->tpl_vars['keylist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['keylist']->_loop = false;
global $config;$paramer=array("limit"=>"20","recom"=>"1","type"=>"3","item"=>"“keylist“","nocache"=>"")
;$list=array();
        
        $ParamerArr = GetSmarty($paramer,$_GET,$_smarty_tpl);
		$paramer = $ParamerArr[arr];
		//是否推荐
		if($paramer[recom]){
			$tuijian = 1;
		}
		//类别
		if($paramer[type]){
			$type = $paramer[type];
		}
		//查询条数
		if($paramer[limit]){
			$limit=$paramer[limit];
		}else{
			$limit=5;
		}
		include PLUS_PATH."/keyword.cache.php";
		if($paramer[iswap]){
			$wap = "/wap";
		}else{
			$index =1;
		}
		if(is_array($keyword)){
			if($paramer[iswap]){
				$i=0;
				foreach($keyword as $k=>$v){
					if($tuijian && $v[tuijian]!=1){
						continue;
					}
					if($type && $v[type]!=$type){
						continue;
					}

					$i++;
					if($v[type]=="1"){
						$v[url] = Url("wap",array("c"=>"once","keyword"=>$v['key_name']));
						$v[type_name]='店铺招聘';
					}elseif($v['type']=="13"){
						$v['url'] = Url("wap",array("c"=>"tiny","keyword"=>$v['key_name']));
						$v['type_name']='普工简历';
					}elseif($v[type]=="3"){
						$v[url] = Url("wap",array("c"=>"job","keyword"=>$v['key_name']));
						$v[type_name]='职位';
					}elseif($v['type']=="4"){
						$v['url'] = Url("wap",array("c"=>"company","keyword"=>$v['key_name']));
						$v['type_name']='公司';
					}elseif($v['type']=="5"){
						$v['url'] = Url("wap",array("c"=>"resume","keyword"=>$v['key_name']));
						$v['type_name']='人才';
					}
					$v['key_title']=$v['key_name'];
					if($v['color']){
						$v['key_name']="<font color='".$v['color']."'>".$v['key_name']."</font>";
					}
					$list[] = $v;
					if($i==$limit){
						break;
					}
				}
			}else{
				$i=0;
				foreach($keyword as $k=>$v){
					if($tuijian && $v['tuijian']!=1){
						continue;
					}
					if($type && $v['type']!=$type){
						continue;
					}
					$i++;
					if($v['type']=="1"){
						$v['url'] = Url("once",array("keyword"=>$v['key_name']));
						$v['type_name']='店铺招聘';
					}elseif($v['type']=="2"){
						$v['url'] = Url("part",array("keyword"=>$v['key_name']));
						$v['type_name']='兼职';
					}elseif($v['type']=="13"){
						$v['url'] = Url("tiny",array("keyword"=>$v['key_name']));
						$v['type_name']='普工简历';
					}elseif($v['type']=="3"){
						$v['url'] = Url("job",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='职位';
					}elseif($v['type']=="4"){
						$v['url'] = Url("company",array("keyword"=>$v['key_name']));
						$v['type_name']='公司';
					}elseif($v['type']=="5"){
						$v['url'] = Url("resume",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='人才';
					}else if($v['type']=="12"){
						$v['url'] = Url("ask",array("c"=>"search","keyword"=>$v['key_name']));
						$v['type_name']='问答';
					}
					$v['key_title']=$v['key_name'];
					if($v['color']){
						$v['key_name']="<font color='".$v['color']."'>".$v['key_name']."</font>";
					}

					$list[] = $v;
					if($i==$limit){
						break;
					}
				}
			}
		}$list = $list; if (!is_array($list) && !is_object($list)) { settype($list, 'array');}
foreach ($list as $_smarty_tpl->tpl_vars['keylist']->key => $_smarty_tpl->tpl_vars['keylist']->value) {
$_smarty_tpl->tpl_vars['keylist']->_loop = true;
?>
                                <a href="<?php echo $_smarty_tpl->tpl_vars['keylist']->value['url'];?>
"><?php echo $_smarty_tpl->tpl_vars['keylist']->value['key_name'];?>
</a>
                                <?php } ?>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="com_details_right">
        <div class="com_details_rightbox">
            <div class="com_details_tit"><span class="com_details_tit_s">公司地址</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <?php if ($_smarty_tpl->tpl_vars['com']->value['address']) {?>
            <div class="com_details_tel_me">
                <div class=""><?php echo $_smarty_tpl->tpl_vars['com']->value['address'];?>
</div>
            </div>
            <?php }?>
            <?php if ($_smarty_tpl->tpl_vars['com']->value['x']!=''&&$_smarty_tpl->tpl_vars['com']->value['y']!=''&&$_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
            <div class="com_show_lmap">
                <div class="frc_map" id="company_baidu_map">
                    <?php echo '<script'; ?>
 type="text/javascript">
                        window._AMapSecurityConfig = {
                            securityJsCode: '<?php echo $_smarty_tpl->tpl_vars['config']->value['map_secret'];?>
'
                        }
                    <?php echo '</script'; ?>
>
                    <?php echo '<script'; ?>
 type="text/javascript" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['mapurl'];?>
" charset="utf-8"><?php echo '</script'; ?>
>
                    <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/map.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
                    <div id="maps_container" style="width:245px;height:205px;"></div>
                    <input type="hidden" id="map_x" value="<?php echo $_smarty_tpl->tpl_vars['com']->value['x'];?>
"/>
                    <input type="hidden" id="map_y" value="<?php echo $_smarty_tpl->tpl_vars['com']->value['y'];?>
"/>
                </div>
                <div class="frc_map_look">
                    <a href="javascript:showmap('map_show','<?php echo $_smarty_tpl->tpl_vars['com']->value['x'];?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['y'];?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['name'];?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['address'];?>
');">查看完整地图</a>
                </div>
            </div>
            <?php }?>
        </div>
        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_message']==1) {?>
        <div class="com_details_rightbox">
            <div class="com_details_tit"><span class="com_details_tit_s">公司问答</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <!--有提问前-->
            <?php if (!$_smarty_tpl->tpl_vars['msgList']->value) {?>
            <div class="job_details_comask_p">有疑问？快来问问吧</div>
            <div class="job_details_comask_bth">
                <?php if ($_smarty_tpl->tpl_vars['uid']->value) {?>
                <?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
                <a href="javascript:;" onclick="showmessage('$uid','<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
')" class="job_details_comask_bth_a">我要提问</a>
                <?php } else { ?>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>
                <a onclick="layer.msg('请先申请个人用户才能提问', 2, 8)" href="javascript:;" class="job_details_comask_bth_a">我要提问</a>
                <?php } else { ?>
                <a onclick="layer.msg('只有个人用户才能提问', 2, 8)" href="javascript:;" class="job_details_comask_bth_a">我要提问</a>
                <?php }?>
                <?php }?>
                <?php } else { ?>
                <a href="javascript:;" onclick="showlogin()" class="job_details_comask_bth_a">我要提问</a>
                <?php }?>
            </div>
            <?php } else { ?>
            <!--有提问后-->
            <div class="yun_newedition_askbox">
                <?php if ($_smarty_tpl->tpl_vars['uid']->value) {?>
                <?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
                <a href="javascript:;" onclick="showmessage('$uid','<?php echo $_smarty_tpl->tpl_vars['usertype']->value;?>
')" class="job_details_tw">我要提问</a>
                <?php } else { ?>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_user_change']==1) {?>
                <a onclick="layer.msg('请先申请个人用户才能提问', 2, 8)" href="javascript:;" class="job_details_tw">我要提问</a>
                <?php } else { ?>
                <a onclick="layer.msg('只有个人用户才能提问', 2, 8)" href="javascript:;" class="job_details_tw">我要提问</a>
                <?php }?>
                <?php }?>
                <?php } else { ?>
                <a href="javascript:;" onclick="showlogin()" class="job_details_tw">我要提问</a>
                <?php }?>
                <?php  $_smarty_tpl->tpl_vars['msglist'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['msglist']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['msgList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['msglist']->key => $_smarty_tpl->tpl_vars['msglist']->value) {
$_smarty_tpl->tpl_vars['msglist']->_loop = true;
?>
                <div class="yun_newedition_asklist">
                    <div class="yun_newedition_showask">
                        <i class="yun_newedition_showask_icon">问</i>
                        <?php echo $_smarty_tpl->tpl_vars['msglist']->value['content'];?>

                    </div>
                    <div class="yun_newedition_showand">
                        <i class="yun_newedition_showand_icon">答</i>
                        <?php if ($_smarty_tpl->tpl_vars['msglist']->value['reply']) {
echo $_smarty_tpl->tpl_vars['msglist']->value['reply'];
} else { ?>企业尚未回复<?php }?>
                        <div class="yun_newedition_showand_time"> <?php echo smarty_modifier_date_format($_smarty_tpl->tpl_vars['msglist']->value['datetime'],"%Y-%m-%d");?>
</div>
                    </div>
                </div>
                <?php } ?>
            </div>
            <?php }?>
            <!--提问 end-->
            <div class="" id="showmessage" style="display:none;">
                <div class="job_hr_ly_box" style="padding-top:10px;">
                    <form action="<?php echo smarty_function_url(array('m'=>'job','c'=>'comapply','a'=>'msg'),$_smarty_tpl);?>
" method="post" target="supportiframe" onsubmit="return com_msg();">
                        <div>
                            <textarea class="comapply_Leave_fb_text" name="content" id='msg_content' placeholder='请输入您的疑问。比如所在地、年薪、福利等等，我会及时给您回复！期待与您合作。'></textarea>
                            <input type="hidden" name="job_uid" value="<?php echo $_smarty_tpl->tpl_vars['com']->value['uid'];?>
"/>
                        </div>

                        <div class="affirm affirm_yz">
                            <input class="zx_yx_input fl" id="msg_CheckCode" type="text" placeholder="请输入验证码" value="" maxlength="4" name="authcode"/>
                            <img class="zx_yx_input_img fl" id="vcode_imgs" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" onclick="checkCode('vcode_imgs');"/>
                            <?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
                            <input type="submit" value="提交咨询" name="submit" class="comapply_Leave_fb_sub "/>
                            <?php } else { ?>
                            <?php if ($_smarty_tpl->tpl_vars['uid']->value) {?>
                            <input type="button" value="提交咨询" onclick="layer.msg('只有个人用户才能咨询', 2, 8)" class="comapply_Leave_fb_sub"/>
                            <?php } else { ?>
                            <input type="button" value="提交咨询" onclick="showlogin('1');" class="comapply_Leave_fb_sub "/>
                            <?php }?>
                            <?php }?>
                        </div>
                        <div class="comapply_Leave_fb_s">
                            <?php if ($_smarty_tpl->tpl_vars['usertype']->value==1) {?>
                            <a class="comapply_lea_a" href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/member/index.php?c=commsg" target="_blank"> 查看我的咨询>> </a>
                            <?php }?>
                        </div>
                    </form>
                </div>
            </div>
        </div>
        <?php }?>
        
        <?php if (!empty($_smarty_tpl->tpl_vars['NewsList']->value)) {?>
        <div class="com_details_rightbox">
            <div class="com_details_tit"><span class="com_details_tit_s">企业新闻</span><i class="com_details_tit_line yun_bg_color"></i></div>
            <ul class="com_show_news">
                <?php  $_smarty_tpl->tpl_vars['row'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['row']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['NewsList']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['row']->key => $_smarty_tpl->tpl_vars['row']->value) {
$_smarty_tpl->tpl_vars['row']->_loop = true;
?>
                <li><p><a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'newsshow','id'=>$_smarty_tpl->tpl_vars['com']->value['uid'],'nid'=>$_smarty_tpl->tpl_vars['row']->value['id']),$_smarty_tpl);?>
" target="_blank"><?php echo $_smarty_tpl->tpl_vars['row']->value['title'];?>
</a></p></li>
                <?php } ?>
            </ul>
        </div>
        <?php }?>
    </div>
</div>
<div id="map_show" style="display:none;">
    <div class="map_query_box">
        <div id="map_container" style="width:1100px;height:450px;"></div>
        <div class="map_query" id="map_search">
            <div class="map_query_tit">线路查询 <span class="map_query_tit_s">注：地图可以左击拖拽任意位置</span></div>
            <div class="map_query_list"><span class="map_query_list_s">起点</span><i class="map_query_list_line"></i>
                <input type="text" class="map_startlist map_query_text" placeholder="输入起点" id="map_start" name="map_start" />
            </div>
            <div class="map_query_list"><span class="map_query_list_s">终点</span><i class="map_query_list_line"></i>
                <?php if ($_smarty_tpl->tpl_vars['com']->value['x']) {?>
                <input type="text" class="map_startend map_query_text" id="map_end" name="map_end"/>
                <?php } else { ?>
                <input type="text" class="map_startend map_query_text" placeholder="输入终点" value="<?php echo $_smarty_tpl->tpl_vars['Info']->value['address'];?>
" id="map_end" name="map_end"/>
                <?php }?>
            </div>
            <div class="map_query_bth_box">
                <input type="button" class="mapsubmit map_query_bth " value="公交查询" name="submit" onclick="bus_query('map_container', '<?php echo $_smarty_tpl->tpl_vars['com']->value['x'];?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['y'];?>
');"/>
                <span><input type="button" class="mapsubmit map_query_bth map_query_bth_car" value="驾车查询" name="submit" onclick="map_drving('map_container', '<?php echo $_smarty_tpl->tpl_vars['com']->value['x'];?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['y'];?>
');"/></span>
            </div>
        </div>
        <div class="map_query_result">
            <div id="panel"></div>
        </div>
    </div>
</div>
<div id="telQrcodeBox" class="none">
    <div class="job_tel_wx_box">
        <div id="tel_wxqrcode" class="job_tel_wx_zs">正在获取二维码...</div>
        <div class="job_tel_wx_p"><span class="job_tel_wx_bth">请使用微信扫一扫</span></div>
    </div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/public_search/hb.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/jquery/jquery_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/css/layui.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/layui.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/phpyun_layer.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/lazyload.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/jquery.json.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<!--[if IE 6]>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/png.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
    DD_belatedPNG.fix('.png');
<?php echo '</script'; ?>
>
<![endif]-->
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/js/ScrollPic.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>
    layui.use(['layer'], function () {
        var layer = layui.layer,
            $ = layui.$;
        layer.photos({
            photos: '#layer-pic',
            anim: 5
        });
    });

    <!--//--><![CDATA[//><!--
    var li_num = $("#layer-pic div").length;
    if (li_num > 3) { //如果图片数量不足，就不执行切换
        var scrollPic_02 = new ScrollPic();
        scrollPic_02.scrollContId = "layer-pic"; //内容容器ID
        scrollPic_02.arrLeftId = "LeftArr"; //左箭头ID
        scrollPic_02.arrRightId = "RightArr"; //右箭头ID
        scrollPic_02.frameWidth = 820; //显示框宽度
        scrollPic_02.pageWidth = 275; //翻页宽度
        scrollPic_02.speed = 10; //移动速度(单位毫秒，越小越快)
        scrollPic_02.space = 10; //每次移动像素(单位px，越大越快)
        scrollPic_02.autoPlay = true; //自动播放
        scrollPic_02.autoPlayTime = 2; //自动播放间隔时间(秒)
        scrollPic_02.initialize(); //初始化
    }
    //--><!]]>

    $(function () {
        tianyancha('<?php echo smarty_function_url(array('m'=>'ajax','c'=>'getbusiness'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['com']->value['name'];?>
');

        <?php if ($_smarty_tpl->tpl_vars['com']->value['y']!=''&&$_smarty_tpl->tpl_vars['com']->value['x']!=''&&$_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
            getmapnoshowcont_diffDomains('maps_container', "<?php echo $_smarty_tpl->tpl_vars['com']->value['x'];?>
", "<?php echo $_smarty_tpl->tpl_vars['com']->value['y'];?>
", "map_x", "map_y", 'company_baidu_map');
        <?php }?>

        $(".firm_post").hover(function () {
            $(this).css("background", "#f4f4f4");
            $(this).find(".firm_post_right").show();
        }, function () {

            $(this).css("background", "");
            $(this).find(".firm_post_right").hide();
        });
        var cheight = $("#com_content").height();
        if (parseInt(cheight) > 350) {
            $("#com_content").attr('style', 'width:100%;height:350px; overflow:hidden');
            $(".company_show_more").show();
        }
    });

    function showcc() {
        $("#com_content").attr('style', 'width:100%;height:auto; overflow:hidden');
        $(".company_show_more").find('a').html('收起');
        $(".company_show_more").find('a').attr('onclick', 'hidecc()');
    }

    function hidecc() {

        $("#com_content").attr('style', 'width:100%;height:350px; overflow:hidden');
        $(".company_show_more").find('a').html('查看更多');
        $(".company_show_more").find('a').attr('onclick', 'showcc()');
    }

    function page(id, page, limit, updown) {
        if (page < 1) {
            page = 1;
        }
        loadlayer();
        $.ajax({
            type: "POST",
            global: false,
            url: "<?php echo smarty_function_url(array('m'=>'company','c'=>'prestr'),$_smarty_tpl);?>
",
            data: {
                id: id,
                page: page,
                limit: limit,
                updown: updown
            },
            success: function (data) {
                layer.closeAll('loading');
                var data = eval("(" + data + ")");
                if (data.num < 0) {
                    var $html = '<div class="firm_tips_no"> 该企业还没有发布职位信息!</div>'
                } else {
                    var $html = data.joblist;
                }
                $("#company_job_list").html($html);
            }
        });
    }

    function searchtype(id) {
        $(".search_curs").removeClass("search_curs");
        $("#type" + id).addClass("search_curs");
        $(".contentBox_conts").hide();
        $("#type_" + id).show();
    }

    function showmessage($uid, usertype) {

        if ($uid) {
            if (usertype != 1) {
                layer.msg('只有求职者可以提问！', 2, 8);
                return false;
            }
            checkCode('vcode_imgs');
            var msgLayer = layer.open({

                type: 1,
                title: '我要提问',
                closeBtn: 1,
                border: [10, 0.3, '#000', true],
                area: ['auto', 'auto'],
                content: $("#showmessage")
            });
        } else {
            showlogin(1);
        }
    }

    $(document).ready(function () {
        $(".department_a").click(function () {
            $(".department_a").removeClass('department_a_cur');
            $(this).addClass('department_a_cur');
            var departmentName = $(this).attr('data-name');
            var comuid = $(this).attr('data-uid');
            var style = $(this).attr('data-style');
            var html = '';
            if (departmentName) {
                $.ajax({
                    url: "<?php echo smarty_function_url(array('m'=>'company','c'=>'departmentjob'),$_smarty_tpl);?>
",
                    data: {comuid: comuid, departmentName: departmentName, style: style},
                    type: 'POST',
                    success: function (data) {
                        $('#company_job_list').html(data);
                    }
                });
            } else {
                page(comuid, 1, 5, 1);
            }
        });
    });

    function departmentjobpage(comuid, departmentName, style, page, limit, updown) {
        var html = '';
        $.ajax({
            url: "<?php echo smarty_function_url(array('m'=>'company','c'=>'departmentjob'),$_smarty_tpl);?>
",
            data: {
                comuid: comuid,
                departmentName: departmentName,
                style: style,
                page: page,
                limit: limit,
                updown: updown
            },
            type: 'POST',
            success: function (data) {
                $('#company_job_list').html(data);
            }
        });
    }

    function wxscanshowtel(id) {
        $('#tel_wxqrcode').html('<img src="index.php?c=telQrcode&id=' + id + '" width="120" height="120">');
        $.layer({
            type: 1,
            title: '微信扫码查看联系方式',
            closeBtn: [0, true],
            offset: ['100px', ''],
            border: [10, 0.3, '#000', true],
            area: ['300px', '320px'],
            page: {
                dom: "#telQrcodeBox"
            }
        });
    }

<?php echo '</script'; ?>
>

<iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');" class="none"></iframe>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/public_search/login.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
