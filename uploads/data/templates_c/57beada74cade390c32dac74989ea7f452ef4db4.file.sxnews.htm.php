<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:03:24
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/sxnews.htm" */ ?>
<?php /*%%SmartyHeaderCode:80815255969e89cecd05309-06684312%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    '57beada74cade390c32dac74989ea7f452ef4db4' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/wap/member/user/sxnews.htm',
      1 => 1700725935,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '80815255969e89cecd05309-06684312',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'wap_style' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e89cecd1d6e4_54739089',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e89cecd1d6e4_54739089')) {function content_69e89cecd1d6e4_54739089($_smarty_tpl) {?><?php if (!is_callable('smarty_function_url')) include '/www/wwwroot/zzzz.com/uploads/app/include/libs/plugins/function.url.php';
?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['wapstyle']->value)."/member/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
 
<div id="yunvue" class="none">
<div class="m_cardbox">
	<div class="m_cardbg" v-for="(item, lkey) in list" :key="lkey">
	
		
		<div class="sx_new_tit">管理员</div>
		<div class="sx_new_cont">
		
				
							<span v-for="(content, ckey) in item.content_arr" :key="ckey">
								<span v-if="content.url">
									<a href="javascript:void(0)" @click="pageTo(content.url)" class="sys_a">{{content.n}}</a>
								</span>
								<span v-else>
									{{content.n}}
								</span>
							</span>
						
		
		
		</div><div class="sx_new_bot">
	<div class="sx_new_data">{{item.ctime_n}}	</div>
		 <div class="sx_new_icon" @click="delsx(item.id,lkey)">
			<img src="<?php echo $_smarty_tpl->tpl_vars['wap_style']->value;?>
/images/resume_del.png" class="    " />
			
		</div></div>
	</div>
		
	 <div v-if="count==2">
    <van-pagination v-model="page" :total-items="total" :items-per-page="limit" force-ellipses @change="pageChange" /> 
    </div>
     <div class="wap_member_no" v-show="count==0">
        暂无记录！
      </div>
</div>
</div>

<?php echo '<script'; ?>
>  
  var currentPage = parseInt('<?php echo $_GET['page'];?>
');       
      new Vue({
        el: '#yunvue',
        data: {
          list: [],
          limit:20,              
          total:0,
          page: currentPage ? currentPage : 1,          
        },
        computed: {
          count(){
            if(this.total > this.limit){
              return 2;
            }else if(this.list.length==0){
              return 0;
            }else{
                return 1;
            }
          }
        },
        created() {   
         this.getsixin();
        },
        methods:{          
          getsixin:function(){
            showLoading();
            var that = this;
            var paramer = {};
                paramer['page'] = that.page;
                paramer['type'] = 'wap';
                paramer['limit'] = that.limit;   
            $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'msg','c'=>'sxnews'),$_smarty_tpl);?>
',paramer,function(res){
                hideLoading();      
                that.list = res.data;
                that.total = res.total;
                that.page = currentPage ? currentPage : 1;
              
               $("#yunvue").css('display', 'block');
            },'json');            
          },
          delsx:function(id,key){
            var that = this;
            var paramer = {
                id: id,                
            };
            showConfirm("确定删除？",function(){
                showLoading('删除中...');
                $.post('<?php echo smarty_function_url(array('d'=>'wxapp','h'=>'user','m'=>'msg','c'=>'delsxnews'),$_smarty_tpl);?>
',paramer,function(data){
                    hideLoading();      
                    
                    if (data.error == 1) {
                        showToast('删除成功',2, function() {
                            that.list.splice(key, 1);
                            
                        });
                    } else {
                       showToast('删除失败');
                    }
                },'json');  
            })
          },          
          pageChange:function(e){
          
            location.href = 'index.php?c=sxnews&chat=1&page='+e;
          },
          pageTo:function(url){
              window.location.href = url;
          }
        } 
      });
<?php echo '</script'; ?>
>
</body>
</html><?php }} ?>
