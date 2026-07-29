<?php

class user_controller extends common{
	function add_action(){

        $locoyinfo = locoy_config();
        if(!$_POST['info_name']){
			echo 2;die;
		}
		$uid=$this->add_user_info($_POST,$locoyinfo);
		$this->add_resume($_POST,$locoyinfo,$uid);
	}

	function add_resume($p,$l,$uid){
			$v['name'] = trim($p['info_classid']);
			$v['ctime'] = time();
			$v['lastupdate']=time();
			$v['uid']=$uid;
		    $v['r_status']=1;
			
			include(PLUS_PATH."industry.cache.php");
			$v['hy']=$this->locoytostr($industry_name,$p['info_hy'],$l['locoy_rate']);
			if(!$v['hy']){
				$v['hy']=$l['locoy_resume_hy'];
			}
			$city_row=$this->get_city($p['info_city'],$l['locoy_rate']);
			if($city_row){
				$i=1;
				foreach($city_row as $vs){
					if($i==1)$v['provinceid']=$vs;
					if($i==2)$v['cityid']=$vs;
					if($i==3)$v['three_cityid']=$vs;
					$i++;
				}
			}else{
				$v['provinceid']=$l['locoy_resume_province'];
				$v['cityid']=$l['locoy_resume_city'];
				$v['three_cityid']=$l['locoy_resume_three'];
			}
			
      
			$v['city_classid']=$v['cityid'];
			$job_row=$this->get_job_class($p['info_classid'],$l['locoy_rate']);
			if($job_row){
				foreach($job_row as $vs){
					$job_arr[] = $vs;
				}
				$v['job_classid']=pylode(',',$job_arr);
			}
			if(!$v['job_classid']){
				$v['job_classid']=$l['locoy_resume_post'];
			}
			$report=$p['info_report'];
			$v['report']=$this->locoytostr($this->get_user_type('report'),$report,$l['locoy_rate']);
			if(!$v['report']){
				$v['report']=$l['locoy_user_report'];
			}
      
			$type=$p['info_type'];
            $v['type']=$this->locoytostr($this->get_user_type('type'),$type,$l['locoy_rate']);
			
			if($p['info_hits']){
				$v['hits']=trim($p['info_hits']);
			}else{
				$row=explode('-',$locoyinfo['locoy_resume_rand']);
				if(is_array($row)){
					$rand=rand(trim($row[0]),trim($row[1]));
				}else{
					$rand=!trim($row)?0:$row;
				}
				$v['hits']=$rand;
			}
			$v['source'] = 6;
			if($p['jobstatus']){
				$v['jobstatus']=$p['jobstatus'];
			}else{
				$v['jobstatus']=45;
			}
			
			$numresume=55;
			if($p['skill_name'] || $p['skill_skill'] || $p['skill_ing']){
				$numresume=$numresume+10;
			}
			if($p['work_name'] || $p['work_sdate']){
				$numresume=$numresume+10;
			}
			if($p['pro_name']|| $p['pro_sdate']){
				$numresume=$numresume+8;
			}
			if($p['edu_name'] || $p['edu_title']){
				$numresume=$numresume+10;
			}
			if($p['train_name'] || $p['train_title']){
				$numresume=$numresume+7;
			}
			
			$v['integrity']=$numresume;
			$v['defaults']=1;
			$v['edu']=$this->locoytostr($this->get_user_type('edu'),$p['info_edu'],$l['locoy_rate']);
			if(!$v['edu']){
				$v['edu']=$l['locoy_user_edu'];
			}
			$v['exp']=$this->locoytostr($this->get_user_type('word'),$p['info_exp'],$l['locoy_rate']);
			if(!$v['exp']){
				$v['exp']=$l['locoy_user_exp'];
			}
			$v['uname']=trim($p['info_name']);
				if($p['info_sex']=="男" || $p['info_sex']==1){
				$v['sex']=1;
			}elseif($p['info_sex']=="女" || $p['info_sex']==2){
				$v['sex']=2;
			}
			if(!$v['sex']){
				$v['sex']=$l['locoy_user_sex'];
			}
		
			$v['r_status']=1;
            if($p['minsalary']){
    		    $v['minsalary']=$p['minsalary'];
    		}else{
    		    $v['minsalary']=$l['locoy_minsalary'];
    	    }
    		if($p['maxsalary']){
    		    $v['maxsalary']=$p['maxsalary'];
    		}else{
    			$v['maxsalary']=$l['locoy_maxsalary'];
    		}
    		if($l['locoy_user_status']==1){
    		    $v['state'] = 1;
    		}
    		$resumeM = $this->MODEL('resume');
    		$res = $resumeM->addInfo(array('uid'=>$uid,'eData'=>$v));
    		$nid = $res['id'];
			if($nid){

				if($this->locoyField($p, 'skill_name') || $this->locoyField($p, 'skill_skill') || $this->locoyField($p, 'skill_ing')){
					$this->obj->insert_into('resume_skill', array(
						'uid'      => (int) $uid,
						'eid'      => (int) $nid,
						'name'     => $this->locoyField($p, 'skill_name'),
						'skill'    => $this->locoyField($p, 'skill_skill'),
						'ing'      => (int) $this->locoyField($p, 'skill_ing'),
						'longtime' => (int) $this->locoyField($p, 'skill_longtime')
					));
				}

				for($workIndex = 0; $workIndex <= 4; $workIndex++){
					$suffix = $workIndex === 0 ? '' : (string) $workIndex;
					if($this->locoyField($p, 'work_name'.$suffix) || $this->locoyField($p, 'work_sdate'.$suffix)){
						$this->obj->insert_into('resume_work', array(
							'uid'        => (int) $uid,
							'eid'        => (int) $nid,
							'name'       => $this->locoyField($p, 'work_name'.$suffix),
							'sdate'      => $this->locoyDate($this->locoyField($p, 'work_sdate'.$suffix)),
							'edate'      => $this->locoyDate($this->locoyField($p, 'work_edate'.$suffix)),
							'department' => $this->locoyField($p, 'work_department'.$suffix),
							'content'    => $this->locoyField($p, 'work_content'.$suffix),
							'title'      => $this->locoyField($p, 'work_title'.$suffix)
						));
					}
				}

				if($this->locoyField($p, 'pro_name') || $this->locoyField($p, 'pro_sdate')){
					$this->obj->insert_into('resume_project', array(
						'uid'     => (int) $uid,
						'eid'     => (int) $nid,
						'name'    => $this->locoyField($p, 'pro_name'),
						'sdate'   => $this->locoyDate($this->locoyField($p, 'pro_sdate')),
						'edate'   => $this->locoyDate($this->locoyField($p, 'pro_edate')),
						'sys'     => $this->locoyField($p, 'pro_sys'),
						'content' => $this->locoyField($p, 'pro_content'),
						'title'   => $this->locoyField($p, 'pro_title')
					));
				}

				for($eduIndex = 0; $eduIndex <= 2; $eduIndex++){
					$suffix = $eduIndex === 0 ? '' : (string) $eduIndex;
					$eduName = $this->locoyField($p, 'edu_name'.$suffix);
					$eduTitle = $this->locoyField($p, 'edu_title'.$suffix);
					if($eduName || $eduTitle){
						$eduData = array(
							'uid'       => (int) $uid,
							'eid'       => (int) $nid,
							'name'      => $eduName,
							'sdate'     => $this->locoyDate($this->locoyField($p, 'edu_sdate'.$suffix)),
							'edate'     => $this->locoyDate($this->locoyField($p, 'edu_edate'.$suffix)),
							'specialty' => $this->locoyField($p, 'edu_specialty'.$suffix),
							'content'   => $this->locoyField($p, 'edu_content'.$suffix)
						);
						if($eduIndex === 0){
							$eduData['education'] = $this->locoytostr($this->get_user_type('edu'), $eduTitle, $l['locoy_rate']);
						}else{
							$eduData['title'] = $eduTitle;
						}
						$this->obj->insert_into('resume_edu', $eduData);
					}
				}

				if($this->locoyField($p, 'cert_name') || $this->locoyField($p, 'cert_title')){
					$this->obj->insert_into('resume_cert', array(
						'uid'     => (int) $uid,
						'eid'     => (int) $nid,
						'name'    => $this->locoyField($p, 'cert_name'),
						'sdate'   => $this->locoyDate($this->locoyField($p, 'cert_sdate')),
						'content' => $this->locoyField($p, 'cert_content'),
						'title'   => $this->locoyField($p, 'cert_title')
					));
				}

				if($this->locoyField($p, 'other_content') || $this->locoyField($p, 'other_title')){
					$this->obj->insert_into('resume_other', array(
						'uid'     => (int) $uid,
						'eid'     => (int) $nid,
						'content' => $this->locoyField($p, 'other_content'),
						'name'    => $this->locoyField($p, 'other_name')
					));
				}

				for($trainIndex = 0; $trainIndex <= 1; $trainIndex++){
					$suffix = $trainIndex === 0 ? '' : (string) $trainIndex;
					if($this->locoyField($p, 'train_name'.$suffix) || $this->locoyField($p, 'train_title'.$suffix)){
						$this->obj->insert_into('resume_training', array(
							'uid'     => (int) $uid,
							'eid'     => (int) $nid,
							'name'    => $this->locoyField($p, 'train_name'.$suffix),
							'sdate'   => $this->locoyDate($this->locoyField($p, 'train_sdate'.$suffix)),
							'edate'   => $this->locoyDate($this->locoyField($p, 'train_edate'.$suffix)),
							'content' => $this->locoyField($p, 'train_content'.$suffix),
							'title'   => $this->locoyField($p, 'train_title'.$suffix)
						));
					}
				}
				echo 1;die;
			}
	}

	private function locoyField($data, $field, $default = '')
	{
		return isset($data[$field]) && !is_array($data[$field]) ? $data[$field] : $default;
	}

	private function locoyDate($value)
	{
		$value = str_replace(array('年', '月', '日'), '-', trim((string) $value));
		if($value !== '' && substr($value, -1) === '-'){
			$value .= '01';
		}
		$timestamp = $value !== '' ? strtotime($value) : false;
		return $timestamp === false ? 0 : $timestamp;
	}

	function add_user_info($p,$l){
		$row = $this->obj->select_once('resume', array(
			'name' => $this->locoyField($p, 'info_name')
		));
		if(is_array($row)){
			return $row['uid'];
		}else{
			$userid=$this->add_user($p,$l);
			$where['uid']=$userid;
			$data['name']=trim($p['info_name']);
			$data['address']=trim($p['info_address']);
			$data['height']=trim($p['info_height']);
			$data['weight']=trim($p['info_weight']);
			$data['birthday']=$p['info_birthday'];
			$data['telphone']=$p['info_telphone'];
			$data['homepage']=$p['info_homepage'];
			$info_description=strip_tags(html_entity_decode($p['info_description']),"<p> <br>");
			$data['description']=$info_description;
			$data['living']=$p['info_living'];
			$data['domicile']=$p['info_domicile'];
			$data['email']=$p['info_email'];
            $data['qq']=$p['info_qq'];
			
			if($p['info_sex']=="男" || $p['info_sex']==1){
				$data['sex']=1;
			}elseif($p['info_sex']=="女" || $p['info_sex']==2){
				$data['sex']=2;
			}
			if(!$data['sex']){
				$data['sex']=$l['locoy_user_sex'];
			}
			if (!empty($p['info_photo'])){
			    $data['resume_photo']=$p['info_photo'];
			    $data['photo']=$p['info_photo'];
			}else{
			    // 处理随机头像
			    if($data['sex']=='2'){
			        $icon_arr = $this->config['sy_member_iconv_arr'];
			    }else{
			        $icon_arr = $this->config['sy_member_icon_arr'];
			    }
			    
			    if(!empty($icon_arr)){
			        $key  = array_rand($icon_arr,1);
			        $data['photo'] = $icon_arr[$key];
			        $data['defphoto'] = 2;
			        $data['photo_status'] = 1;
			    }
			}
			$data['marriage']=$this->locoytostr($this->get_user_type('marriage'),$p['info_marriage'],$l['locoy_rate']);
			if(!$data['marriage']){
				$data['marriage']=$l['locoy_user_marriage'];
			}
			$data['edu']=$this->locoytostr($this->get_user_type('edu'),$p['info_edu'],$l['locoy_rate']);
			if(!$data['edu']){
				$data['edu']=$l['locoy_user_edu'];
			}
			$data['exp']=$this->locoytostr($this->get_user_type('word'),$p['info_exp'],$l['locoy_rate']);
			if(!$data['exp']){
				$data['exp']=$l['locoy_user_exp'];
			}
			if(!$p['nationality']){
				$data['nationality']=$l['locoy_user_nationality'];
			}else{
				$data['nationality']=$p['nationality'];
			}
			$nid=$this->obj->update_once("resume",$data,$where);
			return $userid;
		}
	}
	
	function add_user($p,$l){
		$salt = substr(bin2hex(random_bytes(8)), -6);
		$password = isset($l['locoy_pwd']) ? (string) $l['locoy_pwd'] : '';
		$pass = passCheck($password, $salt);
		$ip = fun_ip_get();
		$time = time();
		$username = $this->get_username($l);
		
		$userid = $this->obj->insert_into('member', array(
			'username' => $username,
			'password' => $pass,
			'moblie'   => $this->locoyField($p, 'info_telphone'),
			'email'    => $this->locoyField($p, 'info_email'),
			'usertype' => 1,
			'status'   => 1,
			'salt'     => $salt,
			'reg_date' => $time,
			'reg_ip'   => $ip,
			'source'   => 6
		));
		if($userid){
			$this->obj->insert_into('resume', array('uid' => (int) $userid));
			$this->obj->insert_into('member_statis', array('uid' => (int) $userid));
		}
		return $userid;
	}
	
	function get_username($l){
		$row = array("a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","0","1","2","3","4","5","6","7","8","9");
		$va="";
		for($i=0;$i<$l['locoy_length'];$i++){
			$rand=rand(0,61);
			$va.=$row[$rand];
		}
		$data=$l['locoy_name'].$va;
		return $data;
	}
	
	function get_city($name,$locoy_rate){
		include(PLUS_PATH."city.cache.php");
		$name = str_replace(array('/','-',','),'',trim($name));
		$name=str_replace(array("省","市","县","区"),"/",$name);
		$arr=explode("/",$name);
		if(is_array($arr)){
			foreach($arr as $v){
				$data[]=$this->locoytostr($city_name,$v,$locoy_rate);
			}
		}
		$city_type[0]=$city_index;
		$val=$this->get_all_city($city_type,$data);
		if(count($val)==1){
			$val[]=$this->get_once_city($city_type,$city_name,$val[0],$locoy_rate);
		}
		return $val;
	}
	
	function get_job_class($name,$locoy_rate){
		include(PLUS_PATH."job.cache.php");
		$arr=explode(",",$name);
		if(is_array($arr)){
			foreach($arr as $v){
				$data[]=$this->locoytostr($job_name,$v,$locoy_rate);
			}
		}
		return $data;
	}
	
	function get_all_city($city_type,$data,$locoy_rate,$k=""){
		if(is_array($data)){
			foreach($data as $v){
				foreach($city_type as $key=>$value){
					$a=$k?$k:$v;
					if(in_array($a,$value)){
						if($key){
							$val=$this->get_all_city($city_type,$data,$locoy_rate,$key);
						}
						$val[$key]=$a;
					}
				}
			}
		}
		return $val;
	}
	
	function get_once_city($t,$n,$id,$locoy_rate){
		$row=$n[$id];
		if(is_array($t[$id])){
			foreach($t[$id] as $k=>$v){
				$array[$v]=$n[$v];
			}
		}
		$r=$this->locoytostr($array,$row,$locoy_rate);
		return $r;
	}
	
	function get_user_type($cat){
		include(PLUS_PATH."user.cache.php");
		foreach($userdata["user_".$cat] as $v){
			$data[$v]=$userclass_name[$v];
		}
		return $data;
	}
	
	function locoytostr($arr,$str,$locoy_rate="60"){
			$str_array=$this->tostring($str);
			foreach($arr as $key =>$list){
				$h=0;
				foreach($str_array as $val){
					if(substr_count($list,$val))$h++;
				}
				$categoryname_array=$this->tostring($list);
				$j=round($h/count($categoryname_array)*100,2);
				$rows[$j]=$key;
			}
			krsort($rows);
			foreach($rows as $k =>$v){			 
				if ($k>=$locoy_rate){
					return $v;
				}else{
					return false;
				}					
			}
	}
	
	function tostring($string){ 
		$length=strlen($string); 
		$retstr=''; 
		for($i=0;$i<$length;$i++) { 
			$retstr[]=ord($string[$i])>127?$string[$i].$string[++$i]:$string[$i]; 
		} 
		return $retstr; 
	}
}
?>