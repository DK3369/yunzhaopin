var cityLoadNum = 0,
	jobLoadNum = 0;
function cityCategory(param){

	var cityData = [],
		oneall = true,
		twoall = true,
		threeall = true,
		oneid = '',
		twoid = '',
		threeid = '';
	var {sy_web_city_one,sy_web_city_two,sy_web_city_three,one_all,two_all,three_all} = param;
	//城市分类
	if(typeof ci != "undefined" && typeof ct != "undefined" && typeof cn != "undefined") {
		
		if(sy_web_city_one){
			oneall = false;
			oneid = sy_web_city_one;
		}else if(typeof(one_all) != 'undefined' && !one_all){
			oneall = false;
		}

		if(sy_web_city_two){
			twoall = false;
			twoid = sy_web_city_two;
		}else if(typeof(two_all) != 'undefined' && !two_all){
			twoall = false;
		}

		if(sy_web_city_three){
			threeall = false;
			threeid = sy_web_city_three;
		}else if(typeof(three_all) != 'undefined' && !three_all){
			threeall = false;
		}

		if(oneall){
			cityData.push({
				value: '',
				text: '全部',
				children: null
			});
		}
		

		for(var i = 0; i < ci.length; i++) {

			if(oneid && ci[i]!=oneid){//指定一级城市
				continue;
			}

			var city = [];
	    	var cvlaue = [];
	    	var ctext = [];

	    	if(typeof ct[ci[i]] != "undefined"){
	    		if(twoall){
	    			city.push({
						value: '0',
						text: '全部',
						children: null
					});
	    		}
	    		
				for(var j = 0; j < ct[ci[i]].length; j++) {

					if(twoid && ct[ci[i]][j]!=twoid){//指定二级级城市
						continue;
					}

					var threecity = [];
	       			
					if(ct[ct[ci[i]][j]]) {
						if(threeall){
							threecity.push({
				              	value: '0',
								text: '全部',
								children: null
							})
						}
	          			
						for(var k = 0; k <ct[ct[ci[i]][j]].length; k++) {

							if(threeid && ct[ct[ci[i]][j]][k]!=threeid){//指定一级城市
								continue;
							}

				            cvlaue=	ct[ct[ci[i]][j]][k];
				            ctext=	cn[ct[ct[ci[i]][j]][k]];
				            
				            threecity.push({
				              	value: cvlaue,
								text: ctext,
								children: null
							})
				        }
					}
					city.push({
					
	          			value: ct[ci[i]][j],
						text: cn[ct[ci[i]][j]],
						children: threecity.length>0?threecity:null
					})
				}
			}
			
			cityData.push({
				value: ci[i],
				text: cn[ci[i]],
				children: city.length>0?city:null
			})
		}
		return cityData;
	}else{
		// 循环处理，防止缓存文件未加载
		cityLoadNum++
		if(cityLoadNum < 20){
			setTimeout(function(){
				cityCategory(param)
			},50);
		}
	}
}
function jobCategory(param){
	//城市分类
	var jobData = [];
	if(typeof ji != "undefined" && typeof jt != "undefined" && typeof jn != "undefined") {

		if(!param || param.one_all){
			jobData.push({
				value: '',
				text: '全部',
				children: null
			});
		}

		for(var i = 0; i < ji.length; i++) {

			var job = [];
	    	var cvlaue = [];
	    	var jtext = [];

	    	if(typeof jt[ji[i]] != "undefined"){
				if(!param || param.two_all){
					job.push({
						value: '0',
						text: '全部',
						children: null
					});
				}
				for(var j = 0; j < jt[ji[i]].length; j++) {
					var threejob = [];
	       			
					if(jt[jt[ji[i]][j]]) {
	          
						for(var k = 0; k <=jt[jt[ji[i]][j]].length; k++) {            
				            if(k==0){
				              cvlaue='0';
				              jtext='全部';
				            }else{
				              t=k-1;
				              cvlaue=	jt[jt[ji[i]][j]][t];
				              jtext=	jn[jt[jt[ji[i]][j]][t]];
				            }
				            threejob.push({
				              	value: cvlaue,
								text: jtext,
								children: null
							})
						}
					}
					job.push({
					
	          			value: jt[ji[i]][j],
						text: jn[jt[ji[i]][j]],
						children: threejob.length>0?threejob:null
					})
				}
			}
			
			jobData.push({
				value: ji[i],
				text: jn[ji[i]],
				children: job.length>0?job:null
			})
		}
		return jobData;
	}else{
		// 循环处理，防止缓存文件未加载
		jobLoadNum++
		if(jobLoadNum < 20){
			setTimeout(function(){
				jobCategory(param)
			},50);
		}
	}
}