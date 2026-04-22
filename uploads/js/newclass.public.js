/*********************************职位类别选择开始*******************************/
function index_job_new(allow_select_jobclass_count, target_jobclassin_names, target_jobclassin_ids,jobdiv_style, isShowBigClass) {
	window.allow_select_jobclass_count = allow_select_jobclass_count;//选择数量
	window.target_jobclassin_names = target_jobclassin_names;//点击id
	window.target_jobclassin_ids = target_jobclassin_ids;//选择类别存放的id
	window.isShowBigClass = isShowBigClass;//允许选择二级
	if(window.allow_select_jobclass_count==1){
		window.style="style=\"visibility:hidden;\"";
	}else{
		window.style="style=\"\"";
	}

	var layerHtml = [];
	layerHtml.push("<div class=\"layerContain\"  id=\"ChooseJobClassLayer\">");

	layerHtml.push("<div class=\"lbsx_box\"><input id=\"ChooseJobSearch\" type=\"text\" class=\"sx_input_n\" value=\"搜索职位类别\" /></div><div class=\"layerHead\"><div class=\"chose\"><h4  class=\"layerHead_h4\">最多可选择"+window.allow_select_jobclass_count+"项：</h4><div class=\"ShowJobClass\"></div><a style=\" cursor:pointer;\" target=\"_self\" class=\"RemoveJobClass\"></a></div><button class=\"queding\">确定</button></div><div class=\"layerNei\"><ul class=\"z_prof_n\" style=\"display:block;\"></ul><ul class=\"t_sort_n\" style=\"display:none;\"></ul><ul class=\"nonesearch\" style=\"display:none;\"><div style=\"text-align:center; font-weight:bold; margin-top:250px;\">抱歉，没有找到结果！</div></ul></div></div>");

	$("body").append(layerHtml.join(""));
	var listHtml = [];
	var num = 0;
	$(ji).each(function (i, data) {//一级类别显示
		listHtml.push("<li class=\"category_list\"><div class=\"category\" style=\"\">"+jn[data]+"</div><div class=\"detail\" ><ul>");
		$(jt[data]).each(function(i,data2){//二级类别显示
			if(jt[data2]=='undefined'||!jt[data2]){//没有三级类别
				if (isShowBigClass == "1") {
					listHtml.push("<li class=\"twojobclass\"><span class=\"big-class-icon reduce-icon\"></span><label><input "+window.style+" type=\"checkbox\" value=\"" + data2 + "\" name=\"" + jn[data2] + "\" />"+jn[data2]+"</label></li>");
				}else {
					listHtml.push("<li code=\""+data2+"\" name=\""+jn[data2]+"\" ><span class=\"big-class-icon  reduce-icon\"></span>"+jn[data2]+"<div class=\"flo\"></div></li>")
				}
			}else{//有三级类别，
				listHtml.push("<li code=\""+data2+"\" name=\""+jn[data2]+"\"><span class=\"big-class-icon\"></span>"+jn[data2]+"<div class=\"flo\"></div></li>")
			}
		});
		listHtml.push("</ul></div><div class=\"clear\"></div></li>");
	});
	$(".z_prof_n").html(listHtml.join(""));
	$("#ChooseJobClassLayer .z_prof_n").children("li:even").addClass("li_2");
	$(".z_prof_n>li").each(function () {
		var num = $(this).find(".detail li").length;
		if (num < 4) {
			$(this).find(".category").css({ "line-height": "20px" });
		} else if (num < 7) {
			$(this).find(".category").css({ "line-height": "40px" });
		} else {
			$(this).find(".category").css({ "line-height": "60px" });
		}
	})
	if ($(""+window.target_jobclassin_ids+"").val().length == 0) {
		$("#ChooseJobClassLayer .ShowJobClass").html("");
	}else {
		var JobClassArray = $(""+window.target_jobclassin_ids+"").val().length > 0 ? $(""+window.target_jobclassin_ids+"").val().split(",") : new Array(0);
		$("#ChooseJobClassLayer .ShowJobClass div").remove();
		$(ji).each(function (i, data) {
			$(jt[data]).each(function(i,data2){
				for(var i=0;i<JobClassArray.length;i++){
					if(JobClassArray[i]==data2){
						$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" code=\"" + data2+ "\" type=\"big\" title=\"" + jn[data2] + "\" ><span>" + jn[data2] + "</span><label class=\"job_lab \"></label></div>");
					}
				}
				if(jt[data2]=='undefined'||!jt[data2]){
					if (isShowBigClass == "1") {
						if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + data2 + "\"]").length > 0) {
							$("#ChooseJobClassLayer :input[value=\"" + data2+ "\"]").prop({ "checked": true });
						}
					}
				}
				$(jt[data2]).each(function (i, data3) {
					for(var i=0;i<JobClassArray.length;i++){
						if(JobClassArray[i]==data3){
							$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" parentcode=\"" +data2 + "\" type=\"small\" title=\"" + jn[data3]+ "\"  code=\"" + data3+ "\"><span>" + jn[data3]+ "</span><label class=\"job_lab\"></label></div>");
						}
					}
				});
			});
		});
		showcheckedclassstyle();

	}
	window.newjob_layer = $.layer({
		type : 1,
		title :'请选择职位类别',
		shift : 'top',
		closeBtn : [0 , true],
		border : [10 , 0.3 , '#000', true],
		area: ['830px', '620px'],
		page : {dom :"#ChooseJobClassLayer"},
		close: function(index){
			$("#ChooseJobClassLayer").remove();
		}
	});

}
//已选择类别里显示的样式
function showcheckedclassstyle(){
	if(window.allow_select_jobclass_count==5){
		$("#ChooseJobClassLayer .ShowJobClass .job").css("width","90");
		$("#ChooseJobClassLayer .ShowJobClass .job span").css("width","70");
	}else if(window.allow_select_jobclass_count==4){
		$("#ChooseJobClassLayer .ShowJobClass .job").css("width","88");
		$("#ChooseJobClassLayer .ShowJobClass .job span").css("width","70");
	}else if(window.allow_select_jobclass_count==3){
		$("#ChooseJobClassLayer .ShowJobClass .job").css("width","118");
		$("#ChooseJobClassLayer .ShowJobClass .job span").css("width","100");
	}else if(window.allow_select_jobclass_count==2){
		$("#ChooseJobClassLayer .ShowJobClass .job").css("width","178");
		$("#ChooseJobClassLayer .ShowJobClass .job span").css("width","160");
	}
}

$(document).ready(function(){
	//下拉层效果,点击二级，显示三级类别
	$("#ChooseJobClassLayer .z_prof_n .detail li").live("click", function () {
		var checkedStr = "";
		var $this = $(this);
		var arrhtml = [];
		var bigClass = $this.attr("code");
		var bigClassName = $this.attr("name");
		if(bigClass){//没有三级类别，不显示下拉层
			//职位二级类别
			arrhtml.push("<div class=\"flo_li\"><div class=\"flo_lin twojobclass\"><label>");
			if (isShowBigClass == "1") {
				if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + $(this).attr("code") + "\"]").length > 0) {
					checkedStr = "checked=\"checked\""
				}
				arrhtml.push("<input "+window.style+" type=\"checkbox\" " + checkedStr + " value=\"" + bigClass + "\" name=\"" + bigClassName + "\" /><span class=\"big-class-icon reduce-icon\"></span>" + bigClassName + "</label></div>");
			}else {
				arrhtml.push(bigClassName + "</div>");
			}
			arrhtml.push("</div>");
			arrhtml.push("<div class=\"hover_div\">");
			//职位三级类别
			var disabledStr = "";
			if (checkedStr.length > 0) {
				disabledStr = "disabled=\"disabled\" checked=\"checked\"";
			}
			$(jt[bigClass]).each(function (i, data) {
				checkedStr = "";
				if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" +data+ "\"]").length > 0) {
					checkedStr = "checked=\"checked\""
				}
				arrhtml.push("<div class=\"inp_n threejobclass\"><label><input "+window.style+" type=\"checkbox\" " + checkedStr + " " + disabledStr + " value=\"" + data + "\" bigclass=\"" + bigClass + "\" name=\"" + jn[data] + "\" />" +jn[data] + "</label></div>");
			});
			arrhtml.push("</div>");
			$this.find("div").html(arrhtml.join(""));
			var lihtml = $this.html();
			var l = $(this).position().left;
			var h = $(this).position().top;
			if (l > 416) {
				$(this).find(".flo_lin").css({"width":"184px","padding-left":"9px"});
				if (l > 416 && h > 338) {
					$(this).find(".hover_div").removeClass().addClass("hover_div3");
				};
				$(this).find(".flo").css({ "left": "-4px" });
				$(this).find(".hover_div").removeClass().addClass("hover_div1");
			}
			if (h > 338) {
				$(this).find(".flo").removeClass().addClass("flo1");
				$(this).find(".flo_li").removeClass().addClass("flo_li1");
				$(this).find(".hover_div").removeClass().addClass("hover_div2");
				if (l > 416) {
					$(this).find(".flo1").css({ "left": "-4px" });
				}
			};
			if($(this).find(".hover_div2").length>0 ){
				var num = $(this).find(".inp_n").length;
				var lastnum = num%2;
				if(lastnum == 1){
					num = (parseInt((num/2))+1)*22;
				}else{
					num = (num/2)*22;
				}
				$(this).find(".hover_div2").css({"height": num+"px","margin-top":"-"+(num+2)+"px"});
			}
			if($(this).find(".hover_div3").length>0){
				var num = $(this).find(".inp_n").length;
				var lastnum = num%2;
				if(lastnum == 1){
					num = (parseInt((num/2))+1)*22;
				}else{
					num = (num/2)*22;
				}
				$(this).find(".hover_div3").css({"min-height": num+"px","margin-top":"-"+(num+2)+"px"});
			}
			$(this).find("div").show();
			$(this).css("z-index","2");
			$(this).unbind("click");
			$(this).find(".flo").hover(function(){$(this).find("div").show();},function(){$(this).parent().removeAttr("style");$(this).hide();});
			$(this).find(".flo1").hover(function(){$(this).find("div").show();},function(){$(this).parent().removeAttr("style");$(this).hide();});
			$(".detail li").click(function () { $(this).find("div").show(); $(this).css("z-index","2"); });
		}
	});

	$("#ChooseJobClassLayer .job_lab").live("click", function () {
		$(this).parent(".job").remove();
		$("#ChooseJobClassLayer :input[value=\"" + $(this).parent(".job").attr("code") + "\"]").prop("checked", false);
		$("#ChooseJobClassLayer :input[bigclass=\"" + $(this).parent(".job").attr("code") + "\"]").prop({ "checked": false, "disabled": false });
	});
	$("#ChooseJobClassLayer .job_lab").live("mouseenter", function () {
		$(this).addClass("job_lab_active");
	});
	$("#ChooseJobClassLayer .job_lab").live("mouseleave", function () {
		$(this).removeClass("job_lab_active");
	});
	$("#ChooseJobClassLayer .RemoveJobClass").live("click",function () {
		$("#ChooseJobClassLayer .ShowJobClass div").remove();
		$("#ChooseJobClassLayer :input").prop({ "checked": false, "disabled": false });
	});
	//确认按钮
	$("#ChooseJobClassLayer .queding").live("click", function () {
		if ($("#ChooseJobClassLayer .ShowJobClass div").length == 0) {
			if($(""+window.target_jobclassin_names+"")){
				$(""+window.target_jobclassin_names+"").val("请选择类别");
			}
			$(""+window.target_jobclassin_ids+"").val("");
		}else {
			var bigClassArry = [];
			var smallClassArray = [];
			var bigsmallClassArray = [];
			var jobClassTextArray = [];
			var bigJobClassTextArray = [];
			var smallJobClassTextArray = [];
			$("#ChooseJobClassLayer .ShowJobClass div").each(function () {
				switch ($(this).attr("type")) {
					case "big":
						bigClassArry.push($(this).attr("code"));
						bigJobClassTextArray.push($(this).attr("title"));
						break;
					case "small":
						smallClassArray.push($(this).attr("code"));

						bigsmallClassArray.push($(this).attr("parentcode")+'-'+$(this).attr("code"));
						smallJobClassTextArray.push($(this).attr("title"));
						break;
				}
			});
			if (bigJobClassTextArray.length > 0) {
				jobClassTextArray.push(bigJobClassTextArray.join(","))
			}
			if (smallJobClassTextArray.length > 0) {
				jobClassTextArray.push(smallJobClassTextArray.join(","))
			}
			if($(""+window.target_jobclassin_names+"")){
				$(""+window.target_jobclassin_names+"").val(jobClassTextArray.join(","));
			}

			var bigJobClassArray = bigClassArry.length > 0 ? bigClassArry : new Array(0);
			var smallJobClassArray = smallClassArray.length > 0 ? smallClassArray : new Array(0);
			var jobClassArray = bigJobClassArray.concat(smallJobClassArray);
			$(""+window.target_jobclassin_ids+"").val(jobClassArray.join(","));
			confirm_selected_jobclass_itemss(jobClassArray.join(","));
			$("#hidjob_class").hide();

		}
		if(typeof jobSearchReset === "function"){
			jobSearchReset();
		}

		layer.close(window.newjob_layer);
	});

	//下拉层大类点击
	$("#ChooseJobClassLayer .twojobclass label").live("click", function (e) {
		e.stopPropagation();
	});
	$("#ChooseJobClassLayer .twojobclass :input").live("change", function () {
		if ($(this).attr("checked")) {
			if(window.allow_select_jobclass_count==1){
				$("#ChooseJobClassLayer .ShowJobClass div").remove();
				$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"big\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
			}else{
				$("#ChooseJobClassLayer :input[bigclass=\"" + $(this).val() + "\"]").prop({ "disabled": true, "checked": true });
				$("#ChooseJobClassLayer .ShowJobClass div[parentcode=\"" + $(this).val() + "\"]").remove();
				if ($("#ChooseJobClassLayer .ShowJobClass").find("div").length >=window.allow_select_jobclass_count) {
					$(this).prop("checked", false);
					layer.msg('最多选择'+window.allow_select_jobclass_count+'项！',2,8);
					$("#ChooseJobClassLayer :input[bigclass=\"" + $(this).val() + "\"]").prop({ "disabled": false, "checked": false });
					$("#ChooseJobClassLayer .ShowJobClass div[parentcode=\"" + $(this).val() + "\"]").remove();
					return false;
				}else {
					$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"big\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
				}
			}
		}else {
			$("#ChooseJobClassLayer :input[bigclass=\"" + $(this).val() + "\"]").prop({ "disabled": false, "checked": false });
			$("#ChooseJobClassLayer .ShowJobClass div[code=\"" + $(this).val() + "\"]").remove();
		}
		showcheckedclassstyle();
	});
	//下拉小类点击
	$("#ChooseJobClassLayer .threejobclass label").live("click", function (e) {
		e.stopPropagation();
	});
	$("#ChooseJobClassLayer .threejobclass :input").live("change", function () {
		if ($(this).attr("checked")) {
			if(window.allow_select_jobclass_count==1){
				$("#ChooseJobClassLayer .ShowJobClass div").remove();
				$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"small\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
			}else{
				if ($("#ChooseJobClassLayer .ShowJobClass").find("div").length >= window.allow_select_jobclass_count) {
					$(this).prop("checked", false);
					layer.msg('最多选择'+window.allow_select_jobclass_count+'项！',2,8);
					return false;
				} else {
					$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"small\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
				}
			}
			showcheckedclassstyle();
		}else {
			$("#ChooseJobClassLayer .ShowJobClass div[code=\"" + $(this).val() + "\"]").remove();
		}
	});
	//根据搜索结果，点击选择
	$("#ChooseJobClassLayer .t_sort_n :input").live("change", function () {
		var ischeck = $(this).prop("checked");
		var bigvalue = $(this).val();
		if ($(this).attr("typeid") == "big") {//选择2级
			if (ischeck) {
				if(window.allow_select_jobclass_count==1){
					$("#ChooseJobClassLayer .ShowJobClass div").remove();
					$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"big\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
				}else{
					$("#ChooseJobClassLayer .t_sort_n :input[bigclass='" + bigvalue + "']").attr({ "disabled": ischeck, "checked": true });
					$("#ChooseJobClassLayer .ShowJobClass div[parentcode=\"" + $(this).val() + "\"]").remove();
					if ($("#ChooseJobClassLayer .ShowJobClass").find("div").length >=window.allow_select_jobclass_count) {
						$(this).prop("checked", false);
						$("#ChooseJobClassLayer .t_sort_n :input[bigclass='" + bigvalue + "']").prop({ "disabled": false, "checked": false });
						layer.msg('最多选择'+window.allow_select_jobclass_count+'项！',2,8);
						return false;
					}else {
						$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"big\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
					}
				}
			}else {
				$("#ChooseJobClassLayer .t_sort_n :input[bigclass='" + bigvalue + "']").prop({ "disabled": false, "checked": false });
				$("#ChooseJobClassLayer .ShowJobClass div[code=\"" + $(this).val() + "\"]").remove();
			}
		}else {//选择3级
			if(ischeck) {
				if(window.allow_select_jobclass_count==1){
					$("#ChooseJobClassLayer .ShowJobClass div").remove();
					$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"small\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
				}else{
					if ($("#ChooseJobClassLayer .ShowJobClass").find("div").length >= window.allow_select_jobclass_count) {
						$(this).prop("checked", false);
						layer.msg('最多选择'+window.allow_select_jobclass_count+'项！',2,8);
						return false;
					}else {
						$("#ChooseJobClassLayer .ShowJobClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"small\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
					}
				}
			}else {
				$("#ChooseJobClassLayer .ShowJobClass div[code=\"" + $(this).val() + "\"]").remove();
			}
		}
		showcheckedclassstyle();
	});

	//搜索
	$("#ChooseJobSearch").live("focus", function () {
		if ($(this).val() == this.defaultValue) {
			$(this).val("");
		}
	}).live("blur",function () {
		if ($(this).val() == '') {
			$(this).val(this.defaultValue);
		}
	});
	$("#ChooseJobSearch").live("keyup",function () {
		var $this = $(this);
		var txt = $this.val();
		if (txt.length == 0) {//没有关键字，显示全部
			$("#ChooseJobClassLayer .z_prof_n").show();
			$("#ChooseJobClassLayer .t_sort_n").hide();
			$("#ChooseJobClassLayer .nonesearch").hide();
		} else {
			//搜索数据源
			var searchHtml = [];
			jn.forEach(function(data,i) {
				if(ji.indexOf(i) <0){
					if (data.toString().indexOf(txt) > -1) {//2级里有关键字，显示关键字
						var dataname=false;
						$(ji).each(function (onek, onev) {
							$(jt[onev]).each(function(twok,twov){
								if(twov==i){
									dataname=true;
								}
							});
						});
						if(typeof jt[i]!= "undefined"||dataname){
							searchHtml.push("<li style=\"\"><div class=\"category\" style=\" color:#9a326a; text-align:left;text-overflow:ellipsis; overflow:hidden; white-space:nowrap; line-height:20px;\">");
							searchHtml.push("<label title=\"" + data + "\" style=\"padding-left:8px;\" >");
							if (window.isShowBigClass == "1") {
								searchHtml.push("<input "+window.style+" type=\"checkbox\" typeid=\"big\" title=\"" + data + "\" name=\"" + data + "\" value=\"" + i + "\"");
								if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + i+ "\"]").length > 0) {
									searchHtml.push(" checked=\"checked\" ");
								}
								searchHtml.push(" />");
							}
							searchHtml.push(data + "</label></div>");
							searchHtml.push("<div class=\"detail\"><ul>");
							var smallSearch1 = [];//3级里有关键字，显示关键字
							var smallSearch2 = [];//3级里没有关键字显示区别
							$(jt[i]).each(function (i2, n) {
								var isbigcheck = $("#ChooseJobClassLayer .ShowJobClass div[code=\"" + i+ "\"]").length > 0;//2级是否已选择
								if (jn[n].toString().indexOf(txt) > -1) {//3级里有关键字，显示关键字
									smallSearch1.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + jn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" + jn[n] + "\" bigclass=\"" + i + "\" value=\"" + n + "\" ");
									if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + i+ "\"]").length > 0) {//2级已选择，3级显示全选
										smallSearch1.push(" checked=\"checked\" ");
									}
									if (isbigcheck) {//2级已选择，3级显示不可选
										smallSearch1.push(" disabled=\"disabled\" ");
									}
									smallSearch1.push(" />" + jn[n] + "</label></div></li>");
								}
								smallSearch2.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + jn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" +jn[n] + "\" bigclass=\"" + i+ "\" value=\"" + n + "\" ");
								if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + i+ "\"]").length > 0) {//2级已选择，3级显示全选
									smallSearch2.push(" checked=\"checked\" ");
								}
								if (isbigcheck) {//2级已选择，3级显示不可选
									smallSearch2.push(" disabled=\"disabled\" ");
								}
								smallSearch2.push(" />" +jn[n] + "</label></div></li>");
							});
							searchHtml.push(smallSearch1.length == 0 ? smallSearch2.join("") : smallSearch1.join(""));
							searchHtml.push("</ul></div></li>");
						}

					}else {
						//2级没有关键字，查询三级
						var smallSearch1 = [];
						$(jt[i]).each(function (i2, n) {
							if (jn[n].toString().indexOf(txt) > -1) {
								var isbigcheck = $("#ChooseJobClassLayer .ShowJobClass div[code=\"" +i + "\"]").length > 0;
								smallSearch1.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + jn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" + jn[n] + "\" bigclass=\"" + i + "\" value=\"" + n + "\" ");
								if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" + n + "\"]").length > 0) {
									smallSearch1.push(" checked=\"checked\" ");
								}
								if (isbigcheck) {
									smallSearch1.push(" disabled=\"disabled\" ");
								}
								smallSearch1.push(" />" + jn[n] + "</label></div></li>");
							}
						});
						if (smallSearch1.length > 0) {
							searchHtml.push("<li style=\"\"><div class=\"category\" style=\" color:#9a326a; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap; line-height:20px;\">");
							searchHtml.push("<label title=\"" + data + "\" style=\"padding-left:8px;\">");
							if (window.isShowBigClass == "1") {
								searchHtml.push("<input "+window.style+" type=\"checkbox\" typeid=\"big\" title=\"" + data + "\" name=\"" + data + "\" value=\"" + i + "\"");
								if ($("#ChooseJobClassLayer .ShowJobClass div[code=\"" +i + "\"]").length > 0) {
									searchHtml.push(" checked=\"checked\" ");
								}
								searchHtml.push(" />");
							}
							searchHtml.push(data+ "</label></div>");
							searchHtml.push("<div class=\"detail\"><ul>");
							searchHtml.push(smallSearch1.join(""));
							searchHtml.push("</ul></div></li>");
						}
					}
				}

			});
			if (searchHtml.length == 0) {//没有关键字
				$("#ChooseJobClassLayer .t_sort_n").hide();
				$("#ChooseJobClassLayer .nonesearch").show();
			}else {
				$("#ChooseJobClassLayer .t_sort_n").html(searchHtml.join(""));
				$("#ChooseJobClassLayer .t_sort_n").children("li:odd").addClass("li_3");
				$("#ChooseJobClassLayer .t_sort_n").show();
				$("#ChooseJobClassLayer .nonesearch").hide();
			}
			$("#ChooseJobClassLayer .z_prof_n").hide();
		}
	})
})
/*********************************职位类别选择结束*******************************/
//查询职位描述模板--start
function confirm_selected_jobclass_itemss(id){

	$.post(weburl+"/index.php?m=ajax&c=getcontent",{ids:id},function(data){
		if(data){
			var datas=data.split('@@@@');
			for(var i=0;i<datas.length;i++){
				var ndata=datas[i].split('###');
				$("#JobRequInfoTemplate").html("<a href=\"javascript:void(0)\" onclick=\"setexample('"+ndata[0]+"')\">"+ndata[1]+"</a>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;");
			}
			$(".Description").show();
		}else{
			$("#JobRequInfoTemplate").html();
			$(".Description").hide();
		}
	});

	return true;
}
function setexample(id){
	$.post(weburl+"/index.php?m=ajax&c=setexample",{id:id},function(data){
		if(data){
			editor.setContent(data);
		}
	});

}
//查询职位描述模板--END
/*********************************城市类别选择开始*******************************/
function index_city_new(allow_select_cityclass_count, target_cityclass_names, target_cityclass_ids,citydiv_style, codeids,isShowBigCity) {
	window.allow_select_cityclass_count = allow_select_cityclass_count;//选择数量
	window.target_cityclass_names = target_cityclass_names;//点击id
	window.target_cityclass_ids = target_cityclass_ids;//选择类别存放的id
	window.isShowBigCity	=	isShowBigCity ? isShowBigCity :1;
	if(window.allow_select_cityclass_count==1){
		window.style="style=\"visibility:hidden;\"";
	}else{
		window.style="style=\"\"";
	}
	var areaHtml = [];
	areaHtml.push("<div class=\"layerAreaContain\" id=\"ChooseCityClassLayer\">");

	areaHtml.push("<div class=\"lbsx_box\"><input id=\"ChooseCitySearch\" type=\"text\" class=\"sx_input_n\" value=\"搜索城市名称\" /></div><div class=\"layerHead chose_city_h1\"><div class=\"chose\"><h4 style=\"margin: 0px; padding: 2px 0 0 2px; font-size: 12px; color: #999; height: 24px;line-height: 25px; *line-height: 24px; float: left;\">您已经选择的城市是：</h4><div class=\"ShowCityClass\"></div><a style=\" cursor:pointer;\" target=\"_self\" class=\"RemoveCityClass\" ></a></div><button class=\"queding\" onclick=\"queding()\">确定</button></div><div class=\"layerNei\"><ul class=\"a_prof_n a_prof_citylist\" style=\"display:block;\"></ul><ul class=\"a_sort_n\" style=\"display:none;\"></ul><ul class=\"nonesearch\" style=\"display:none;\"><div style=\"text-align:center; font-weight:bold; margin-top:250px;\">抱歉，没有找到结果！</div></ul></div></div>");

	var n=$("#ChooseCityClassLayer").top;
	var m = $("#ChooseCityClassLayer").left;
	var inpH = $("#ChooseCityClassLayer").outerHeight();
	var f = n + inpH;
	$("body").append(areaHtml.join(""));
	$(".layerAreaContain").css({ "top": f, "left": m });
	var AreaLayer = $("#ChooseCityClassLayer");//弹层
	var listHtml = [];
	var num = 0;//二级类个数确定一级类行高
	$(ci).each(function (i, data) {
		listHtml.push("<li><div class=\"category\"><label><input "+window.style+" type=\"checkbox\" value=\""+data+"\" name=\""+cn[data]+"\" />"+cn[data]+"</label></div><div class=\"detail\" ><ul>");
		$(ct[data]).each(function(i,datas){
			listHtml.push("<li cityvalue=\""+datas+"\" parentvalue=\""+data+"\" name=\""+cn[datas]+"\"><span class=\"big-class-icon\"></span>"+cn[datas]+"<div class=\"flo\"></div></li>");
		});
		listHtml.push("</ul></div><div class=\"clear\"></div></li>");
	});
	$(".a_prof_n").html(listHtml.join(""));
	AreaLayer.find(".a_prof_n").children("li:even").addClass("li_2");
	$(".a_prof_n>li").each(function () {
		var num = $(this).find(".detail li").length;
		last = num%3;
		num = parseInt(num/3);
		if (last == 0) {
			$(this).find(".category").css({ "line-height": num*20+"px" });
		}else {
			$(this).find(".category").css({ "line-height": (num+1)*20+"px" });
		}
	})


	//选项回显
	if ($(window.target_cityclass_ids).val().length == 0) {
		$("#ChooseCityClassLayer .ShowCityClass").html("");
	}else {
		var cityClassArrayAll = $(""+window.target_cityclass_ids+"").val().length > 0 ? $(""+window.target_cityclass_ids+"").val().split(",") : new Array(0);
		$("#ChooseCityClassLayer .ShowCityClass div").remove();
		$(ci).each(function (i, data) {
			for(var i=0;i<cityClassArrayAll.length;i++){
				if(cityClassArrayAll[i]==data){
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" +data+ "\" type=\"province\" title=\"" +cn[data]+ "\" ><span>" + cn[data] + "</span><label class=\"job_lab \"></label></div>");
					$("#ChooseCityClassLayer .category :input[value=\""+data+"\"]").prop({"checked": true});
				}
			}
			$(ct[data]).each(function(i,data2){
				for(var i=0;i<cityClassArrayAll.length;i++){
					if(cityClassArrayAll[i]==data2){
						$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" provincecode=\""+data+"\" code=\"" + data2 + "\" type=\"city\" title=\"" +cn[data2]+ "\" ><span>" +cn[data2] + "</span><label class=\"job_lab \"></label></div>");
					}
				}
				$(ct[data2]).each(function (i, data3) {
					for(var i=0;i<cityClassArrayAll.length;i++){
						if(cityClassArrayAll[i]==data3){
							$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\"  provincecode=\""+data+"\"  citycode=\""+data2+"\" code=\"" + data3 + "\" type=\"area\" title=\"" + cn[data3] + "\" ><span>" + cn[data3] + "</span><label class=\"job_lab \"></label></div>");
						}
					}
				});
			});
		});
	}
	window.newcity_layer = $.layer({
		type : 1,
		title :'请选择城市',
		shift : 'top',
		closeBtn : [0 , true],
		border : [10 , 0.3 , '#000', true],
		area: ['830px', '620px'],
		page : {dom :"#ChooseCityClassLayer"},
		close: function(index){
			$("#ChooseCityClassLayer").remove();
		}
	});
}

function queding(){
	if ($("#ChooseCityClassLayer .ShowCityClass div").length == 0) {
		if($(""+window.target_cityclass_names+"")){
			$(""+window.target_cityclass_names+"").val("请选择城市");
		}

		$(""+window.target_cityclass_ids+"").val("");
	}else {
		var ProvinceClassArry = [];
		var CityClassArray = [];
		var TreeClassArray = [];
		var addressClassTextArray = [];
		var ProvinceTextArray = [];
		var CityTextArray = [];
		var ThreeTextArray = [];
		$("#ChooseCityClassLayer .ShowCityClass div").each(function () {
			switch ($(this).attr("type")) {
				case "province":
					ProvinceClassArry.push($(this).attr("code"));
					ProvinceTextArray.push($(this).attr("title"));
					break;
				case "city":
					CityClassArray.push($(this).attr("code"));
					CityTextArray.push($(this).attr("title"));
					break;
				case "area":
					TreeClassArray.push($(this).attr("code"));
					ThreeTextArray.push($(this).attr("title"));
					break;
			}
		});
		if (ProvinceTextArray.length > 0) {
			addressClassTextArray.push(ProvinceTextArray.join(","))
		}
		if (CityTextArray.length > 0) {
			addressClassTextArray.push(CityTextArray.join(","))
		}
		if (ThreeTextArray.length > 0) {
			addressClassTextArray.push(ThreeTextArray.join(","))
		}
		if($(window.target_cityclass_names)){
			$(window.target_cityclass_names).val(addressClassTextArray.join(","));
		}


		var ProvinceClassArry = ProvinceClassArry.length > 0 ? ProvinceClassArry : new Array(0);
		var CityClassArray = CityClassArray.length > 0 ? CityClassArray : new Array(0);
		var TreeClassArray = TreeClassArray.length > 0 ? TreeClassArray : new Array(0);
		var cityClassArrayAll = ProvinceClassArry.concat(CityClassArray).concat(TreeClassArray);
		$(""+window.target_cityclass_ids+"").val(cityClassArrayAll.join(","));
		$("#hidcity_class").hide();
	}
	if(typeof citySearchReset === "function"){
		citySearchReset();
	}

	layer.close(window.newcity_layer);$("#mask").remove();
}
$(document).ready(function(){
	//二级类点击
	$("#ChooseCityClassLayer .a_prof_n .detail li").live("click",function(){
		var checkedStr = "";
		var arrhtml = [];
		var $this = $(this);
		var cityClass = $this.attr("cityvalue");
		var cityClassName = $this.attr("name");
		var provinceValue = $this.attr("parentvalue");
		//市
		arrhtml.push("<div class=\"flo_li\"><div class=\"flo_lin\"><label>");
		if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + cityClass + "\"]").length > 0 || $("#ChooseCityClassLayer .ShowCityClass div[code=\"" + provinceValue + "\"]").length > 0&&window.allow_select_cityclass_count>1) {
			checkedStr = "checked=\"checked\"";
		}
		arrhtml.push("<input value=\""+cityClass+"\" "+checkedStr+" provincevalue=\""+provinceValue+"\" name=\""+cityClassName+"\" "+window.style+" type=\"checkbox\"><span class=\"big-class-icon reduce-icon\"></span>"+cityClassName+"</label></div>");
		arrhtml.push("</div>");
		arrhtml.push("<div class=\"hover_div\">");
		//区
		var disabledStr = "";
		if (checkedStr.length > 0&&window.allow_select_cityclass_count>1) {
			disabledStr = "disabled=\"disabled\" checked=\"checked\"";
		}
		$(ct[cityClass]).each(function (i, data) {
			checkedStr = "";
			if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + data + "\"]").length > 0) {
				checkedStr = "checked=\"checked\""
			}
			arrhtml.push("<div class=\"inp_n\"><label><input type=\"checkbox\" "+window.style+" "+checkedStr+" "+disabledStr+" value=\"" + data + "\" provincevalue=\"" + provinceValue + "\" cityvalue=\"" + cityClass + "\" name=\"" + cn[data] + "\" />" + cn[data] + "</label></div>");
		});
		arrhtml.push("</div>");
		$this.find("div").html(arrhtml.join(""));
		var lihtml = $this.html();
		var l = $this.position().left;
		var h = $this.position().top;
		if (l > 400) {
			if (l > 400 && h > 212) {
				$this.find(".hover_div").removeClass().addClass("hover_div3");
			};
			$this.find(".hover_div").removeClass().addClass("hover_div1");
		}
		if (h > 312) {
			$this.find(".flo").removeClass().addClass("flo1");
			$this.find(".flo_li").removeClass().addClass("flo_li1");
			$this.find(".hover_div").removeClass().addClass("hover_div2");
			if (l > 400) {
				$this.find(".flo1").css({ "left": "-4px" });
			}
		};
		if($(ct[cityClass]).length == 0){
			$this.find(".hover_div").remove();
			$this.find(".hover_div1").remove();
			$this.find(".hover_div2").remove();
			$this.find(".hover_div3").remove();
			$this.find(".flo_li").addClass("noarea");
			$this.find(".flo_li1").addClass("noarea");
		}
		$this.find("div").show();
		$this.css("z-index","2");
		$this.unbind("click");
		$this.find(".flo").hover(function(){$(this).find("div").show();},function(){$(this).parent().removeAttr("style");$(this).hide();});
		$this.find(".flo1").hover(function(){$(this).find("div").show();},function(){$(this).parent().removeAttr("style");$(this).hide();});
		$(".detail li").click(function () { $(this).find("div").show(); $(this).css("z-index","2"); });
	})
	//一级类点击
	$("#ChooseCityClassLayer .category :input").live("change",function(){
		var code = $(this).attr("value");
		var checkedStr = "";
		if(window.allow_select_cityclass_count==1){
			$("#ChooseCityClassLayer .ShowCityClass div").remove();
			$("#ChooseCityClassLayer :input[type=\"checkbox\"]").prop("checked", false);
			$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"province\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
		}else{
			if ($(this).attr("checked")) {
				$("#ChooseCityClassLayer .ShowCityClass div[provincecode=\"" + code + "\"]").remove();
				if($("#ChooseCityClassLayer .ShowCityClass").find("div").length >=window.allow_select_cityclass_count) {
					$(this).prop("checked", false);
					layer.msg('最多选择'+window.allow_select_cityclass_count+'项！',2,8);
					$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + code + "\"]").remove();
					return false;
				}else {
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + code + "\" type=\"province\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
				}
			}else{
				$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + code + "\"]").remove();
			}
		}
	})
	//下拉二级区域点击取消冒泡
	$("#ChooseCityClassLayer").find(" .flo_li label, .flo_li1 label").live("click", function (e) {
		e.stopPropagation();
	});
	$("#ChooseCityClassLayer").find(" .flo_li :input, .flo_li1 :input").live("change", function () {
		if(window.allow_select_cityclass_count==1){
			$("#ChooseCityClassLayer .ShowCityClass div").remove();
			$("#ChooseCityClassLayer :input[type=\"checkbox\"]").prop("checked", false);
			$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" provincecode=\""+$(this).attr("provincevalue")+"\" type=\"city\" title=\"" + $(this).attr("name") + "\"  ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
		}else{
			if ($(this).attr("checked")) {
				$("#ChooseCityClassLayer :input[cityvalue=\"" + $(this).val() + "\"]").prop({ "disabled": true, "checked": true });
				$("#ChooseCityClassLayer .ShowCityClass div[citycode=\"" + $(this).val() + "\"]").remove();
				if ($("#ChooseCityClassLayer .ShowCityClass").find("div").length >= window.allow_select_cityclass_count) {
					$(this).prop("checked", false);
					layer.msg('最多选择'+window.allow_select_cityclass_count+'项！',2,8);
					$("#ChooseCityClassLayer :input[cityvalue=\"" + $(this).val() + "\"]").prop({ "disabled": false, "checked": false });
					$("#ChooseCityClassLayer .ShowCityClass div[citycode=\"" + $(this).val() + "\"]").remove();
					return false;
				}else {
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" provincecode=\""+$(this).attr("provincevalue")+"\" type=\"city\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
				}
				////4.26
				var nowvalue=$(this).attr("provincevalue");
				var alllen = $(ct[nowvalue]).length;

				var provincelength = $("#ChooseCityClassLayer .ShowCityClass").find("div[provincecode="+nowvalue+"]").length;
				if(provincelength==alllen){
					$(this).parents("li").find(".category input").click();
				}
			}else {
				if($("#ChooseCityClassLayer .category :input[value=\"" + $(this).attr("provincevalue") + "\"]").attr("checked")){
					$("#ChooseCityClassLayer .category :input[value=\"" + $(this).attr("provincevalue") + "\"]").prop({ "checked": false });
					$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + $(this).attr("provincevalue") + "\"]").remove();
				}
				$("#ChooseCityClassLayer li[parentvalue=\"" + $(this).attr("provincevalue") + "\"] input").prop({ "disabled": false, "checked": false });
				$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + $(this).val() + "\"]").remove();
			}
		}
	});
	//下拉小类点击
	$("#ChooseCityClassLayer .inp_n label").live("click", function (e) {
		e.stopPropagation();
	});
	$("#ChooseCityClassLayer .inp_n :input").live("change", function () {
		if(window.allow_select_cityclass_count==1){
			$("#ChooseCityClassLayer .ShowCityClass div").remove();
			$("#ChooseCityClassLayer :input[type=\"checkbox\"]").prop("checked", false);
			$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" provincecode=\"" + $(this).attr("provincevalue") + "\"   citycode=\"" + $(this).attr("cityvalue") + "\" type=\"area\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() +  "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
		}else{
			if ($(this).attr("checked")) {
				if ($("#ChooseCityClassLayer .ShowCityClass").find("div").length >= window.allow_select_cityclass_count) {
					$(this).prop("checked", false);
					layer.msg('最多选择'+window.allow_select_cityclass_count+'项！',2,8);
					return false;
				}else {
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" provincecode=\"" + $(this).attr("provincevalue") + "\"   citycode=\"" + $(this).attr("cityvalue") + "\" type=\"area\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
				}
			}else {
				$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + $(this).val() + "\"]").remove();
			}
		}
	});
	$("#ChooseCityClassLayer .job_lab").live("click", function () {
		$(this).parent(".job").remove();
		$("#ChooseCityClassLayer :input[value=\"" + $(this).parent(".job").attr("code") + "\"]").prop("checked", false);
		$("#ChooseCityClassLayer :input[cityvalue=\"" + $(this).parent(".job").attr("code") + "\"]").prop({ "checked": false, "disabled": false });
	});
	$("#ChooseCityClassLayer .job_lab").live("mouseenter", function () {
		$(this).addClass("job_lab_active");
	});
	$("#ChooseCityClassLayer .job_lab").live("mouseleave", function () {
		$(this).removeClass("job_lab_active");
	});
	$("#ChooseCityClassLayer .RemoveCityClass").live("click",function () {
		$("#ChooseCityClassLayer .ShowCityClass div").remove();
		$("#ChooseCityClassLayer :input").prop({ "checked": false, "disabled": false });
	});
	//搜索
	$("#ChooseCitySearch").live("focus", function () {
		if ($(this).val() == this.defaultValue) {
			$(this).val("");
		}
	}).live("blur",function () {
		if ($(this).val() == '') {
			$(this).val(this.defaultValue);
		}
	});
	$("#ChooseCitySearch").live("keyup",function () {
		var $this = $(this);
		var txt = $this.val();
		if (txt.length == 0) {//没有关键字，显示全部
			$("#ChooseCityClassLayer .a_prof_n").show();
			$("#ChooseCityClassLayer .a_sort_n").hide();
			$("#ChooseCityClassLayer .nonesearch").hide();
		} else {
			//搜索数据源
			var searchHtml = [];
			var onechoosed = $("#ChooseCityClassLayer .ShowCityClass div[type=\"province\"]");
			var onechoosedArr = [];
			if(onechoosed && onechoosed.length>0){
				onechoosed.each(function(okey,oval){
					onechoosedArr.push(Number($(oval).attr("code")));
				})
			}
			cn.forEach(function(data,i) {
				if(ci.indexOf(i) <0){
					if (data.toString().indexOf(txt) > -1) {//2级里有关键字，显示关键字

						var dataname=false;
						$(ci).each(function (onek, onev) {
							$(ct[onev]).each(function(twok,twov){
								if(twov==i){
									dataname=true;
								}
							});
						});

						if(typeof ct[i]!= "undefined"||dataname){
							var upchecked	 = false;
							if(onechoosedArr.length>0){
								for(var oi=0;oi<onechoosedArr.length;oi++){
									if(onechoosedArr[oi] && (ct[onechoosedArr[oi]].indexOf(i)>-1 || ct[onechoosedArr[oi]].indexOf(i.toString())>-1) ){
										upchecked = true;
									}
								}
							}

							searchHtml.push("<li style=\"\"><div class=\"categorysc\" style=\" color:#9a326a; text-align:left;text-overflow:ellipsis; overflow:hidden; white-space:nowrap; line-height:20px;\">");
							searchHtml.push("<label title=\"" + data + "\" style=\"padding-left:8px;\" >");
							if (window.isShowBigCity == 1) {
								searchHtml.push("<input "+window.style+" type=\"checkbox\" typeid=\"big\" title=\"" + data + "\" name=\"" + data + "\" value=\"" + i + "\"");
								if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + i+ "\"]").length > 0 || upchecked) {
									searchHtml.push(" checked=\"checked\" ");
								}
								searchHtml.push(" />");
							}
							searchHtml.push(data + "</label></div>");
							searchHtml.push("<div class=\"detail\"><ul>");
							var smallSearch1 = [];//3级里有关键字，显示关键字
							var smallSearch2 = [];//3级里没有关键字显示区别



							$(ct[i]).each(function (i2, n) {
								var isbigcheck = false;
								if($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + i+ "\"]").length > 0
									|| upchecked){//2级是否已选择 或一级是否已选择
									isbigcheck = true;
								}

								if (cn[n].indexOf(txt) > -1) {//3级里有关键字，显示关键字
									smallSearch1.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + cn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" + cn[n] + "\" bigclass=\"" + i + "\" value=\"" + n + "\" ");
									if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + n+ "\"]").length > 0 || isbigcheck) {//2、1级已选择，3级显示全选 或三级已选择
										smallSearch1.push(" checked=\"checked\" ");
									}
									if (isbigcheck) {//2 1级已选择，3级显示不可选
										smallSearch1.push(" disabled=\"disabled\" ");
									}
									smallSearch1.push(" />" + cn[n] + "</label></div></li>");
								}
								smallSearch2.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + cn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" +cn[n] + "\" bigclass=\"" + i+ "\" value=\"" + n + "\" ");
								if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + n+ "\"]").length > 0 ||  isbigcheck) {//2、1级已选择，3级显示全选 或三级已选择
									smallSearch2.push(" checked=\"checked\" ");
								}
								if (isbigcheck) {//2级已选择，3级显示不可选
									smallSearch2.push(" disabled=\"disabled\" ");
								}
								smallSearch2.push(" />" +cn[n] + "</label></div></li>");
							});

							searchHtml.push(smallSearch1.length == 0 ? smallSearch2.join("") : smallSearch1.join(""));
							searchHtml.push("</ul></div></li>");
						}

					}else {
						//2级没有关键字，查询三级
						var upchecked	 = false;
						if(onechoosedArr.length>0){
							for(var oi=0;oi<onechoosedArr.length;oi++){
								if(onechoosedArr[oi] && (ct[onechoosedArr[oi]].indexOf(i)>-1 || ct[onechoosedArr[oi]].indexOf(i.toString())>-1) ){
									upchecked = true;
								}
							}
						}
						var smallSearch1 = [];
						$(ct[i]).each(function (i2, n) {
							if (cn[n].indexOf(txt) > -1) {
								var isbigcheck = $("#ChooseCityClassLayer .ShowCityClass div[code=\"" +i + "\"]").length > 0 || upchecked;
								smallSearch1.push("<li><div style=\"width:145px; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap;\"><label title=\"" + cn[n] + "\"><input "+window.style+" type=\"checkbox\" name=\"" + cn[n] + "\" bigclass=\"" + i + "\" value=\"" + n + "\" ");
								if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" + n + "\"]").length > 0 || isbigcheck) {
									smallSearch1.push(" checked=\"checked\" ");
								}
								if (isbigcheck) {
									smallSearch1.push(" disabled=\"disabled\" ");
								}
								smallSearch1.push(" />" + cn[n] + "</label></div></li>");
							}
						});

						if (smallSearch1.length > 0) {
							searchHtml.push("<li style=\"\"><div class=\"categorysc\" style=\" color:#9a326a; text-align:left; text-overflow:ellipsis; overflow:hidden; white-space:nowrap; line-height:20px;\">");
							searchHtml.push("<label title=\"" + data + "\" style=\"padding-left:8px;\">");
							if (window.isShowBigCity == "1") {
								searchHtml.push("<input "+window.style+" type=\"checkbox\" typeid=\"big\" title=\"" + data + "\" name=\"" + data + "\" value=\"" + i + "\"");
								if ($("#ChooseCityClassLayer .ShowCityClass div[code=\"" +i + "\"]").length > 0 || upchecked) {
									searchHtml.push(" checked=\"checked\" ");
								}
								searchHtml.push(" />");
							}
							searchHtml.push(data+ "</label></div>");
							searchHtml.push("<div class=\"detail\"><ul>");
							searchHtml.push(smallSearch1.join(""));
							searchHtml.push("</ul></div></li>");

						}

					}
				}

			});

			if (searchHtml.length == 0) {//没有关键字
				$("#ChooseCityClassLayer .a_sort_n").hide();
				$("#ChooseCityClassLayer .nonesearch").show();
			}else {
				$("#ChooseCityClassLayer .a_sort_n").html(searchHtml.join(""));
				$("#ChooseCityClassLayer .a_sort_n").children("li:odd").addClass("li_3");
				$("#ChooseCityClassLayer .a_sort_n").show();
				$("#ChooseCityClassLayer .nonesearch").hide();
			}
			$("#ChooseCityClassLayer .a_prof_n").hide();
		}
	})
	//根据搜索结果，点击选择
	$("#ChooseCityClassLayer .a_sort_n :input").live("change", function () {
		var ischeck = $(this).prop("checked");
		var bigvalue = $(this).val();
		var onechoosed = $("#ChooseCityClassLayer .ShowCityClass div[type=\"province\"]");
		var onechoosedArr = [];
		if(onechoosed && onechoosed.length>0){
			onechoosed.each(function(okey,oval){
				onechoosedArr.push(Number($(oval).attr("code")));
			})
		}
		if ($(this).attr("typeid") == "big") {//选择2级

			if (ischeck) {

				if(window.allow_select_cityclass_count==1){
					$("#ChooseCityClassLayer .ShowCityClass div").remove();
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"city\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
				}else{
					$("#ChooseCityClassLayer .a_sort_n :input[bigclass='" + bigvalue + "']").attr({ "disabled": ischeck, "checked": true });

					if($("#ChooseCityClassLayer .ShowCityClass div[parentcode=\"" + $(this).val() + "\"]")){//搜索后选中的三级城市
						$("#ChooseCityClassLayer .ShowCityClass div[parentcode=\"" + $(this).val() + "\"]").remove();
					}
					if($("#ChooseCityClassLayer .ShowCityClass div[citycode=\"" + $(this).val() + "\"]")){//非搜索情况下主页面选中的三级城市
						$("#ChooseCityClassLayer .ShowCityClass div[citycode=\"" + $(this).val() + "\"]").remove();
					}

					if ($("#ChooseCityClassLayer .ShowCityClass").find("div").length >=window.allow_select_cityclass_count) {
						$(this).prop("checked", false);
						$("#ChooseCityClassLayer .a_sort_n :input[bigclass='" + bigvalue + "']").prop({ "disabled": false, "checked": false });
						layer.msg('最多选择'+window.allow_select_cityclass_count+'项！',2,8);
						return false;
					}else {
						$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" code=\"" + $(this).val() + "\" type=\"city\" title=\"" + $(this).attr("name") + "\" ><span>" + $(this).attr("name") + "</span><label class=\"job_lab \"></label></div>");
					}
				}
			}else {
				$("#ChooseCityClassLayer .a_sort_n :input[bigclass='" + bigvalue + "']").prop({ "disabled": false, "checked": false });
				$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + $(this).val() + "\"]").remove();


				if(onechoosedArr.length>0){
					for(var i=0;i<onechoosedArr.length;i++){
						if(ct[onechoosedArr[i]].indexOf(bigvalue)!=-1 || ct[onechoosedArr[i]].indexOf(Number(bigvalue))!=-1){
							$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + onechoosedArr[i] + "\"]").remove();
						}
					}
				}
			}
		}else {//选择3级
			if(ischeck) {
				if(window.allow_select_cityclass_count==1){
					$("#ChooseCityClassLayer .ShowCityClass div").remove();
					$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"area\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
				}else{
					if ($("#ChooseCityClassLayer .ShowCityClass").find("div").length >= window.allow_select_cityclass_count) {
						$(this).prop("checked", false);
						layer.msg('最多选择'+window.allow_select_cityclass_count+'项！',2,8);
						return false;
					}else {
						$("#ChooseCityClassLayer .ShowCityClass").append("<div class=\"job\" parentcode=\"" + $(this).attr("bigclass") + "\" type=\"area\" title=\"" + $(this).attr("name") + "\"  code=\"" + $(this).val() + "\"><span>" + $(this).attr("name") + "</span><label class=\"job_lab\"></label></div>");
					}
				}
			}else {
				$("#ChooseCityClassLayer .ShowCityClass div[code=\"" + $(this).val() + "\"]").remove();
			}
		}
	});
})
/*********************************城市类别选择结束*******************************/