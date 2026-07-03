/**
 * Make layer.min.js calls from before 2017-11-15 compatible with layui.use(['layer']).

 * This file must be loaded after layui/layui.js. Remove the old js/layer/layer.min.js include.
*/

/*
 * Make old layer.msg() calls compatible with layui.use(['layer']).
 *
 * msg: message content.
 * timeSecond: seconds; decimals are supported.
 * icon: 1 check, 2 cross, 5/9 sad face, 6/8 smile, 7 exclamation.
 * callback: called after the message closes.
*/
layui.use(['layer'], function(){
	var layer = layui.layer,
    $ = layui.$;

	// Save the original layer.msg() method.
	layer.oriMsg = layer.msg;

	// Then override layer.msg().
  // layer.msg = function (msg, timeSecond = 1.5, icon = 6, callback = function(){}){
  layer.msg = function (msg, timeSecond , icon , callback ){
    timeSecond = (typeof timeSecond !== 'undefined') ?  timeSecond : 1.5;
    icon = (typeof icon !== 'undefined') ?  icon : 6;
    callback = (typeof callback !== 'undefined') ?  callback : function(){};

	// Preserve the original layui.use(['layer']) layer.msg() call style.
  	if(typeof(timeSecond) == 'object'){
  		if(typeof(icon) == 'function'){
  			return layer.oriMsg(msg, timeSecond, icon);
  		}
  		else{
  			return layer.oriMsg(msg, timeSecond);
  		}
  	}

    var tm = timeSecond * 1000;

		// In layui.use(['layer']): icon 1 check, 2 cross, 5 sad face, 6 smile, 7 exclamation.
		// In layer.min.js: icon 8 failure, 9 success.
		if(icon == 8){
			icon = 5;
		}
		if(icon == 9){
			icon = 6;
		}

		return layer.oriMsg(msg,
			{
				time : tm,
				icon : icon,
                shade: [0.8, '#393D49'] // Add a black transparent shade layer.
			},
			function(){
				callback();
			}
		);
  };//end layer.msg

	// Add a shade layer to the loading animation.
	layer.oriLoad = layer.load;
	layer.load = function(icon,options)
	{
		icon = (typeof icon !== 'undefined') ? icon : 0;
		options = (typeof options == 'object') ? options : {};
		
		options.shade = [0.8, '#393D49'];
		return layer.oriLoad(icon, options);
	};

	// Alert dialog.
	layer.oriAlert = layer.alert;
	layer.alert = function(msg, icon, title, callback)
	{
		if(typeof icon == 'object'){
			// Original layui layer module call style.
			if(typeof title == 'function'){
				return layer.oriAlert(msg, icon, title);
			}
			else{
				return layer.oriAlert(msg, icon);
			}
		}else if(typeof icon != 'undefined' && typeof title == 'undefined'){
			// Compatible with layer.min.js call style.
			return layer.msg(msg, 1.5, icon);
		}else if(typeof callback == 'function'){
			return layer.oriAlert(msg, {title : title}, callback);
		}else{
			return layer.oriAlert(msg);
		}
	}

  /**
   * Page layer in the same HTML page as the parent window. Wraps layer.open({type:1}).
   *
   * content: display content, such as HTML strings or DOM nodes like $("#id").
   * area : ['300px', '200px']
   * offset: ['100px', '50px'], 'auto', 'r', etc.
   * options: other parameters from the layui documentation.
  */
  // layer.page = function (content, title, area, offset = 'auto', options = {}){
  layer.page = function (content, title, area, offset , options ){
    offset = (typeof offset !== 'undefined') ?  offset : 'auto';
    options = (typeof options !== 'undefined') ?  options : {};

  	options.type = 1;
  	options.content = content;
  	options.area = area;
  	options.offset = offset;
  	options.title = title;

  	return layer.open(options);
  };

  // Wrap layer.open({type:2}) for iframe page layers.
  // layer.iframe = function (url, title, area, offset = 'auto', options = {}){
  layer.iframe = function (url, title, area, offset, options ){// Browser-compatible syntax.
    offset = (typeof offset !== 'undefined') ?  offset : 'auto';
    options = (typeof options !== 'undefined') ?  options : {};

  	options.type = 2;
  	options.content = url;
  	options.area = area;
  	options.offset = offset;
  	options.title = title;

  	return layer.open(options);
  };
});//end layui.use()
function monthclick(laydate,elem,hasdone){
	var timestamp=new Date();
	nowyear = timestamp.getFullYear(),
	nowmonth = timestamp.getMonth() + 1;
	if(nowmonth<10){
		nowmonth = "0"+ nowmonth;
	}
	nowday = timestamp.getDate();
	var max	=	'';
	if(elem=='#eduedate' || elem.indexOf("#edu_edate") >= 0){
		max	=	'2099-12-30';
	}else{
		max	=	nowyear+'-'+nowmonth+'-'+nowday;
	}
	laydateobj = 
	laydate.render({
		elem: elem,
		type: 'month',
		trigger : 'click',
		max: max,
		change: function(value, date, endDate){
			var nowtimestr=nowyear+'-'+nowmonth;
			var oldVal = $(elem).val();
			if(nowtimestr>=value){
				$(elem).val(value);
			}else{
				$(elem).val(nowtimestr);
			}
			if(oldVal.substr(0, 4) == value.substr(0, 4) || nowyear == value.substr(0, 4)){
			  $('.laydate-btns-confirm').click();
			}
		},
		done: function(value, date, endDate) {
			if(hasdone==1){
				var id=elem.replace('#','');
				//checkonblur(id);
			}
		}
	});
}
if(typeof($) !== 'undefined'){
  $.layer = function(obj){
    var retval;
    layui.use(['layer'], function(){
		var layer = layui.layer,
			$ = layui.$;

		var offset = 'auto';
		if(obj.offset){
			offset = obj.offset;
		}

		var content = '';
		if(obj.page){
			if(obj.page.dom){
				content = $(obj.page.dom);
			}else if(obj.page.html){
				content = obj.page.html;
			} 
		}else if(obj.iframe){
			if(obj.iframe.src){
				content = obj.iframe.src;
			}
		}
      
		var id = obj.id ? obj.id : '';
      
		var close = obj.close ? obj.close : function(){};
		var laydata = {
			content : content,
			offset : offset,
			id : id,
			end : close
		};
		if(obj.type){
			laydata.type = obj.type;
		}
		if(obj.title){
			laydata.title = obj.title;
		}
		if(obj.area){
			laydata.area = obj.area;
		}
		if(obj.zIndex){
			laydata.zIndex = obj.zIndex;
		}
		if(obj.success){
			laydata.success = obj.success;
		}
		retval = layer.open(laydata);
    });//
    
    return retval;
  };//end $.layer  
}
