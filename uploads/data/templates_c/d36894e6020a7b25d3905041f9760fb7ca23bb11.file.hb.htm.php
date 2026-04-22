<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:19:32
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/default/public_search/hb.htm" */ ?>
<?php /*%%SmartyHeaderCode:41653303169e8a0b40767b3-37084536%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'd36894e6020a7b25d3905041f9760fb7ca23bb11' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/default/public_search/hb.htm',
      1 => 1700725931,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '41653303169e8a0b40767b3-37084536',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'jobs' => 0,
    'v' => 0,
    'comHb' => 0,
    'jobCnt' => 0,
    'hbids' => 0,
    'hbNum' => 0,
    'hb_uid' => 0,
    'Info' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0b407a4b9_87368522',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0b407a4b9_87368522')) {function content_69e8a0b407a4b9_87368522($_smarty_tpl) {?><style>
	
	.hb_tc_bth{padding:10px 0; text-align: center;backgroud-color:#fff !important;}
	.hb_tc_bth .hb_tc_hyz{width:120px;height:40px; line-height: 40px; text-align: center; display: inline-block; background-color: #3478ea;color:#fff;border-radius:3px;color:#fff;}
	.hb_tc_xz{width:120px;height:40px; line-height: 40px; text-align: center; display: inline-block; margin-left: 10px; background-color: #01af67;color:#fff;border-radius:3px;color:#fff;}
	.hb_tc_bth a:hover{color:#fff}
	
	.hb_close_box{padding:0px 0 0 25px}
	.hb_close_box_tit{color:#333; font-weight: bold;}
	.hb_close_job{width:100%;max-height: 130px; overflow: auto;min-height: 45px;}
	.hb_close_box_job{width:168px;height:30px; line-height: 30px; padding-left:30px; position: relative;    overflow: hidden;text-overflow: ellipsis;white-space: nowrap;  display: inline-block; margin-right: 10px; margin-top:8px;border-radius:4px;}
	.hb_close_box_job input{ position: absolute;left:10px;top:8px;}
	.poster_pic{width:100%;}
	.poster_pic img{width:100%;border-radius:3px;box-shadow: 0px 5px 10px 0px rgba(111, 116, 132, 0.1);}
	.hb_listbox{ display:block; }
	.hb_listbox_name{ font-size:15px;width:100%; text-align:center; padding-top:10px;}
	.hb_listbox_sc{ display:inline-block;padding:3px 20px; background:#3d7dfd;color:#fff;border-radius:20px; cursor: pointer;}
	.hbCloseBoxAll{overflow: hidden;padding: 0 12px 12px 12px;}
	.swiperPhpup{overflow: hidden;min-height: 200px;position: relative;padding: 12px 0 20px 0;}
	
	#hbSwiper .swiper-container {width: 100%;height: 100%;}
	#hbSwiper .swiper-slide {text-align: center;font-size: 18px;background: #fff;display: -webkit-box;display: -ms-flexbox;display: -webkit-flex;display: flex;-webkit-box-pack: center;-ms-flex-pack: center;-webkit-justify-content: center;justify-content: center;-webkit-box-align: center;-ms-flex-align: center;-webkit-align-items: center;align-items: center;cursor: pointer;}

.swiperContBoxs{
    overflow: hidden;
}
.swiperContBoxs .swiper-button-next{
    background: rgba(0, 0, 0, 0.6);
    background-size: 100%;
    width: 45px;
    height: 45px;
    border-radius: 50%;
    overflow: hidden;
}
.swiperContBoxs .swiper-button-next:after{
    font-size: 20px;
    color: #fff;
}
.swiperContBoxs .swiper-button-prev{
    background: rgba(0, 0, 0, 0.6);
    width: 45px;
    height: 45px;
    border-radius: 50%;
    overflow: hidden;
}
.swiperContBoxs .swiper-button-prev:after{
    font-size: 20px;
    color: #fff;
}
</style>
<link href="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/swiper/swiper-bundle.min.css?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
" rel="stylesheet" type="text/css"/>
<?php echo '<script'; ?>
 src="<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
/js/swiper/swiper-bundle.min.js?v=<?php echo $_smarty_tpl->tpl_vars['config']->value['cachecode'];?>
"><?php echo '</script'; ?>
>
<!--选择海报提示框-->
<div id="hb_tip" class="hb_tip" style="display: none;">
    <!--选择职位-->
    <div class="hb_close_box hbCloseBoxAll">
        <div class="hb_close_box_tit">1. 选择海报上展示的职位</div>
        <div id="jobs_container" class="hb_close_job" >
            <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['jobs']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
            <label>
                <span class="hb_close_box_job">
                    <input type="checkbox" class="jobChecked" name="jobIds[]" value="<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
" <?php if (!isset($_smarty_tpl->tpl_vars['jobCnt'])) $_smarty_tpl->tpl_vars['jobCnt'] = new Smarty_Variable(null);if ($_smarty_tpl->tpl_vars['jobCnt']->value = 1) {?>checked<?php }?>> <?php echo $_smarty_tpl->tpl_vars['v']->value['name'];?>

                </span>
            </label>
            <?php } ?>
        </div>
        <div class="hb_close_box_tit mt10">2. 在喜欢的图片下方点击生成海报</div>
        <div class="hb_close_img">
            <div class="swiperPhpup">
                <div id="hbSwiper" class="swiper-container swiperContBoxs">
                    <div class="swiper-wrapper">
                        <?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['comHb']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>
                        <div onclick="showHbJob()" class="swiper-slide">
                            <span class="hb_listbox">
                                <div class="poster_pic"><img src="<?php echo $_smarty_tpl->tpl_vars['v']->value['pic_n'];?>
"/></div>
                                <div class="hb_listbox_name">
                                    <span class="hb_listbox_sc" onclick="createHb('<?php echo $_smarty_tpl->tpl_vars['v']->value['style'];?>
','<?php echo $_smarty_tpl->tpl_vars['v']->value['id'];?>
')">生成海报</span>
                                </div>
                            </span>
                        </div>
                        <?php } ?>
                    </div>
                    <div class="swiper-button-next"></div>
                    <div class="swiper-button-prev"></div>
                </div>
            </div>
        </div>
    </div>
</div>
<?php echo '<script'; ?>
>
    var weburl = '<?php echo $_smarty_tpl->tpl_vars['config']->value['sy_weburl'];?>
';
    var jobCnt = '<?php echo $_smarty_tpl->tpl_vars['jobCnt']->value;?>
';
    var hbids = [];
    '<?php  $_smarty_tpl->tpl_vars['v'] = new Smarty_Variable; $_smarty_tpl->tpl_vars['v']->_loop = false;
 $_from = $_smarty_tpl->tpl_vars['hbids']->value; if (!is_array($_from) && !is_object($_from)) { settype($_from, 'array');}
foreach ($_from as $_smarty_tpl->tpl_vars['v']->key => $_smarty_tpl->tpl_vars['v']->value) {
$_smarty_tpl->tpl_vars['v']->_loop = true;
?>'
    hbids.push('<?php echo $_smarty_tpl->tpl_vars['v']->value;?>
')
    '<?php } ?>'
    var hbJobId = '';
    var isJobDtl = 0;
    function getComHb(hb) {
        layer.closeAll();

        var hbNum = '<?php echo $_smarty_tpl->tpl_vars['hbNum']->value;?>
';
        var id = '<?php echo $_smarty_tpl->tpl_vars['hb_uid']->value;?>
';
        var url = weburl + '/index.php?m=ajax&c=getComHb&uid=' + id + '&hb=' + hbids[hb];
        if (hb < (parseInt(hbNum) - 1)) {
            var next = hb + 1;
        } else {
            var next = 0;
        }

        var loading = layer.load('生成中...', 0);

        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.closeAll();

            layer.open({
                type: 1,
                title: false,
                content: '<div class="hb_tc"><img src="' + image.src + '" style="max-width: 100%;"><div class="hb_tc_bth"><a href="javascript:;" onclick="getComHb(' + next + ');" class="hb_tc_hyz">换一张</a><a href="javascript:;" onclick="downWhb(' + hb + ');" class="hb_tc_xz">下载海报</a></div></div>',
                area: ['360px', 'auto'],
                offset: '55px',
                closeBtn: 0,
                shadeClose: true
            });
        };
    }
    // 生成指定职位的为海报
    function getComHbNew(style, hbJobId, hbid) {
        // layer.closeAll();
        var url = '';
        if (isJobDtl) {
            // const url 	= 	weburl+'/index.php?m=ajax&c=getJobHb&id=' + id + '&hb=' + hbids[hb];
            url = weburl + '/index.php?m=ajax&c=getJobHb&id=' + hbJobId + '&hb=' + hbid;
        } else {
            var id = '<?php echo $_smarty_tpl->tpl_vars['hb_uid']->value;?>
';
            url = weburl + '/index.php?m=ajax&c=getComHb&uid=' + id + '&style=' + style +'&jobids=' + hbJobId;
        }
        var loading = layer.load('生成中...', 0);
        localStorage.setItem('jobids', hbJobId)
        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.close(loading);
            layer.open({
                type: 1,
                title: false,
                content: '<div class="hb_tc"><img src="' + image.src + '" style="max-width: 100%;"><div class="hb_tc_bth"><a href="javascript:;" data-jobs="'+ hbJobId +'" onclick="downWhbNew(' + style + ',' + hbid +');" class="hb_tc_xz">下载海报</a></div></div>',
                area: ['360px', 'auto'],
                offset: '55px',
                closeBtn: 0,
                shadeClose: true
            });
        };
    }

    function downWhb(hb) {
    	var loading = layer.load('下载中...', 0);
    	var id = '<?php echo $_smarty_tpl->tpl_vars['hb_uid']->value;?>
';
    	var url   =   weburl + '/index.php?m=ajax&c=getComHb&uid=' + id + '&hb=' +  + hbids[hb];
        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.closeAll();
            var a = document.createElement('a');          // 创建一个a节点插入的document
            var event = new MouseEvent('click')           // 模拟鼠标click点击事件
            a.download = 'whb' + id + '_' +hbids[hb];     // 设置a节点的download属性值
            a.href = url;                                 // 将图片的src赋值给a节点的href
            a.dispatchEvent(event);
        }
    }
    // 下载指定职位的为海报
    function downWhbNew(style, hbid) {
        hbJobId = localStorage.getItem('jobids')
        var loading = layer.load('下载中...', 0);
        var url = '';
        if (isJobDtl) {
            url = weburl + '/index.php?m=ajax&c=getJobHb&id=' + hbJobId + '&hb=' + hbid;
        } else {
            var id = '<?php echo $_smarty_tpl->tpl_vars['hb_uid']->value;?>
';
            url = weburl + '/index.php?m=ajax&c=getComHb&uid=' + id + '&style=' + style +'&jobids=' + hbJobId;
        }
        var image = new Image();
        image.src = url;
        image.onload = function() {
            layer.closeAll();
            var a = document.createElement('a');          // 创建一个a节点插入的document
            var event = new MouseEvent('click')           // 模拟鼠标click点击事件
            a.download = 'whb' + style + '_' + hbJobId.split(',').join('_');     // 设置a节点的download属性值
            a.href = url;                                 // 将图片的src赋值给a节点的href
            a.dispatchEvent(event);
        }
    }
    function getJobHb(hb) {

		layer.closeAll();

		const hbNum	=	'<?php echo $_smarty_tpl->tpl_vars['hbNum']->value;?>
';
		const id 	= 	'<?php echo $_smarty_tpl->tpl_vars['Info']->value['id'];?>
';

		const url 	= 	weburl+'/index.php?m=ajax&c=getJobHb&id=' + id + '&hb=' + hbids[hb];

		if (hb < (parseInt(hbNum)-1)){
			var  next  =   hb+1;
		}else{
			var  next  =   0;
		}

		const loading = layer.load('生成中...', 0);

		var image = new Image();
		image.src = url;
		image.onload = function() {
			layer.closeAll();
			layer.open({
				type: 1,
				title: false,
				content: '<div class="hb_tc"><img src="'+url+'" style="max-width: 100%;"><div class="hb_tc_bth"><a href="javascript:;" onclick="getJobHb('+next+');" class="hb_tc_hyz">换一张</a></div></div>',
				area: ['360px', 'auto'],
				offset: '55px',
				closeBtn: 0,
				shadeClose: true
			});
		}
	}

    // 选择海报模板页面
    function selectHb(job_cnt, job_dtl = 0, job_id = 0, job_name = ''){
        if (job_dtl) {
            isJobDtl = 1;
        }
        if (job_id > 0 && job_name) {
            var h5 = '<label><span class="hb_close_box_job"><input type="checkbox" class="jobChecked" name="jobIds[]" value="' + job_id +'" checked>'+job_name+'</span></label>';
            $('#jobs_container').html(h5);
        }
        if (job_cnt == 0) {
            layer.msg('请先发布职位', 2, 8, function () {
                window.location.href = 'index.php?c=jobadd';
            });
            return false;
        }
		var jheight = '500px';
		if(Math.ceil(job_cnt/4) >= 3){
			jheight = '600px';
		}else if(Math.ceil(job_cnt/4) == 2){
			jheight = '550px';
		}
        var selHbLayer = layer.open({
            type: 1,
            title: '生成招聘海报',
            closeBtn: 1,
            area: ['900px', jheight],
            content: $("#hb_tip"),
			success: function(){
				var swiper = new Swiper('#hbSwiper', {
				    slidesPerView: 5,
				    spaceBetween: 20,
                    navigation: {
                      nextEl: ".swiper-button-next",
                      prevEl: ".swiper-button-prev",
                    },
				});
			}
        });
        // 只有一个职位时，默认选中;职位数大于1时，所有职位全部初始化为未选中状态
        if (job_cnt > 1) {
            hbJobId = '';
            $("input[class=jobChecked]").attr('checked', false);
        }
    }

    // 显示职位复选框
    function showHbJob() {
        $("#hbjob_tip").attr("style", "display:block;");
    }
    // 点击生成海报按钮
    function createHb(style,hbid) {
        hbJobId = ""
        $.each($("input[class=jobChecked]:checked"),function(i){
            if (hbJobId == "") {
                hbJobId = $(this).val();
            } else {
                hbJobId += "," + $(this).val();
            }
        })
        if (hbJobId == '') {
            layer.msg('请选择海报职位', 2, 8);
            return false;
        }
        getComHbNew(style, hbJobId, hbid)
    }
<?php echo '</script'; ?>
><?php }} ?>
