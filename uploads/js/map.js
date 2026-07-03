function mapPublicT(key, params, fallback) {
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

var localCityName = '';
// Get map coordinates.
function getmaplnglat(id,x,y,xid,yid){
	var data=get_map_config();
	if(data && dataindexOf('map_x')>-1){
		var config=eval('('+data+')');
		var rating,map_control_type,map_control_anchor;
		if(!x && !y){x=config.map_x;y=config.map_y;}
		var map = getMap(id, x, y);
		map.on("click",function(e){
			var lngLat = e.lnglat;
			document.getElementById(xid).value=lngLat.lng;// Write X to hidden field.
			document.getElementById(yid).value=lngLat.lat;// Write Y to hidden field.
			map.clearMap();
			var marker = new AMap.Marker({
				position: new AMap.LngLat(lngLat.lng, lngLat.lat)
			});
			map.add(marker);
		});
	}
}

// Show content on the map.
function getmapshowcont(id,x,y,title,cont){
	var map = getMap(id, x, y);
	var marker = new AMap.Marker({
		position: new AMap.LngLat(x,y)
	});
	map.add(marker);
	AMap.plugin('AMap.CitySearch', function () {
		var citySearch = new AMap.CitySearch();
		citySearch.getLocalCity(function (status, result){
			localCityName = result.city;
		})
	})
	var infoWindow = new AMap.InfoWindow({
	   content: title,
	   offset: [0,-30]
	});
	// Open info window on the map.
	infoWindow.open(map, [x,y]);
	
	document.getElementById("map_end").value=cont;
}
//getmapshowcont('map_container',116.404, 39.915,'ddd','aaaa');

function getmapnoshowcont_diffDomains(id,x,y,xid,yid,nodeId){	
	$.ajax({
        type : "get",
        async : false,       
        url : weburl + "/index.php?m=ajax&c=mapconfigdiffdomains",
        dataType : "jsonp",
        jsonpCallback:"diffdomains",
        success : function(json){
            var config = json;
            var rating,map_control_type,map_control_anchor;
            if (!x && !y) {
				x = config.map_x; 
				y = config.map_y; 
		    }
			var map = getMap(id, x, y);
			var marker = new AMap.Marker({
				position: new AMap.LngLat(x,y)
			});
			map.add(marker);
            
			AMap.plugin('AMap.CitySearch', function () {
				var citySearch = new AMap.CitySearch();
				citySearch.getLocalCity(function (status, result){
					localCityName = result.city;
					if(x==''||y==''){
						map.setCity(localCityName);
					}
				})
			})
			map.on("click",function(e){
				getmaplnglat(id,x,y,xid,yid);
			});
            $("#"+nodeId).append(map);
        },
        error:function(e1,e2,e3){
        }
    });
}




function get_map_config(){
	var config="";
	$.ajax( {
			async : false,
			type : "post",
			url :'../index.php?m=ajax&c=mapconfig',
			data : {id:""},
			success : function(set) {
			config=set;
		}
		});
	return config;
}
function getMap(id,x, y){

	var map = new AMap.Map(id);

	if (x && y) {
		setTimeout(function() {
			map.setZoomAndCenter(17,[x, y]);
		}, 50);
	}
	return map;
}
function showmap(id, x, y, com_name, address) {
	$.layer({
		type: 1,
		title: mapPublicT('map_js_00001', null, 'Map View'),
		closeBtn: [0, true],
		offset: ['100px', ''],
		border: [10, 0.3, '#000', true],
		area: ['1100px', 'auto'],
		page: {
			dom: "#" + id
		}
	});
	getmapshowcont('map_container', x, y, com_name, address);
}
// Transit query.
function bus_query(id,x, y) {
	var map_start = $.trim($("#map_start").val());
	if (map_start == "") {
		layer.msg(mapPublicT('map_js_00002', null, 'Please enter a route starting point.'), 2,8);
		return false;
	}
	var map_end = $.trim($("#map_end").val());
	var map = getMap(id, x, y);
	map.clearMap();
	$('#panel').html('');
	if(window.driving){
		driving.clear();
	}
	AMap.plugin('AMap.Transfer', function() {
	// Transit route planning.
	   var transOptions = {
		   map: map,
		   city: localCityName,
		   panel: 'panel',
		   policy: AMap.TransferPolicy.LEAST_TIME // Transit policy.
	   };
		window.transfer = new AMap.Transfer(transOptions);
		// Search route by start and end names.
		transfer.search([
			{keyword: map_start},
			{keyword: map_end}
		], function(status, result) {
			if (status === 'complete') {
				console.log('Transit route drawn successfully')
			} else {
				console.log('Transit route query failed');
				console.log(result);
			}
		});		
	});
}

// Driving query.
function map_drving(id,x, y) {
	var map_start = $.trim($("#map_start").val());
	if (map_start == "") {
		layer.msg(mapPublicT('map_js_00002', null, 'Please enter a route starting point.'), 2,8);
		return false;
	}
	var map_end = $.trim($("#map_end").val());
	var map = getMap(id, x, y);
	map.clearMap();
	$('#panel').html('');
	if(window.transfer){
		transfer.clear();
	}
	AMap.plugin('AMap.Driving', function() {
	// Driving route planning.
	   var drivingOptions = {
		   map: map,
		   city: localCityName,
		   panel: 'panel',
		   policy: AMap.DrivingPolicy.LEAST_TIME
	   };
		window.driving  = new AMap.Driving(drivingOptions);
		// Search route by start and end names.
		driving.search([
			{keyword: map_start},
			{keyword: map_end}
		], function(status, result) {
			if (status === 'complete') {
				console.log('Driving route completed')
			} else {
				console.log('Driving route query failed');
				console.log(result);
			}
		});		
	});
}