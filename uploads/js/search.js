function searchPublicT(key, params, fallback) {
    var text;
    if (typeof yunT === 'function') {
        text = yunT(key, params, fallback);
    } else if (typeof yunAt === 'function') {
        text = yunAt(key, params, fallback);
    } else {
        text = fallback !== undefined ? fallback : key;
    }
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

function searchPublicIsMore(text) {
    var value = $.trim(text || '');
    return value === '\u66f4\u591a' || value === searchPublicT('search_js_00001', null, 'More');
}

function checkmore(type){
	var html=$("#"+type).text();
	//$("."+type).slideToggle();
	$("."+type).toggle();
	if(searchPublicIsMore(html)){
		$("#"+type).attr('class','showcheck');
		$("#"+type).html(searchPublicT('search_js_00002', null, 'Collapse'));
	}else{
		$("#"+type).attr('class','hidecheck');
		$("#"+type).html(searchPublicT('search_js_00001', null, 'More'));
	}
}
$(document).ready(function(){
	$('.Search_jobs_more_chlose').hover(function(){
		$(this).find('.none').show();  
	},function(){
		$(this).find('.none').hide(); 
	});
	
	$('.delete').on('click',function(){
		var id = $(this).attr('data-id');
		var pid = $(this).attr('data-pid');
		if(parseInt(pid)>0){
			unsel(id,pid);
		}else{
			unsel(id)	
		}
	});	
	$('.search_job_list').hover(function(){
		$(".search_job_list").removeClass("search_job_list_cur_line");
		$(this).addClass('search_job_list_cur_line');  
		$(".search_job_list_cur_line>.search_job_list_box").show();
	},function(){
		var ltype=$('#ltype').val();
		if(ltype==''){
			$(".search_job_list_cur_line>.search_job_list_box").hide();
			$(".search_job_list").removeClass("search_job_list_cur_line");}
		} 
	);
	
	// Show level 1 and level 2 city search lists.
	$('.Search_jobs_sub_a').bind('mouseenter',function(){
		var dataid = $(this).attr('data-id');
		if(dataid){
			$('.Search_jobs_select').hide();
			$('#citytype'+dataid).show();
			// Adjust the pointer arrow based on the current position.
			var leftPx = $(this).position().left; 
			// Adjust arrow position.
			$('#icon_'+dataid).css('left',leftPx-60);
		}
		
	});
	$('.Search_jobs_form_list').bind('mouseleave',function(){
		$('.Search_jobs_select').hide();
		$('.oldshow').show();
	});
	/* More city options for job and talent lists. */
	$('#acity').hover(function(){
		$('.Search_cityall').removeClass('none');  
	},function(){
		$('.Search_cityall').addClass('none');
	});
	$('.Search_cityall').hover(function(){
		$('.Search_cityall').removeClass('none');  
	},function(){
		$('.Search_cityall').addClass('none'); 
	});
	/* End. */
});

function addfinder(para,usertype,type){
	if(para==''){
		layer.msg(searchPublicT('search_js_00003', null, 'No conditions to save!'),2,8);return false;
	}
	loadlayer();
	$.post(weburl+"/job/index.php?c=addfinder",{para:para,usertype:usertype},function(data){
		layer.closeAll('loading');
		var data=eval('('+data+')');
		if(type=='1'){
			layer.msg(data.msg, Number(data.tm), Number(data.st),function(){location.reload();});return false;		
		}else{
			layer.msg(data.msg, Number(data.tm), Number(data.st));return false;		
		} 
	});
}
function showurl(url){
	window.location.href=url;
}
// City tabs for talent and job search.
function acityshow(id){
	if(id==1){
		$(".acity_two").addClass('search_city_active');
		$(".acity_three").removeClass('search_city_active');
		$("#acity_two").removeClass('none');
		$("#acity_three").addClass('none');
	}else if(id==2){
		$(".acity_three").addClass('search_city_active');
		$(".acity_two").removeClass('search_city_active');
		$("#acity_two").addClass('none');
		$("#acity_three").removeClass('none');
	}
}
// Handle more industry filters in list search.
function hy_more_click(obj){
	if(searchPublicIsMore($(obj).text())){
		$(obj).text(searchPublicT('search_js_00002', null, 'Collapse'));
		$(".list_hy_more").removeClass('none');
	}else{
		$(obj).text(searchPublicT('search_js_00001', null, 'More'));
		$(".list_hy_more").addClass('none');
	}
}