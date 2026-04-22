<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:08
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/yqms.htm" */ ?>
<?php /*%%SmartyHeaderCode:8675637269e8a0d8784018-89886241%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '7d4286ee0ab4bfb26b6f3ff6a35ac8ea8f6cb2cd' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/yqms.htm',
      1 => 1700725932,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '8675637269e8a0d8784018-89886241',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'company_job' => 0,
    'v' => 0,
    'ymlist' => 0,
    'yv' => 0,
    'ymcan' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d87889f1_89986908',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d87889f1_89986908')) {function content_69e8a0d87889f1_89986908($_smarty_tpl) {?><div id='job_box' style="display:none;float:left">
	<div class="com_resume_yqbox" style="width:500px">
		<dl style="z-index:1000">
			<dt>面试职位：</dt>
			<dd>
				<div class="Interview_text_box">
					<input type="button" value="请选择" class="Interview_text_box_t" id="name" onclick="search_show('job_name');">
					<input type="hidden" id="nameid" name="name" value="">
					<div class="Interview_text_box_list none" id="job_name" style="display: none;">
						<ul>
							<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_smarty_tpl->tpl_vars['j'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['company_job']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
 $_smarty_tpl->tpl_vars['j']->value = $_smarty_tpl->tpl_vars['v']->key;
?>  
								<li>
									<a href="javascript:;" onclick="selecteInviteJob('<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
', 'name', '<?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['link_man'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['link_moblie'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['address'];?>
');"><?php echo mb_substr($_smarty_tpl->tpl_vars['v']->value['name'],0,12,'utf-8');?>
</a>
								</li>    
							<?php } ?> 
						</ul>
					</div>
				</div>
			</dd>
		</dl>
		<?php if (!empty($_smarty_tpl->tpl_vars['ymlist']->value)) {?>
		<dl style="z-index:1000">
			<dt>选择面试模板：</dt>
			<dd>
				<div class="Interview_text_box">
					<input type="button" value="请选择" class="Interview_text_box_t" id="mbname" onclick="search_show('mb_name');">
					<input type="hidden" id="ymid" name="ymid" value='' />
					<div class="Interview_text_box_list none" id="mb_name" style="display: none;">
						<ul>
							<?php  $_smarty_tpl->tpl_vars['yv'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['yv']->_loop = false;
 $_smarty_tpl->tpl_vars['yk'] = new Smarty_Variable;
 $_from = $_smarty_tpl->tpl_vars['ymlist']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['yv']->key => $_smarty_tpl->tpl_vars['yv']->value) {
$_smarty_tpl->tpl_vars['yv']->_loop = true;
 $_smarty_tpl->tpl_vars['yk']->value = $_smarty_tpl->tpl_vars['yv']->key;
?>  
								<li>
									<a href="javascript:;" onclick="selecteYqmb('<?php echo $_smarty_tpl->tpl_vars['yv']->value['linkman'];?>
','<?php echo $_smarty_tpl->tpl_vars['yv']->value['linktel'];?>
', '<?php echo $_smarty_tpl->tpl_vars['yv']->value['address'];?>
','<?php echo $_smarty_tpl->tpl_vars['yv']->value['intertime'];?>
','<?php echo $_smarty_tpl->tpl_vars['yv']->value['content'];?>
','<?php echo $_smarty_tpl->tpl_vars['yv']->value['name'];?>
','<?php echo $_smarty_tpl->tpl_vars['yv']->value['id'];?>
');"><?php echo mb_substr($_smarty_tpl->tpl_vars['yv']->value['name'],0,12,'utf-8');?>
</a>
								</li>    
							<?php } ?> 
						</ul>
					</div>
				</div>
			</dd>
		</dl>
		<?php }?>
		<dl><dt>联系人：</dt><dd><input size='20'  id='linkman' value='' class="resume_yqbox_text"></dd></dl>
		<dl><dt>联系电话：</dt><dd><input size='20'  id='linktel' value='' class="resume_yqbox_text"></dd></dl>
		<dl><dt>面试时间：</dt><dd><input size='34' id='intertime' value='' class="resume_yqbox_text" autocomplete="off"></dd></dl>
		<dl><dt>面试地址：</dt><dd><input size='34' id='address' value='' class="resume_yqbox_text resume_yqbox_textadd"></dd></dl>
		<dl><dt>面试备注：</dt><dd> <textarea id="content" cols="40" rows="5" class="resume_yqbox_textarea"></textarea></dd></dl>
		
		<dl id="ymctrl" class=" <?php if (!$_smarty_tpl->tpl_vars['ymcan']->value) {?>none<?php }?>">
			<dt>&nbsp;</dt>
			<dd>
		        <form class="layui-form">
		            <input type="checkbox" id="save_yqmb" name="save_yqmb"  title="保存为面试模板" value="1" lay-filter="save_yqmb" lay-skin="primary" />
		        </form>
	    	</dd>
	    </dl>
	    
		<dl id="resume_job" style="height:50px;">
			<dt>&nbsp;</dt>
			<dd> 
				<input type="hidden" id="uid" value="">
				<input type="hidden" id="username" value="">
				<input class="resume_sub_yq" id="click_invite" type="button" value="邀请面试"  >
			</dd>
		</dl>
	</div>
</div>

<?php echo '<script'; ?>
>
	layui.use(['form','laydate'], function() {
		var $ = layui.$, form = layui.form;
	});
	var ymcan = '<?php echo $_smarty_tpl->tpl_vars['ymcan']->value;?>
';
	$('#intertime').datetimepicker({
		format:'Y-m-d H:i',
		step:10
	});
	$.datetimepicker.setLocale('zh');
	function selecteInviteJob(id,type,name,man,tel,address){

		$("#job_"+type).hide();
		$("#"+type).val(name);
		$("#"+type+"id").val(id);
		if(man && tel){
			$("#linkman").val(man);
			$("#linktel").val(tel);
			$("#address").val(address);
		}
		
	}
	function selecteYqmb(man,tel,address,intertime,content,name,id){

		$("#ymid").val(id);
        $("#linkman").val(man);
        $("#linktel").val(tel);
        $("#address").val(address);
        $("#content").val(content);
        $("#intertime").val(intertime);
        $('#mbname').val(name);

        $('#ymctrl').removeClass('none');
        $('#save_yqmb').attr('title','更新面试模板')

        $('#save_yqmb').prop("checked", false);
        
        layui.use(['form', 'layer'], function() {
            var $ = layui.$,
                form = layui.form,
                layer = layui.layer;

            form.render();
        });

        $("#mb_name").hide();
    }
<?php echo '</script'; ?>
>
<?php }} ?>
