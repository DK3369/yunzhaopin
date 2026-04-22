<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:34:47
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/login.htm" */ ?>
<?php /*%%SmartyHeaderCode:117777315769e896373e1740-46369120%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'c408256877cba5275a226583702f442d0bbf6cb2' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/login.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '117777315769e896373e1740-46369120',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'wxid' => 0,
    'wxnickname' => 0,
    'wxpic' => 0,
    'backurl' => 0,
    'config' => 0,
    'config_wapdomain' => 0,
    'checkurl' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89637405302_35143557',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89637405302_35143557')) {function content_69e89637405302_35143557($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<style>
    body {
        background: #fff
    }
</style>
<!-- 页面头部返回按钮 -->
<div class="Back_to_the_previous_level">
    <div class="login_back" onclick="goBack()">
        <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/return.png" alt="" width="100%" height="100%">
    </div>
</div>
<div class="login_cont">
	<div class="bottom_nav_bom" style="padding-top: 0px; text-align: right;"><i class="bottom_nav_bom_word">还没有账号，</i> <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register'),$_smarty_tpl);?>
" class="register_1">马上注册</a></div>
	<div class="login_welcome">
		<div>您好，</div>
		<div>欢迎登录</div>
	</div>
    <?php if ($_smarty_tpl->tpl_vars['wxid']->value&&($_smarty_tpl->tpl_vars['wxnickname']->value||$_smarty_tpl->tpl_vars['wxpic']->value)) {?>
        <div class="login_titlebox">
            <div class="login_title_logo">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wxpic']->value;?>
" alt="" width="100%" height="100%">
            </div>
            <div class="login_title_info">
                您已登录微信账号，<i>登录绑定已有账户</i>
            </div>
        </div>
    <?php }?>

    <!-- form表单区域 -->
    <form id="login_form">
        <input id="qfyuid" name="qfyuid" type="hidden" value=""/>
        <input type="hidden" name="backurl" id="backurl" value="<?php echo $_smarty_tpl->tpl_vars['backurl']->value;?>
"/>
        <input type="hidden" name="act_login" id="act_login" value="0"/>
        <input name="usertype" type="hidden" value="<?php echo $_GET['usertype'];?>
"/>
        <input name="wxid" type="hidden" value="<?php echo $_GET['wxid'];?>
"/>
        <?php if ($_GET['job']) {?>
        <input name="job" type="hidden" value="<?php echo $_GET['job'];?>
"/>
        <?php }?>

        <div class="The_login_subject">
            <div id="login_normal_box">
                <div class="login_textbox">
                    <input type="text" name="username" placeholder="请输入用户名/邮箱/手机号" class="account_number">
                </div>
                <div class="login_textbox">
                    <input type="password" id="password" value="" name="password" placeholder="请输入密码"/>
                    <input type="text" id="login_password" style="display: none"/>
                    <div class="close_open"><img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/conceal.png" alt="" width="100%" height="100%" id="close"></div>
                </div>
            </div>
            <!--手机动态码登录-->
            <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_login']==1) {?>
            <div id="login_sj_box" class="login_textbox" style="display:none;">
                <input type="tel" value="" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" placeholder="请输入手机号" name="moblie" id="usermoblie" class="account_number">
            </div>
            <?php }?>

            <?php if (strstr($_smarty_tpl->tpl_vars['config']->value['code_web'],'前台登录')) {?>
                <div>
                    <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
                    <div class="gtdx-captcha">
                        <div id="bind-captcha" data-id='login_bth' data-type='click'></div>
                        <input type='hidden' id="verify_token" name="verify_token" value="" />
						<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
                        <input type='hidden' id="verify_str" name="verify_str" value="" />
                        <?php }?>
                        <input type='hidden' id="popup-submit" />
                        <input type='hidden' id="bind-submit" />
                    </div>
                    <?php } else { ?>
                    <div class="login_textbox">
                        <input class="inputitemtxt" placeholder="图片验证码" name="authcode" id="checkcode" type="text"
                               maxlength="6"/>
                        <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php"
                             onclick="checkCode('vcode_img');"/>
                    </div>

                    <?php }?>
                </div>
            <?php }?>

            <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_login']==1) {?>
                <div class="passwords" id="login_sjyz_box" style="display:none;">
                    <div class="login_textbox">
                        <input name="dynamiccode" type="text" maxlength='6' class="inputitemtxt" value="" placeholder="输入短信验证码" onblur="changeCaptcha()"/>
                        <input type="password" id="login_code" style="display: none"/>
                        <div class="dx_yz_hq" id="send_msg_tip" onclick="send_msg('<?php echo $_smarty_tpl->tpl_vars['config_wapdomain']->value;?>
/index.php?c=login&a=sendmsg');"> 获取验证码</div>
                    </div>
                </div>
            <?php }?>
            <div class="login_xy">
                <div class="login_xy_zx">
                    <input type="checkbox" id="xieyicheck" name="xieyicheck" value="1" checked onclick="choosexie(this)"/>
                </div>
                <div>
                    <i class="policy">我已同意</i>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'protocol'),$_smarty_tpl);?>
" class="Privacy">《用户协议》</a>
                    <i class="policy">和</i>
                    <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'privacy'),$_smarty_tpl);?>
" class="Privacy">《隐私政策》</a>
                </div>
            </div>
            <div id="login_bth" class="login_bth" onclick="login()">登录</div>
        </div>

        <?php ob_start();
echo smarty_function_url(array('m'=>'wap','c'=>'forgetpw'),$_smarty_tpl);
$_tmp1=ob_get_clean();?><?php if ($_smarty_tpl->tpl_vars['checkurl']->value!=$_tmp1) {?>
        <input type="hidden" name="checkurl" value="<?php echo $_smarty_tpl->tpl_vars['checkurl']->value;?>
"/>
        <?php }?>
    </form>
    <!-- 验证码 -->
    <div class="login_otherfs">
        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_isopen']==1&&$_smarty_tpl->tpl_vars['config']->value['sy_msg_login']==1) {?>
        <div class="verification_code_word" id="mobile_login">短信登录</div>
        <?php }?>
        <div class="verification_code_word" id="acount_login" style="display: none"> 用户名登录</div>
        <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'forgetpw'),$_smarty_tpl);?>
" class="login_wjmm">忘记密码</a>
    </div>
</div>
<!-- 另外的登录方式 -->
<?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']==1||$_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']==1||($_smarty_tpl->tpl_vars['config']->value['wx_rz']==1&&!$_smarty_tpl->tpl_vars['wxid']->value)) {?>
<div class="bottom_nav">
    <div class="bottom_nav_top">或通过以下方式登录</div>
    <div class="bottom_nav_center">
        <?php if ($_smarty_tpl->tpl_vars['config']->value['wx_rz']==1&&!$_smarty_tpl->tpl_vars['wxid']->value) {?>
        <a class="bottom_nav_center_logo" href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'wxconnect'),$_smarty_tpl);?>
">
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/weixin.png" alt="" width="100%" height="100%">
        </a>
        <?php }?>
        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_qqlogin']==1) {?>
            <a class="bottom_nav_center_logo" href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/qqlogin.php">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/QQ@2x.png" alt="" width="100%" height="100%">
            </a>
        <?php }?>
        <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_sinalogin']==1) {?>
            <a class="bottom_nav_center_logo" href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'sinaconnect'),$_smarty_tpl);?>
">
                <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/weibu.png" alt="" width="100%" height="100%">
            </a>
        <?php }?>
    </div>

</div>
<?php }?>

<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/reg_ajax.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
>
    var wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
",
        wapRegUrl = "<?php echo smarty_function_url(array('m'=>'wap','c'=>'register'),$_smarty_tpl);?>
",
        weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
        code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
        code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
',
        sy_login_type = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_login_type'];?>
';

    $(document).ready(function () {
        $('#close').on('click', function () {
            var $inp = $('#password');
            $inp.attr('type') === 'password' ? $inp.attr('type', 'text') : $inp.attr('type', 'password')
            $(this).attr('src') == '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/conceal.png' ? $(this).attr('src', '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/conceal_1.png') : $(this).attr('src', '<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/conceal.png')
        });
        $("#username").focus(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue) {
                $(this).attr("placeholder", "");
            }
        }).blur(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue || $(this).val() == "") {
                $(this).attr("placeholder", "请输入用户名/邮箱/手机号");
            }
        })
        $("#password").focus(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue) {
                $(this).attr("placeholder", "");
            }
        }).blur(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue || $(this).val() == "") {
                $(this).attr("placeholder", "请输入您的密码");
            }
        })
        $("#usermoblie").focus(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue) {
                $(this).attr("placeholder", "");
            }
        }).blur(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue || $(this).val() == "") {
                $(this).attr("placeholder", "请输入手机号");
            }
        })
        $(".inputitemtxt").focus(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue) {
                $(this).attr("placeholder", "");
            }
        }).blur(function () {
            var txAreaVal = $(this).val();
            if (txAreaVal == this.defaultValue || $(this).val() == "") {
                $(this).attr("placeholder", "请输入短信验证码");
            }
        })
        //账号登录和手机登录tab选择
        $('#acount_login').click(function (data) {
            $('#acount_login').css("display", "none");
            $('#mobile_login').css("display", "block");
            $('#login_normal_box').css("display", "block");
            $('#login_sj_box').css("display", "none");
            $('#login_sjyz_box').css("display", "none");
            $('#act_login').val('0');
            $('#bind-captcha').attr('data-id', 'login_bth');
            $('#bind-captcha').attr('data-type', 'click');
        });
        $('#mobile_login').click(function (data) {
            $('#mobile_login').css("display", "none");
            $('#acount_login').css("display", "block");
            $('#login_sj_box').css("display", "block");
            $('#login_sjyz_box').css("display", "block");
            $('#login_normal_box').css("display", "none");
            $('#act_login').val('1');
            $('#bind-captcha').attr('data-id', 'send_msg_tip');
            $('#bind-captcha').attr('data-type', 'click');
        });

        if (sy_login_type == '2' && $('#mobile_login')) {
            $('#mobile_login').trigger("click");
        }
    });
    var Timer;
    var smsTimer_time = 90; //倒数 90
    var smsTimer_flag = 90; //倒数 90
    var smsTime_speed = 1000; //速度 1秒钟

    //发送手机短信
    function send_msg(url) {
        var moblie = $('#usermoblie').val();
        var code;

        var verify_token,verify_str;
        if (moblie == "" || moblie == "请输入手机号码") {

            showToast("请输入手机号码！");
            return false;
        } else {

            if (!isjsMobile(moblie)) {
                showToast('手机格式错误！');
                return false;
            }
        }
        var codesear = new RegExp('前台登录');
        if (codesear.test(code_web)) {
            if (code_kind == 1) {
                code = $.trim($("#checkcode").val());
                if (!code) {
                    showToast('请填写图片验证码！');
                    return false;
                }
            } else if (code_kind > 2) {
                // 验证类型改成短信
                $('#bind-captcha').attr('data-id', 'send_msg_tip');
                $('#bind-captcha').attr('data-type', 'click');
                verify_token = $('input[name="verify_token"]').val();
                if (verify_token == '') {
                    if (code_kind == 6) {
                        $("#bind-captcha").trigger("click");
                    } else {
                        $("#bind-submit").trigger("click");
                    }
                    return false;
                }
				verify_str = $('input[name="verify_str"]').val();
            }
        }
        if (smsTimer_time == smsTimer_flag) {
            showLoading();
            $.post(url, {
                moblie: moblie,
                authcode: code,
                verify_token: verify_token,
                verify_str: verify_str
            }, function (data) {
                hideLoading();
                if (data) {
                    var res = JSON.parse(data);
                    if (res.error == 1) {
                        Timer = setInterval("smsTimer($('#send_msg_tip'))", smsTime_speed);
                    }
                    showToast(res.msg, 2, function () {
                        if (res.error != 1) {
                            if (code_kind == 1) {
                                checkCode('vcode_img');

                            } else if (code_kind > 2) {
                                $("#popup-submit").trigger("click");
                            }
                            if (res.url){
                                location.href = res.url;
                            }
                        }
                        if (res.msg == '请先注册账号') {
                            location.href = wapRegUrl;
                        }
                    });
                }
            })
        } else {
            showToast('请勿重复发送！', 2);
            return false;
        }
    }

    //倒计时
    function smsTimer(obj) {
        if (smsTimer_flag > 0) {
            $(obj).html('重新发送(' + smsTimer_flag + 's)');
            $(obj).attr({
                'style': 'background: ;'
            });
            smsTimer_flag--;
        } else {
            $(obj).html('重新发送');
            $(obj).attr({
                'style': 'background: ;'
            });
            smsTimer_flag = smsTimer_time;
            clearInterval(Timer);
        }
    }

    function changeCaptcha() {
        // 验证类型改成提交
        $('#bind-captcha').attr('data-id', 'login_bth');
        $('#bind-captcha').attr('data-type', 'click');
    }
<?php echo '</script'; ?>
>
</body>
</html>
<?php }} ?>
