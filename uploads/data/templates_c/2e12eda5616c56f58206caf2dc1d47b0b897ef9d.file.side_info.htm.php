<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:01
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/side_info.htm" */ ?>
<?php /*%%SmartyHeaderCode:35729500169e8a0d1055fa5-26552332%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '2e12eda5616c56f58206caf2dc1d47b0b897ef9d' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/side_info.htm',
      1 => 1700725931,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '35729500169e8a0d1055fa5-26552332',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'user_style' => 0,
    'row' => 0,
    'tv' => 0,
    'comdata' => 0,
    'v' => 0,
    'comclass_name' => 0,
    'addjobnum' => 0,
    'uid' => 0,
    'isremind' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d1077d74_27896367',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d1077d74_27896367')) {function content_69e8a0d1077d74_27896367($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
 
<style>
	.laydate-time-list{padding-bottom:0;overflow:hidden}
	.laydate-time-list>li{width:50%!important;}
	.laydate-time-list>li:last-child { display: none;}
	.none{display: none !important;}
</style>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/binding.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
> 
<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?> 
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['mapurl'];?>
"><?php echo '</script'; ?>
>
<?php }?>
<?php echo '<script'; ?>
>
	var userstyle	=	'<?php echo $_smarty_tpl->tpl_vars['user_style']->value;?>
';
	var setPosition	=	'<?php echo $_smarty_tpl->tpl_vars['config']->value['com_enforce_setposition'];?>
';
  	
	function checkpostcom(){
	
	    var chk_value 		= 	[];
	    var shortname 		= 	$.trim($("#shortname").val());
	    var sdate 			= 	$.trim($("#sdate").val());
	    var moneytype 		= 	$.trim($("#moneytype").val());
	    var money 			= 	$.trim($("#money").val());
	    var notDisturb 		= 	$.trim($("#notDisturb").val());
	    
	    var linkqq			=	$.trim($("#linkqq").val());
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
	   	
	    $.post("index.php?c=info&act=saveside",{shortname:shortname,sdate:sdate,moneytype:moneytype,money:money,website:website,linkqq:linkqq,website:website,busstops:busstops,infostatus:infostatus,welfare:welfare,notDisturb:notDisturb},function(data){
	 		
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
					<li><a href="index.php?c=info">基本信息</a></li>
					<li class="newmember_titcur"><a href="index.php?c=info&act=side">补充信息</a></li>
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
			 
		        <iframe id="supportiframe"  name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
		        <form id="infoform" name="myform" method="post" target="supportiframe"  autocomplete="off" enctype="multipart/form-data" class="layui-form">
				
					
					<input type="hidden" id="layupload_type" value="2"/>
					<input type="hidden" id="laynoupload" value="1"/>
					
					<div class="clear"></div>
		
		     		<div class="com_release_box" id="cominfoblack" >        
				   	 
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
											<option value="1" <?php if ($_smarty_tpl->tpl_vars['row']->value['infostatus']==1) {?> selected<?php }?>>公开</option>
											<option value="2" <?php if ($_smarty_tpl->tpl_vars['row']->value['infostatus']==2) {?> selected<?php }?>>不公开</option>
										</select>
									</div>   
								</div>
							</li>
							<li class="disturb" <?php if ($_smarty_tpl->tpl_vars['row']->value['infostatus']!='1') {?>style="display:none;"<?php }?>>
								<div class="com_release_name" > 免打扰时间</div>
								<input type="text" name="notDisturb" id="notDisturb" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['not_disturb'];?>
" class="com_info_text">
								<em class="com_release_box_h1_z">联系方式设置公开，在免打扰时间段内求职者无法查看企业联系方式~</em>
							</li>
							<li>
								<div class="com_release_name"> 公司二维码</div>
								<div class="comReleaErweim">
									<button type="button" class="yun_bth_pic adminupload" lay-data="{imgid: 'ewm',parentid: 'imgparent',path: 'company','uid': '<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
', 'usertype': 2}">上传二维码</button>
				    				<div class="com_release_nameewm_img <?php if (!$_smarty_tpl->tpl_vars['row']->value['comqcode']) {?>none<?php }?>" id="imgparent"> 
										<img id="ewm" src="<?php echo $_smarty_tpl->tpl_vars['row']->value['comqcode'];?>
" width="40" height="40">										
										<a href="javascript:void(0)" onclick="delcomqcode('<?php echo $_smarty_tpl->tpl_vars['row']->value['uid'];?>
')">清除</a>										
									</div>
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
										<div class="">
										<div class="addwelfare_b">
											<input class="addwelfare_text" type="text" tabindex="1000" id="addwelfare" onblur="checkWelfare('addwelfare');" ><a class="addwelfarebox">添加福利</a>    
										</div>
									</div></div>
								</div>
							</li>
							<li>
								<input class="btn_01" style="_margin-left:-3px" type="button" onclick="checkpostcom()" value="保存信息">
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

<?php echo '<script'; ?>
>
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
		var welfare = $.trim($('#'+id).val());
		if(welfare){
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
		
	}
	//ajax提交表单
	$(function () {
		//添加福利
		$('.addwelfarebox').click(function(){
	
			var welfare = $.trim($('#addwelfare').val());
			if(welfare.length>=2 && welfare.length<=8){
				$('#addwelfarelist').append('<input name="welfare[]" value="'+welfare+'" checked="checked"  type="checkbox" title="'+welfare+'" data-tag="'+welfare+'" class="changewelfare" lay-skin="primary">');
			
				layui.use(['layer', 'form'], function(){
					var layer = layui.layer
						,form = layui.form
						,$ = layui.$;
					form.render('checkbox');
				});

				$('#addwelfare').val('');
			}else{
				layer.msg('请输入2-8个福利字符！', 2,8);
				$('#'+id).val('');
				return false;
			}
			
			 
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
	function delcomqcode(id){
		layer.confirm('确定要清除企业二维码！',function(){
			$.post('index.php?c=info&act=delcomqcode',{cuid:id},function(data){
				layer.closeAll();
				layer.msg('清除成功！',2,9,function(){
					$("#imgparent").addClass('none');
				})
			});
		})
	}
	var aaa = new Date();
	layui.use(['form','layer', 'laydate','upload'], function(){
		var $ = layui.$,
			form = layui.form,
			laydate = layui.laydate,
			upload = layui.upload,
			layer = layui.layer;
		
      	laydate.render({
        	elem: '#sdate'
        	,type: 'year'
			,max: ''+aaa.getFullYear(),
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
      	laydate.render({
			elem: '#notDisturb'
			,type: 'time'
			,format:'HH:mm'
			,range: true,
		});
		form.on('select(moneytype)', function(data){
	   		if(data.value == 1){
	        	$("#money_1").show();
	        	$("#money_2").hide();
	      	}else{
	        	$("#money_2").show();
	        	$("#money_1").hide();
	      	}
	    });
	    form.on('select(infostatus)', function (data) {
			if (data.value == 1){
				$(".disturb").show();
			} else {
				$(".disturb").hide();
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
<?php if ($_smarty_tpl->tpl_vars['isremind']->value==1) {?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['temstyle']->value)."/member/public/remind.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }?>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
