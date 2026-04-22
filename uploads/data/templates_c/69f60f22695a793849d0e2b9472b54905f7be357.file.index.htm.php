<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:42:32
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/register/index.htm" */ ?>
<?php /*%%SmartyHeaderCode:128273667469e89808c32d16-44177278%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '69f60f22695a793849d0e2b9472b54905f7be357' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/register/index.htm',
      1 => 1700725928,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '128273667469e89808c32d16-44177278',
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
    'type' => 0,
    'bind' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89808c6cf45_46594064',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89808c6cf45_46594064')) {function content_69e89808c6cf45_46594064($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">

    <head>
        <meta http-equiv="Content-Type" content="text/html;charset=utf-8" />
        <title><?php echo $_smarty_tpl->tpl_vars['title']->value;?>
</title>
        <meta name="keywords" content="<?php echo $_smarty_tpl->tpl_vars['keywords']->value;?>
" />
        <meta name="description" content="<?php echo $_smarty_tpl->tpl_vars['description']->value;?>
" />
        <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/css.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css" />
        <link rel="stylesheet" href="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/style/login.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/css" />
        <?php echo '<script'; ?>
>
            var weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
                integral_pricename = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
                pricename = '<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
',
                code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',

                reg_namemaxlen =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_namemaxlen'];?>
',//用户名长度最大值
                reg_nameminlen =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_nameminlen'];?>
',//用户名长度最小值
                reg_name_sp    =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['reg_name_sp'];?>
',//是否必须包含其它字符@!#.$-_
                reg_name_zm    =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['reg_name_zm'];?>
',//是否必须包含字母
                reg_name_num   =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['reg_name_num'];?>
',//是否必须包含数字
                reg_name_han   =   '<?php echo $_smarty_tpl->tpl_vars['config']->value['reg_name_han'];?>
',//是否必须包含汉字

                code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
';
        <?php echo '</script'; ?>
>
    </head>

    <body class="register_bg">
        <div class="register_header">
            <div class="reg_w980">
                <div class="reg_logo">
                    <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
"><img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_logo'];?>
" class="png"></a>
                </div>
                <div class="reg_msg">欢迎注册</div>
                <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
" class="reg_fh yun_text_color fr">返回首页</a>
            </div>
        </div>
        <div class="clear"></div>
        <!--content  start-->
        <div class="reg_content">
        <div class="reg_stepbox">
        <ul class="reg_stepbox_list">
        <li class="reg_stepcur"><span class="reg_stepbox_n">1</span><em class="reg_stepbox_name">创建账户</em></li>
        <li class="reg_stepcur"><span class="reg_stepbox_line"></span></li>
        <li><span class="reg_stepbox_n">2</span><em class="reg_stepbox_name">设置身份信息</em></li>
        <li><span class="reg_stepbox_line"></span></li>
        <li><span class="reg_stepbox_n"><i class="reg_stepbox_cgicon"></i></span><em class="reg_stepbox_name">注册成功</em></li>
        </ul>
        </div>
            <div class="logoin_cont fl">
                <div class="register_left">

                    <div class="reg_newtit_box">
                        <ul class="reg_newtit">

                            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_moblie']=='1') {?>
                            <li class="<?php if ($_smarty_tpl->tpl_vars['type']->value==2) {?>reg_newtit_cur<?php }?>">
                                <a class="reg_h1_icon" href="<?php if ($_smarty_tpl->tpl_vars['bind']->value) {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>2,'bind'=>$_smarty_tpl->tpl_vars['bind']->value),$_smarty_tpl);
} else {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>2),$_smarty_tpl);
}?>"><i class="reg_newtit_icon"></i>手机注册</a>
                            </li>
                            <?php }?> 
							<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_email']=='1') {?>
                            <li class="<?php if ($_smarty_tpl->tpl_vars['type']->value==3) {?>reg_newtit_cur<?php }?>">
                                <a class="reg_h1_icon" href="<?php if ($_smarty_tpl->tpl_vars['bind']->value) {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>3,'bind'=>$_smarty_tpl->tpl_vars['bind']->value),$_smarty_tpl);
} else {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>3),$_smarty_tpl);
}?>"><i class="reg_newtit_icon"></i>邮箱注册</a>
                            </li>
                            <?php }?> 
							<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_user']=='1') {?>
                            <li class="<?php if ($_smarty_tpl->tpl_vars['type']->value==1) {?>reg_newtit_cur<?php }?>">
                                <a class="reg_h1_icon" href="<?php if ($_smarty_tpl->tpl_vars['bind']->value) {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>1,'bind'=>$_smarty_tpl->tpl_vars['bind']->value),$_smarty_tpl);
} else {
echo smarty_function_url(array('m'=>'register','usertype'=>1,'type'=>1),$_smarty_tpl);
}?>"><i class="reg_newtit_icon"></i>用户名注册</a>
                            </li>
                            <?php }?>
                        </ul>
                    </div>

                    <?php if (($_smarty_tpl->tpl_vars['type']->value==2||$_smarty_tpl->tpl_vars['type']->value=='')&&$_smarty_tpl->tpl_vars['config']->value['reg_moblie']) {?>
                    <div class="register_Switching_box" id="regtype2">
                        <ul class="register_list">
                            <li class="mt30"><em><font color="#FF0000">*</font> 手机号码：</em>
                                <input type="text" class="logoin_text tips_class" id="moblie" onblur="reg_checkAjax('moblie');" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" placeholder="请输入您的手机号码" />
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie"></span>
                            </li>

                            <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
                      
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
								<div class="gtdx-captcha">
									<div id="bind-captcha" data-id='subreg' data-type='click'></div>
									<input type='hidden' id="verify_token"  name="verify_token" value="">
									<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
                                    <input type='hidden' id="verify_str"  name="verify_str" value="">
                                    <?php }?>
									<input type='hidden' id="popup-submit">	
									<input type='hidden' id="bind-submit">
								</div>
                                <?php } else { ?>
                                     <li class="mt30"> <em><font color="#FF0000">*</font> 验证码：</em>
                                <div class="reg_textbox">
                                    <input id="CheckCode" type="text" class="logoin_textw150 tips_class" onblur="reg_checkAjax('CheckCode');" maxlength="6" placeholder="请输入图片验证码" autocomplete="off"/>
                                    <a href="javascript:void(0);" onclick="checkCode('vcode_img');" class="registe_yzm"> <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" /></a>
                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_CheckCode"></span> <?php }?>
                            </li>
                            <?php }?> 
							<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_regcode']=="1"||$_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=="1") {?>
                            <li class="mt30"><em><font color="#FF0000">*</font> 短信验证：</em>
                                <div class="reg_textbox"><input type="text" id="moblie_code" class="logoin_text tips_class" onblur="reg_checkAjax('moblie_code');" placeholder="请输入短信验证码" />
                                    <i class="reg_textbox_line"></i>
                                    <a class="reg_dxyz" href="javascript:void(0);" id="subreg" onclick="sendmsg('vcode_img');"><span id="time">获取短信验证</span></a>
                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie_code"></span>
                            </li>
                            <?php }?>

                            <li class="mt30"><em><font color="#FF0000">*</font> 密&nbsp;&nbsp;码：</em>
                                <span id="pass2"><input type="password" onkeyup="uppassword(2)" id="password" onblur="reg_checkAjax('password');" class="logoin_text tips_class" value="" placeholder="请输入6-20位（字母、数字、符号）"/></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_password"></span>
                            </li>

                            <li><em>&nbsp;</em>
                                <div class="kg_lgn_psw_strong">
                                    <div class="kg_lgn_psw_txt">密码强度:</div>
                                    <div class="kg_lgn_psw_strong_cnt">
                                        <span id="pass1_2" class="psw_span">危险</span>
                                        <span id="pass2_2" class="psw_span ">一般</span>
                                        <span id="pass3_2" class="psw_span ">安全</span>
                                    </div>
                                    <div class="psw_xs" onmousemove="showtip(2)" onmouseout="hidetip(2)">
                                        <a href="javascript:void(0);" onclick="showpw(2)" id="showpw2" class="psw_xs_a">显示密码</a>
                                        <i class="psw_xs_icon"></i>
                                        <div class="psw_xs_tips none" id="tip2">你可以点击这里检查已填写的密码</div>
                                    </div>
                                </div>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>
                            
                                <div class="mmset">
                                    密码必须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>其它字符@!#.$-_<?php }?>
                                </div>
                       
                            <?php }?>
                            </li>
                            
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
                            <li class="mt20"><em><font color="#FF0000">*</font> 确认密码：</em>
                                <span id="pass1"><input type="password" id="passconfirm" onblur="reg_checkAjax('passconfirm');" class="logoin_text tips_class" value=""placeholder="请确认您输入的密码"/></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_passconfirm"></span>
                            </li>
                            <?php }?>
							

                            <li class="mt20"><em>&nbsp;</em><input type="button" class="register_submit" value="立即注册" onclick="check_user(2,'vcode_img');" /></li>
                            <li class="m10">
                                <em>&nbsp;</em>
                                <div class="reg_xy">
                                    <input type="checkbox" id="xieyi2" class="logoin_check" value="1" checked="checked" />我已同意
                                    <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/about/service.html" target="_blank" class="reg_yhxy">《用户协议》</a>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <?php } elseif ($_smarty_tpl->tpl_vars['type']->value==3&&$_smarty_tpl->tpl_vars['config']->value['reg_email']==1) {?>
                    <div class="register_Switching_box" id="regtype3">
                        <ul class="register_list">
                            <li class="mt30 reg_re"><em><font color="#FF0000">*</font> 邮&nbsp;&nbsp;箱：</em>
                                <input type="text" class="logoin_text tips_class" id="email3" onkeyup="get_def_email(this.value,'3');" placeholder="请输入您的邮箱作为账号" />
                                <div class="reg_email_box none" id="defemail3"></div>
                                <span class="reg_tips reg_tips_red none" id="ajax_email3"></span>
                            </li>

                            <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
                         
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
								<div class="gtdx-captcha">
									<div id="bind-captcha" data-id='subreg' data-type='click'></div>
									<input type='hidden' id="verify_token"  name="verify_token" value="">
									<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
									<input type='hidden' id="verify_str"  name="verify_str" value="">
                                    <?php }?>
									<input type='hidden' id="popup-submit">	
									<input type='hidden' id="bind-submit">
								</div>
                                <?php } else { ?>
                                  <li class="mt30"> <em><font color="#FF0000">*</font> 验证码：</em>
                                <div class="reg_textbox"> <input id="CheckCode" type="text" class="logoin_textw150 tips_class" onblur="reg_checkAjax('CheckCode');" maxlength="6" placeholder="请输入图片验证码" autocomplete="off"/>
                                    <a href="javascript:void(0);" onclick="checkCode('vcodeimg');" class="registe_yzm"> <img id="vcodeimg" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" /></a>
                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_CheckCode"></span> <?php }?>
                            </li>
                            <?php }?> <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=='1') {?>
                            <li class="mt30"><em><font color="#FF0000">*</font> 手&nbsp;&nbsp;机：</em>
                                <input type="text" class="logoin_text tips_class" id="moblie" onblur="reg_checkAjax('moblie');" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" placeholder="请输入您的手机号码" />
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie"></span>
                            </li>
                            
                            <li class="mt30"><em><font color="#FF0000">*</font> 短信验证：</em>
                                <div class="reg_textbox"><input type="text" id="moblie_code" class="logoin_text tips_class" onblur="reg_checkAjax('moblie_code');" placeholder="请输入短信验证" /><i class="reg_textbox_line"></i>
                                    <a class="reg_dxyz" href="javascript:void(0);" id="subreg" onclick="sendmsg('vcodeimg');"><span id="time">获取短信验证</span></a>
                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie_code"></span>
                            </li>
                            <?php }?>

                            <li class="mt30"><em><font color="#FF0000">*</font> 密&nbsp;&nbsp;码：</em>
                                <span id="pass3"><input type="password" onkeyup="uppassword(3)" id="password" onblur="reg_checkAjax('password');" class="logoin_text tips_class" value=""placeholder="请输入6-20位（字母、数字、符号）"/></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_password"></span>
                            </li>

                            <li><em>&nbsp;</em>
                                <div class="kg_lgn_psw_strong">
                                    <div class="kg_lgn_psw_txt">密码强度:</div>
                                    <div class="kg_lgn_psw_strong_cnt">
                                        <span id="pass1_3" class="psw_span">危险</span>
                                        <span id="pass2_3" class="psw_span ">一般</span>
                                        <span id="pass3_3" class="psw_span ">安全</span>
                                    </div>
                                    <div class="psw_xs" onmousemove="showtip(3)" onmouseout="hidetip(3)">
                                        <a href="javascript:void(0);" onclick="showpw(3)" id="showpw3" class="psw_xs_a">显示密码</a>
                                        <i class="psw_xs_icon"></i>
                                        <div class="psw_xs_tips none" id="tip3">你可以点击这里检查已填写的密码</div>
                                    </div>
                                </div>
								 <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>
                           
                                <div class="mmset">
                                    密码必须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>其它字符@!#.$-_<?php }?>
                                </div>
                                <?php }?>
							
                            </li>
                           
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
                            <li class="mt20"><em><font color="#FF0000">*</font> 确认密码：</em>
                                <span id="pass1"><input type="password" id="passconfirm" onblur="reg_checkAjax('passconfirm');" class="logoin_text tips_class" value=""placeholder="请确认您输入的密码"></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_passconfirm"></span>
                            </li>
                            <?php }?> 

                            <li class="mt30"><em>&nbsp;</em>
							<input type="button" class="register_submit" <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=='0') {?> id='subreg' <?php }?> value="立即注册" onclick="check_user(3,'vcodeimg');" /></li>
                            <li class="m10">
                                <em>&nbsp;</em>
                                <div class="reg_xy">
                                    <input type="checkbox" id="xieyi3" class="logoin_check" value="1" checked="checked" />我已同意
                                    <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/about/service.html" target="_blank" class="reg_yhxy">《用户协议》</a>
                                </div>
                            </li>
                        </ul>
                    </div>

                    <?php } elseif ($_smarty_tpl->tpl_vars['type']->value==1&&$_smarty_tpl->tpl_vars['config']->value['reg_user']==1) {?>
                    <div class="register_Switching_box" id="regtype1">
                        <ul class="register_list">

                            <li class="mt30"><em><font color="#FF0000">*</font> 用户名：</em>
                                <input type="text" id="username1" class="logoin_text tips_class" onblur="reg_checkAjax('username1');" placeholder="请输入用户名作为账号" />
                                <span class="reg_tips reg_tips_red none" id="ajax_username1"></span>
                            <div class="mmset mmsetpd">用户名要求长度<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_nameminlen'];?>
-<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_namemaxlen'];?>
位
                                    <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>
                                        ,必须包含
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1') {?>汉字 <?php }?>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_num']=='1') {?>数字 <?php }?>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>字母 <?php }?>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1') {?>其它字符@!#.$-_<?php }?>
                                    <?php }?>
                                    </div>
									</li>
                           
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=="1") {?>
                            <li class="mt30"><em><font color="#FF0000">*</font> 手&nbsp;&nbsp;机：</em>
                                <input type="text" class="logoin_text tips_class" id="moblie" onblur="reg_checkAjax('moblie');" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" placeholder="请输入您的手机号码" />
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie"></span>
                            </li>
                            <?php }?> 
							<?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
                         
                                <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
									<div class="gtdx-captcha">
										<div id="bind-captcha" data-id='subreg' data-type='click'></div>
										
										<input type='hidden' id="verify_token"  name="verify_token" value="">
										<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
                                        <input type='hidden' id="verify_str"  name="verify_str" value="">
                                        <?php }?>
										<input type='hidden' id="popup-submit">	
										<input type='hidden' id="bind-submit">
									</div>
                                <?php } else { ?>
                                   <li class="mt30"><em><font color="#FF0000">*</font> 验证码：</em>
                                <div class="reg_textbox">
                                    <input id="CheckCode" type="text" class="logoin_textw150 tips_class" onblur="reg_checkAjax('CheckCode');" maxlength="6" placeholder="请输入图片验证码" autocomplete="off"/>
                                    <a href="javascript:void(0);" onclick="checkCode('vcode_imgs');" class="registe_yzm"><img id="vcode_imgs" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" /></a>

                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_CheckCode"></span> <?php }?>
                            </li>
                            <?php }?> <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=='1') {?>
                            <li class="mt30">
                                <em><font color="#FF0000">*</font> 短信验证：</em>
                                <div class="reg_textbox">
                                    <input type="text" id="moblie_code" class="logoin_text tips_class" onblur="reg_checkAjax('moblie_code');" placeholder="请输入短信验证码" /><i class="reg_textbox_line"></i>
                                    <a class="reg_dxyz" href="javascript:void(0);" id="subreg" onclick="sendmsg('vcode_imgs');"><span id="time">获取短信验证</span></a>
                                </div>
                                <span class="reg_tips reg_tips_red none" id="ajax_moblie_code"></span>
                            </li>
                            <?php }?>

                            <li class="mt30"><em><font color="#FF0000">*</font> 密&nbsp;&nbsp;码：</em>
                                <span id="pass1"><input type="password" onkeyup="uppassword(1)" id="password" onblur="reg_checkAjax('password')" class="logoin_text tips_class" value=""placeholder="请输入6-20位（字母、数字、符号）"/></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_password"></span>
                            </li>

                            <li class="reg_re"><em>&nbsp;</em>
                                <div class="kg_lgn_psw_strong">
                                    <div class="kg_lgn_psw_txt">密码强度:</div>
                                    <div class="kg_lgn_psw_strong_cnt">
                                        <span id="pass1_1" class="psw_span">危险</span>
                                        <span id="pass2_1" class="psw_span ">一般</span>
                                        <span id="pass3_1" class="psw_span ">安全</span>
                                    </div>
                                    <div class="psw_xs" onmousemove="showtip(1)" onmouseout="hidetip(1)">
                                        <a href="javascript:void(0);" onclick="showpw(1);" id="showpw1" class="psw_xs_a">显示密码</a>
                                        <i class="psw_xs_icon"></i>
                                        <div class="psw_xs_tips none" id="tip1">你可以点击这里检查已填写的密码</div>
                                    </div>
                                </div>
								 <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>
                           
                                <div class="mmset">
                                    密码必须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>其它字符@!#.$-_<?php }?>
                                </div>
                         
                            <?php }?>
                            </li>
                           
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
                            <li class="mt20"><em><font color="#FF0000">*</font> 确认密码：</em>
                                <span id="pass1"><input type="password" id="passconfirm" onblur="reg_checkAjax('passconfirm');" class="logoin_text tips_class" value=""placeholder="请确认您输入的密码"//></span>
                                <span class="reg_tips reg_tips_red none" id="ajax_passconfirm"></span>
                            </li>
                            <?php }?> 

                            <li class="mt30"><em>&nbsp;</em><input type="button" class="register_submit" <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']=='0') {?> id="subreg" <?php }?> value="立即注册" onclick="check_user('1','vcode_imgs');" /></li>
                            <li class="m10">
                                <em>&nbsp;</em>
                                <div class="reg_xy">
                                    <input type="checkbox" id="xieyi1" class="logoin_check" value="1" checked="checked" />我已同意
                                    <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/about/service.html" target="_blank" class="reg_yhxy">《用户协议》</a>
                                </div>
                            </li>
                        </ul>
                    </div>
                    <?php }?>

                    <input type="hidden" id="default" value="-1" />
                    <input type="hidden" id="def" value="" />
                    <input type="hidden" id="event" value="" />
                    <input type="hidden" id="allnum" value="0" />
                    <input type="hidden" id="send" value="0" />
                    <input type="hidden" id="usertype" value="1" />
                    <input type="hidden" id="defEmail" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_def_email'];?>
" />
					<input type="hidden" id="reg_bind" value="<?php echo $_smarty_tpl->tpl_vars['bind']->value;?>
"/>
                </div>
                <div class="register_right">
                   <div class="register_right_icon"></div>
                    <div class="register_right_c">

                        已有账号? <a href="<?php echo smarty_function_url(array('m'=>'login'),$_smarty_tpl);?>
">直接登录></a>
                       
                    </div>
                    <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']==1||$_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']==1||$_smarty_tpl->tpl_vars['config']->value['wx_author']==1) {?>
                    <div class="register_right_c">其他方式登录 </div>
                    <div class="box_third">

                        <?php if ($_smarty_tpl->tpl_vars['config']->value['wx_author']==1) {?>
                        <a href="<?php echo smarty_function_url(array('m'=>'wxconnect'),$_smarty_tpl);?>
"><i class="l-icon weixin-icon"></i></a>
                        <?php }?> <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']==1) {?>
                        <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/qqlogin.php"><i class="l-icon qq-icon"></i></a> <?php }?> <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']==1&&$_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']==1) {
}?> <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']==1) {?>
                        <a href="<?php echo smarty_function_url(array('m'=>'sinaconnect'),$_smarty_tpl);?>
"><i class="l-icon sina-icon"></i></a> <?php }?>
                    </div>

                    <?php }?> </div>
            </div>
        </div>
        <div class="clear"></div>
        
        
        <!--手机号被用弹出框-->
        <div class="" style="width:550px; background:#fff; margin:20px auto; display: none;" id="written_off">
            <div class="reg_have_tip">
                <i class="reg_have_tip_icon"></i>
                <div class="reg_have_tip_tit"><span id="zy_type">该手机号已被注册</span></div>
                <div class="reg_have_tip_zc" id="zy_name"></div>
            </div>

            <div class="reg_have_tip_p_c">
                <div class="reg_have_tip_p">1. 如果是你本人，但不记得密码，您可以找回密码
                    <a href="<?php echo smarty_function_url(array('m'=>'forgetpw'),$_smarty_tpl);?>
" class="reg_have_tip_bth">找回密码</a>
                </div>
                <div class="reg_have_tip_p">
					<span id="desc_toast">2. 解除手机号与该账号的绑定，解除绑定后，您无法继续用手机号登录该账号</span>
                    <a href="javascript:void(0);" onclick="CheckPW()" class="reg_have_tip_bth">解除绑定</a>
                </div>
            </div>
            <div class="reg_have_tip_kf">如有疑问可拨打客服服务热线：<span class="reg_have_tip_kf_tel"><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span></div>
			<input type="hidden" id="zy_uid" value="" />
			<input type="hidden" id="zy_mobile" value="" />
			<input type="hidden" id="zy_email" value="" />
        </div>
        <!--手机号被用弹出框 end-->
        
        <!--验证密码弹出框-->
        <div class="none" id="postpw">
          	<ul class="reg_have_jc">
              	<li> 
                 	<span class="reg_have_jc_name"><font color="#FF3300">*</font> 登录密码：</span>
              		<input id="pw" type='password' class="reg_have_jc_name_text" placeholder="请输入登录密码" />
            	</li>
            	 
             	<li> 
                 	<span class="reg_have_jc_name"><font color="#FF3300">*</font> 验&nbsp;证&nbsp;码：</span>
                   	<input type="text" id="code" class="reg_have_jc_name_textyz"  maxlength="6" autocomplete="off"/>
                  	<a><img id="vcode_imgss" onclick="checkCode('vcode_imgss');" src="" /></a>
            	</li>
            	 
           		<li> 
                  	<span class="reg_have_jc_name">&nbsp; </span>
                  	<input class="reg_have_jc_bth" type="button" value="确认解绑" onclick="post_pass('vcode_imgss');" />
             	</li>
             	
             	
          	</ul>
        </div>
        <!--验证密码弹出框End-->
        
        <style>
            .icon2 {
                background-image: none;
                background-repeat: no-repeat;
            }
        </style>
        
        <!--content  end-->
        <div class="clear"></div>
        <div class="reg_footer">如果您有任何疑问或在注册过程中遇到任何困难，您都可以通过以下方式与我们联系
           <div class="">电话： <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
 <span class="reg_footer_yz">客服QQ：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_qq'];?>
</span> </div>
                     </div>
        <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/jquery/jquery_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui/css/layui.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css" />
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
/js/public.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" language="javascript"><?php echo '</script'; ?>
>
        <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['style']->value;?>
/js/reg_ajax.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type="text/javascript"><?php echo '</script'; ?>
>
        
		<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/verify/verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

        <!--[if IE 6]>
		<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/png.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
		<?php echo '<script'; ?>
>
		  DD_belatedPNG.fix('.png,.#bg');
		<?php echo '</script'; ?>
>
		<![endif]-->
        <?php echo '<script'; ?>
>
            <?php if ($_smarty_tpl->tpl_vars['type']->value==1) {?>
               	$(document).ready(function() {
                 	$(document).keydown(function(e) {
                        var ev = document.all ? window.event : e;
                        if(ev.keyCode == 13) {
                         	check_user(1, 'vcode_imgs');
                   		}
                 	});
                	$("#xieyi1").keyup(function(event) {
                     	var myEvent = event || window.event;
                     	if(myEvent.keyCode == 13) {
                        	check_user(1, 'vcode_imgs');
                     	} else {
                        	 return;
                     	};
                	});
             	}) 
      		<?php } elseif ($_smarty_tpl->tpl_vars['type']->value==2) {?>
            	$(document).ready(function() {
                 	$(document).keydown(function(e) {
                      	var ev = document.all ? window.event : e;
                     	if(ev.keyCode == 13) {
                         	check_user(2, 'vcode_img');
                     	}
                 	});
            		$("#xieyi2").keyup(function(event) {
                    	var myEvent = event || window.event;
                     	if(myEvent.keyCode == 13) {
                        	check_user(2, 'vcode_img');
                  		} else {
                         	return;
                    	};
               		});
              	}) 
         	<?php } elseif ($_smarty_tpl->tpl_vars['type']->value==3) {?>
             	$(document).ready(function() {
              		$(document).keydown(function(e) {
                    	var ev = document.all ? window.event : e;
                    	if(ev.keyCode == 13) {
                       		check_user(3, 'vcodeimg');
                     	}
                 	});
                	$("#xieyi3").keyup(function(event) {
                      	var myEvent = event || window.event;
                     	if(myEvent.keyCode == 13) {
                        	check_user(3, 'vcodeimg');
                       	} else {
                          	return;
                     	};
                  	});
             	}) 
            <?php }?>
        <?php echo '</script'; ?>
>
        
        <div style="display:none"><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
</div><?php }} ?>
