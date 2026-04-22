<?php

if($config['sy_web_site']=="1" && !isset($_POST['xcxCode'])){
	if(($_COOKIE['cityid'] || $_COOKIE['hyclass'] || $_COOKIE['three_cityid'] || $_COOKIE['province'])&&$_COOKIE['did']){
		
		$config['sy_wapurl']	=  $_COOKIE['sy_wapurl'];
		$config['province']		=  (int)$_COOKIE['province'];
		$config['cityid']		=  (int)$_COOKIE['cityid'];
		$config['three_cityid']	=  (int)$_COOKIE['three_cityid'];
		$config['cityname']		=  $_COOKIE['cityname'];
		$config['sy_indexcity_n']=  $config['sy_indexcity']	;
		$config['sy_indexcity']	=  $_COOKIE['cityname'];
		$config['did']			=  (int)$_COOKIE['did'];
		$config['fz_type']		=  $_COOKIE['fz_type'];
		$config['hyclass']		=  $_COOKIE['hyclass'];
		$config['style']		=  $_COOKIE['style'];  
		if($_COOKIE['sy_webtitle']!=""){
			$config['sy_webtitle']	=  $_COOKIE['sy_webtitle'];
		}
		if($_COOKIE['sy_webname']!=""){
			$config['sy_webname']	=  $_COOKIE['sy_webname'];
		}
		if($_COOKIE['sy_webkeyword']!=""){
			$config['sy_webkeyword']	=  $_COOKIE['sy_webkeyword'];
		}
		if($_COOKIE['sy_webmeta']!=""){
			$config['sy_webmeta']	=  $_COOKIE['sy_webmeta'];
		}
		if($_COOKIE['sy_logo']!=""){
			$config['sy_logo']	=  $_COOKIE['sy_logo'];
		}

	}else{
		include(PLUS_PATH."/domain_cache.php");
		include(PLUS_PATH."/city.cache.php");
		include(PLUS_PATH."/industry.cache.php");
		
		if(is_array($site_domain)){
			foreach($site_domain as $key=>$value){
				if($value['host']==$_SERVER['HTTP_HOST']){
					$parseDate['did']=$value['id'];
					$parseDate['fz_type']=$value['fz_type'];
					if($parseDate['fz_type']=='1'){
						if($value['three_cityid']>0){
							$parseDate['three_cityid']	=	$value['three_cityid'];
							$parseDate['cityname']		=	$city_name[$value['three_cityid']];
						}elseif($value['cityid']>0){
							$parseDate['cityid']	=	$value['cityid'];
							$parseDate['cityname']	=	$city_name[$value['cityid']];
						}else{
							$parseDate['province']	=	$value['province'];
							$parseDate['cityname']	=	$city_name[$value['province']];
						}
						setcookies('hyclass',time()-86400,"/");
					}else if($parseDate['fz_type']=='2'&&$value['hy']){
						$parseDate['hyclass']=$value['hy'];
						$parseDate['cityname']		=	$value['webname'];
						setcookies(
							array(
							'cityid'=>'',
							'three_cityid'=>''
							),time()-86400,"/"
						);
					}
					if($value['webname']){$parseDate['sy_webname']  =	$value['webname'];}
					if($value['webtitle']){$parseDate['sy_webtitle']  =	$value['webtitle'];}
					if($value['weblogo']){$parseDate['sy_logo']  =	$value['weblogo'];}
					if($value['webkeyword']){$parseDate['sy_webkeyword']  =	$value['webkeyword'];}
					if($value['webmeta']){$parseDate['sy_webmeta']  =	$value['webmeta'];}
					if($value['style']){$parseDate['style']  =	$value['style'];}
					$parseDate['sy_wapurl']  =	$host;
					
					setcookies($parseDate,time()+86400,"/");
					$config = array_merge($config,$parseDate);
					
				}
			}
		}
		if(!$parseDate && $_COOKIE['fz_type']){
			setcookies(
				array(
				'sy_wapurl'=>'',
				'did'=>'',
				'fz_type'=>'',
				'province'=>'',
				'cityid'=>'',
				'hyclass'=>'',
				'three_cityid'=>'',
				'cityname'=>'',
				'sy_webkeyword'=>'', 
				'sy_logo'=>'',
				'style'=>'',
				'sy_webtitle'=>'',
				'sy_webmeta'=>'',
				'sy_webname'=>''

			),time()-86400,$config['sy_onedomain']);
			
		}
	}
}

 

?>