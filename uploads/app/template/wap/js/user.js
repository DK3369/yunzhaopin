// 头部导航栏保存
function headSave(type){
	if(type == 'info'){
		saveInfo();
	}else if(type == 'addexpect'){
		saveExpect();
	}
}
function saveExpect(){
	var field = getFormValue('addexpectForm');
	let that = this;
				
	if (field.name == '') {
		return showToast('请填写期望职位');
	}else if(field.name.length > field.sy_rname_num){
		return showToast('期望职位最多填写'+field.sy_rname_num+'个字');
	}
	if (field.jobclassid == '') {
		return showToast('请选择期望职位');
	}
	if (field.city_classid == '') {
		return showToast('请选择期望城市');
	}

	if (field.type == '') {
		return showToast('请选择工作性质');
	}
	if (field.report.length == 0) {
		return showToast('请选择到岗时间');
	}
	if (field.jobstatus == '') {
		return showToast('请选择求职状态');
	}
	if (field.minsalary.length == 0 ||field.minsalary.length == 0) {
		return showToast('请填写期望薪资');		
	} else if (parseInt(field.minsalary) > parseInt(field.maxsalary) && field.maxsalary.length > 0) {
		return showToast('请正确填写期望薪资范围');		
	}
	let formData = {		
		eid: field.eid,
		name: field.name,
		hy: field.hy,
		job_classid: field.jobclassid,
		minsalary: field.minsalary,
		maxsalary: field.maxsalary,
		city_classid: field.city_classid,
		type: field.type,
		report: field.report,
		jobstatus: field.jobstatus,
		provider: 'wap'
	};
	showLoading('保存中...');
	$.post(field.url, formData, function(data){
		hideLoading();	
		if (data.error == 1) {
			window.localStorage.setItem("needRefresh", 1);
			showToast(data.msg,2, function() {
				history.back();
			});
		}else{
			showToast(data.msg);
		}
	});
}
function headDelete(type,eid,id,url){
	let formData={};
	formData.table = type;
	formData.eid = eid;
	formData.id = id;
	showConfirm('确定删除？', function(){
		showLoading('删除中...');
		$.post(url, formData, function(data){
			hideLoading();	
			if (data.error == 1) {
				window.localStorage.setItem("needRefresh", 1);
				showToast(data.msg,2, function() {
					history.back();
				});
			}else{
				showToast(data.msg);
			}
		});
	});
}
function saveInfo(){
	var field = getFormValue('infoForm');
	var idcard_status = $("#idcard_status").val() ? $("#idcard_status").val() : 0;
	
	if(!field.name) {
		return showToast('请输入姓名！');
	} else {
		if(idcard_status!=1 && parseInt(resumename) && parseInt(resumename) > 0 && !isChinaName(field.name)){
			return showToast('姓名请输入2-6位汉字');
		}
	}
	if(!field.sex) {
		return showToast('请选择性别！');
	}
	if(!field.birthday) {
		return showToast('请选择出生年月！');
	}
	if(!field.edu || field.edu == 0) {
		return showToast('请选择最高学历！');
	}
	if(!field.exp || field.exp == 0) {
		return showToast('请选择工作经验！');
	}
	if(!field.living) {
		return showToast('请输入现居住地！');
	}
	if(!field.telphone) {
		return showToast('请输入手机！');
	}else if (!isjsMobile(field.telphone)) {
		return showToast('请输入正确的手机号！');
	}
	if(field.email != "" && !check_email(field.email)) {
		return showToast('邮箱格式错误！');
	}
	field.provider = 'wap';
	showLoading('保存中...');
	$.post(field.url, field, function(res) {
		hideLoading();
		if (res.error == 1) {
			window.localStorage.setItem("needRefresh", 1);
			showToast(res.msg, 2, function() {
				history.back();
			});
		} else {
			showToast(res.msg);
		}
	}, 'json');
}