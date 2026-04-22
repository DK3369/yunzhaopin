<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:35:03
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/register.htm" */ ?>
<?php /*%%SmartyHeaderCode:116493070169e89647753d34-22455308%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '409f2c97fedacf82f4f0107209595193d8a7bd39' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/register.htm',
      1 => 1700725936,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '116493070169e89647753d34-22455308',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
    'wxid' => 0,
    'wxnickname' => 0,
    'wxpic' => 0,
    'config' => 0,
    'type' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8964778c846_07250002',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8964778c846_07250002')) {function content_69e8964778c846_07250002($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<style>
body{ background:#fff}
</style>
<div id="yunvue" class="none">        
    <!-- 页面头部返回按钮 -->
    <div class="Back_to_the_previous_level"  >
        <div class="login_back" onclick="goBack()">
            <img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/return.png" alt=""  width="100%" height="100%">
        </div>
      </div>
      <!-- 页面主体部分 -->
   <div class="login_cont">
		<?php if ($_smarty_tpl->tpl_vars['wxid']->value&&($_smarty_tpl->tpl_vars['wxnickname']->value||$_smarty_tpl->tpl_vars['wxpic']->value)) {?>
		<div class="login_titlebox">
			<div class="login_title_logo">
				<img src="<?php echo $_smarty_tpl->tpl_vars['wxpic']->value;?>
" alt="" width="100%" height="100%">
			</div>
			<div class="login_title_info">您已登录微信账号</div>
		</div>
		<?php } else { ?>
		<!-- 用户名注册 -->
		<div class=" bottom_nav_bom"  style="padding-top: 0px; text-align: right;"><i class="bottom_nav_bom_word">已有账号，</i>
		     <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'login'),$_smarty_tpl);?>
" class="register_1">立即登录</a>
		</div>
         <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_moblie']=='1'&&$_smarty_tpl->tpl_vars['type']->value==2) {?>
          <div class="login_welcome">手机号注册</div>
          <?php }?> 
          <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_email']=='1'&&$_smarty_tpl->tpl_vars['type']->value==3) {?>
          <div class="login_welcome">邮箱注册</div>
          <?php }?> 
          <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_user']=='1'&&$_smarty_tpl->tpl_vars['type']->value==1) {?>
          <div class="login_welcome">用户名注册</div>
          <?php }?>
		<?php }?>

           <!-- form表单区域 -->
           <div class="The_login_subject">
        <form id="reg_form">
          <input name="regway" id="regway" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['type']->value;?>
" />
          <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_email']=='1'&&$_smarty_tpl->tpl_vars['type']->value==3) {?>
             <div class="login_textbox">
            <input type="text" value="" placeholder="请输入邮箱" name="email" id="email" onBlur="check_email();" class="account_number">
           </div>
           <div class="login_textbox">
            <input type="password" value="" placeholder="请输入密码 " id="password" name="password" onblur="check_password();" />


           </div><?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>
			<div class="zc_tip">
			提示：密码须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字<?php }?>
			<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>,字母 <?php }?>
			<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>,字符@!#.$-_<?php }?>
			</div>
			<?php }?>
            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
            <div class="login_textbox">
            <input type="password" value="" placeholder="请确认密码" name="passconfirm" id="passconfirm" />
            
           </div>
           <?php }?>
           <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
           <div>
              <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
              <input type='hidden' id="noblur" value="1" />
              <div class="gtdx-captcha">
              <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_regcode']==1||$_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']==1) {?>
              <div id="bind-captcha" data-id='send_msg_tips' data-type='click'></div>
              <?php } else { ?>
              <div id="bind-captcha" data-id='login_bth' data-type='click'></div>
              <?php }?>
              
              
              <input type='hidden' id="verify_token"  name="verify_token" value="">
			  <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
              <input type='hidden' id="verify_str" name="verify_str" value="" />
              <?php }?>
              <input type='hidden' id="popup-submit"> 
              <input type='hidden' id="bind-submit">
            </div>
            <?php } else { ?>
           <div class="login_textbox">
            <input class="inputitemtxt" placeholder="请输入图片验证码" onblur="check_code()" name="checkcode" id="checkcode" type="text" maxlength="6" />
            <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" onclick="checkCode('vcode_img');" />
           </div>
              <?php }?>
            </div>
           <?php }?>
           <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']==1) {?>
           <div class="login_textbox">
            <input type="tel" value="" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"  placeholder="请输入手机号" name="moblie" id="moblie" onBlur="check_moblie();" class="account_number">
           </div>
           <div class="login_textbox">
            <input type="text" value="" autocomplete="off"  placeholder="请输入验证码" name="moblie_code" id="moblie_code" />
            <input type="password" id="login_code" style="display: none"/>
            <div class="dx_yz_hq" id="send_msg_tips" onclick="sendmsg('vcode_img');"><span id="time">获取验证码</span></div>
           </div>
           <?php }?> 
          <?php } elseif ($_smarty_tpl->tpl_vars['type']->value==2||$_smarty_tpl->tpl_vars['type']->value=='') {?>
            <div class="login_textbox">
            <input type="tel" value="" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"  placeholder="请输入手机号" name="moblie" id="moblie" onBlur="check_moblie();" class="account_number">
           </div>
           <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
           <div class="">
              <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
              <input type='hidden' id="noblur" value="1" />
              <div class="gtdx-captcha">
              <?php if ($_smarty_tpl->tpl_vars['config']->value['sy_msg_regcode']==1||$_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']==1) {?>
              <div id="bind-captcha" data-id='send_msg_tips' data-type='click'></div>
              <?php } else { ?>
              <div id="bind-captcha" data-id='login_bth' data-type='click'></div>
              <?php }?>
              
              
              <input type='hidden' id="verify_token"  name="verify_token" value="">
			  <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
              <input type='hidden' id="verify_str" name="verify_str" value="" />
              <?php }?>
              <input type='hidden' id="popup-submit"> 
              <input type='hidden' id="bind-submit">
            </div>
            <?php } else { ?>
           <div class="login_textbox">
            <input class="inputitemtxt" placeholder="请输入图片验证码" onblur="check_code()" name="checkcode" id="checkcode" type="text" maxlength="6" />
            <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" onclick="checkCode('vcode_img');" />
           </div>
              <?php }?>
            </div>
           <?php }?>
           <div class="login_textbox">
            <input type="text" value="" autocomplete="off"  placeholder="请输入验证码" name="moblie_code" id="moblie_code" />
            <input type="password" id="login_code" style="display: none"/>
            <div class="dx_yz_hq" id="send_msg_tips" onclick="sendmsg('vcode_img');"><span id="time">获取验证码</span></div>
           </div>
            <div class="login_textbox">
            <input type="password" value="" placeholder="请输入密码 " id="password" name="password" onblur="check_password();" />

           </div>
		   <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>
		   <div class="zc_tip">提示：密码须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字<?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>,字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>,字符@!#.$-_<?php }?></div><?php }?>
            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
            <div class="login_textbox">
            <input type="password" value="" placeholder="请确认密码" name="passconfirm" id="passconfirm" />
            
           </div>
           <?php }?>
          <?php } elseif ($_smarty_tpl->tpl_vars['type']->value==1) {?>
           <div class="login_textbox">
            <input type="text" placeholder="请输入用户名 <?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_nameminlen'];?>
-<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_namemaxlen'];?>
位" name="username" id="username" class="account_number" onblur="check_username();">
           </div>
		   <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>

			<div class="zc_tip">提示：用户名须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1') {?>汉字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_name_num']=='1') {?>数字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>,字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1') {?>,字符@!#.$-_<?php }?></div><?php }?>



           <div class="login_textbox">
            <input type="password" value="" placeholder="请输入密码 " id="password" name="password" onblur="check_password();" />

           </div>
		   <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?><div class="zc_tip">提示：密码须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字<?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>,字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>,字符@!#.$-_<?php }?></div><?php }?>
            <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_passconfirm']=='1') {?>
            <div class="login_textbox">
            <input type="password" value="" placeholder="请确认密码" name="passconfirm" id="passconfirm" />
            
           </div>
           <?php }?>
           <?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"注册会员")!==false) {?>
            <div>
              <?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
              <div class="gtdx-captcha">
                  <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']==1) {?>
                <div id="bind-captcha" data-id='send_msg_tips' data-type='click'></div>
                <?php } else { ?>
                <div id="bind-captcha" data-id='login_bth' data-type='click'></div>
                <?php }?>
                
                <input type='hidden' id="verify_token"  name="verify_token" value="">
				<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
				<input type='hidden' id="verify_str" name="verify_str" value="" />
				<?php }?>
                <input type='hidden' id="popup-submit"> 
                <input type='hidden' id="bind-submit">
              </div>
              <?php } else { ?>
              <div class="login_textbox">
                <input class="inputitemtxt" placeholder="请输入图片验证码" onblur="check_code()" name="checkcode" id="checkcode" type="text" maxlength="6" />
                <img id="vcode_img" class="authcode" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_wapdomain'];?>
/authcode.inc.php" onclick="checkCode('vcode_img');" />
              </div>
              <?php }?>
            </div>
           <?php }?>
           <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_real_name_check']==1) {?>
           <div class="login_textbox">
            <input type="tel" value="" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')"  placeholder="请输入手机号" name="moblie" id="moblie" onBlur="check_moblie();" class="account_number">
           </div>
           <div class="login_textbox">
            <input type="text" value="" autocomplete="off"  placeholder="请输入验证码" name="moblie_code" id="moblie_code" />
            <input type="password" id="login_code" style="display: none"/>
            <div class="dx_yz_hq" id="send_msg_tips" onclick="sendmsg('vcode_img');"><span id="time">获取验证码</span></div>
           </div>
           <?php }?> 
           <?php }?>
           <div class="login_xy">
    <div class="login_xy_zx">
                   <input type="checkbox" id="xieyicheck" value="1" checked onclick="choosexie(this)" />
               </div>
               
               <i class="policy">我已同意</i>
               <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'protocol'),$_smarty_tpl);?>
" class="Privacy" class="Privacy">《用户协议》</a>
               <i class="policy">和</i>
               <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'privacy'),$_smarty_tpl);?>
" class="Privacy">《隐私政策》</a>
                
              </div>
           
      <div class="login_bthbox">
                <input type="hidden" id="send" name="send" value="0" />
                <input type="hidden" id="isRealnameCheck" name="isRealnameCheck" value="<?php echo $_smarty_tpl->tpl_vars['config']->value['reg_real_name_check'];?>
" />
               <div id="login_bth" class="login_bth" onclick="checkRegUser()">注册</div>
           </div>
        </form>
           </div>
             <!-- 验证码 -->
         <div class="login_otherfs">
           <?php if ($_smarty_tpl->tpl_vars['config']->value['reg_moblie']=='1'&&$_smarty_tpl->tpl_vars['type']->value!=2) {?>
				<a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register','type'=>2),$_smarty_tpl);?>
" class="<?php if ($_smarty_tpl->tpl_vars['type']->value!=2&&$_smarty_tpl->tpl_vars['type']->value!=1) {?>login_wjmm<?php }?>">手机号注册</a>
			<?php }?> 
			<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_email']=='1'&&$_smarty_tpl->tpl_vars['type']->value!=3) {?>
				<a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register','type'=>3),$_smarty_tpl);?>
" class="<?php if ($_smarty_tpl->tpl_vars['type']->value!=3&&$_smarty_tpl->tpl_vars['type']->value!=2) {?>login_wjmm<?php }?>">邮箱注册</a>
			<?php }?> 
			<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_user']=='1'&&$_smarty_tpl->tpl_vars['type']->value!=1) {?>
				<a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'register','type'=>1),$_smarty_tpl);?>
" class="<?php if ($_smarty_tpl->tpl_vars['type']->value!=1&&$_smarty_tpl->tpl_vars['type']->value!=3) {?>login_wjmm<?php }?>">用户名注册</a>
			<?php }?>
           </div>
           <!-- 有账号 -->
      </div>
      <input type="hidden" id="zy_uid" value="" />
      <input type="hidden" id="zy_mobile" value="" />
      <input type="hidden" id="zy_email" value="" />

      <van-popup v-model="checkmobileshow" round   closeable :style="{width: '80%'}">
          <div id="written_off">
			
            <div class="reg_have_tipbox">
                <div class="reg_have_tip">
                    <div class="reg_have_tip_tit_name">温馨提示</div> 
                    <div class="reg_have_tip_tit"><span id="zy_type">{{zy_type}}</span></div>
                    <div class="reg_have_tip_zc" id="zy_name"><span class=reg_have_tip_comname>{{zy_name}}</span></div>
                </div>
                <div class="reg_have_tip_p">
                     如果是你本人，但不记得密码，您可以找回密码<br><span class=""id="jcbind"> <span id="desc_toast"> {{desctoast}}</span></span><br>
                 如有疑问可拨打客服服务热线：<br><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>

                </div>  <div class="reg_have_tip_bthbox">
               <a href="<?php echo smarty_function_url(array('m'=>'wap','c'=>'forgetpw'),$_smarty_tpl);?>
" class="reg_have_tip_bth">找回密码</a>
                    <a href="javascript:void(0);" onclick="CheckPW();" class="reg_have_tip_bth" id="jcbind">解除绑定</a>
                 </div>
                
            </div>
        </div>
      </van-popup>
      <van-popup v-model="checkPWshow" closeable round :style="{  width: '80%' }">
         <div class="login_mmbshow">  <div class=reg_have_tip_tit_name>登录密码</div>
        <div class=login_mmbox>
          <input type="password" value="" id="login_password"placeholder="请输入登录密码" class="login_mm">
          <div class=login_mmboxbth><input type="submit" value="确定" class=tiny_show_tckbox_bth1 onclick="post_pass();" /></div></div></div>
      </van-popup>
 </div>     
    <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/publichtm/public_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

    <?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/js/reg_ajax.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
    <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

</body>
<?php echo '<script'; ?>
>
    var wapurl = "<?php echo smarty_function_url(array('m'=>'wap'),$_smarty_tpl);?>
",
        weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
        pricename = "<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
",
        integral_pricename = "<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>
",
        code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
        code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
',
        sy_reg_type = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_type'];?>
';
    var yunvue =  new Vue({
        el:"#yunvue",
        data:{
          checkmobileshow: false,
          zy_name: '',
          zy_type:'',
          checkPWshow:false,
          desctoast:'',
        },
        created() {   
         this.getInfo();
        },
        methods: {
          getInfo() {
           
               
               $("#yunvue").css('display', 'block');
          },
          yinsi() {
            this.yinsidata = true;
          },
        },

    })
    $(function(){
        $("#moblie").focus(function(event) {
          if ($(this).val()=='') {
            $(this).attr("placeholder","");
          }
        });
        $("#moblie").blur(function(event) {
          if ($(this).val()=='') {
            $(this).attr("placeholder","请输入手机号");
          }
        });
        //账号部分
          $("#username").focus(function(){
            
            if($(this).val()==''){
            $(this).attr("placeholder","");
             }
         });
         $("#username").blur(function(){
             if($(this).val() == ""){
             $(this).attr("placeholder","请输入用户名 长度<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_nameminlen'];?>
-<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_reg_namemaxlen'];?>
位<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_name_han']=='1') {?>汉字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_ne_num']=='1') {?>,数字 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_name_zm']=='1') {?>,字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_name_sp']=='1') {?>,字符@!#.$-_<?php }
}?>");
             }
         });
         // 密码部分
        $('#password').focus(function(){
        if($(this).val()==''){
            $(this).attr("placeholder","")
        }
         });
         $('#password').blur(function(){
             var text_value = $(this).val();
             if(text_value==""){
                 $(this).attr("placeholder","填写密码<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1'||$_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>须包含<?php if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_num']=='1') {?>数字<?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_zm']=='1') {?>,字母 <?php }
if ($_smarty_tpl->tpl_vars['config']->value['reg_pw_sp']=='1') {?>,字符@!#.$-_<?php }
}?>");
             }
             
         });
        //  确认密码部分
        $('#passconfirm').focus(function(){
        var text_value=$(this).val();
        if(text_value == ""){
            $(this).attr("placeholder","")
        }
         });
         $('#passconfirm').blur(function(){
             var text_value = $(this).val();
             if(text_value==""){
                 $(this).attr("placeholder","请确认密码");
             }
         });


         
    });
<?php echo '</script'; ?>
>
</html><?php }} ?>
