<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 17:43:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/info.htm" */ ?>
<?php /*%%SmartyHeaderCode:184609127769e89826c8ad93-75960087%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '65bd79a9d647cdd2a5d554ff81bd90a9aa999624' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/info.htm',
      1 => 1702031896,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '184609127769e89826c8ad93-75960087',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'user_style' => 0,
    'row' => 0,
    'cert' => 0,
    'industry_index' => 0,
    'v' => 0,
    'industry_name' => 0,
    'comdata' => 0,
    'comclass_name' => 0,
    'city_type' => 0,
    'city_index' => 0,
    'city_name' => 0,
    'tv' => 0,
    'company' => 0,
    'addjobnum' => 0,
    'uid' => 0,
    'isremind' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89826cb7298_31899399',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89826cb7298_31899399')) {function content_69e89826cb7298_31899399($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/binding.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>

	var userstyle	=	'<?php echo $_smarty_tpl->tpl_vars['user_style']->value;?>
';
	var setPosition	=	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_setposition'];?>
';
	var map_key = '<?php echo $_smarty_tpl->tpl_vars['config']->value['map_key'];?>
';

	function checkIsUsed(typeStr){

		 var checkStr = $("#"+typeStr).val();
		 if(checkStr){
			 $.ajax({
				type: "POST",
				async: false,
				url: "index.php?c=info&act=ajaxCheck",
				data: {
					typeStr:typeStr,
					checkStr:checkStr
				},
				success: function(data) {

					var data	= 	eval('('+data+')');
					if(data.errcode == 8){
						if(typeStr == 'name'){
							layer.msg('企业名称已存在，请重新填写！', 2, 8);
				            return false;
						}else if(typeStr == 'linktel'){
							layer.msg('手机号码已存在，请重新填写！', 2, 8);
				            return false;
						}
					}
				}
			});
		}
	}
  	function checknext(){
      	var isform= checkform();
      	if(isform==false){
      		return false;
      	}
		window.scrollTo(0,0);
        $("#cominfonext").hide();
		$("#cominfoblack").show();
  	}
  	function checkform(){
  		var cionly	=	$.trim($("#cionly").val());
  		if(cionly=='1'){
			var citycheck = check_form(document.myform.provinceid.value=="",'by_cityid');
		}else{
			var citycheck = check_form(document.myform.cityid.value=="",'by_cityid');
		}


  		var ifcheck =check_form(document.myform.address.value=="",'by_address') &
  		citycheck &
        check_form(document.myform.mun.value=="",'by_mun') &
        check_form(document.myform.pr.value=="",'by_pr') &
        check_form(document.myform.hy.value=="",'by_hy') &
        check_form(document.myform.name.value=="",'by_name');

        if(ifcheck==0){ return false; }

        var x 	=	$.trim($("#map_x").val());
	    var y 	=	$.trim($("#map_y").val());
	    if(setPosition == '1' && x == '' && y == '' && map_key !=''){
	    	layer.msg('请设置企业地图！', 2, 8);
            return false;
	    }

		var html	= 	editor.getContent();

		if(html == ''){
            layer.msg('企业简介不能为空！', 2, 8);
            return false;
        }

		if(document.myform.linkman.value == ''){
            layer.msg($('#by_linkman').html(),2,8);
            return false;
        }

		<?php if ($_smarty_tpl->tpl_vars['row']->value['moblie_status']==1) {?>
          	ifmoblie = true;
        <?php } else { ?>
          	ifmoblie = isjsMobile(document.myform.linktel.value);
        <?php }?>

        <?php if ($_smarty_tpl->tpl_vars['row']->value['email_status']==1) {?>
          	ifemail = true;
        <?php } else { ?>
          	var mail=document.myform.linkmail.value;
	     	if(mail==""){
	       		ifemail = true;
	      	}else{
	       		ifemail = check_email(mail);
	     	}
        <?php }?>



        if(ifemail==false){

             layer.msg('企业邮箱格式错误！', 2, 8);return false;
        }
        if(!document.myform.linktel.value){
          	layer.msg('请填写联系手机！', 2, 8);return false;
        }

        if(ifmoblie==false && document.myform.linktel.value!=''){
          	layer.msg('联系手机格式不正确！', 2, 8);return false;
        }

		var linkphone	=	document.myform.linkphone.value;
		if(!isjsTell(linkphone) && linkphone!=""){
			layer.msg('固话格式不正确！', 2, 8);return false;
		}
  	}

  	function checkblack(){

  		window.scrollTo(0,0);
      	$("#cominfonext").show();
      	$("#cominfoblack").hide();
  	}


	function checkpostcom(){
		var isform= checkform();
      	if(isform==false){
      		return false;
      	}
	    var chk_value 		= 	[];
	    var name          	=  	$.trim($("#name").val());
	    var hy            	=  	$.trim($("#hy").val());
	    var pr            	= 	$.trim($("#pr").val());
	    var mun           	= 	$.trim($("#mun").val());
	    var provinceid    	=  	$.trim($("#provinceid").val());
	    var cityid        	= 	$.trim($("#cityid").val());
	    var three_cityid  	= 	$.trim($("#three_cityid").val());
	    var address       	=	$.trim($("#address").val());
	    var x       		=	$.trim($("#map_x").val());
	    var y       		=	$.trim($("#map_y").val());
	    var content 		= 	editor.getContent();
	    var linkman 		= 	$.trim($("#linkman").val());
	    var linkjob 		= 	$.trim($("#linkjob").val());
	    var linktel 		= 	$.trim($("#linktel").val());
	    var linkphone 		= 	$.trim($("#linkphone").val());
	    var linkmail 		= 	$.trim($("#linkmail").val());
	    var shortname 		= 	$.trim($("#shortname").val());
	    var sdate 			= 	$.trim($("#sdate").val());
	    var moneytype 		= 	$.trim($("#moneytype").val());
	    var money 			= 	$.trim($("#money").val());

	    var linkqq			=	$.trim($("#linkqq").val());

		if (linkman !=''){
			linkman = linkman.replace(/[-_ ]/g,'');// 去掉空格
			if(!linkman){
				return ;
			}
			var  test = linkman.replace(/[0-9]/g,'');
			if (!test){
				layer.msg('联系人不支持全数字', 2, 8);
				return false;
			}else{
				if(/\d/.test(linkman)){
					if(linkman.length>8){
						// obj.value = obj.value.substring(0,8);
						layer.msg('联系人填写字数不能超过8个', 2, 8);
						return false;
					}
				}
			}
		}
		if(linkqq != '' && isQQ(linkqq) == false){

			layer.msg('QQ格式不正确！', 2, 8);return false;
		}

		var website			=	$.trim($("#website").val());
		if(website!=''){
			if(check_url(website)==false){

				layer.msg('企业网址格式不正确！', 2, 8);return false;
			}
		}

	    var busstops 		= 	$.trim($("#busstops").val());
	    var infostatus 		= 	$.trim($("#infostatus").val());
		var i 				= 	0;

		$('input[name="welfare[]"]:checked').each(function() {

	    	chk_value.push($(this).val());
	       	i++;
	   	});

	   	var welfare  		=	chk_value;

	   	/* 强制认证条件 */
	   	var   yyzz_n 		=  	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_licensecert'];?>
';
	    var   moblie_n 		=  	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_mobilecert'];?>
';
	    var   email_n 		=  	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_emailcert'];?>
';
	    var   map_n 		=  	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_setposition'];?>
';

        var params = {name:name,hy:hy,pr:pr,mun:mun,provinceid:provinceid,cityid:cityid,three_cityid:three_cityid,address:address,x:x,y:y,content:content,linkman:linkman,linkjob:linkjob,linktel:linktel,linkphone:linkphone,linkmail:linkmail,shortname:shortname,sdate:sdate,moneytype:moneytype,money:money,website:website,linkqq:linkqq,website:website,busstops:busstops,infostatus:infostatus,welfare:welfare}

	    if (yyzz_n == 1) {
            var com_social_credit = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_social_credit'];?>
';
            if (com_social_credit == 1) {
                var social_credit 	= 	$.trim($("#social_credit").val());
                if (social_credit == '') {
                    layer.msg('统一社会信用代码不能为空！', 2, 8);return false;
                }
                params.social_credit = social_credit
            }

	        var certpic = $.trim($("#cert").attr('src'));
            if (certpic == '') {
                layer.msg('请上传营业执照！', 2, 8);return false;
            }
            var com_cert_owner = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_cert_owner'];?>
';
            if (com_cert_owner == 1) {
                var ownercertpic = $.trim($("#owner_cert").attr('src'));
                if (ownercertpic == '') {
                    layer.msg('请上传经办人身份证！', 2, 8);return false;
                }
            }
            var com_cert_wt = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_cert_wt'];?>
';
            if (com_cert_wt == 1) {
                var wtcertpic = $.trim($("#wt_cert").attr('src'));
                if (wtcertpic == '') {
                    layer.msg('请上传委托书/承诺函！', 2, 8);return false;
                }
            }
            var com_cert_other = '<?php echo $_smarty_tpl->tpl_vars['config']->value['com_cert_other'];?>
';
            if (com_cert_other == 1) {
                var othercertpic = $.trim($("#other_cert").attr('src'));
                if (othercertpic == '') {
                    layer.msg('请上传其他证明材料！', 2, 8);return false;
                }
            }
	    }

	    $.post("index.php?c=info&act=save",params,function(data){

	    	var data = eval('(' + data + ')');

	 		if(data.errcode==9){

	 			//第一次完善基本信息资料
	    		if(data.integralnum==1){
	           		$('#firstComInfo').show();
	       		}

	 			if(yyzz_n == '1' && !data.yyzz_i){

	 				$('#zzrz_msg').html('企业尚未进行资质认证，将无法发布职位');
	 				$('#zzrz_a').show();
	 				$('#zzrz_a').html('开始资质认证');
	 			}else if(moblie_n == '1' && data.moblie_i == '0'){

	 				$('#zzrz_msg').html('企业尚未进行手机认证，将无法发布职位');
	 				$('#zzrz_a').show();
	 				$('#zzrz_a').html('开始手机认证');
	 			}else if(email_n == '1' && data.email_i == '0'){

	 				$('#zzrz_msg').html('企业尚未进行邮箱认证，将无法发布职位');
	 				$('#zzrz_a').show();
	 				$('#zzrz_a').html('开始邮箱认证');
	 			}else if(map_n == '1' && data.map_i == '0'){

	 				$('#zzrz_msg').html('企业尚未进行地图设置，将无法发布职位');
	 				$('#map_a').show();
	 			}else{

	 				if(data.yyzz_i != '1'){

	 					$('#zzrz_msg').html('企业资质验证，提升招聘效果哦');
	 					$('#zzrz_a').show();
	 					$('#zzrz_a').html('开始资质认证');
	 				}else if(data.moblie_i == '0'){

	 					$('#zzrz_msg').html('企业手机认证，提升招聘效果哦');
	 					$('#zzrz_a').show();
	 					$('#zzrz_a').html('开始手机认证');
	 				}else if(data.email_i == '0'){

	 					$('#zzrz_msg').html('企业邮箱认证，提升招聘效果哦');
	 					$('#zzrz_a').show();
	 					$('#zzrz_a').html('开始邮箱认证');
	 				}else{

	 					$('#zzrz_msg').html('发布职位吸引优秀人才上门');
	 				}

	    			if(data.addjobnum == 1){
	    				$('#job_a').hide();
	    				$('#job_first').show();
					}else{
						$('#job_a').show();
	    				$('#job_first').hide();
					}
	 			}


	           	var msglayer = layer.open({
	           		type: 1,
	              	skin: 'yun_skin',
	             	title: false,
	              	closeBtn : 0,
	              	content: $('#comInfoNotice')
	            });

	        }else{

	          	layer.msg(data.msg, 2, 8);
	        }
		});

	}
<?php echo '</script'; ?>
>
<style>
.layui-input, .layui-select, .layui-textarea{border-radius:6px;}
.com_release_textnew{border-radius:6px;}
</style>
<div class="w1000">
	<div class="admin_mainbody">
		<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

    	 <div class=right_box>
    	 	<div class="newmember_tit">
    	 		<ul>
					<li class="newmember_titcur"><a href="index.php?c=info">基本信息</a></li>
					<li><a href="index.php?c=info&act=side">补充信息</a></li>
					<li><a href="index.php?c=address">地址管理</a></li>
					<li><a href="index.php?c=show"> 公司相册</a></li>
					<li><a href="index.php?c=uppic"> 公司LOGO</a></li>
					<li><a href="index.php?c=product">产品介绍</a></li>
					<li><a href="index.php?c=news">公司资讯</a></li>
					<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
					<li><a href="index.php?c=map">公司地图</a></li>
					<?php }?>
					<li><a href="index.php?c=comtpl">个性化模板</a></li>

    	 		</ul>
    	 	</div>


		<!--<div class="com_topbth_box">
		    <a  href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>'`$uid`'),$_smarty_tpl);?>
" target="_blank"class="com_ylbth">预览主页</a>
		</div>-->


			 <div class="clear"></div>
			            <div class=admincont_box>



			                <div class="com_body">

			<div class="vip_box" style="margin-top: 0px;;">
		        <iframe id="supportiframe"  name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
		        <form id="infoform" name="myform" method="post" target="supportiframe"  autocomplete="off" enctype="multipart/form-data" class="layui-form">


					<input type="hidden" id="layupload_type" value="2"/>
					<input type="hidden" id="laynoupload" value="1"/>

					<div class="clear"></div>

		      		<div class="com_release_box" id="cominfonext">
						<ul>

							<li style="z-index:10">

								<div class="com_release_name"><i class="ff0">*</i>  企业全称：</div>
								<?php if ($_smarty_tpl->tpl_vars['row']->value['yyzz_status']==1) {?>
									<label><div class="info_comname_text"><?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
</div></label>
									<input type="hidden" id="name"  name="name" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
">
								<?php } else { ?>
									<div class="com_release_textnew">
										<input type="text" size="45" id="name" name="name" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['name'];?>
" lay-verify="required" class="com_release_textnew_text" placeholder="请与贵公司营业热照注册名保持一致" onblur="checkIsUsed('name');" />
									</div>
					<!--	<div id="cdiv" class="com_">
										<?php if (!empty($_smarty_tpl->tpl_vars['cert']->value)) {?>
											<?php if ($_smarty_tpl->tpl_vars['cert']->value['status']==2) {?>
												<div id="dcert" class="com_info_tipbox_p">

														<div class="com_info_tipbox_box">
															<i class="com_info_tip_icon"></i>
															<?php if ($_smarty_tpl->tpl_vars['cert']->value['statusbody']) {?>
															<div id="showcomstatusbody" style="display:none;">
			                                             	<div class="rzwtg">
			                                                <div class="rzwtg_hi">尊敬的用户你好！</div>
			                                                 <div>您的企业资质认证未通过审核，未通过原因如下：</div>
															<div class="rzwtg_yy"><?php echo $_smarty_tpl->tpl_vars['cert']->value['statusbody'];?>
请按要求重新认证审核</div>
			                                                <div class="rzwtg_bthbox"><a href="index.php?c=binding&act=comcert"  class="rzwtg_bth" >重新认证</a>    </div>
			                                               </div>
															</div>
															<?php }?>
			 												<a href="index.php?c=binding&act=comcert"  class="com_set_a fl" >重新认证</a>
															<span class="" style="margin-left:5px;">
																审核未通过
																<?php if ($_smarty_tpl->tpl_vars['cert']->value['statusbody']) {?>
																<a class="infor_check" href="javascript:void(0)" onclick="showcomstatusbody();" style="color:#09F">说明</a>
																<?php }?>
															</span>
														</div>

												</div>
											<?php } else { ?>
												<div id="dcert" class="com_info_tipbox_p">
													<div class="com_info_tipbox_box"><i class="com_info_tip_icon"></i>企业资质已上传，请等待审核</div>
												</div>
											<?php }?>
										<?php } else { ?>
											<div id="dcert" class="com_info_tipbox_p">
												<div class="com_info_tipbox_box"><i class="com_info_tip_icon"></i>请与贵公司企业资质保持一致 <a href="index.php?c=binding&act=comcert" class="com_set_a" ><i class="com_set_a_rzicon"></i>认证资质</a></div>
											</div>
										<?php }?>
									</div -->
								<?php }?>
								<span id="by_name" class="errordisplay">企业全称不能为空</span>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 从事行业：</div>
								<div class="com_release_selectbox">
									<div class="layui-input-inline">
										<select name="hy" lay-filter="hy" id="hy">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['industry_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['hy']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['industry_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
									<span id="by_hy" class="errordisplay">请选择从事行业</span>
								</div>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 企业性质：</div>
								<div class="com_release_selectbox">
									<div class="layui-input-inline">
										<select name="pr" lay-filter="pr" id="pr">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_pr']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['pr']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
									<span id="by_pr" class="errordisplay">企业性质不能为空！</span>
								</div>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 企业规模：</div>
								<div class="com_release_selectbox">
									<div class="layui-input-inline">
										<select name="mun" lay-filter="mun" id="mun">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_mun']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['mun']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
									<span id="by_mun" class="errordisplay">请选择企业规模</span>
								</div>
							</li>
							<li>
								<input type="hidden" id="cionly" name="cionly" value="<?php if (empty($_smarty_tpl->tpl_vars['city_type']->value)) {?>1<?php }?>">
								<div class="com_release_name"><i class="ff0">*</i> 所在地区：</div>

								<div class="com_release_select com_release_selectw145">
									<div class="layui-input-inline">
										<select name="provinceid" lay-filter="citys" id="provinceid">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_index']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['provinceid']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
								</div>

								<div class="com_release_select com_release_selectw145">
									<div class="layui-input-inline">
										<select name="cityid" lay-filter="citys" id="cityid">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['row']->value['provinceid']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['cityid']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
								</div>
			                    <div class="com_release_select com_release_selectw145">
									<div class="layui-input-inline" id="cityshowth" <?php if ($_smarty_tpl->tpl_vars['row']->value['three_cityid']<1) {?>style="display:none"<?php }?>>
										<select name="three_cityid" id="three_cityid">
											<option value="">请选择</option>
											<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['city_type']->value[$_smarty_tpl->tpl_vars['row']->value['cityid']]; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>
												<option value="<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
" <?php if ($_smarty_tpl->tpl_vars['row']->value['three_cityid']==$_smarty_tpl->tpl_vars['v']->value) {?> selected<?php }?>><?php echo $_smarty_tpl->tpl_vars['city_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
</option>
											<?php } ?>
										</select>
									</div>
								</div>
								<span id="by_cityid" class="errordisplay">请选择所在地</span>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 详细地址：</div>

								<div class="com_release_cont_text com_info_tip" style="width:640px">
									<input type="text" id="address" name="address" size="45" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['address'];?>
" class="layui-input" placeholder="建议格式：XX市XX县XX路" onkeyup="addressKeyup();"/>
									<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
									<i class="com_release_mapicon" onclick="clickSearch();"></i>
									<?php }?>
									<div class="comEleaseMaps" id="poiSearch" style="display: none;"></div>
								</div>
								<span id="by_address" class="errordisplay">请填写正确的公司地址</span>

								<div class="clear"></div>
                                <div style="margin-top:10px;">
			                  	<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
			                  	<div id="map_container" style="width:640px;height:300px; left:0px;"></div>
			                  	<?php }?>
			                  	<input name="x" id="map_x" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['x'];?>
">
			                  	<input name="y" id="map_y" type="hidden" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['y'];?>
">
								</div>
							</li>

							<li>
								<div class="com_release_name"><i class="ff0">*</i>  企业简介：</div>
								<div class=textbox>
			 						<?php echo '<script'; ?>
 id="content" name="content" type="text/plain" style="width:640px; height:150px;"> <?php echo $_smarty_tpl->tpl_vars['row']->value['content'];?>
 <?php echo '</script'; ?>
>
								</div>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 联  系  人：</div>
								<div class="com_release_cont_text">
									<input type="text" id="linkman" name="linkman" size="15" lay-verify="required" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkman'];?>
" class="layui-input" placeholder="请输入您的姓名" style="width:225px;display: inline-block;"  onblur="notNumber(this)" />
									 <input type="text" id="linkjob" name="linkjob" size="15" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkjob'];?>
" class="layui-input" placeholder="职位称呼：如 经理 "style="width:160px;display: inline-block;margin-left:10px;"//>
								</div>

								<span id="by_linkman" class="errordisplay">请填写招聘联系人</span>
							</li>
							<li>
								<div class="com_release_name"><i class="ff0">*</i> 联系手机：</div>
								<div class="com_release_cont_text">
									<div id="bdphone">
										<?php if ($_smarty_tpl->tpl_vars['row']->value['moblie_status']==1) {?>
										<div class="com_release_bd">
											<input type="text" size="35" id="linktel" name="linktel" lay-verify="phone" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linktel'];?>
" class="com_release_bd_text" readonly="readonly" />
											<a href="javascript:void(0)" onclick="getshow('moblie','绑定手机号码');" class="com_release_bd_a">更换号码</a>

										</div>
										<?php } else { ?>
										<div class="com_release_textnew">
											<input type="text" id="linktel" name="linktel" size="25" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linktel'];?>
" onkeyup="this.value=this.value.replace(/[^0-9-]/g,'')" class="com_release_textnew_text" placeholder="请输入手机号码" onblur="checkIsUsed('linktel');"/>
											<a href="javascript:void(0)" onclick="getshow('moblie','绑定手机号码');" class="com_release_textnew_a">绑定手机</a>
										</div>
										<span id="by_linktel" class="errordisplay">手机格式不正确</span>
										<?php }?>
									</div>
								</div>

							</li>
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_enforce_licensecert']==1&&$_smarty_tpl->tpl_vars['config']->value['com_social_credit']==1) {?>
							<li>
								<div class="com_release_name"><i class="ff0">*</i>统一社会信用代码：</div>
								<div class="com_release_textnew">
                                        <input maxlength="18" type="text" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['social_credit'];?>
" name="social_credit" id="social_credit" class="com_release_textnew_text">
								</div>
							</li>
                            <?php }?>
                            <?php if ($_smarty_tpl->tpl_vars['config']->value['com_enforce_licensecert']==1) {?>
							<li >
                                <div class="com_release_name">企业资质：</div>
								<div class="info_zz_box ">
									<div class="info_zz_list">
                                        <div class="info_logo">
                                        <img id="cert" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['check'];?>
" width="100" height="100" <?php if ($_smarty_tpl->tpl_vars['row']->value['check']=='') {?>class="none"<?php }?>/>

                                        <button type="button" class="info_logosc adminupload" lay-data="{imgid: 'cert',path: 'cert', uid: '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
',usertype: 2}">上传图片</button>
                                        </div>
									    <div class="info_zz_name"><i class="ff0">*</i>营业执照 </div>
									</div>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_cert_owner']==1) {?>
                                    <div class="info_zz_list">
                                        <div class="info_logo">
                                        <img id="owner_cert" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['owner_cert'];?>
" width="100" height="100" <?php if ($_smarty_tpl->tpl_vars['row']->value['owner_cert']=='') {?>class="none"<?php }?>/>
                                        <button type="button" class="info_logosc adminupload" lay-data="{imgid: 'owner_cert',path: 'owner_cert',uid: '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
',usertype: 2}">上传图片</button>
                                        </div>
                                        <div class="info_zz_name"><i class="ff0">*</i>经办人身份证 </div>
                                    </div>
                                            <?php }?>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_cert_wt']==1) {?>
                                    <div class="info_zz_list">
                                        <div class="info_logo">
                                        <img id="wt_cert" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['wt_cert'];?>
" width="100" height="100" <?php if ($_smarty_tpl->tpl_vars['row']->value['wt_cert']=='') {?>class="none"<?php }?>/>
                                        <button type="button" class="info_logosc adminupload" lay-data="{imgid: 'wt_cert',path: 'wt_cert',uid: '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
',usertype: 2}">上传图片</button>
                                        </div>
                                        <div class="info_zz_name"><i class="ff0">*</i>委托书/承诺函 </div>
                                    </div>
                                            <?php }?>
                                        <?php if ($_smarty_tpl->tpl_vars['config']->value['com_cert_other']==1) {?>
                                    <div class="info_zz_list">
                                        <div class="info_logo">
                                        <img id="other_cert" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['other_cert'];?>
" width="100" height="100" <?php if ($_smarty_tpl->tpl_vars['row']->value['other_cert']=='') {?>class="none"<?php }?>/>
                                        <button type="button" class="info_logosc adminupload" lay-data="{imgid: 'other_cert',path: 'other_cert',uid: '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
',usertype: 2}">上传图片</button>
                                        </div>
                                        <div class="info_zz_name"><i class="ff0">*</i>其他证明材料 </div>
                                    </div>
                                            <?php }?>
								</div>
                                <div class="info_zz_tip">
								    <div class="info_zz_tipname">关于资质认证	</div>
								    <div>1.请上传和你公司名称一致的复印件或照片，我们将在2个工作日内完成认证审核，实名认证不收取任何费用</div>
								    <div>2.认证成功后您的公司名称将无法修改，如需更换公司名称，请联系客服重新认证	</div>
								    <div>3.支持5M,支持<?php echo $_smarty_tpl->tpl_vars['config']->value['pic_type'];?>
格式</div>
                                    <?php if ($_smarty_tpl->tpl_vars['config']->value['com_cert_wt']==1) {?>
									<div>4.声明：委托书/承诺函材料必须使用网站统一格式,
                                        <a href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['exa_cert_wt'];?>
" target="_blank" style="color:red">请单击这里，下载委托书/承诺函模板</a>
                                    </div>
                                    <?php }?>
                                </div>
							</li>
                            <?php }?>
			<li style="z-index:10">
						<div class="com_release_name">企业LOGO：</div>
				<div class="info_logo_box ">
					<div class="info_logo">
						<img id="logo" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['logo'];?>
" width="100" height="100" onerror="showImgDelay(this,'<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_ossurl'];?>
/<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_unit_icon'];?>
','2')"/>

							<button type="button" class="info_logosc adminupload" lay-data="{imgid: 'logo',path: 'company',uid: '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
',usertype: 2}">上传LOGO</button>
						</div>
						<div class="info_logotip">1.请用<?php if ($_smarty_tpl->tpl_vars['config']->value['pic_type']) {
echo $_smarty_tpl->tpl_vars['config']->value['pic_type'];
} else { ?>jpg , png , jpeg , bmp , gif<?php }?>格式的图片,大小不超过2M;<br>2.尺寸不小于300*300像素，正方形1:1效果最佳.</div>
					</div>

			</li>
							<li>
								<div class="com_release_name">固定电话：</div>
								<div class="com_release_textnew">
									<input type="text" id="linkphone" name="linkphone" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkphone'];?>
" onkeyup="this.value=this.value.replace(/[^0-9-]/g,'')" class="com_release_textnew_text" placeholder="固定电话" />
								</div>
							</li>
							<li>
								<div class="com_release_name"> 企业邮箱：</div>
								<div class="com_release_cont_text" id="bdmail">
									<?php if ($_smarty_tpl->tpl_vars['row']->value['email_status']==1) {?>
										<div class="com_release_bd">
										<input type="text" id="linkmail" name="linkmail" size="35" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkmail'];?>
" class="com_release_bd_text"  readonly="readonly"/>

												<a href="javascript:void(0)"  onclick="getshow('email','绑定邮箱');" class="com_release_bd_a">更换邮箱</a>

										</div>
									<?php } else { ?>
									<div class="com_release_textnew">
										<input type="text" id="linkmail" name="linkmail" size="35" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkmail'];?>
" class="com_release_textnew_text"placeholder="请输入您工作中使用的邮箱用于接受简历" />


												<a href="javascript:void(0)"  onclick="getshow('email','绑定邮箱');" class="com_release_textnew_a">绑定邮箱</a>

										</div>
									<?php }?>
									<span id="by_linkmail" class="errordisplay">企业邮箱格式错误</span>
								</div>
							</li>
			        		<li>
								<input class="btn_01" style="_margin-left:-3px" type="button" onclick="checkpostcom()" value="保存信息">
							</li>
			        	</ul>
		        	</div>
		     		<div class="com_release_box" id="cominfoblack"  style="display:none;">
				   		<div class="com_release_box_h1"><span class="com_release_box_h1_n">完善信息</span><em class="com_release_box_h1_z">完善基本信息可以让求职者更好的了解您的企业~</em></div>

						<ul>
							<li>
								<div class="com_release_name">企业简称</div>
								<div class="com_release_cont_text">
									<input type="text" size="45" id="shortname" name="shortname" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['shortname'];?>
" lay-verify="required" class="layui-input"/>
								</div>
								<span id="by_shortname" class="errordisplay">简称不能为空</span>
							</li>

							<li>
								<div class="com_release_name"> 创办时间</div>
			 					<input type="text" name="sdate" id="sdate" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['sdate'];?>
" class="com_info_text">
							</li>
							<li>
								<div class="com_release_name"> 注册资金</div>

								<div class="com_release_select com_release_selectw145">
									<div class="layui-input-inline">
										<select name="moneytype" lay-filter="moneytype" id="moneytype">
											<option value="">请选择</option>
											<option value="1" <?php if ($_smarty_tpl->tpl_vars['row']->value['moneytype']==1) {?> selected<?php }?>>人民币</option>
											<option value="2" <?php if ($_smarty_tpl->tpl_vars['row']->value['moneytype']==2) {?> selected<?php }?>>美元</option>
										</select>
			 						</div>
								</div>

								<div class="layui-input-inline">
									<input type="text" id="money" name="money" size="10" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['money'];?>
" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" class="com_info_text com_info_text_120" />
			 						<span class='moneyname' id="money_1" <?php if ($_smarty_tpl->tpl_vars['row']->value['moneytype']!='1') {?>style="display:none;"<?php }?>>万元</span>
			 						<span class='moneyname' id="money_2" <?php if ($_smarty_tpl->tpl_vars['row']->value['moneytype']!='2') {?>style="display:none;"<?php }?>>万美元</span>
								</div>
							</li>
							<li>
								<div class="com_release_name"> 联 系 QQ</div>
								<div class=textbox>
									<input type="text" id="linkqq" name="linkqq" size="15" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['linkqq'];?>
" onkeyup="this.value=this.value.replace(/[^0-9]/g,'')" maxlength='12' class="com_info_text"/>
								</div>
							</li>
							<li>
								<div class="com_release_name"> 企业网址</div>
								<div class="textbox">
									<input type="text" id="website" name="website" size="35" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['website'];?>
" class="com_info_text"/>
									如：<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>

								</div>
							</li>
							<li>
								<div class="com_release_name"> 公交站点</div>
								<div class="textbox">
									<textarea name="busstops" id="busstops" class="com_info_textarea" rows="3"><?php echo $_smarty_tpl->tpl_vars['row']->value['busstops'];?>
</textarea>
								</div>
							</li>
							<li>
								<div class="com_release_name"> 联系方式</div>
								<div class="com_release_selectbox">
									<div class="layui-input-inline">
										<select name="infostatus" lay-filter="infostatus" id="infostatus">
											<option value="">请选择</option>
											<option value="1" <?php if ($_smarty_tpl->tpl_vars['row']->value['infostatus']==1) {?> selected<?php }?>>公开</option>
											<option value="2" <?php if ($_smarty_tpl->tpl_vars['row']->value['infostatus']==2) {?> selected<?php }?>>不公开</option>
										</select>
									</div>
								</div>
							</li>
							<li>
								<div class="com_release_name"> 公司二维码</div>
								<button type="button" class="yun_bth_pic adminupload" lay-data="{imgid: 'ewm',parentid: 'imgparent',path: 'company','uid': '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
', 'usertype': 2}">上传二维码</button>
			    				<div class="com_release_nameewm_img <?php if (!$_smarty_tpl->tpl_vars['row']->value['comqcode']) {?>none<?php }?>" id="imgparent">
									<img id="ewm" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['comqcode'];?>
" width="40" height="40">
								</div>
							</li>
							<li class="jobadd_list_fl">
								<div class="com_release_name">福利待遇</div>
			 					<div class="layui-form-item">
									<div class="layui-input-block">
										<span class="" id="addwelfarelist">
											<?php  $_smarty_tpl->tpl_vars['tv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['tv']->_loop = false;
 $_smarty_tpl->tpl_vars['key'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['row']->value['arraywelfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['tv']->key => $_smarty_tpl->tpl_vars['tv']->value) {
$_smarty_tpl->tpl_vars['tv']->_loop = true;
 $_smarty_tpl->tpl_vars['key']->value = $_smarty_tpl->tpl_vars['tv']->key;
?>
												<input name="welfare[]" id="welfare<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" value="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" <?php if (in_array($_smarty_tpl->tpl_vars['tv']->value,$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?> checked="checked" <?php }?> type="checkbox" title="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" data-tag="<?php echo $_smarty_tpl->tpl_vars['tv']->value;?>
" class="changewelfare" lay-skin="primary">
											<?php } ?>
										</span>

			 							<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comdata']->value['job_welfare']; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
			 								<?php ob_start();?><?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
<?php $_tmp1=ob_get_clean();?><?php if (!in_array($_tmp1,$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?>
												<input name="welfare[]" id="welfare<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" value="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" <?php if (in_array($_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value],$_smarty_tpl->tpl_vars['row']->value['arraywelfare'])) {?> checked="checked" <?php }?> type="checkbox" title="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" data-tag="<?php echo $_smarty_tpl->tpl_vars['comclass_name']->value[$_smarty_tpl->tpl_vars['v']->value];?>
" class="changewelfare" lay-skin="primary">
											<?php }?>
										<?php } ?>
										<div class="addwelfare_b">
											<input class="addwelfare_text" type="text" tabindex="1000" id="addwelfare" onblur="checkWelfare('addwelfare');" ><a class="addwelfarebox">添加福利</a>
										</div>
									</div>
								</div>
							</li>
							<li>
								<input class="btn_01" style="_margin-left:-3px" type="button" onclick="checkpostcom()" value="保存信息">
								<input class="bth_return" type="button" onclick="checkblack()" value="<返回上一步 ">
							</li>
			            </ul>
		            	<div class="clear"></div>
		          	</div>
				  	<div class="clear"></div>
		        </form>
				<div class="clear"></div>
	      	</div>
	    </div>
  	</div>
</div>
</div>

<div class="clear"></div>

<!--绑定手机弹出框-->
<div id="moblie" style=" display:none;">
  	<div class="Binding_pop_box" style="padding:10px;width:480px;background:#fff;">
  		<div class="Binding_pop_box_list" style=" display:block">
   			<span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>手机号码：</span>
  			<input type="text" name="moblie" class="Binding_pop_box_list_text" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linktel'];?>
">
   		</div>

	 	<div class="Binding_pop_box_list"><span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>验证码：</span>
	 		<input type="text" name="phoneimg_code" maxlength="6" class="Binding_pop_box_list_text">
	 		<img id="pcode_img" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" style=" margin:0 5px 5px 5px; vertical-align:middle;">
	 		<a href="javascript:void(0);" onclick="checkCode('pcode_img');">看不清</a>
	 	</div>
		<div class="Binding_pop_box_list">
			<span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>手机校验码：</span>
			<div class="Binding_pop_right">
				<input type="text" id="moblie_code" class="Binding_pop_box_list_text">
				<a href="javascript:;" onclick="sendmoblie('pcode_img');" class="Binding_pop_box_magbth" id="time">获取短信校验码</a>
			</div>
	    </div>
    	<div class="Binding_pop_box_list">  <input class="Binding_pop_bth" onclick="check_moblie();" type="button" value="提交"> </div>

    	<div class="Binding_pop_tip">收不到短信验证码?</div>
    	<div class="Binding_pop_tip_p">
    	1.短信验证码有效时效为120秒，超过120秒请点击重新发送；<br>
    	2.如您手机无法收取短信或者收取延迟，请关机重启或者联系运营商处理；<br/>
    	3.如以上措施还无法解决，请确认用户名，手机号以及邮箱联系我司客服
    	</div>
  	</div>
</div>
<!--绑定邮箱弹出框-->
<div id="email" style="display:none;">
   	<div class="Binding_pop_box" style="padding:10px;width:480px;background:#fff">
    	<div >
	      	<div class="Binding_pop_box_list">
	      		<span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>我的邮箱：</span>
	        	<input type="text" name="email" value="<?php echo $_smarty_tpl->tpl_vars['company']->value['linkmail'];?>
" class="Binding_pop_box_list_text Binding_pop_box_list_textw200">
	      	</div>
	      	<div class="Binding_pop_box_list">
	      		<span class="Binding_pop_box_list_left"><i class="Binding_pop_box_list_left_i">*</i>验证码：</span>
	        	<input type="text" name="email_code" maxlength="6" class="Binding_pop_box_list_text">
	        	<img id="vcode_img" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/app/include/authcode.inc.php" style=" margin:0 5px 5px 5px; vertical-align:middle;">
	        	<a href="javascript:void(0);" onclick="checkCode('vcode_img');">看不清</a>
	        </div>
	       	<div class="Binding_pop_box_list"><input class=" Binding_pop_bth" type="button" onclick="sendbemail('vcode_img');" value="发送验证邮件"> </div>
			<div class="Binding_pop_tip">没收到邮件？</div>
			<div class="Binding_pop_tip_p">
			1. 请检查您的垃圾箱或者广告箱，邮件有可能被误认为垃圾或者广告邮件；<br/>
			2.验证邮件24小时内有效，请尽快登录您的邮箱点击验证链接完成验证
		    </div>
    	</div>
  	</div>
</div>
<input type="hidden" id="info" value="1" />
<!--弹出框 end-->
<?php echo '<script'; ?>
>
	function notNumber(obj){
		obj.value = obj.value.replace(/[-_ ]/g,'');// 去掉空格
		if(!obj.value){
			return ;
		}
		var  test = obj.value.replace(/[0-9]/g,'');
		if (!test){
			layer.msg('联系人不支持全数字', 2, 8);
			obj.value = ''
			$('#linkman').focus()
			return false;
		}else{
			if(/\d/.test(obj.value)){
				if(obj.value.length>8){
					// obj.value = obj.value.substring(0,8);
					layer.msg('联系人填写字数不能超过8个', 2, 8);
					obj.value = ''
					$('#linkman').focus()
					return false;
				}
			}
		}
	}
	function showcomstatusbody(){
		$.layer({
			type : 1,
			title : '审核说明',
			closeBtn : [0 , true],
			border : [10 , 0.3 , '#000', true],
			area : ['350px','auto'],
			page : {dom :"#showcomstatusbody"}
		});
	}


	function checkWelfare(id){
		var welfare = $('#'+id).val();
		if(welfare.length>=2 && welfare.length<=8){
			//判断信息是否已经存在
			$('.changewelfare').each(function(){
				var otag = $(this).attr('data-tag');
				if(welfare == otag){
					layer.msg('相同福利已存在，请选择或重新填写！', 2,8);
					$('#'+id).val('');
					return false;
				}
			});

		}else{
			layer.msg('请输入2-8个福利字符！', 2,8);
			$('#'+id).val('');
			return false;
		}
	}
	//ajax提交表单
	$(function () {
		//添加福利
		$('.addwelfarebox').click(function(){

			var welfare = $('#addwelfare').val();

			$('#addwelfarelist').append('<input name="welfare[]" value="'+welfare+'" checked="checked"  type="checkbox" title="'+welfare+'" data-tag="'+welfare+'" class="changewelfare" lay-skin="primary">');

			layui.use(['layer', 'form'], function(){
				var layer = layui.layer
					,form = layui.form
					,$ = layui.$;
				form.render('checkbox');
			});

			$('#addwelfare').val('');

		});
	});

    function showpic(obj){
		var url =	$(obj).attr('data_url');
        var ourl =   $(obj).attr('data_ourl');
        var wurl =   $(obj).attr('data_wurl');
		var picjson={
		  "data": []
		}
        if(url){
            picjson.data.push({"title":'营业执照',"src": url,"thumb": url});
        }
        if(ourl){
            picjson.data.push({"title":'经办人身份证',"src": ourl,"thumb": ourl});
        }
        if(wurl){
            picjson.data.push({"title":'委托书/承诺函',"src": wurl,"thumb": wurl});
        }
		layer.photos({
			photos: picjson
			,anim: 5 //0-6的选择，指定弹出图片动画类型，默认随机（请注意，3.0之前的版本用shift参数）
		});
	}
	layui.use(['form','layer', 'laydate','upload'], function(){
		var $ = layui.$,
			form = layui.form,
			laydate = layui.laydate,
			upload = layui.upload,
			layer = layui.layer;

      	laydate.render({
        	elem: '#sdate'
        	,type: 'year',
        	ready: function(date){
          		$('.layui-laydate li').click(function () {
            		$('.laydate-btns-confirm').trigger('click');
          		});
       	 	},
        	change: function(value, date, endDate){
          		$('.layui-laydate li').click(function () {
            		$('.laydate-btns-confirm').trigger('click');
          		});
        	}
      	})

		form.on('select(moneytype)', function(data){
	   		if(data.value == 1){
	        	$("#money_1").show();
	        	$("#money_2").hide();
	      	}else{
	        	$("#money_2").show();
	        	$("#money_1").hide();
	      	}
	    });

  	});

  	$('#showUploadQrcode').on('mouseover', function(){
  		var that = this;
    	layer.tips( '<img src="<?php echo smarty_function_url(array('m'=>'upload','c'=>'qrcode','type'=>1),$_smarty_tpl);?>
" alt="手机扫码拍照上传" />', that);
    	return false;
  	});

  	$('#showUploadQrcode').on('mouseout', function(){
    	layer.closeAll('tips');
  	});

<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 language=javascript src='<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/data/plus/city.cache.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
'><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 language=javascript src='<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/city.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
'><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/layui.upload.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" type='text/javascript'><?php echo '</script'; ?>
>

<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/ueditor/ueditor.config.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/ueditor/ueditor.all.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 language="javascript">
	var editor = UE.getEditor('content',{
		toolbars:[[ 'Source','|', 'Undo', 'Redo','Bold', 'italic', 'underline', 'fontborder', 'strikethrough', 'fontfamily', 'fontsize',  'forecolor', 'backcolor', 'removeformat', 'autotypeset', 'pasteplain', '|','insertorderedlist', 'insertunorderedlist', 'selectall', 'cleardoc', '|','simpleupload', '|', 'indent', '|','justifyleft', 'justifycenter', 'justifyright', 'justifyjustify']],wordCount:false,elementPathEnabled:false,initialFrameHeight:200
	});
<?php echo '</script'; ?>
>
<!-- 不强制但是第一次认证 -->
<div class="yun_prompt_box" id="comInfoNotice" style="display:none;" >

	<div class="yun_tipsbox_tit">
		<span class="yun_tipsbox_tit_name">温馨提示</span>
		<a href="javascript:void(0)" onclick="window.location.reload();" class="yun_tipsbox_close"></a>
	</div>

 	<div class="yun_prompt_writingicon"><i class="yun_prompt_writingicon_right"></i></div>

	<div class="yun_prompt_writing">企业资料保存成功！</div>

	<div class="yun_prompt_writing_obtain" id="firstComInfo" style="display:none;">
		完善企业资料获得<span class="yun_prompt_writing_obtain_n">+<?php echo $_smarty_tpl->tpl_vars['config']->value['integral_userinfo'];?>
</span><?php echo $_smarty_tpl->tpl_vars['config']->value['integral_priceunit'];
echo $_smarty_tpl->tpl_vars['config']->value['integral_pricename'];?>

	</div>

	<div class="yun_prompt_writing_tip" id='zzrz_msg'>企业尚未进行资质验证，将无法发布职位</div>

	<div class="yun_prompt_writing_operation">
	    <a href="index.php?c=binding" id='zzrz_a' class="yun_prompt_writing_operation_bth" style='display:none;'>开始资质认证</a>
		<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
	    <a href="index.php?c=map" id='map_a' class="yun_prompt_writing_operation_bth" style='display:none;'>设置地图</a>
		<?php }?>
	    <a href="javascript:void(0)" id='job_a' onclick="jobadd_url('<?php echo $_smarty_tpl->tpl_vars['addjobnum']->value;?>
');return false;" class="yun_prompt_writing_operation_bth" style="display:none;">发布职位</a>
		<a href="javascript:void(0)" id='job_first' onclick="jobadd_url('1');return false;" class="yun_prompt_writing_operation_bth" style="display:none;">发布职位</a>
	    <a href="<?php echo smarty_function_url(array('m'=>'company','c'=>'show','id'=>$_smarty_tpl->tpl_vars['uid']->value),$_smarty_tpl);?>
" id='com_a' target="_blank" class="yun_prompt_writing_operation_bth">预览主页</a>
	</div>
	<!--提示部分  end-->


</div>
<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
<?php echo '<script'; ?>
 type="text/javascript">
	window._AMapSecurityConfig = {
		securityJsCode:'<?php echo $_smarty_tpl->tpl_vars['config']->value['map_secret'];?>
'
	}
<?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['mapurl'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
 type="text/javascript" src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/map.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<?php echo '<script'; ?>
>

	var map = new AMap.Map("map_container");
	$(document).ready(function() {
		<?php if (($_smarty_tpl->tpl_vars['row']->value['x']==''||$_smarty_tpl->tpl_vars['row']->value['y']=='')&&$_smarty_tpl->tpl_vars['row']->value['address']!='') {?>
			localsearch('全国');
		<?php } elseif ($_smarty_tpl->tpl_vars['row']->value['x']!=''&&$_smarty_tpl->tpl_vars['row']->value['y']!='') {?>
			setLocation('map_container',<?php echo $_smarty_tpl->tpl_vars['row']->value['x'];?>
,<?php echo $_smarty_tpl->tpl_vars['row']->value['y'];?>
,"map_x","map_y");
		<?php } else { ?>
			//根据IP到城市开始
			AMap.plugin('AMap.CitySearch', function () {
				var citySearch = new AMap.CitySearch();
				citySearch.getLocalCity(function (status, result){
					map.setCity(result.city);
				})
			});
			//根据IP到城市结结束
		<?php }?>

		$('.right_box').click(function(){
            $("#poiSearch").hide();
        })
	});
	var timeout = null;
	function debounce(func, wait = 500) {
		// 清除定时器
		if (timeout !== null) clearTimeout(timeout);
		timeout = setTimeout(function() {
			typeof func === 'function' && func();
		}, wait);
	}
	function addressKeyup(){
		debounce(localsearch, 1000);
	}
	function clickSearch(){
		var address = $("#address").val().replace(/\s+/g,"");
		if(address=="" || address=="建议格式：XX市XX县XX路"){
			layer.msg('请输入公司地址！', 2, 8);return false;
		}
		localsearch();
	}
	function localsearch(city){
		var address = $("#address").val().replace(/\s+/g,"");
		map.clearMap();
		if(address.length > 3){
			$.post('index.php?m=ajax&c=poi', {address: address}, function(e){
				if(e.status == '1' && e.tips.length > 0){
					var html = '';
					html +=  '<ul>';
					for(var i in e.tips){
						html +=  '<li data-id="' + i + '">';
						html += 	  '<div class="comEleaseMapTite">';
						html +=			  '<span>名称:</span>';
						html +=  		  '<b>' + e.tips[i].name + '</b>';
						html +=  	  '</div>';
						html +=  	  '<div class="comEleaseMapTipst">';
						html +=  		   '<span>地址:' + e.tips[i].address + '</span>';
						html +=  	   '</div>';
						html +=  '</li>';
					}
					html +=  '</ul>';
					
					$("#poiSearch").html(html);
					$("#poiSearch").show();
					
					setTimeout(function(){
						map.clearMap();
						$("#poiSearch li").on('click', function(){
							var id = $(this).attr('data-id');
							var location = e.tips[id].location;
							var c = location.split(',');
							document.getElementById("map_x").value = c[0];
							document.getElementById("map_y").value = c[1];
							// 设置marker
							var lngLat = new AMap.LngLat(c[0],c[1]);
							map.setZoomAndCenter(17,lngLat);
							var marker = new AMap.Marker({
								position: lngLat
							});
							map.add(marker);
							// 地图监听点击事件
							map.on("click",function(e){
								var lngLat = e.lnglat;
								document.getElementById("map_x").value = lngLat.lng;
								document.getElementById("map_y").value = lngLat.lat;
								map.clearMap();
								var marker = new AMap.Marker({
									position: new AMap.LngLat(lngLat.lng, lngLat.lat)
								});
								map.add(marker);
							});
							
							$("#poiSearch").html('');
							$("#poiSearch").hide();
						})
					},200);
				}
			}, 'json');
		}

	}
	function setLocation(id,x,y,xid,yid){
		var data=get_map_config();
		var config=eval('('+data+')');
		var rating,map_control_type,map_control_anchor;
		if(!x && !y){x=config.map_x;y=config.map_y;}
		map.setZoomAndCenter(17,[x,y]);
		var marker = new AMap.Marker({
			position: new AMap.LngLat(x, y)
		});
		map.add(marker);
		map.on("click",function(e){
			var lngLat = e.lnglat;
			document.getElementById(xid).value=lngLat.lng;
			document.getElementById(yid).value=lngLat.lat;
			map.clearMap();
			var marker = new AMap.Marker({
				position: new AMap.LngLat(lngLat.lng, lngLat.lat)
			});
			map.add(marker);
		});
	}
<?php echo '</script'; ?>
>
<?php }?>
<?php if ($_smarty_tpl->tpl_vars['isremind']->value==1) {?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['temstyle']->value)."/member/public/remind.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
