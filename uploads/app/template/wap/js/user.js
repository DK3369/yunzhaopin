// Header save handler
function headSave(type){
	if(type == 'info'){
		saveInfo();
	}else if(type == 'addexpect'){
		saveExpect();
	}
}
function saveExpect(){
	var field = getFormValue('addexpectForm');
	var i18n = typeof USER_EXPECT_I18N !== 'undefined' ? USER_EXPECT_I18N : {};

	if (field.name == '') {
		return showToast(i18n.fillExpectJob);
	}else if(field.name.length > field.sy_rname_num){
		return showToast(i18n.expectJobMaxPrefix + field.sy_rname_num + i18n.charSuffix);
	}
	if (field.jobclassid == '') {
		return showToast(i18n.selectExpectJob);
	}
	if (field.city_classid == '') {
		return showToast(i18n.selectExpectCity);
	}

	if (field.type == '') {
		return showToast(i18n.selectWorkType);
	}
	if (field.report.length == 0) {
		return showToast(i18n.selectReportTime);
	}
	if (field.jobstatus == '') {
		return showToast(i18n.selectJobStatus);
	}
	if (field.minsalary.length == 0 ||field.minsalary.length == 0) {
		return showToast(i18n.fillExpectSalary);		
	} else if (parseInt(field.minsalary) > parseInt(field.maxsalary) && field.maxsalary.length > 0) {
		return showToast(i18n.salaryRangeInvalid);		
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
	showLoading(i18n.saving);
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
	var i18n = typeof USER_COMMON_I18N !== 'undefined' ? USER_COMMON_I18N : {};
	showConfirm(i18n.confirmDelete, function(){
		showLoading(i18n.deleting);
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
	var i18n = typeof USER_INFO_I18N !== 'undefined' ? USER_INFO_I18N : {};
	
	if(!field.name) {
		return showToast(i18n.fillName);
	} else {
		if(idcard_status!=1 && parseInt(resumename) && parseInt(resumename) > 0 && !isChinaName(field.name)){
			return showToast(i18n.nameFormatInvalid);
		}
	}
	if(!field.sex) {
		return showToast(i18n.selectGender);
	}
	if(!field.birthday) {
		return showToast(i18n.selectBirthday);
	}
	if(!field.edu || field.edu == 0) {
		return showToast(i18n.selectEducation);
	}
	if(!field.exp || field.exp == 0) {
		return showToast(i18n.selectExperience);
	}
	if(!field.living) {
		return showToast(i18n.fillLiving);
	}
	if(!field.telphone) {
		return showToast(i18n.fillMobile);
	}else if (!isjsMobile(field.telphone)) {
		return showToast(i18n.mobileInvalid);
	}
	if(field.email != "" && !check_email(field.email)) {
		return showToast(i18n.emailInvalid);
	}
	field.provider = 'wap';
	showLoading(i18n.saving);
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
