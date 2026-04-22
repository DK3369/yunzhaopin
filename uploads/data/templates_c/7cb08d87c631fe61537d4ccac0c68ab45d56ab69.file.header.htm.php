<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:35:40
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/header.htm" */ ?>
<?php /*%%SmartyHeaderCode:154383221169e8966c36ec18-49974110%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '7cb08d87c631fe61537d4ccac0c68ab45d56ab69' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/header.htm',
      1 => 1700725934,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '154383221169e8966c36ec18-49974110',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'wap_style' => 0,
    'config_wapdomain' => 0,
    'backurl' => 0,
    'headertitle' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8966c37af78_25018372',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8966c37af78_25018372')) {function content_69e8966c37af78_25018372($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><!DOCTYPE html PUBLIC "-//WAPFORUM//DTD XHTML Mobile 1.0//EN" "http://www.wapforum.org/DTD/xhtml-mobile10.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">

    <head>
        <meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
        <meta http-equiv="Cache-Control" content="no-cache" />
        <title><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
</title>
        <meta name="keywords" content="人才招聘,网络招聘,wap" />
        <meta name="description" content="人才招聘网wap网站" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0,minimum-scale=1.0,maximum-scale=1.0,user-scalable=no" />
        <meta name="MobileOptimized" content="240" />
        <meta http-equiv="Expires" content="0" />
        <meta http-equiv="Pragma" content="no-cache" />
        <meta content="yes" name="apple-mobile-web-app-capable" />
        <meta content="black" name="apple-mobile-web-app-status-bar-style" />
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/flexible.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
        <link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/base.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
		<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/member/memberuserwap.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
		<link href="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/css/member/memberwap.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet">
		<link href="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/index.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" />
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/jquery.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/vue.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/lib/vant.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/js/vant/phpyun_vant.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/user.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
    </head>

    <body>
    	
        <?php if (isset($_GET['c'])&&!in_array($_GET['c'],array('sysnews','addresume'))) {?>
		
		<?php if (in_array($_GET['c'],array('finance','loglist','change','pay','withdraw','integral'))) {?>
		<!--黑色背景头部-->
		<div class="m_backheader">
			
				<div class="m_headericon"><a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" width="100%" height="100%">
				</a></div>
			
			<div class="m_header_cont"><?php echo $_smarty_tpl->tpl_vars['headertitle']->value;?>
</div>
		</div>
		<?php } elseif (in_array($_GET['c'],array('resume','info','addexpect','addresumeson'))) {?>
		<!--白色背景头部-->
		<div class="m_whiteheader">
			<div class="m_whiteheaderfid ">
				<a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>"
					 class="m_headericon">
						<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" width="100%" height="100%">
					
				</a>
				<div class="m_header_cont"><?php echo $_smarty_tpl->tpl_vars['headertitle']->value;?>
</div>
				<?php if ($_GET['c']=='resume') {?>
				<!--编辑简历 -->
				<div class="Edit_your_resume_header_right" onclick="resumeMore()">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/add_newdot.png" alt="" width="100%" height="100%">
				</div>
				<?php } elseif ($_GET['c']=='info'||$_GET['c']=='addexpect') {?>
				<!--基本信息和编辑求职意向 -->
				<!--<div class="personal_details_header_save" id="headSave" onclick="headSave('<?php echo $_GET['c'];?>
')">保存</div>-->
				<?php } elseif ($_GET['c']=='addresumeson') {?>
				<!--简历附表 -->
				<div class="personal_details_header_save" onclick="headDelete('<?php echo $_GET['type'];?>
','<?php echo $_GET['eid'];?>
','<?php echo $_GET['id'];?>
','<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'resume','c'=>'delResumeFb'),$_smarty_tpl);?>
')"><?php if ($_GET['id']) {?>删除<?php }?></div>
				<?php }?>
			</div>
        </div>
		<?php } elseif ($_GET['c']=='partapply') {?>

		<!--新增兼职头部-->
		<div class="more_position_header">
			<div class="position_header_back">
				<a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>" style="display: block">
					<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="">
				</a>
			</div>
			<div class="position_header_serch">
				<?php if ($_GET['type']=='bm'||!$_GET['type']) {?>
				<input type="search" id="partBm" placeholder="搜索兼职">
				<?php } elseif ($_GET['type']=='sc') {?>
				<input type="search" id="partSc" placeholder="搜索兼职">
				<?php }?>
			</div>
		</div>
		<!--新增兼职头部- end-->
		<?php } else { ?>
		<!--蓝色背景头部-->
		<div class="m_header">
			<div class="m_header_c">
				<div class="m_headericon">
					<a href="<?php if ($_smarty_tpl->tpl_vars['backurl']->value) {
echo $_smarty_tpl->tpl_vars['backurl']->value;
} else { ?>javascript:goBack();<?php }?>">
						<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/nav_icon_return.png" alt="" width="100%" height="100%">
					</a>
				</div>
				<div class="m_header_cont"><?php echo $_smarty_tpl->tpl_vars['headertitle']->value;?>
</div>
			</div>
		</div>
		<?php }?>
		<?php }?>
	<?php echo '<script'; ?>
>
		var weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
			wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
";
	<?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
>
		window.localStorage.removeItem("needRefresh");
		window.addEventListener('pageshow', function(){
			var storage = window.localStorage.getItem("needRefresh");
			if(storage){
				window.location.reload();
			}
		});
	<?php echo '</script'; ?>
>
<?php }} ?>
