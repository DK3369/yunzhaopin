if(typeof pickerType !== 'undefined'){
	var workYear = [],
		workMonth = [],
		defaultYear = 0,
		defaultMonth = 0,
		nowYear = timestamp.getFullYear(),
		nowMonth = timestamp.getMonth() + 1;
		
	for(let i=nowYear; i>= 1970; i--){
		workYear.push(i);
		if(nowYear == i){
			defaultYear = workYear.length - 1;
		}
	}
	for(let j=12; j>= 1; j--){
		workMonth.push(j);
		if(nowMonth == j){
			defaultMonth = workMonth.length - 1;
		}
	}
}
// 处理开始、结束时间选择器默认值
function timePicker(type, time, date){
	let year = deepClone(workYear),
		dy = deepClone(defaultYear);
	if(type == 'workList' && time == 'edate'){
		year.unshift('至今');
		dy = dy + 1;
	}else if(type == 'eduList' && time == 'edate'){
		for(let k = nowYear + 1; k<= 2030; k++){
			year.unshift(k);
		}
		dy = dy + (2030 - nowYear);
	}
	var timeData = [];
	if(date){
		// 有默认值
		let datearr = date.split('-');
		if(datearr.length == 2){
			// 非至今
			timeData = [
				{
					values: year
				},
				{
					values: workMonth
				}
			];
			year.forEach(function(val, index){
				if(val == datearr[0]){
					timeData[0].defaultIndex = index;
				}
			});
			workMonth.forEach(function(val, index){
				if(val == datearr[1]){
					timeData[1].defaultIndex = index;
				}
			});
		}else{
			// 至今
			timeData = [
				{
					values: year,
					defaultIndex: 0
				},
				{
					values: [],
					defaultIndex: 0
				}
			];
		}
	}else{
		// 没有默认值
		timeData = [
			{
				values: year,
				defaultIndex: dy
			},
			{
				values: workMonth,
				defaultIndex: defaultMonth
			}
		];
	}
	return timeData;
}
// 姓名展示形式
function namePicker(value, show = true){
	var nameData = [{value: 1, text: '完全公开'},{value: 2, text: '显示编号(例:NO.11)'},{value: 3, text: '性别称呼(例:X先生)'}],
		nameIndex = 0;
	for(let i = 0; i < nameData.length; i++){
		let val = nameData[i].value;
		if(value && value == val){
			nameIndex = i;			
		}
	}
	yunvue.$data.name = nameData;
	yunvue.$data.nameIndex = nameIndex;
	if(show){
		yunvue.$data.nameShow = true;
	}
}
// 学历
function eduPicker(value, show = true){
	var eduIndex = 0;
	var eduData = [{value: 0, text: '请选择学历'}];
	if(typeof useri.user_edu !== 'undefined'){
		var edu = useri['user_edu'];
		for(var i = 0; i < edu.length; i++){
			var val = edu[i];
			eduData.push({
				value: val,
				text: usern[val]
			});
			if(value && value == val){
				eduIndex = i + 1;
			}
		}
	}

	yunvue.$data.edu = eduData;
	yunvue.$data.eduIndex = eduIndex;
	if(show){
		yunvue.$data.eduShow = true;
	}
}
// 工作经验
function expPicker(value, show = true){
	var expIndex = 0;
	var expData = [{value: 0, text: '请选择工作经验'}];
	if(typeof useri.user_word !== 'undefined'){
		var exp = useri['user_word'];
		for(var i = 0; i < exp.length; i++){
			var val = exp[i];
			expData.push({
				value: val,
				text: usern[val]
			});
			if(value && value == val){
				expIndex = i + 1;
			}
		}
	}
	
	yunvue.$data.exp = expData;
	yunvue.$data.expIndex = expIndex;
	if(show){
		yunvue.$data.expShow = true;
	}
}
// 婚姻状况
function marriagePicker(value, show = true){
	var marriageIndex = 0;
	var	marriageData = [{value: 0, text: '请选择婚姻状况'}];
	if(typeof useri.user_marriage !== 'undefined'){
		var marriage = useri['user_marriage'];
		for(var i = 0; i < marriage.length; i++){
			var val = marriage[i];
			marriageData.push({
				value: val,
				text: usern[val]
			});
			if(value && value == val){
				marriageIndex = i + 1;
			}
		}
	}
	
	yunvue.$data.marriage = marriageData;
	yunvue.$data.marriageIndex = marriageIndex;
	if(show){
		yunvue.$data.marriageShow = true;
	}
}
// 技能熟练度
function ingPicker(value, show = true){
	var ingIndex = 0;
	var	ingData = [{value: 0, text: '请选择技能熟练度'}];
	if(typeof useri.user_ing !== 'undefined'){
		var ing = useri['user_ing'];
		for(var i = 0; i < ing.length; i++){
			var val = ing[i];
			ingData.push({
				value: val,
				text: usern[val]
			});
			if(value && value == val){
				ingIndex = i + 1;
			}
		}
	}
	
	yunvue.$data.ing = ingData;
	yunvue.$data.ingIndex = ingIndex;
	if(show){
		yunvue.$data.ingShow = true;
	}
}
// 工作性质
function typePicker(value, show = true){
	var typeIndex = 0;
	var typeData = [];
	if(typeof useri.user_type !== 'undefined'){
		var type = useri['user_type'];
		for(var i = 0; i < type.length; i++){
			var val = type[i];
			typeData.push({
				value: val,
				text: usern[val]
			});
			if(value && value == val){
				typeIndex = i ;
			}
		}
	}
	
	yunvue.$data.type = typeData;
	yunvue.$data.typeIndex = typeIndex;
	if(show){
		yunvue.$data.typeShow = true;
	}else{
		// 非必填项，初始化
		if(!value){
			yunvue.$data.type_n = typeData[0].text;
			yunvue.$data.info.type = typeData[0].value;
		}
	}
}
// 从事行业
function hyPicker(value, show = true){
	var hyIndex = 0;
	var hyData = [{value: 0, text: '不限'}];
	if(typeof hi !== 'undefined'){
		// 从事行业
		for(let i = 0; i < hi.length; i++){
			let val = hi[i];
			hyData.push({
				value: val,
				text: hyname[val]
			});
			if(value && value == val){
				hyIndex = i +1;
			}
		}
	}
	yunvue.$data.hy = hyData;
	yunvue.$data.hyIndex = hyIndex;
	if(show){
		yunvue.$data.hyShow = true;
	}
}
// 求职状态-到岗时间
function rjPicker(jv, rv, show = true){
	var jobstatusData = [],
		jobstatusIndex = 0,
		reportData = [],
		reportIndex = 0;
	if(typeof useri.user_jobstatus !== 'undefined' && typeof useri.user_report !== 'undefined'){
		let jobstatus = useri['user_jobstatus'];
		for(let i = 0; i < jobstatus.length; i++){
			let val = jobstatus[i];
			jobstatusData.push({
				value: val,
				text: usern[val]
			});
			if(jv && jv == val){
				jobstatusIndex = i ;
			}
		}
		let report = useri['user_report'];
		for(let i = 0; i < report.length; i++){
			let val = report[i];
			reportData.push({
				value: val,
				text: usern[val]
			});
			if(rv && rv == val){
				reportIndex = i;
			}
		}
	}
	
	yunvue.$data.rj = [{
		values: jobstatusData,
		defaultIndex: jobstatusIndex
	}, 
	{
		values: reportData,
		defaultIndex: reportIndex
	}];
	if(show){
		yunvue.$data.rjShow = true;
	}else{
		// 非必填项，初始化
		if(!jv && !rv){
			yunvue.$data.rjValue = jobstatusData[0].text + (reportData.length > 0 ? '-' + reportData[0].text : '');
			yunvue.$data.info.jobstatus = jobstatusData[0].value;
			yunvue.$data.info.report = reportData[0].value;
		}
	}
}
function birthdayPicker(){
	yunvue.$data.birthdayShow = true;
}
function timeFormat(nowtime = null, type = 'date') {
	if(nowtime){
		var year = nowtime.getFullYear(),
			month = nowtime.getMonth() + 1,
			day = nowtime.getDate(),
			hour = nowtime.getHours(),
			minute = nowtime.getMinutes();
		if (month < 10) {
			month = '0' + month;
		}
		if (day < 10 && day > 0) {
			day = '0' + day;
		}
		if (hour < 10) {
			hour = '0' + hour;
		}
		if (minute < 10) {
			minute = '0' + minute;
		}
		if(type == 'month'){
			var time = year + '-' + month;
		}else if(type == 'datetime'){
			var time = year + '-' + month + '-' + day + ' ' + hour + ':' + minute;
		}else{
			var time = year + '-' + month + '-' + day;
		}
		return time;
	}
}