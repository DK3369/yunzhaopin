<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 13:00:09
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/admin/login.htm" */ ?>
<?php /*%%SmartyHeaderCode:132954638369e855d9d5ace4-63911370%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '83c9f04c73fd04e18f13c2f4f6342a064e96bda1' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/admin/login.htm',
      1 => 1705977741,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '132954638369e855d9d5ace4-63911370',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'pytoken' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e855d9d77172_10493566',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e855d9d77172_10493566')) {function content_69e855d9d77172_10493566($_smarty_tpl) {?><!DOCTYPE html>
<html lang="en">

<head>
	<meta http-equiv="Content-Type" content="text/html;charset=utf-8">
	<link href="../app/template/admin/js/element.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet"
		type="text/css" />
	<link href="../app/template/admin/images/admin.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet"
		type="text/css" />
	<title><?php echo $_smarty_tpl->tpl_vars['config']->value['sy_webname'];?>
 - 后台管理中心</title>
</head>

<body>
	<div id="bind-captcha" data-id='submit_bt' data-type='click'></div>
	<div class="adminDomeAll">
		<div class="logoinLogo"><img
				src="../app/template/admin/images/admin_new_logo.png?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"></div>
		<div class="logoinBlock1"><img src="../app/template/admin/images/lo_fk2.png"></div>
		<div class="logoinBlock2"><img src="../app/template/admin/images/lo_fk.png"></div>
		<div class="logoinBlock3"><img src="../app/template/admin/images/lo_fk3.png"></div>
		<!-- loao -->
		<div class="loginCont">
			<div class="logoinContImg">
				<div class="logoinBacimg1"><img src="../app/template/admin/images/lo_phpyun.png"></div>
				<div class="logoinBacimg2"><img src="../app/template/admin/images/lo_dp.png"></div>
				<div class="logoinBacimg3"><img src="../app/template/admin/images/lo_zzj.png"></div>
				<div class="logoinBacimg4"><img src="../app/template/admin/images/lo_sj.png"></div>
				<div class="logoinBacimg5"><img src="../app/template/admin/images/lo_ht.png"></div>
				<div class="logoinBacimg6"><img src="../app/template/admin/images/lo_sjdp.png"></div>
				<div class="logoinBacimg7"><img src="../app/template/admin/images/lo_znj.png"></div>
				<div class="logoinBacimg8"><img src="../app/template/admin/images/lo_app.png"></div>
				<div class="logoinBacimg9"><img src="../app/template/admin/images/lo_sj2.png"></div>
				<!-- +++++++++++++++++++++++++++++++++++++++++++ -->
				<div class="logoinBacimg10"><img src="../app/template/admin/images/lo_cxbg.png"></div>
				<div class="logoinBacimg100"><img src="../app/template/admin/images/lo_wx.png"></div>
				<div class="logoinBacimg11"><img src="../app/template/admin/images/lo_cxbg.png"></div>
				<div class="logoinBacimg110"><img src="../app/template/admin/images/lo_xcx.png"></div>
				<div class="logoinBacimg12"><img src="../app/template/admin/images/lo_cxbg.png"></div>
				<div class="logoinBacimg120"><img src="../app/template/admin/images/lo_dy.png"></div>
				<div class="logoinBacimg13"><img src="../app/template/admin/images/lo_cxbg.png"></div>
				<div class="logoinBacimg130"><img src="../app/template/admin/images/lo_bd.png"></div>
				<!-- +++++++++++++++++++++++++++++++++++++++++++ -->
				<div class="logoinBacimg14"><img src="../app/template/admin/images/lo_sm.png"></div>
				<div class="logoinBacimg15"><img src="../app/template/admin/images/lo_sm.png"></div>
				<!-- +++++++++++++++++++++++++++++++++++++++++++ -->
				<div class="logoinBacimg16"><img src="../app/template/admin/images/lo_sm.png"></div>
				<div class="logoinBacimg17"><img src="../app/template/admin/images/lo_sm.png"></div>
				<!-- +++++++++++++++++++++++++++++++++++++++++++ -->
				<div class="logoinBacimg18"><img src="../app/template/admin/images/lo_sm.png"></div>
				<div class="logoinBacimg19"><img src="../app/template/admin/images/lo_sm.png"></div>
			</div>
			<div class="loginBoxs">
				<div class="logoinRight">

					<div class="logoinName">
						<span>欢迎登录后台管理系统</span>
					</div>
					<div class="loginIptbox" id="loginapp">
						<template v-if="islook">
							<div class="loginTabse" v-if="showDiv1">
								<ul class="logoinList">
									<li>
										<div class="adminLogins"><input type="text" class="ipt" placeholder="请输入用户名"
												v-model="username" />
									</li>
									<li>
										<div class="adminLogins adminLoginTwo"><input type="password" class="ipt"
												placeholder="请输入密码" v-model="password" /></div>
									</li>
									<?php if (strpos($_smarty_tpl->tpl_vars['config']->value['code_web'],"后台登录")!==false) {?>

									<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']>2) {?>
									<div class="gtdx-captcha">
										<input type='hidden' ref="verify_token" name="verify_token" id="verify_token" value="">
										<?php if ($_smarty_tpl->tpl_vars['config']->value['code_kind']==6) {?>
										<input type='hidden' ref="verify_str" id="verify_str" name="verify_str" value="">
										<?php }?>
										<input type='hidden' id="popup-submit">
										<input type='hidden' id="bind-submit">
									</div>
									<?php } else { ?>
									<li>
										<div class="loginMessage">
											<input type="text" placeholder="输入验证码" class="ipt" style="width:125px;"
												v-model="authcode" />
											<img src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php"
												align="absmiddle" id='vcode_imgs' @click="checkCode('vcode_imgs');">
										</div>
									</li> <?php }?>

									<?php }?>
									<li>
										<div class="adminLoginsButton">
											<span id="submit_bt" @click="login" class="adminLogiSub" name="login_sub" value="登录">登录</span>
										</div>
									</li>
								</ul>
								<input type="hidden" name="pytoken" value="<?php echo $_smarty_tpl->tpl_vars['pytoken']->value;?>
">
								<div class="loginIptText" style="color: #999; font-size: 13px; text-align:center;">
									<span>官方服务电话：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span>
								</div>
								<?php if ($_smarty_tpl->tpl_vars['config']->value['wx_author_htlogin']!=='0') {?>
								<div class="weixnLogins">
									<div @click="toggleDiv">
										<img src="../app/template/admin/images/chyasw.png" alt="">
										<span>微信扫码登录</span>
									</div>
								</div>
								<?php }?>
							</div>

							<div class="loginTabse" v-else>
								<div class="weixinsapoyis">
									<div class="weixinsapImg">
										<img :src="code_img" alt="">
									</div>
									<div class="weixinsapTxte">
										<span>请使用微信扫一扫登录</span>
									</div>
								</div>
								<div class="loginIptText" style="color: #999; font-size: 13px; text-align:center;">
									<span>官方服务电话：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_freewebtel'];?>
</span>
								</div>
								<div class="weixnLogins">
									<div @click="toggleDiv">
										<img src="../app/template/admin/images/zhanghao.png" alt="">
										<span>账号密码登录</span>
									</div>
								</div>
							</div>
						</template>
					</div>
				</div>
			</div>

		</div>
	</div>
	<?php echo '<script'; ?>
 src="../app/template/admin/js/jquery.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
 src="../app/template/admin/js/vue.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
 src="../app/template/admin/js/element.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
 src="../app/template/admin/js/axios.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php echo '<script'; ?>
 src="../app/template/admin/js/api.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
	<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['tplstyle']->value)."/verify/verify_js.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

	<?php echo '<script'; ?>
>
		var weburl = "<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
",
			code_web = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_web'];?>
',
			code_kind = '<?php echo $_smarty_tpl->tpl_vars['config']->value['code_kind'];?>
';

		const loginapp = new Vue({
			el: '#loginapp',
			data: function () {
				return {
					username: '',
					password: '',
					verify_token: '',
					verify_str: '',
					authcode: '',
					cannext: true,
					showDiv1: true,
					code_img:'',
					codesetval:null,
					islook:false,
				}
			},
			created() {
				this.$nextTick(function () {
					this.islook = true;
				})
			},
			methods: {
				toggleDiv() {
					if(this.showDiv1){
						this.getcode();
					}else{
						this.showDiv1 = true;
					}
				},
				async wxbindstatus() {
				    var that = this;
				    
					axios.post('index.php?c=getwxloginstatus',{}).then(function (result) {
						var res = result.data;
						if (res.error == 0) {
							clearInterval(that.codesetval);
						    message.success(res.msg, function () {
						        // 清空localStorage
								localStorage.clear();
								
								// window.location.reload();
								window.location.href = location.origin + location.pathname;
								localStorage.setItem('indexPath', res.data.path); // 解决默认进首页还是进工作台
						    });
						}else if(res.msg!=''){
							message.error(res.msg);
						}
					}).catch(function (error) {
						console.log(error);
					})
				},
				// 获取微信绑定二维码
				async getcode() {
				    var that = this;
				    
					axios.post('index.php?c=wxlogin',{}).then(function (result) {
						var res = result.data;
						if (res.error == 0) {
						    that.code_img = res.data.code_url;
						    that.codesetval = setInterval(function () {
						        that.wxbindstatus()
						    }, 2000);
							
							that.showDiv1 = false;
						} else {
						    message.error(res.msg);
						}
					}).catch(function (error) {
						console.log(error);
					})
				},
				login: function () {
					if (!this.cannext) {
						return false;
					}
					if (this.username == '') {
						this.$message.error('请填写用户名');
						return false;
					}
					if (this.password == '') {
						this.$message.error('请填写密码');
						return false;
					}
					var codesear = new RegExp('后台登录');
					if (codesear.test(code_web)) {
						if (code_kind == 1) {
							if (this.authcode == '') {
								this.$message.error('请填写验证码');
								return false;
							}
						} else if (code_kind > 2) {
							if (this.$refs.verify_token.value == '') {
								if (code_kind == 6) {
									$("#bind-captcha").trigger("click");
								} else {
									$("#bind-submit").trigger("click");
								}
								return false;
							}
						}
					}

					var _this = this;
					var param = {
						username: this.username,
						password: this.password,
						authcode: this.authcode,
					};
					if (this.$refs.verify_token) {
                        param.verify_token = this.$refs.verify_token.value
					}
                    if (this.$refs.verify_str) {
                        param.verify_str = this.$refs.verify_str.value
                    }
					this.cannext = false;

					axios.post('index.php?c=login', param).then(function (response) {
						var res = response.data;
						if (res.error == 0) {
							// 清空localStorage
							localStorage.clear();

							// window.location.reload();
							window.location.href = location.origin + location.pathname;
							localStorage.setItem('indexPath', res.data.path); // 解决默认进首页还是进工作台
						} else {
							_this.cannext = true;
							_this.$message.error(res.msg);
							_this.checkCode('vcode_imgs');
						}
					}).catch(function (error) {
						console.log(error);
					})
				},
				keyDown: function (e) {
					if (e.keyCode === 13) {
						this.login()
					}
				},
				checkCode: function (id) {
					if (document.getElementById(id)) {
						document.getElementById(id).src = weburl + "/app/include/authcode.inc.php?" + Math.random();
					}
				}
			},
			mounted() {
				// 绑定监听事件
				window.addEventListener("keydown", this.keyDown);
			},
			destroyed() {
				// 销毁事件
				window.removeEventListener("keydown", this.keyDown, false);
			}
		});
	<?php echo '</script'; ?>
>
</body>

</html><?php }} ?>
