<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:07
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/invite.htm" */ ?>
<?php /*%%SmartyHeaderCode:18875856269e8a0d79fc335-44800139%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'cd0d1f486a204b9ed605d41b22f9f50bbfe01b60' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/invite.htm',
      1 => 1700725933,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '18875856269e8a0d79fc335-44800139',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'now_url' => 0,
    'rows' => 0,
    'v' => 0,
    'statis' => 0,
    'pagenav' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d7a19ca4_04884484',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d7a19ca4_04884484')) {function content_69e8a0d7a19ca4_04884484($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
if (!is_callable('smarty_function_Url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
	<div class="admin_mainbody">
	 <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

	 <div class="clear"></div>
	 
	 <div class=right_box>
	 <div class="newmember_screenbox">
	 <div class="newmember_screen">	
	 <div class="com_topbth_box">
				<a href="index.php?c=yqmb" class="com_topbth">面试模板</a>
				<div class="com_topbth_zh" style="display: none;">
					<div class="com_topbth_zh_pd">面试邀请若求职者没有回复，可下载简历后，拨打电话进行联系。 您还可以添加邀请面试模板以便快捷操作。</div>
					<div class="">知道了</div>
				</div>
			</div>
	<div class="newmember_screenname">	面试管理</div>
	 <div class="joblist_search">
				<form action="index.php" method="get">
					<div class="joblist_search_box">
						<input name="c" type="hidden" value="<?php echo $_GET['c'];?>
">
						<input name="keyword" id="keyword" type="text" class="joblist_search_box_text" value="<?php echo $_GET['keyword'];?>
" placeholder="搜索简历 ">
						<input name="" type="submit" class="joblist_search_bth" value=" ">
					</div>
				</form>
			</div>
	 </div>
	 </div>
	  <div class="clear"></div>
	 
	 
			<div class=admincont_box>
				<div class="com_body">
					<div class="clear"></div>
					<div class=admin_textbox_04>
						<iframe id="supportiframe" name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
						<form action="<?php echo $_smarty_tpl->tpl_vars['now_url']->value;?>
&act=del" method="post" target="supportiframe" id='myform' class="layui-form">
							<div id="job_checkbokid">
								<table class="com_table">
									<?php if (!empty($_smarty_tpl->tpl_vars['rows']->value)) {?>
									<tr>
										<th style="border-radius:6px 0 0 6px ;">基本信息</th>
										<th>面试职位</th>
										<th>联系电话</th>
										<th>面试时间</th>
										<th>状态 </th>
										<th width="160" style="border-radius:0 6px 6px 0 ;">操作</th>
									</tr>
									<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['rows']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
									<tr>
										<td>
											<div class="newcom_user_info">
												<span class="newcom_user_infoheckb">
													<input type="checkbox" name="delid[]" value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" class="com_Release_job_qx_check" lay-skin="primary" />
												</span>
												<div class="newcom_user_pic"><img src="<?php echo $_smarty_tpl->tpl_vars['v']->value['photo'];?>
"></div>
												<div>
													<?php if (!$_smarty_tpl->tpl_vars['v']->value['eid']) {?>
													<font color="red">简历已删除</font>
													<?php } else { ?>
													<a href="<?php echo smarty_function_url(array('m'=>'resume','c'=>'show','id'=>$_smarty_tpl->tpl_vars['v']->value['eid']),$_smarty_tpl);?>
" target=_blank class="newcom_user_name" id='name<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
'><?php if ($_smarty_tpl->tpl_vars['v']->value['down']==1) {
echo $_smarty_tpl->tpl_vars['v']->value['realname'];
} else {
echo $_smarty_tpl->tpl_vars['v']->value['name'];
}?></a>
													<?php }?>
												</div>
												<div class="newcom_user_infop">
													<?php echo $_smarty_tpl->tpl_vars['v']->value['sex'];?>
 · <?php if ($_smarty_tpl->tpl_vars['v']->value['exp']) {
echo $_smarty_tpl->tpl_vars['v']->value['exp'];?>
经验 · <?php }
if ($_smarty_tpl->tpl_vars['v']->value['edu']) {
echo $_smarty_tpl->tpl_vars['v']->value['edu'];?>
学历 · <?php }
echo $_smarty_tpl->tpl_vars['v']->value['age'];?>
岁
												</div>
												<div>意向 <span style="color:#4a89e8"><?php echo $_smarty_tpl->tpl_vars['v']->value['jobclassname'];?>
</span></div>
											</div>
										</td>
										<td align="left"><?php echo $_smarty_tpl->tpl_vars['v']->value['jobname'];?>
</td>
										<td align="center">
											<?php if ($_smarty_tpl->tpl_vars['v']->value['down']==1) {?>
												<span class="com_received_tel"><?php echo $_smarty_tpl->tpl_vars['v']->value['telphone'];?>
</span>
											<?php } elseif ($_smarty_tpl->tpl_vars['statis']->value['down_resume']>0) {?>
												<a class="com_received_tellook" href="javascript:void(0);" onclick="isDownResume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo smarty_function_Url(array('m'=>'ajax','c'=>'for_link'),$_smarty_tpl);?>
', '<?php echo $_smarty_tpl->tpl_vars['statis']->value['down_resume'];?>
')">查看联系方式</a>
											<?php } else { ?>
												<a class="com_received_tellook" href="javascript:void(0);" onclick="downResume('<?php echo $_smarty_tpl->tpl_vars['v']->value['eid'];?>
','<?php echo smarty_function_Url(array('m'=>'ajax','c'=>'for_link'),$_smarty_tpl);?>
')">查看联系方式</a>
											<?php }?>
										</td>
										<td align="center"> <?php echo $_smarty_tpl->tpl_vars['v']->value['ms_time'];?>

										</td>
										<td align="center">
											<?php if ($_smarty_tpl->tpl_vars['v']->value['is_browse']=="1") {?>
											<span class="tip_no"><i class="tip_no_icon"></i>待接受</span>
											<?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=="2") {?>
											已查看
											<?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=="3") {?>
												<?php if ($_smarty_tpl->tpl_vars['v']->value['over']==1) {?>
												<span style="color:#ff2b00">已结束</span>
												<?php } else { ?>
												<span style="color:#008000">已接受</span>
												<?php }?>
											<?php } elseif ($_smarty_tpl->tpl_vars['v']->value['is_browse']=="4") {?>
											<?php if ($_smarty_tpl->tpl_vars['v']->value['remark']) {?>
											<span onclick="layer.alert('<?php echo $_smarty_tpl->tpl_vars['v']->value['remark'];?>
');" style=" display:block; padding-top:5px;color:#4a89e8; text-decoration:underline; cursor:pointer">查看原因</span>
											<?php }?>
											<font color="#FF00000">无法面试</font>
											<?php }?>
										</td>
										<td align="center">
											<a href="javascript:getcont('<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
');" class="com_bth cblue">邀请函</a>
											<a href="javascript:void(0)" onclick="layer_del('确定要删除？', '<?php echo $_smarty_tpl->tpl_vars['now_url']->value;?>
&act=del&id=<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
')" class="com_bth cblue">删除</a>
										</td>
									</tr>
									<?php } ?>
									<tr>
										<td colspan="7" class="">
											<div class="com_Release_job_bot">
												<span class="com_Release_job_qx">
													<input id='checkAll' type="checkbox" lay-filter="allcom" lay-skin="primary" class="com_Release_job_qx_check">全选
												</span>
												<input class="c_btn_02" type="button" name="subdel" value="批量删除" onclick="return really('delid[]');">
											</div>
											<div class="diggg"><?php echo $_smarty_tpl->tpl_vars['pagenav']->value;?>
</div>
										</td>
									</tr>
									<?php } elseif ($_GET['keyword']!='') {?>
									<tr>
										<td colspan="7" class="table_end">
											<div class="msg_no">
												<p>暂无符合的面试安排。</p>
											</div>
										</td>
									</tr>
									<?php } else { ?>
									<tr>
										<td colspan="7" class="table_end">
											<div class="msg_no">
												<p>暂无符合的面试安排</p>
											</div>
									</tr>
									<?php }?>
								</table>
							</div>
						</form>
					</div>
				</div>
			</div>
		</div>
		<!-- 邀请函弹出框 Start -->
		<div id="getcont" style="width:100%; display:none;">
			<div class="aud_face">
				<div class="audition_list"><span class="audition_list_span"></span></div>
				<div style="padding:10px;">
					<div class="invitation_user">尊敬的 <span class="invitation_user_name" id='manname'></span> 您好!</div>
					<div class="invitation_cont">&nbsp;&nbsp;&nbsp;&nbsp;经过人力资源部的初步筛选,我们认为您基本具备 <i class="invitation_cont_job" id="jobname"></i> 岗位的任职资格，因此正式邀请您来我公司参加面试。</div>
					<div class="invitation_cont_tip">具体详见如下：</div>
					<div class="invitation_cont_p"><span class="invitation_cont_pn">【面试时间】</span><em class="audition_list_e" id="intertime"></em></div>
					<div class="invitation_cont_p"><span class="invitation_cont_pn">【面试地址】</span><em class="audition_list_e" id="address"></em></div>
					<div class="invitation_cont_p nocontent"><span class="invitation_cont_pn">【面试备注】</span><em class="" id="content"></em></div>
					<div class="invitation_cont_p"><span class="invitation_cont_pn">【联系方式】</span><em class="audition_list_e" id="linkman"></em> ( TEL：<em class="invitation_cont_tel" id="linktel"></em> )</div>
				</div>
				<div class="invitation_cont_jy">
					<div class="invitation_cont_d">
						特此邀请&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div>
					<div class="invitation_cont_d"><em class="" id="comname"></em>&nbsp;&nbsp;&nbsp;&nbsp;</div>
					<div class="invitation_cont_d"><em class="" id="datetime"></em>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;</div>
				</div>
			</div>
		</div>
		<!-- 邀请函弹出框 End -->
		<?php echo '<script'; ?>
>
			layui.use(['form', 'layer', 'laydate'], function() {
				var $ = layui.$,
					form = layui.form,
					laydate = layui.laydate,
					layer = layui.layer;

				form.on('checkbox(allcom)', function(data) {
					$("input[name='delid[]']").each(function() {
						this.checked = data.elem.checked;
					});
					form.render('checkbox');
				});
			});

			function getcont(id) {
				$.post("index.php?c=invite&act=ajax", {
					id: id
				}, function(data) {
					var data = eval('(' + data + ')');
					$("#comname").html(data.comname);
					$("#jobname").html(data.jobname);
					$("#manname").html($('#name' + id).text());
					$("#linkman").html(data.linkman);
					$("#linktel").html(data.linktel);
					$("#intertime").html(data.intertime);
					$("#address").html(data.address);
					if (data.content) {
						$("#content").html(data.content);
					} else {
						$(".nocontent").attr("style", "display: none;");
					}

					$("#datetime").html(data.datetime);
					$.layer({
						type: 1,
						title: '面试邀请函',
						shade: [0],
						closeBtn: [0, true],
						border: [10, 0.3, '#000', true],
						area: ['600px', 'auto'],
						page: {
							dom: "#getcont"
						}
					});
				})
			}
		<?php echo '</script'; ?>
>
		<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<?php }} ?>
