<?php /* Smarty version Smarty-3.1.21-dev, created on 2026-04-22 18:20:02
         compiled from "/www/wwwroot/zzzz.com/uploads/app/template/member/com/map.htm" */ ?>
<?php /*%%SmartyHeaderCode:146807863769e8a0d2a2e507-87457017%%*/if(!defined('SMARTY_DIR')) exit('no direct access allowed');
$_valid = $_smarty_tpl->decodeProperties(array (
  'file_dependency' => 
  array (
    'd48050fb2541ea35f4b49f9f95c8f7706dcac05b' => 
    array (
      0 => '/www/wwwroot/zzzz.com/uploads/app/template/member/com/map.htm',
      1 => 1702031896,
      2 => 'file',
    ),
  ),
  'nocache_hash' => '146807863769e8a0d2a2e507-87457017',
  'function' => 
  array (
  ),
  'variables' => 
  array (
    'config' => 0,
    'row' => 0,
  ),
  'has_nocache_code' => false,
  'version' => 'Smarty-3.1.21-dev',
  'unifunc' => 'content_69e8a0d2a43d43_32718593',
),false); /*/%%SmartyHeaderCode%%*/?>
<?php if ($_valid && !is_callable('content_69e8a0d2a43d43_32718593')) {function content_69e8a0d2a43d43_32718593($_smarty_tpl) {?><?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/header.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

<div class="w1000">
  <div class="admin_mainbody">
  <?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/left.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>

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
	<?php }?> 
	 
	<div class=right_box>
		<div class="newmember_tit">
            <ul>
                <li><a href="index.php?c=info">基本信息</a></li>
                <li><a href="index.php?c=info&act=side">补充信息</a></li>
                <li><a href="index.php?c=address">地址管理</a></li>
                <li><a href="index.php?c=show"> 公司相册</a></li>
                <li><a href="index.php?c=uppic"> 公司LOGO</a></li>
                <li><a href="index.php?c=product">产品介绍</a></li>
                <li><a href="index.php?c=news">公司资讯</a></li>
                <?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
                <li class="newmember_titcur"><a href="index.php?c=map">公司地图</a></li>
                <?php }?>
                <li><a href="index.php?c=comtpl">个性化模板</a></li>

            </ul>
		</div>
		
		
	   <div class="clear"></div>
	              <div class=admincont_box>
	   
	                
	   
	                  <div class="com_body">
	   	 
								<div class="mapbox">
     <div class="com_new_tip mt20"> 
          <span class="com_new_tip_h">温馨小提示</span>搜索框内输入地址搜索，在地图区域点击公司所在的位置，点击保存地图</div>
      
          
    
     <div class="joblist_search">
		
			<div class="joblist_mapsearch_box">
		
            <input id="map_keyword" type="text" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['address'];?>
" class="joblist_mapsearch_text" placeholder="建议格式：XX市XX县XX路" onkeyup="addressKeyup();" /><input type="button" value="精准查找" onclick="localsearch();" class="joblist_mapsearch_bth"/>
                <div class="comEleaseMaps" id="poiSearch" style="display: none;"></div>
            </div>
		
	</div>
    
    <div class="clear"></div>
        <div class=admin_note_map style="position:relative; margin-top:20px;">
		<iframe id="supportiframe"  name="supportiframe" onload="returnmessage('supportiframe');" style="display:none"></iframe>
        <form name="myform" onSubmit="return checkpost();" target="supportiframe" action="index.php?c=map&act=save" method="post">
          <table width="100%" border="0" cellspacing="0" cellpadding="0" style="font-size:12px;">
              <tr>
                <th height="30"></th>
                <td><div id="map_container" style="width:100%;height:350px;"></div><br>
                </td>
              </tr>
              <tr>
                <th>&nbsp;</th>
                <td height="40">
                <span class="com_info_text_s"> X轴：</span>
                  <input name="xvalue" id="map_x" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" readonly="readonly" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['x'];?>
"  class="com_info_text">
                 <span class="com_info_text_s"> Y轴：</span>
                  <input name="yvalue" id="map_y" onkeyup="this.value=this.value.replace(/[^0-9.]/g,'')" readonly="readonly" value="<?php echo $_smarty_tpl->tpl_vars['row']->value['y'];?>
"  class="com_info_text">
                
                  <input type="submit" name="savemap" class="com_info_mapbth" value="保存地图" style="">
                  <span id="by_map" class="errordisplay">请先设置地图位置</span></td>
              </tr>
            
          </table>
		</form>
          
        </div>
      </div>
    </div>
    <?php echo '<script'; ?>
>
		<?php if ($_smarty_tpl->tpl_vars['config']->value['map_key']) {?>
		var map = new AMap.Map("map_container");
		$(document).ready(function() {
			<?php if (($_smarty_tpl->tpl_vars['row']->value['x']==''||$_smarty_tpl->tpl_vars['row']->value['y']=='')&&$_smarty_tpl->tpl_vars['row']->value['address']!='') {?>
				localsearch();
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
		<?php }?>
		
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
		function localsearch(city){
            var address = $("#map_keyword").val().replace(/\s+/g,"");
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
		function checkpost(){
			if($.trim($("#map_x").val())==''||$.trim($("#map_y").val())==''){
				layer.msg('请先设置地图位置！', 2, 8);return false;
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
  </div>
</div>
</div>
<?php echo $_smarty_tpl->getSubTemplate (((string)$_smarty_tpl->tpl_vars['comstyle']->value)."/footer.htm", $_smarty_tpl->cache_id, $_smarty_tpl->compile_id, 0, null, array(), 0);?>
<?php }} ?>
