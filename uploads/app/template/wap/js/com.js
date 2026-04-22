function comjobAdd(url ,param, callback){
	
	showLoading();
	$.post(url, {provider: 'h5'}, function (data) {
		hideLoading();
		var tourl = '';

		if (data.error == 0){
			let res = data.data,
				checked = {};
			
			if(res.need){
				
				if(res.need.name){
					checked.name = true;
				}else{
					checked.name = false;
				}
				if(res.need.tel){
					checked.tel = true;
				}else{
					checked.tel = false;
				}
				if(res.need.email){
					checked.email = true;
				}else{
					checked.email = false;
				}
				if(res.need.yyzz){
					checked.yyzz = true;
				}else{
					checked.yyzz = false;
				}
				if(res.need.xy){
					checked.xy = true;
				}else{
					checked.xy = false;
				}
				if(res.need.gzh){
					checked.gzh = res.need.gzh;
				}else{
					checked.gzh = 0;
				}
				if(callback){
					callback(checked);
				}
			}else if(res.job_num == 0){
				callback({error:1,msg: res.msg});
			}else if(res.num>0){
				
				if(res.num == 2){
					
					if(param.job=='part'){
						tourl = wapurl+'member/index.php?c=partadd';
					}else if(param.job=='job'){
						tourl = wapurl+'member/index.php?c=jobadd';
					}
            		
            		callback({error:2,msg:res.msg,tourl:tourl});
				}else if(res.num == 1){
					if(param.job=='part'){
						navigateTo(wapurl+'member/index.php?c=partadd');
					}else if(param.job=='job'){
						navigateTo(wapurl+'member/index.php?c=jobadd');
					}
				}
			}else{

				if(callback){
					callback({error:1,msg: res.msg});
				}
			}
		}else{
			showToast(data.msg);
		}
	}, 'json');
}
function comjobRefresh(url, param, callback, success = null){
	showLoading();
	$.post(url, param, function (data) {
		hideLoading();
		if (data.error == 0) {
			showToast(data.msg, 2, function(){
				typeof success === 'function' && success()
			});
		} else if (data.error == 3) {
			if(callback){
				callback({msg: data.msg});
			}
		} else {
			showToast(data.msg);
		}
	})
}