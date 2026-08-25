function classPublicT(key, params, fallback) {
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

$(document).ready(function () {
    $('.delete').on('click', function () {
        var id = $(this).attr('data-id');
        var pid = $(this).attr('data-pid');
        if (parseInt(pid) > 0) {
            unsel(id, pid);
        } else {
            unsel(id)
        }
    });
    $('.search_job_list').hover(function () {
        $(".search_job_list").removeClass("search_job_list_cur_line");
        $(this).addClass('search_job_list_cur_line');
        $(".search_job_list_cur_line>.search_job_list_box").show();
    }, function () {
        var ltype = $('#ltype').val();
        if (ltype == '') {
            $(".search_job_list_cur_line>.search_job_list_box").hide();
            $(".search_job_list").removeClass("search_job_list_cur_line");
        }
    }
	);

    $(".com_admin_ask").hover(function () {
        layer.tips(classPublicT('class_js_00001', null, 'Add to search filters so you can search directly next time.'), this, {
            guide: 1,
            style: ['background-color:#F26C4F; color:#fff;top:-7px', '#F26C4F']
        });
    }, function () { layer.closeTips(); });
});

// Job category selector popup start.
$(document).ready(function () {
    $('#jobdiv').delegate('.yun_tck_con_list_jobclass1 ul .jobclassid1', 'click', function () {
        if (window.jobclass1_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var jobclassid1 = $(this).attr('codeid');
        var jobclassid2_html = '';
        if (typeof (jt[jobclassid1]) == 'object') {
            if (jt[jobclassid1].length <= 0) {
                // No child category; select current node.
                jobclass_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            } else {
                // Child categories exist; load child list.
                // Select all.
                if (window.jobclass2_checkbox_type != 'hidden') {
                    jobclassid2_html += '<li class="jobclassid2_all jobclassid2" codeid="' + jobclassid1 + '" codename="' + jn[jobclassid1] + '">' +
											'<labelabc for="jobclassid2_all_' + jn[jobclassid1] + '"><input type="' + window.jobclass2_checkbox_type + '" name="jobclassid2_all" class="jobclassid2_all_checkbox" id="jobclassid2_all_' + jn[jobclassid1] + '"/>' + classPublicT('class_js_00002', {name: jn[jobclassid1]}, 'All ({name})') + '</labelabc>' +
										'</li>';
                }
                for (var j = 0; j < jt[jobclassid1].length; j++) {
                    var jobclassid2 = jt[jobclassid1][j];
                    jobclassid2_html += '<li class="jobclassid2" codeid="' + jobclassid2 + '" codename="' + jn[jobclassid2] + '">' +
										'<labelabc for="jobclassid2_' + jn[jobclassid2] + '"><input type="' + window.jobclass2_checkbox_type + '" name="jobclassid2" class="jobclassid2_checkbox" id="jobclassid2_' + jn[jobclassid2] + '"/>' +
                                        	jn[jobclassid2] +
										'</labelabc>' +
									'</li>';
                }
            }
        } else {
            // No child category; select current node.
            jobclass_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
        }
        if (jobclassid2_html != '') {
            $('.yun_tck_con_list_jobclass3 li').remove();
            $('.yun_tck_con_list_jobclass2').show();
            $('.yun_tck_con_list_jobclass2 ul').html(jobclassid2_html);
        }
    });
    $('#jobdiv').delegate('.yun_tck_con_list_jobclass2 ul .jobclassid2', 'click', function () {
        if (window.jobclass2_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var jobclassid2 = $(this).attr('codeid');
        var jobclassid3_html = '';
        if ((typeof (jt[jobclassid2]) == 'object') && (!$(this).hasClass('jobclassid2_all'))) {
            if (jt[jobclassid2].length <= 0) {
                // No child category; select current node.
                var checked_all = jobclass_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
                if ($(this).hasClass('jobclassid2_all')) {
                    // Check whether all items are selected.
                    if (checked_all) {
                        $(this).addClass('selected').siblings().removeClass('selected');
                    } else {
                        $(this).removeClass('selected').siblings().removeClass('selected');
                    }
                    $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
                }
            } else {
                // Child categories exist; load child list.
                if (window.jobclass3_checkbox_type != 'hidden') {
                    jobclassid3_html += '<li class="jobclassid3_all jobclassid3" codeid="' + jobclassid2 + '" codename="' + jn[jobclassid2] + '">' +
											'<labelabc for="jobclassid3_all_' + jn[jobclassid2] + '"><input type="' + window.jobclass3_checkbox_type + '" name="jobclassid3_all" class="jobclassid3_all_checkbox" id="jobclassid3_all_' + jn[jobclassid2] + '"/>' + classPublicT('class_js_00002', {name: jn[jobclassid2]}, 'All ({name})') + '</labelabc>' +
										'</li>';
                }
                for (var j = 0; j < jt[jobclassid2].length; j++) {
                    var jobclassid3 = jt[jobclassid2][j];
                    jobclassid3_html += '<li class="jobclassid3" codeid="' + jobclassid3 + '" codename="' + jn[jobclassid3] + '">' +
										'<labelabc for="jobclassid3_' + jn[jobclassid3] + '"><input type="' + window.jobclass3_checkbox_type + '" name="jobclassid3" class="jobclassid3_checkbox" id="jobclassid3_' + jn[jobclassid3] + '"/>' +
                                        	jn[jobclassid3] +
										'</labelabc>' +
									'</li>';
                }
            }
        } else {
            // No child category; select current node.
            var checked_all = jobclass_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            if ($(this).hasClass('jobclassid2_all')) {
                // Check whether all items are selected.
                if (checked_all) {
                    $(this).addClass('selected').siblings().removeClass('selected');
                } else {
                    $(this).removeClass('selected').siblings().removeClass('selected');
                }
                $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
            }
        }
        if (jobclassid3_html != '') {
            $('.yun_tck_con_list_jobclass3').show();
            $('.yun_tck_con_list_jobclass3 ul').html(jobclassid3_html);
        }
    });
    $('#jobdiv').delegate('.yun_tck_con_list_jobclass3 ul .jobclassid3', 'click', function () {
        // No child category; select current node.
        if ($(this).siblings('.jobclassid3_all').length > 0) {
            if ($(this).siblings('.jobclassid3_all').hasClass('selected')) {
                return;
            }
        }
        var checked_all = jobclass_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
        if ($(this).hasClass('jobclassid3_all')) {
            // Check whether all items are selected.
            if (checked_all) {
                $(this).addClass('selected').siblings().removeClass('selected');
            } else {
                $(this).removeClass('selected').siblings().removeClass('selected');
            }
            $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
        }
    });
    $('#jobdiv').delegate('.yun_tit_selected .selected .delete', 'click', function () {
        var codeid = $(this).parent().parent().attr('codeid');
        $('#jobdiv li[codeid=' + codeid + ']').removeClass('selected');
		if($('#jobdiv li[codeid='+codeid+']').find('input').length>0){
			$('#jobdiv li[codeid='+codeid+']').find('input')[0].checked=false;
		}
        
        $(this).parent().parent().remove();
    });
    $('#jobdiv').delegate('.yun_tck_tit_close,#cancel_btn', 'click', function () {
        layer.close(window.jobclass_layer);
    });
    $('#jobdiv').delegate('#btnSubmitJobsort', 'click', function () {
        confirm_selected_jobclass_items();
    });
});
function get_jobclass_deep() {

    var jt_length = 0, ji_length = 0;
    for (var j = 0; j <= jt.length; j++) {
        if (jt[j]) {
            jt_length++;
        }
    }
    for (var j = 0; j <= ji.length; j++) {
        if (ji[j]) {
            ji_length++;
        }
    }
    if ((jt_length > 0) && (ji_length < jt_length)) {
        window.jobclass_deep = 3;
    } else if ((jt_length > 0) && (ji_length == jt_length)) {
        window.jobclass_deep = 2;
    } else {
        window.jobclass_deep = 1;
    }
    return window.jobclass_deep;
}
// Select category item.
function jobclass_item_select(jobclass_id, jobclass_name, type, jobclass_element) {
    // Single-select mode.
    if (window.allow_select_jobclass_count == 1) {
        $('#jobdiv .yun_tit_selected .selected').html('');
        $('#jobdiv .yun_tit_selected .selected').append('<li codeid="' + jobclass_id + '" codename="' + jobclass_name + '">' +
							'<a class="clean g3 selall" href="javascript:;">' +
								'<span class="text">' +
									jobclass_name +
								'</span>' +
								'<span class="delete">' +
									classPublicT('class_js_00003', null, 'Remove') +
								'</span>' +
							'</a>' +
						'</li>');
        $(jobclass_element).addClass('selected').siblings().removeClass('selected');
        //confirm_selected_jobclass_items()
        //layer.close(window.jobclass_layer);
        //return;
    } else {
        var jobclass_items = $('#jobdiv .yun_tit_selected .selected li');
        // Check whether it is already selected.
        for (var i = 0; i < jobclass_items.length; i++) {
            if ($(jobclass_items[i]).attr('codeid') == jobclass_id) {
                if ($(jobclass_items[i]).find('input').is(":hidden")) {
                    $('#jobdiv li[codeid=' + $(jobclass_items[i]).attr('codeid') + ']').removeClass('selected');
                    $(jobclass_element).find('input')[0].checked = false;
                    return false;
                } else {
                    $(jobclass_items[i]).find('.delete').click();
                    $('#jobdiv li[codeid=' + $(jobclass_items[i]).attr('codeid') + ']').removeClass('selected');
                    $(jobclass_element).find('input')[0].checked = false;
                    return false;
                }
            }
            // Check whether it is a child of the selected item.
            if (typeof (jt[jobclass_id]) == 'object') {
                if (jt[jobclass_id].length > 0) {
                    for (var j = 0; j < jt[jobclass_id].length; j++) {
                        if (jt[jobclass_id][j] == $(jobclass_items[i]).attr('codeid')) {
                            $(jobclass_items[i]).find('.delete').click();
                            $('#jobdiv li[codeid=' + $(jobclass_items[i]).attr('codeid') + ']').removeClass('selected');
                        }
                    }
                }
            }
        }
        // Check whether the limit is exceeded.
        if (jobclass_items.length >= parseInt(window.allow_select_jobclass_count)) {
			layer.msg(classPublicT('class_js_00004', {count: parseInt(window.allow_select_jobclass_count)}, 'Select no more than {count} items!'), 2, 8);return false;
            $(jobclass_element).find('.delete').click();
            $('#jobdiv li[codeid=' + jobclass_id + ']').removeClass('selected');
            $('#jobdiv li[codeid=' + jobclass_id + ']').find('input')[0].checked = false;
            return false;
        }
        $(jobclass_element).find('input')[0].checked = true;
        $('#jobdiv li[codeid=' + jobclass_id + ']').addClass('selected');
        $('#jobdiv .yun_tit_selected .selected').append('<li codeid="' + jobclass_id + '" codename="' + jobclass_name + '">' +
                                '<a class="clean g3 selall" href="javascript:;">' +
                                    '<span class="text">' +
                                        jobclass_name +
                                    '</span>' +
                                    '<span class="delete">' +
                                        classPublicT('class_js_00003', null, 'Remove') +
                                    '</span>' +
                                '</a>' +
                            '</li>');
    }
    return true;
}
// Confirm selected category items.
function confirm_selected_jobclass_items() {
    // Check whether any item is selected.
    var jobclass_items = $('#jobdiv .yun_tit_selected .selected li');
    var jobclass_ids = '';
    var jobclass_names = '';
    for (var i = 0; i < jobclass_items.length; i++) {
        jobclass_ids += ',' + $(jobclass_items[i]).attr('codeid');
        jobclass_names += ',' + $(jobclass_items[i]).attr('codename');
    } 
    if(jobclass_names.length<=0){
        layer.msg(classPublicT('class_js_00005', null, 'Select a specific category!'), 2, 8);return false;
    }else{
        // Write selected IDs and names to target elements.
      if (window.target_jobclassin_names_tagname == 'INPUT') {
          $(window.target_jobclassin_names).val(jobclass_names.substring(1));
		  var addtype=$("#addtype").val();
		  if(addtype=='addexpect'){
			  $("#hidjob_class").attr("class","resume_tipok");
			  $("#hidjob_class").html('');
		  }
      } else {
          $(window.target_jobclassin_names).html(jobclass_names.substring(1));
      }
      if (window.target_jobclassin_ids_tagname == 'INPUT') {
          $(window.target_jobclassin_ids).val(jobclass_ids.substring(1));
      } else {
          $(window.target_jobclassin_ids).html(jobclass_ids.substring(1));
      }
      if (window.index_jobclass_callback) {
          window.index_jobclass_callback();
      }
      layer.close(window.jobclass_layer);
     
	  $.post(weburl+"/index.php?m=ajax&c=getcontent",{ids:jobclass_ids.substring(1)},function(data){
		  if(data){
			var datas=data.split('@@@@'); 
			for(var i=0;i<datas.length;i++){
				var ndata=datas[i].split('###'); 
				$("#JobRequInfoTemplate").html("<a href=\"javascript:void(0)\" onclick=\"setexample('"+ndata[0]+"')\">"+ndata[1]+"</a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
			}
			$(".Description").show();  
		  }
	  });
	   return true;
    } 
}
function setexample(id){
	$.post(weburl+"/index.php?m=ajax&c=setexample",{id:id},function(data){
		if(data){
			editor.setContent(data);
		} 
	});
	
}
// Job category selector supports single/multiple select, max count, and target element type.
function index_job(allow_select_jobclass_count, target_jobclassin_names, target_jobclassin_ids, jobdiv_style, codeids, index_jobclass_callback) {
	if(document.getElementById('jobdiv').style.display=='block'){
		return;
	}
    if ($(target_jobclassin_names).length <= 0) {layer.msg(classPublicT('class_js_00006', null, 'Job category name target element does not exist!'), 2, 8);return false;}
    if ($(target_jobclassin_ids).length <= 0) { layer.msg(classPublicT('class_js_00007', null, 'Job category ID target element does not exist!'), 2, 8);return false;}
    // Maximum selectable count; 1 means single-select.
    window.allow_select_jobclass_count = allow_select_jobclass_count;
    // Name target selector.
    window.target_jobclassin_names = target_jobclassin_names;
    // ID target selector.
    window.target_jobclassin_ids = target_jobclassin_ids;
    // Name target element type: html() or val().
    window.target_jobclassin_names_tagname = $(target_jobclassin_names)[0].nodeName;
    // ID target element type: html() or val().
    window.target_jobclassin_ids_tagname = $(target_jobclassin_ids)[0].nodeName;
    // Popup style.
    window.jobdiv_style = jobdiv_style;
    // Callback after confirming selection.
    window.index_jobclass_callback = index_jobclass_callback;
    // Determine whether checkbox input is needed.

    // Calculate category depth.
    var jobclass_deep = get_jobclass_deep();
    switch (jobclass_deep) {
        case 1:
            window.jobclass1_checkbox_type = 'checkbox';
            window.jobclass2_checkbox_type = 'hidden';
            window.jobclass3_checkbox_type = 'hidden';
            $('.yun_tck_con_list').hide();
            $('.yun_tck_con_list_jobclass1').show();
            break;
        case 2:
            window.jobclass1_checkbox_type = 'hidden';
            window.jobclass2_checkbox_type = 'checkbox';
            window.jobclass3_checkbox_type = 'hidden';
            $('.yun_tck_con_list').hide();
            $('.yun_tck_con_list_jobclass1').show();
            $('.yun_tck_con_list_jobclass2').show();
            break;
        case 3:
            window.jobclass1_checkbox_type = 'hidden';
            window.jobclass2_checkbox_type = 'hidden';
            window.jobclass3_checkbox_type = 'checkbox';
            $('.yun_tck_con_list_jobclass1').show();
            $('.yun_tck_con_list_jobclass2').show();
            $('.yun_tck_con_list_jobclass3').show();
            break;
        default: break;
    }
    // Single-select mode.
    if (window.allow_select_jobclass_count == 1) {
        window.jobclass1_checkbox_type = 'hidden';
        window.jobclass2_checkbox_type = 'hidden';
        window.jobclass3_checkbox_type = 'hidden';
    }

    //$("#jobdiv").attr('style',$("#jobdiv").attr('style')+';'+window.jobdiv_style);
    var html = $("#jobdiv").html();
    if (html.replace(" ", "") == '') {
        var codeids_list = (codeids) ? codeids.split(',') : (new Array());
        var codeids_html = '';
        for (var i = 0; i < codeids_list.length; i++) {
            var codeid = codeids_list[i];
            var codename = jn[codeid];
            codeids_html += '<li codeid="' + codeid + '" codename="' + codename + '">' +
							'<a class="clean g3 selall" href="javascript:;">' +
								'<span class="text">' +
									codename +
								'</span>' +
								'<span class="delete">' +
									classPublicT('class_js_00003', null, 'Remove') +
								'</span>' +
							'</a>' +
						'</li>';
        }
        var jobclass1_html = '';
        for (var i = 0; i < ji.length; i++) {
            var jobclassid1 = ji[i];
            jobclass1_html += '<li class="jobclassid1" codeid="' + jobclassid1 + '" codename="' + jn[jobclassid1] + '">' +
                                    '<labelabc for="jobclassid1_' + jn[jobclassid1] + '"><input type="' + window.jobclass1_checkbox_type + '" name="jobclassid1" class="jobclassid1_checkbox" id="jobclassid1_' + jn[jobclassid1] + '"/>' +
                                        jn[jobclassid1] +
                                    '</labelabc>' +
                                '</li>';
        }
        html = '<div class="yun_tck">' +
            '<div class="yun_tck_box">' +
                '<div class="yun_tck_tit">' +
                    '<span class="yun_tck_tit_span">' +
                        classPublicT('class_js_00012', null, 'Job Category') +
                    '</span>' +
                    '<a href="javascript:;" class="yun_tck_tit_close">' +
                        classPublicT('class_js_00015', null, 'Close') +
                    '</a>' +
                '</div>' +
				'<div class="yun_tck_title">' +
                    '<div class="yun_tck_title_box">' +
                        '<div class="yun_tck_tit_xz">' +
                            '<span class="yun_tck_tit_xz_l">' +
                                classPublicT('class_js_00016', null, 'Selected:') +
                            '</span>' +
                            '<span class="yun_tck_tit_xz_r">' +
                                classPublicT('class_js_00017', {count: allow_select_jobclass_count}, '(Select up to {count} items)') +
                            '</span>' +
                        '</div>' +
						'<div class="yun_tit_selected">' +
                            '<ul class="selected clearfix">' +
								codeids_html +
                            '</ul>' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '<div class="clear">' +
                '</div>' +
				'<div class="dropLst">' +
                    '<div class="yun_tck_con">' +
                        '<div class="yun_tck_con_list yun_tck_con_list_jobclass1">' +
                            '<ul>' +
                                jobclass1_html +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_jobclass2">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_jobclass3">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
                        '<div class="clear">' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '</span>' +
            '</div>' +
            '<div class="clear">' +
            '</div>' +
            '<div class="actions">' +
                '<button class="button_a button_a_red" id="btnSubmitJobsort">' +
                    classPublicT('class_js_00018', null, 'Confirm') +
                '</button>' +
                '<button class="button_a" id="cancel_btn">' +
                    classPublicT('class_js_00019', null, 'Cancel') +
                '</button>' +
            '</div>' +
        '</div>';
        html += ' </tbody></table></div>';
        $("#jobdiv").html(html);
    }
	
    window.jobclass_layer = $.layer({
        type: 1,
        title: false,
        offset: ['100px', ($(window).width() - 620) / 2 + 'px'],
        closeBtn: [0, false],
        fix: false,
        border: [0, 0.3, '#000', true],
        move: false,
        area: ['620px', '440px'],
        page: { dom: '#jobdiv' }
    });
}
// Job category selector popup end.

function check_select_show(id) {
    $("#list" + id).show();
}
function check_onselect(id, val, name) {
    $("#" + id).val(val);
    $("#list" + id).hide();
    $("#button" + id).val(name);
}
function addfinder(para, usertype) {
    $.post(weburl + "/job/index.php?c=addfinder", { para: para, usertype: usertype }, function (data) {
        var data = eval('(' + data + ')');
        layer.msg(data.msg, Number(data.tm), Number(data.st)); return false;
    });
}
// City selector popup start.
$(document).ready(function () {
    $('#citydiv').delegate('.yun_tck_con_list_city1 ul .cityid1', 'click', function () {
        if (window.city1_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var cityid1 = $(this).attr('codeid');
        var cityid2_html = '';
        if (typeof (ct[cityid1]) == 'object') {
            if (ct[cityid1].length <= 0) {
                // No child category; select current node.
                city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            } else {
                // Child categories exist; load child list.
                // Select all.
                if (window.city2_checkbox_type != 'hidden') {
                    cityid2_html += '<li class="cityid2_all cityid2" codeid="' + cityid1 + '" parentid="' + 0 + '" codename="' + cn[cityid1] + '">' +
											'<labelabc for="cityid2_all_' + cn[cityid1] + '"><input type="' + window.city2_checkbox_type + '" name="cityid2_all" class="cityid2_all_checkbox" id="cityid2_all_' + cn[cityid1] + '"/>' + classPublicT('class_js_00002', {name: cn[cityid1]}, 'All ({name})') + '</labelabc>' +
										'</li>';
                }
                for (var j = 0; j < ct[cityid1].length; j++) {
                    var cityid2 = ct[cityid1][j];
                    cityid2_html += '<li class="cityid2" codeid="' + cityid2 + '" parentid="' + cityid1 + '" codename="' + cn[cityid2] + '">' +
										'<labelabc for="cityid2_' + cn[cityid2] + '"><input type="' + window.city2_checkbox_type + '" name="cityid2" class="cityid2_checkbox" id="cityid2_' + cn[cityid2] + '"/>' +
                                        	cn[cityid2] +
										'</labelabc>' +
									'</li>';
                }
                city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            }
        } else {
            // No child category; select current node.
            city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
        }
        if (cityid2_html != '') {
            $('.yun_tck_con_list_city3 li').remove();
            $('.yun_tck_con_list_city2 ul').html(cityid2_html);
        }else{
            $('.yun_tck_con_list_city3 li').remove();
            $('.yun_tck_con_list_city2 ul').html('');
		}
    });
    $('#citydiv').delegate('.yun_tck_con_list_city2 ul .cityid2', 'click', function () {
        if (window.city2_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var cityid1 = $(this).attr('parentid');
        var cityid2 = $(this).attr('codeid');
        var cityid3_html = '';
        if ((typeof (ct[cityid2]) == 'object') && (!$(this).hasClass('cityid2_all'))) {
            if (ct[cityid2].length <= 0) {
                // No child category; select current node.
                var checked_all = city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
                if ($(this).hasClass('cityid2_all')) {
                    // Check whether all items are selected.
                    if (checked_all) {
                        $(this).addClass('selected').siblings().removeClass('selected');
                    } else {
                        $(this).removeClass('selected').siblings().removeClass('selected');
                    }
                    $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
                }
            } else {
                // Child categories exist; load child list.
                city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
                if (window.allow_select_city_level > 2) {
                    if (window.city3_checkbox_type != 'hidden') {
                        cityid3_html += '<li class="cityid3_all cityid3 data-first" codeid="' + cityid2 + '" parentid="' + cityid1 + '" codename="' + cn[cityid2] + '">' +
                                                '<labelabc for="cityid3_all_' + cn[cityid2] + '"><input type="' + window.city3_checkbox_type + '" name="cityid3_all" class="cityid3_all_checkbox" id="cityid3_all_' + cn[cityid2] + '"/>' + classPublicT('class_js_00002', {name: cn[cityid2]}, 'All ({name})') + '</labelabc>' +
                                            '</li>';
                    }
                    for (var j = 0; j < ct[cityid2].length; j++) {
                        var cityid3 = ct[cityid2][j];
                        cityid3_html += '<li class="cityid3" codeid="' + cityid3 + '" parentid="' + cityid2 + '" codename="' + cn[cityid3] + '">' +
                                            '<labelabc for="cityid3_' + cn[cityid3] + '"><input type="' + window.city3_checkbox_type + '" name="cityid3" class="cityid3_checkbox" id="cityid3_' + cn[cityid3] + '"/>' +
                                                cn[cityid3] +
                                            '</labelabc>' +
                                        '</li>';
                    }
                }
            }
        } else {
            // No child category; select current node.
            var checked_all = city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            if ($(this).hasClass('cityid2_all')) {
                // Check whether all items are selected.
                if (checked_all) {
                    $(this).addClass('selected').siblings().removeClass('selected');
                } else {
                    $(this).removeClass('selected').siblings().removeClass('selected');
                }
                $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
            }
        }
        if (cityid3_html != '' && window.allow_select_city_level > 2) {
            $('.yun_tck_con_list_city3').show();
            $('.yun_tck_con_list_city3 ul').html(cityid3_html);
        }
    });
    $('#citydiv').delegate('.yun_tck_con_list_city3 ul .cityid3', 'click', function () {
        // No child category; select current node.
        if ($(this).siblings('.cityid3_all').length > 0) {
            if ($(this).siblings('.cityid3_all').hasClass('selected')) {
                return;
            }
        }
        var checked_all = city_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this),$(this).attr('parentid'));
        if ($(this).hasClass('cityid3_all')) {
            // Check whether all items are selected.
            if (checked_all) {
                $(this).addClass('selected').siblings().removeClass('selected');
            } else {
                $(this).removeClass('selected').siblings().removeClass('selected');
            }
            $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
        }
    });
    $('#citydiv').delegate('.yun_tit_selected .selected .delete', 'click', function () {
        var codeid = $(this).parent().parent().attr('codeid');
        $('#citydiv li[codeid=' + codeid + ']').removeClass('selected');
        if($('#citydiv li[codeid=' + codeid + ']').find('input').length>0){
        	$('#citydiv li[codeid=' + codeid + ']').find('input')[0].checked = false;
		}
        $(this).parent().parent().remove();
    });
    $('#citydiv').delegate('.yun_tck_tit_close,#cancel_btn', 'click', function () {
    	layer.close(window.city_layer);
    });
    $('#citydiv').delegate('#btnSubmitJobsort', 'click', function () {
        confirm_selected_city_items();
    });
});
function get_city_deep() {
    var ct_length = 0, ci_length = 0;
    for (var j = 0; j <= ct.length; j++) {
        if (ct[j]) {
            ct_length++;
        }
    }
    for (var j = 0; j <= ci.length; j++) {
        if (ci[j]) {
            ci_length++;
        }
    }
    if ((ct_length > 0) && (ci_length < ct_length)) {
        window.city_deep = 3;
    } else if ((ct_length > 0) && (ci_length == ct_length)) {
        window.city_deep = 2;
    } else {
        window.city_deep = 1;
    }
    return window.city_deep;
}
// Select category item.
function city_item_select(city_id, city_name, type, city_element,parentid) {
    // Single-select mode.
    if (window.allow_select_city_count == 1) {
        $('#citydiv .yun_tit_selected .selected').html('');
        $('#citydiv .yun_tit_selected .selected').append('<li codeid="' + city_id + '" codename="' + city_name + '">' +
				'<a class="clean g3 selall" href="javascript:;">' +
					'<span class="text">' +
						city_name +
					'</span>' +
					'<span class="delete">' +
						classPublicT('class_js_00003', null, 'Remove') +
					'</span>' +
				'</a>' +
			'</li>');
        $(city_element).addClass('selected').siblings().removeClass('selected');
        //confirm_selected_city_items()
        //layer.close(window.city_layer);
        //return;
    } else {
		// Remove parent element.
    	$('#citydiv li[codeid=' + $(city_element).attr('parentid') + ']').find('.delete').click();
        var city_items = $('#citydiv .yun_tit_selected .selected li');
    	var threecheck = true;
        // Check whether it is already selected.
        for (var i = 0; i < city_items.length; i++) {
            if ($(city_items[i]).attr('codeid') == city_id) {
                if ($(city_items[i]).find('input').is(":hidden")) {
                    $('#citydiv li[codeid=' + $(city_items[i]).attr('codeid') + ']').removeClass('selected');
                    $(city_element).find('input')[0].checked = false;
                    return false;
                } else {
					// First select-all operation for third-level cities.
                	if($('#citydiv li[codeid=' + $(city_items[i]).attr('codeid') + ']').hasClass('data-first')){
                		$('#citydiv li[codeid=' + $(city_items[i]).attr('codeid') + ']').removeClass('data-first');
                		$(city_element).find('input')[0].checked = true;
                		threecheck=false;
                	}else{
                		$(city_items[i]).find('.delete').click();
                        $('#citydiv li[codeid=' + $(city_items[i]).attr('codeid') + ']').removeClass('selected');
                        $(city_element).find('input')[0].checked = false;
                        return false;
                	}
                }
            }
            // Check whether it is a child of the selected item.
            if (typeof (ct[city_id]) == 'object') {
                if (ct[city_id].length > 0) {
                    for (var j = 0; j < ct[city_id].length; j++) {
                        if (ct[city_id][j] == $(city_items[i]).attr('codeid')) {
                            $(city_items[i]).find('.delete').click();
                            $('#citydiv li[codeid=' + $(city_items[i]).attr('codeid') + ']').removeClass('selected');
                        }
                    }
                }
            }
        }
        if(threecheck==true){
            // Check whether the limit is exceeded.
            if (city_items.length >=parseInt(window.allow_select_city_count)) {
				layer.msg(classPublicT('class_js_00004', {count: parseInt(window.allow_select_city_count)}, 'Select no more than {count} items!'), 2, 8);return false;
                $(city_element).find('.delete').click();
                $('#citydiv li[codeid=' + city_id + ']').removeClass('selected');
                $('#citydiv li[codeid=' + city_id + ']').find('input')[0].checked = false;
                return false;
            }
            $(city_element).find('input')[0].checked = true;
            $('#citydiv li[codeid=' + city_id + ']').addClass('selected');
            $('#citydiv .yun_tit_selected .selected').append('<li codeid="' + city_id + '" codename="' + city_name + '">' +
                    '<a class="clean g3 selall" href="javascript:;">' +
                        '<span class="text">' +
                            city_name +
                        '</span>' +
                        '<span class="delete">' +
                            classPublicT('class_js_00003', null, 'Remove') +
                        '</span>' +
                    '</a>' +
                '</li>');
        }
    }
    return true;
}
// Confirm selected category items.
function confirm_selected_city_items() {
    // Check whether any item is selected.
    var city_items = $('#citydiv .yun_tit_selected .selected li');
    var city_ids = '';
    var city_names = '';
    for (var i = 0; i < city_items.length; i++) {
    	city_names += ',' + $(city_items[i]).attr('codename');
    	city_ids += ',' + $(city_items[i]).attr('codeid');
    }
    // Write selected IDs and names to target elements.
    if (window.target_cityin_names_tagname == 'INPUT') {
		var addtype=$("#addtype").val();
		  if(addtype=='addexpect'){
			  $("#hidcity_class").attr("class","resume_tipok");
			  $("#hidcity_class").html('');
		  }
        $(window.target_cityin_names).val(city_names.substring(1));
    } else {
        $(window.target_cityin_names).html(city_names.substring(1));
    }
    if (window.target_cityin_ids_tagname == 'INPUT') {
        $(window.target_cityin_ids).val(city_ids.substring(1));
    } else {
        $(window.target_cityin_ids).html(city_ids.substring(1));
    }
    if (window.index_city_callback) {
        window.index_city_callback();
    }
    layer.close(window.city_layer);
    return true;
}
function index_city(allow_select_city_count, target_cityin_names, target_cityin_ids, citydiv_style, codeids, index_city_callback, allow_select_city_level) {
	if(document.getElementById('citydiv').style.display=='block'){
		return;
	}
    if ($(target_cityin_names).length <= 0) {layer.msg(classPublicT('class_js_00008', null, 'City name target element does not exist!'), 2, 8);return false; }
    if ($(target_cityin_ids).length <= 0) { layer.msg(classPublicT('class_js_00009', null, 'City ID target element does not exist!'), 2, 8);return false;}
    // Maximum selectable count; 1 means single-select.
    window.allow_select_city_count = allow_select_city_count;
    // Name target selector.
    window.target_cityin_names = target_cityin_names;
    // ID target selector.
    window.target_cityin_ids = target_cityin_ids;
    // Name target element type: html() or val().
    window.target_cityin_names_tagname = $(target_cityin_names)[0].nodeName;
    // ID target element type: html() or val().
    window.target_cityin_ids_tagname = $(target_cityin_ids)[0].nodeName;
    // Popup style.
    window.citydiv_style = citydiv_style;
    // Callback after confirming selection.
    window.index_city_callback = index_city_callback;
    // Lowest selectable city level.
    window.allow_select_city_level = allow_select_city_level ? allow_select_city_level : 99;
    // Determine whether checkbox input is needed.

    // Calculate category depth.
    var city_deep = get_city_deep();
    switch (city_deep) {
        case 1:
            window.city1_checkbox_type = 'checkbox';
            window.city2_checkbox_type = 'hidden';
            window.city3_checkbox_type = 'hidden';
            break;
        case 2:
            window.city1_checkbox_type = 'hidden';
            window.city2_checkbox_type = 'checkbox';
            window.city3_checkbox_type = 'hidden';
            break;
        case 3:
            window.city1_checkbox_type = 'hidden';
            window.city2_checkbox_type = 'hidden';
            window.city3_checkbox_type = 'checkbox';
            break;
        default: break;
    }
    // Single-select mode.
    if (window.allow_select_city_count == 1) {
        window.city1_checkbox_type = 'hidden';
        window.city2_checkbox_type = 'hidden';
        window.city3_checkbox_type = 'hidden';
    }

    //$("#citydiv").attr('style',$("#citydiv").attr('style')+';'+window.citydiv_style);
    var html = $("#citydiv").html();
    if (html.replace(" ", "") == '') {
        var codeids_list = (codeids) ? codeids.split(',') : (new Array());
        var codeids_html = '';
        for (var i = 0; i < codeids_list.length; i++) {
            var codeid = codeids_list[i];
            var codename = cn[codeid];
            codeids_html += '<li codeid="' + codeid + '" codename="' + codename + '">' +
							'<a class="clean g3 selall" href="javascript:;">' +
								'<span class="text">' +
									codename +
								'</span>' +
								'<span class="delete">' +
									classPublicT('class_js_00003', null, 'Remove') +
								'</span>' +
							'</a>' +
						'</li>';
        }
        var city1_html = '';
        for (var i = 0; i < ci.length; i++) {
            var cityid1 = ci[i];
            city1_html += '<li class="cityid1" codeid="' + cityid1 + '" parentid="' + 0 + '" codename="' + cn[cityid1] + '">' +
                                    '<labelabc for="cityid1_' + cn[cityid1] + '"><input type="' + window.city1_checkbox_type + '" name="cityid1" class="cityid1_checkbox" id="cityid1_' + cn[cityid1] + '"/>' +
                                        cn[cityid1] +
                                    '</labelabc>' +
                                '</li>';
        }
        html = '<div class="yun_tck">' +
            '<div class="yun_tck_box">' +
                '<div class="yun_tck_tit">' +
                    '<span class="yun_tck_tit_span">' +
                        classPublicT('class_js_00013', null, 'City Selection') +
                    '</span>' +
                    '<a href="javascript:;" class="yun_tck_tit_close">' +
                        classPublicT('class_js_00015', null, 'Close') +
                    '</a>' +
                '</div>' +
				'<div class="yun_tck_title">' +
                    '<div class="yun_tck_title_box">' +
                        '<div class="yun_tck_tit_xz">' +
                            '<span class="yun_tck_tit_xz_l">' +
                                classPublicT('class_js_00016', null, 'Selected:') +
                            '</span>' +
                            '<span class="yun_tck_tit_xz_r">' +
                                classPublicT('class_js_00017', {count: allow_select_city_count}, '(Select up to {count} items)') +
                            '</span>' +
                        '</div>' +
						'<div class="yun_tit_selected">' +
                            '<ul class="selected clearfix">' +
								codeids_html +
                            '</ul>' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '<div class="clear">' +
                '</div>' +
				'<div class="dropLst">' +
                    '<div class="yun_tck_con">' +
                        '<div class="yun_tck_con_list yun_tck_con_list_city1">' +
                            '<ul>' +
                                city1_html +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_city2">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_city3">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
                        '<div class="clear">' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '</span>' +
            '</div>' +
            '<div class="clear">' +
            '</div>' +
            '<div class="actions">' +
                '<button class="button_a button_a_red" id="btnSubmitJobsort">' +
                    classPublicT('class_js_00018', null, 'Confirm') +
                '</button>' +
                '<button class="button_a" id="cancel_btn">' +
                    classPublicT('class_js_00019', null, 'Cancel') +
                '</button>' +
            '</div>' +
        '</div>';
        html += ' </tbody></table></div>';
        $("#citydiv").html(html);
    }
    switch (city_deep) {
        case 1:
            $('.yun_tck_con_list').hide();
            $('.yun_tck_con_list_city1').show();
            break;
        case 2:
            $('.yun_tck_con_list_city1').show();
            $('.yun_tck_con_list_city2').show();
            break;
        case 3:
            $('.yun_tck_con_list_city1').show();
            $('.yun_tck_con_list_city2').show();
            $('.yun_tck_con_list_city3').show();
            break;
        default: break;
    }
    if (window.allow_select_city_level <= 1) {
        $('.yun_tck_con_list_city1').show();
        $('.yun_tck_con_list_city2').hide();
        $('.yun_tck_con_list_city3').hide();
    } else if (window.allow_select_city_level <= 2) {
        $('.yun_tck_con_list_city1').show();
        $('.yun_tck_con_list_city2').show();
        $('.yun_tck_con_list_city3').hide();
    } else {
        $('.yun_tck_con_list_city1').show();
        $('.yun_tck_con_list_city2').show();
        $('.yun_tck_con_list_city3').show();
    }
    window.city_layer = $.layer({
        type: 1,
        title: false,
        offset: ['100px', ($(window).width() - 620) / 2 + 'px'],
        closeBtn: [0, false],
        fix: false,
        border: [0, 0.3, '#000', true],
        move: false,
        area: ['620px', '440px'],
        page: { dom: '#citydiv' }
    });
}
// City selector popup end.
function addsel(id, pid) {
    // Check count.
    var i = 0;
    $(".selall").each(function () {
        i++;
    });
    if (parseInt(pid) > 0) {
        if (i > 5) {
            unsel(id, pid);
            layer.msg(classPublicT('class_js_00020', null, 'You can select up to five items!'), 2, 8);
            return false;
        } else {
            var name = $('#job_class_' + id).attr('data-name');
            html = '<li class="job_class_' + id + ' parent_' + pid + '"><a class="clean g3 selall" href="javascript:void(0);" data-val="' + id + '+' + name + '"><span class="text">' + name + '</span><span class="delete" data-id="' + id + '" data-pid ="' + pid + '">' + classPublicT('class_js_00003', null, 'Remove') + '</span></a></li>';
            $('.job_class_' + id).remove();
            $('.selected').first().append(html);
        }
    } else {
        if (i > 4) {
            unsel(id);
            layer.msg(classPublicT('class_js_00020', null, 'You can select up to five items!'), 2, 8);
            return false;
        } else {
            var name = $('#all' + id).attr('data-name');
            html = '<li class="all' + id + '"><a class="clean g3 selall" href="javascript:void(0);"  data-val="' + id + '+' + name + '"><span class="text">' + name + '</span><span class="delete" data-id="' + id + '">' + classPublicT('class_js_00003', null, 'Remove') + '</span></a></li>';
            $('.parent_' + id).remove();
            $('.all' + id).remove();
            $('.selected').first().append(html);
        }
    }
}
function unsel(id, pid) {
    if (parseInt(pid) > 0) {
        $('.job_class_' + id).remove();
        $('#job_class_' + id).removeAttr("checked", "");
    } else {
        $('.all' + id).remove();
        $('#all' + id).removeAttr("checked", "");
        $('.labelabc' + id).removeAttr("disabled");
        $('.labelabc' + id).removeAttr("checked");
    }
}
function check_this(id) {
    if ($("#job_class_" + id).attr("disabled") != 'disabled') {
        if ($("#job_class_" + id).attr("checked") != "checked") {
            var pid = $("#job_class_" + id).attr('data-pid');
            $("#job_class_" + id).removeAttr("checked");
            unsel(id, pid);
        } else {
            var pid = $("#job_class_" + id).attr('data-pid');
            $("#job_class_" + id).attr("checked", "true");
            addsel(id, pid);
        }
    }
}


// Industry selector popup start.
$(document).ready(function () {
    $('#industrydiv').delegate('.yun_tck_con_list_industry1 ul .industryid1', 'click', function () {
        if (window.industry1_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var industryid1 = $(this).attr('codeid');
        var industryid2_html = '';
        if (typeof (ht[industryid1]) == 'object') {
            if (ht[industryid1].length <= 0) {
                // No child category; select current node.
                industry_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            } else {
                // Child categories exist; load child list.
                // Select all.
                if (window.industry2_checkbox_type != 'hidden') {
                    industryid2_html += '<li class="industryid2_all industryid2" codeid="' + industryid1 + '" codename="' + hyname[industryid1] + '">' +
											'<labelabc for="industryid2_all_' + hyname[industryid1] + '"><input type="' + window.industry2_checkbox_type + '" name="industryid2_all" class="industryid2_all_checkbox" id="industryid2_all_' + hyname[industryid1] + '"/>' + classPublicT('class_js_00002', {name: hyname[industryid1]}, 'All ({name})') + '</labelabc>' +
										'</li>';
                }
                for (var j = 0; j < ht[industryid1].length; j++) {
                    var industryid2 = ht[industryid1][j];
                    industryid2_html += '<li class="industryid2" codeid="' + industryid2 + '" codename="' + hyname[industryid2] + '">' +
										'<labelabc for="industryid2_' + hyname[industryid2] + '"><input type="' + window.industry2_checkbox_type + '" name="industryid2" class="industryid2_checkbox" id="industryid2_' + hyname[industryid2] + '"/>' +
                                        	hyname[industryid2] +
										'</labelabc>' +
									'</li>';
                }
            }
        } else {
            // No child category; select current node.
            industry_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
        }
        if (industryid2_html != '') {
            $('.yun_tck_con_list_industry3 li').remove();
            $('.yun_tck_con_list_industry2 ul').html(industryid2_html);
        }
    });
    $('#industrydiv').delegate('.yun_tck_con_list_industry2 ul .industryid2', 'click', function () {
        if (window.industry2_checkbox_type == 'hidden') {
            $(this).addClass('selected').siblings().removeClass('selected');
        }
        var industryid2 = $(this).attr('codeid');
        var industryid3_html = '';
        if ((typeof (ht[industryid2]) == 'object') && (!$(this).hasClass('industryid2_all'))) {
            if (ht[industryid2].length <= 0) {
                // No child category; select current node.
                var checked_all = industry_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
                if ($(this).hasClass('industryid2_all')) {
                    // Check whether all items are selected.
                    if (checked_all) {
                        $(this).addClass('selected').siblings().removeClass('selected');
                    } else {
                        $(this).removeClass('selected').siblings().removeClass('selected');
                    }
                    $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
                }
            } else {
                // Child categories exist; load child list.
                if (window.industry3_checkbox_type != 'hidden') {
                    industryid3_html += '<li class="industryid3_all industryid3" codeid="' + industryid2 + '" codename="' + hyname[industryid2] + '">' +
											'<labelabc for="industryid3_all_' + hyname[industryid2] + '"><input type="' + window.industry3_checkbox_type + '" name="industryid3_all" class="industryid3_all_checkbox" id="industryid3_all_' + hyname[industryid2] + '"/>' + classPublicT('class_js_00002', {name: hyname[industryid2]}, 'All ({name})') + '</labelabc>' +
										'</li>';
                }
                for (var j = 0; j < ht[industryid2].length; j++) {
                    var industryid3 = ht[industryid2][j];
                    industryid3_html += '<li class="industryid3" codeid="' + industryid3 + '" codename="' + hyname[industryid3] + '">' +
										'<labelabc for="industryid3_' + hyname[industryid3] + '"><input type="' + window.industry3_checkbox_type + '" name="industryid3" class="industryid3_checkbox" id="industryid3_' + hyname[industryid3] + '"/>' +
                                        	hyname[industryid3] +
										'</labelabc>' +
									'</li>';
                }
            }
        } else {
            // No child category; select current node.
            var checked_all = industry_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
            if ($(this).hasClass('industryid2_all')) {
                // Check whether all items are selected.
                if (checked_all) {
                    $(this).addClass('selected').siblings().removeClass('selected');
                } else {
                    $(this).removeClass('selected').siblings().removeClass('selected');
                }
                $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
            }
        }
        if (industryid3_html != '') {
            $('.yun_tck_con_list_industry3').show();
            $('.yun_tck_con_list_industry3 ul').html(industryid3_html);
        }
    });
    $('#industrydiv').delegate('.yun_tck_con_list_industry3 ul .industryid3', 'click', function () {
        if ($(this).siblings('.industryid3_all').length > 0) {
            if ($(this).siblings('.industryid3_all').hasClass('selected')) {
                return;
            }
        }
        // No child category; select current node.
        var checked_all = industry_item_select($(this).attr('codeid'), $(this).attr('codename'), 1, $(this));
        if ($(this).hasClass('industryid3_all')) {
            // Check whether all items are selected.
            if (checked_all) {
                $(this).addClass('selected').siblings().removeClass('selected');
            } else {
                $(this).removeClass('selected').siblings().removeClass('selected');
            }
            $(this).siblings().each(function () { $(this).find('input')[0].checked = checked_all; if (checked_all) { $(this).find('input').attr('disabled', 'disabled'); } else { $(this).find('input').removeAttr('disabled'); } });
        }
    });
    $('#industrydiv').delegate('.yun_tit_selected .selected .delete', 'click', function () {
        var codeid = $(this).parent().parent().attr('codeid');
        $('#industrydiv li[codeid=' + codeid + ']').removeClass('selected');
        $('#industrydiv li[codeid=' + codeid + ']').find('input')[0].checked = false;
        $(this).parent().parent().remove();
    });
    $('#industrydiv').delegate('.yun_tck_tit_close,#cancel_btn', 'click', function () {
        layer.closeAll();
    });
    $('#industrydiv').delegate('#btnSubmitJobsort', 'click', function () {
        confirm_selected_industry_items();
    });
});
function get_industry_deep() {
    var ht_length = 0, hi_length = 0;
    for (var j = 0; j <= ht.length; j++) {
        if (ht[j]) {
            ht_length++;
        }
    }
    for (var j = 0; j <= hi.length; j++) {
        if (hi[j]) {
            hi_length++;
        }
    }
    if ((ht_length > 0) && (hi_length < ht_length)) {
        window.industry_deep = 3;
    } else if ((ht_length > 0) && (hi_length == ht_length)) {
        window.industry_deep = 2;
    } else {
        window.industry_deep = 1;
    }
    return window.industry_deep;
}
// Select industry category item.
function industry_item_select(industry_id, industry_name, type, industry_element) {
    // Single-select mode.
    if (window.allow_select_industry_count == 1) {
        $('#industrydiv .yun_tit_selected .selected').html('');
        $('#industrydiv .yun_tit_selected .selected').append('<li codeid="' + industry_id + '" codename="' + industry_name + '">' +
							'<a class="clean g3 selall" href="javascript:;">' +
								'<span class="text">' +
									industry_name +
								'</span>' +
								'<span class="delete">' +
									classPublicT('class_js_00003', null, 'Remove') +
								'</span>' +
							'</a>' +
						'</li>');
        $(industry_element).addClass('selected').siblings().removeClass('selected');
        //confirm_selected_industry_items()
        //layer.close(window.industry_layer);
        //return;
    } else {
        var industry_items = $('#industrydiv .yun_tit_selected .selected li');
        // Check whether it is already selected.
        for (var i = 0; i < industry_items.length; i++) {
            if ($(industry_items[i]).attr('codeid') == industry_id) {
                if ($(industry_items[i]).find('input').is(":hidden")) {
                    $('#industrydiv li[codeid=' + $(industry_items[i]).attr('codeid') + ']').removeClass('selected');
                    $(industry_element).find('input')[0].checked = false;
                    return false;
                } else {
                    $(industry_items[i]).find('.delete').click();
                    $('#industrydiv li[codeid=' + $(industry_items[i]).attr('codeid') + ']').removeClass('selected');
                    $(industry_element).find('input')[0].checked = false;
                    return false;
                }
            }
            // Check whether it is a child of the selected item.
            if (typeof (ht[industry_id]) == 'object') {
                if (ht[industry_id].length > 0) {
                    for (var j = 0; j < ht[industry_id].length; j++) {
                        if (ht[industry_id][j] == $(industry_items[i]).attr('codeid')) {
                            $(industry_items[i]).find('.delete').click();
                            $('#industrydiv li[codeid=' + $(industry_items[i]).attr('codeid') + ']').removeClass('selected');
                        }
                    }
                }
            }
        }
        // Check whether the limit is exceeded.
        if (industry_items.length >= parseInt(window.allow_select_industry_count)) {
			layer.msg(classPublicT('class_js_00004', {count: parseInt(window.allow_select_industry_count)}, 'Select no more than {count} items!'), 2, 8);return false;
            $(industry_element).find('.delete').click();
            $('#industrydiv li[codeid=' + industry_id + ']').removeClass('selected');
            $('#industrydiv li[codeid=' + industry_id + ']').find('input')[0].checked = false;
            return false;
        }
        $(industry_element).find('input')[0].checked = true;
        $('#industrydiv li[codeid=' + industry_id + ']').addClass('selected');
        $('#industrydiv .yun_tit_selected .selected').append('<li codeid="' + industry_id + '" codename="' + industry_name + '">' +
                                '<a class="clean g3 selall" href="javascript:;">' +
                                    '<span class="text">' +
                                        industry_name +
                                    '</span>' +
                                    '<span class="delete">' +
                                        classPublicT('class_js_00003', null, 'Remove') +
                                    '</span>' +
                                '</a>' +
                            '</li>');
    }
    return true;
}
// Confirm selected industry category items.
function confirm_selected_industry_items() {
    // Check whether any item is selected.
    var industry_items = $('#industrydiv .yun_tit_selected .selected li');
    var industry_ids = '';
    var industry_names = '';
    for (var i = 0; i < industry_items.length; i++) {
        industry_ids += ',' + $(industry_items[i]).attr('codeid');
        industry_names += ',' + $(industry_items[i]).attr('codename');
    }
	 if(industry_names.length<=0){
        layer.msg(classPublicT('class_js_00005', null, 'Select a specific category!'), 2, 8);return false;
    }else{
    // Write selected IDs and names to target elements.
    if (window.target_industryin_names_tagname == 'INPUT') {
        $(window.target_industryin_names).val(industry_names.substring(1));
    } else {
        $(window.target_industryin_names).html(industry_names.substring(1));
    }
    if (window.target_industryin_ids_tagname == 'INPUT') {
        $(window.target_industryin_ids).val(industry_ids.substring(1));
    } else {
        $(window.target_industryin_ids).html(industry_ids.substring(1));
    }
    if (window.index_industry_callback) {
        window.index_industry_callback();
    }
    layer.closeAll();
    return true;
	} 
}
// Industry selector supports single/multiple select, max count, and target element type.
function index_industry(allow_select_industry_count, target_industryin_names, target_industryin_ids, industrydiv_style, codeids, index_industry_callback) {

    if ($(target_industryin_names).length <= 0) { layer.msg(classPublicT('class_js_00010', null, 'Industry category name target element does not exist!'), 2, 8);return false;}
    if ($(target_industryin_ids).length <= 0) { layer.msg(classPublicT('class_js_00011', null, 'Industry category ID target element does not exist!'), 2, 8);return false;}
    // Maximum selectable count; 1 means single-select.
    window.allow_select_industry_count = allow_select_industry_count;
    // Name target selector.
    window.target_industryin_names = target_industryin_names;
    // ID target selector.
    window.target_industryin_ids = target_industryin_ids;
    // Name target element type: html() or val().
    window.target_industryin_names_tagname = $(target_industryin_names)[0].nodeName;
    // ID target element type: html() or val().
    window.target_industryin_ids_tagname = $(target_industryin_ids)[0].nodeName;
    // Popup style.
    window.industrydiv_style = industrydiv_style;
    // Callback after confirming selection.
    window.index_industry_callback = index_industry_callback;
    // Determine whether checkbox input is needed.

    // Calculate category depth.
    var industry_deep = get_industry_deep();
    switch (industry_deep) {
        case 1:
            window.industry1_checkbox_type = 'checkbox';
            window.industry2_checkbox_type = 'hidden';
            window.industry3_checkbox_type = 'hidden';
            $('.yun_tck_con_list').hide();
            $('.yun_tck_con_list_industry1').show();
            break;
        case 2:
            window.industry1_checkbox_type = 'hidden';
            window.industry2_checkbox_type = 'checkbox';
            window.industry3_checkbox_type = 'hidden';
            $('.yun_tck_con_list_industry1').show();
            $('.yun_tck_con_list_industry2').show();
            break;
        case 3:
            window.industry1_checkbox_type = 'hidden';
            window.industry2_checkbox_type = 'hidden';
            window.industry3_checkbox_type = 'checkbox';
            $('.yun_tck_con_list_industry1').show();
            $('.yun_tck_con_list_industry2').show();
            $('.yun_tck_con_list_industry3').show();
            break;
        default: break;
    }
    // Single-select mode.
    if (window.allow_select_industry_count == 1) {
        window.industry1_checkbox_type = 'hidden';
        window.industry2_checkbox_type = 'hidden';
        window.industry3_checkbox_type = 'hidden';
    }

    //$("#industrydiv").attr('style',$("#industrydiv").attr('style')+';'+window.industrydiv_style);
    var html = $("#industrydiv").html();
    if (html.replace(" ", "") == '') {
        var codeids_list = (codeids) ? codeids.split(',') : (new Array());
        var codeids_html = '';
        for (var i = 0; i < codeids_list.length; i++) {
            var codeid = codeids_list[i];
            var codename = hyname[codeid];
            codeids_html += '<li codeid="' + codeid + '" codename="' + codename + '">' +
							'<a class="clean g3 selall" href="javascript:;">' +
								'<span class="text">' +
									codename +
								'</span>' +
								'<span class="delete">' +
									classPublicT('class_js_00003', null, 'Remove') +
								'</span>' +
							'</a>' +
						'</li>';
        }
        var industry1_html = '';
        for (var i = 0; i < hi.length; i++) {
            var industryid1 = hi[i];
            industry1_html += '<li class="industryid1" codeid="' + industryid1 + '" codename="' + hyname[industryid1] + '">' +
                                    '<labelabc for="industryid1_' + hyname[industryid1] + '"><input type="' + window.industry1_checkbox_type + '" name="industryid1" class="industryid1_checkbox" id="industryid1_' + hyname[industryid1] + '"/>' +
                                        hyname[industryid1] +
                                    '</labelabc>' +
                                '</li>';
        }
        html = '<div class="yun_tck">' +
            '<div class="yun_tck_box">' +
                '<div class="yun_tck_tit">' +
                    '<span class="yun_tck_tit_span">' +
                        classPublicT('class_js_00014', null, 'Industry Category') +
                    '</span>' +
                    '<a href="javascript:;" class="yun_tck_tit_close">' +
                        classPublicT('class_js_00015', null, 'Close') +
                    '</a>' +
                '</div>' +
				'<div class="yun_tck_title">' +
                    '<div class="yun_tck_title_box">' +
                        '<div class="yun_tck_tit_xz">' +
                            '<span class="yun_tck_tit_xz_l">' +
                                classPublicT('class_js_00016', null, 'Selected:') +
                            '</span>' +
                            '<span class="yun_tck_tit_xz_r">' +
                                classPublicT('class_js_00017', {count: allow_select_industry_count}, '(Select up to {count} items)') +
                            '</span>' +
                        '</div>' +
						'<div class="yun_tit_selected">' +
                            '<ul class="selected clearfix">' +
								codeids_html +
                            '</ul>' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '<div class="clear">' +
                '</div>' +
				'<div class="dropLst">' +
                    '<div class="yun_tck_con">' +
                        '<div class="yun_tck_con_list yun_tck_con_list_industry1">' +
                            '<ul>' +
                                industry1_html +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_industry2">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
						'<div class="yun_tck_con_list yun_tck_con_list_industry3">' +
                            '<ul>' +
                            '</ul>' +
                        '</div>' +
                        '<div class="clear">' +
                        '</div>' +
                    '</div>' +
                '</div>' +
                '</span>' +
            '</div>' +
            '<div class="clear">' +
            '</div>' +
            '<div class="actions">' +
                '<button class="button_a button_a_red" id="btnSubmitJobsort">' +
                    classPublicT('class_js_00018', null, 'Confirm') +
                '</button>' +
                '<button class="button_a" id="cancel_btn">' +
                    classPublicT('class_js_00019', null, 'Cancel') +
                '</button>' +
            '</div>' +
        '</div>';
        html += ' </tbody></table></div>';
        $("#industrydiv").html(html);
    } else {
        $("#industrydiv").html(html);
    }
    window.industry_layer = $.layer({
        type: 1,
        title: false,
        offset: ['100px', ($(window).width() - 620) / 2 + 'px'],
        closeBtn: [0, false],
        fix: false,
        border: [0, 0.3, '#000', true],
        move: false,
        area: ['620px', '440px'],
        page: { dom: '#industrydiv' }
    });
}
// Industry selector popup end.