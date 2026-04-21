<?php
/* *
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class job_controller extends common{
    function add_action(){
        
        include(APP_PATH."data/api/locoy/locoy_config.php");
        if($locoyinfo['locoy_online']!=1){
            echo 4;die;
        }
        if($locoyinfo['locoy_key']!=trim($_GET['key'])){
            echo 5;die;
        }
        if(!$_POST['job_name'] || !$_POST['com_name']){
            echo 2;die;
        }
        $uid=$this->add_com($_POST,$locoyinfo);
        if (is_numeric($uid)){
            $this->add_job($_POST,$locoyinfo,$uid);
        }else{
            echo 3;die;
        }
    }
    function add_job($p,$l,$uid)
    {
        
        $data['uid']         =  $uid;
        $data['name']        =  $p['job_name'];
        $data['lastupdate']  =  time();
        $data['state']       =  isset($l['locoy_job_status']) ? $l['locoy_job_status'] : 0;
        $data['description'] =  strip_tags(html_entity_decode($p['description'],ENT_NOQUOTES,"GB2312"),"<p> <br>");
        
        $jobM  =  $this->MODEL('job');
        $chenk_row  =  $jobM->getInfo(array('uid'=>$uid,'name'=>$data['name']));
        if(is_array($chenk_row)){
            echo 3;die;
        }
        
        $hy  =  $p['job_hy'] ? $p['job_hy'] : $p['hy'];
        if (!empty($hy)){
            include(PLUS_PATH."industry.cache.php");
            $data['hy']  =  $this->locoytostr($industry_name,$hy,$l['locoy_rate']);
        }
        
        if(!isset($data['hy'])){
            $data['hy']  =  $l['locoy_job_hy'];
        }
        $job_row  =  $this->get_job_class($p['job_cate'],$l['locoy_rate']);
        if($job_row){
            $i=1;
            foreach($job_row as $v){
                if($i==1)$data['job1']=$v;
                if($i==2)$data['job1_son']=$v;
                if($i==3)$data['job_post']=$v;
                $i++;
            }
        }else{
            $data['job1']      =  $l['locoy_job_job1'];
            $data['job1_son']  =  $l['locoy_job1_son'];
            $data['job_post']  =  $l['locoy_job_post'];
        }
        $city  =  $p['job_city']?$p['job_city']:$p['city'];
        $city_row  =  $this->get_city($city,$l['locoy_rate']);
        if($city_row){
            $i=1;
            foreach($city_row as $v){
                if($i==1)$data['provinceid']=$v;
                if($i==2)$data['cityid']=$v;
                if($i==3)$data['three_cityid']=$v;
                $i++;
            }
        }else{
            $data['provinceid']    =  $l['locoy_job_province'];
            $data['cityid']        =  $l['locoy_job_city'];
            $data['three_cityid']  =  $l['locoy_job_three'];
        }
        if(isset($p['sdate'])){
            $data['sdate']  =  strtotime($p['sdate']);
        }else{
            $data['sdate']  =  isset($l['locoy_job_sdate']) ? strtotime($l['locoy_job_sdate']) : time();
        }
        
        if(!empty($p['minsalary'])){
            $data['minsalary']  =  $p['minsalary'];
        }else{
            $data['minsalary']  =  isset($l['locoy_minsalary']) ? $l['locoy_minsalary'] : 0;
        }
        if(!empty($p['maxsalary'])){
            $data['maxsalary']  =  $p['maxsalary'];
        }else{
            $data['maxsalary']  =  isset($l['locoy_maxsalary']) ? $l['locoy_maxsalary'] : 0;
        }
        if (!empty($p['exp'])){
            $data['exp']  =  $this->locoytostr($this->get_com_type('exp'),$p['exp'],$l['locoy_rate']);
        }
        if(!isset($data['exp'])){
            $data['exp']  =  $l['locoy_job_exp'];
        }
        
        if (!empty($p['report'])){
            $data['report']=$this->locoytostr($this->get_com_type('report'),$p['report'],$l['locoy_rate']);
        }
        if(!isset($data['report'])){
            $data['report']  =  $l['locoy_job_report'];
        }
        
        if (!empty($p['age'])){
            $data['age']=$this->locoytostr($this->get_com_type('age'),$p['age'],$l['locoy_rate']);
        }
        if(!isset($data['age'])){
            $data['age']  =  $l['locoy_job_age'];
        }
        
        if (!empty($p['type'])){
            $data['type']=$this->locoytostr($this->get_com_type('type'),$p['type'],$l['locoy_rate']);
        }
        if(!isset($data['type'])){
            $data['type']  =  $l['locoy_job_type'];
        }
        
        if (!empty($p['sex'])){
            $data['sex']=$this->locoytostr($this->get_com_type('sex'),$p['sex'],$l['locoy_rate']);
            
            if($p['sex']=="男" || $p['sex']==1){
                $data['sex']  =  1;
            }elseif($p['sex']=="女" || $p['sex']==2){
                $data['sex']  =  2;
            }elseif($p['sex']=="不限" || $p['sex']==3){
                $data['sex']  =  3;
            }
        }
        if(!isset($data['sex'])){
            $data['sex']  =  $l['locoy_job_sex'];
        }
        
        if (!empty($p['edu'])){
            $data['edu']  =  $this->locoytostr($this->get_com_type('edu'),$p['edu'],$l['locoy_rate']);
        }
        if(!isset($data['edu'])){
            $data['edu']  =  $l['locoy_job_edu'];
        }
        
        if (!empty($p['marriage'])){
            $data['marriage']  =  $this->locoytostr($this->get_com_type('marriage'),$p['marriage'],$l['locoy_rate']);
        }
        if(!isset($data['marriage'])){
            $data['marriage']  =  $l['locoy_job_marriage'];
        }
        
        if (!empty($p['number'])){
            $data['number']  =  $this->locoytostr($this->get_com_type('number'),$p['number'],$l['locoy_rate']);
        }
        if(!isset($data['number'])){
            $data['number']  =  $l['locoy_job_number'];
        }
        
        //设置福利匹配
        $welfare  =  array();
        //最多到8项
        for ($wi=1; $wi<=8; $wi++){
            
            if (!empty($p['welfare'.$wi])){
                
                $data['welfare'.$wi]  =  $this->locoytostr($this->get_com_type('welfare'),$p['welfare'.$wi],$l['locoy_rate']);
                
                if(!empty($data['welfare'.$wi])){
                    
                    $welfare[]  =  $data['welfare'.$wi];
                }
            }
        }
        $data['welfare']  =  implode(',',$welfare);
        //设置语言匹配
        $lang  =  array();
        //最多到3项
        for ($li=1; $li<=3; $li++){
            
            if (!empty($p['lang'.$li])){
                
                $data['lang'.$li]  =  $this->locoytostr($this->get_com_type('lang'),$p['lang'.$li],$l['locoy_rate']);
                
                if(!empty($data['lang'.$li])){
                    
                    $lang[]  =  $data['lang'.$li];
                }
            }
        }
        $data['lang']  =  implode(',',$lang);
        /******************************************************************************/
        
        $data['xuanshang']  =  '0';
        
        $companyM  =  $this->MODEL('company');
        $com  =  $companyM->getInfo($uid);
        $data['com_name']        =  $com['name'];
        $data['com_logo']        =  $com['logo'];
        $data['com_provinceid']  =  $com['provinceid'];
        $data['pr']              =  $com['pr'];
        $data['mun']             =  $com['mun'];
        $data['rating']          =  $this->config['com_rating'];
        if($l['locoy_user_status']==1){
			$data['r_status']    =  1;
		}else{
			$data['r_status']    =  0;
		}
        $data['source']          =  6;
        
        $nid  =  $jobM->addInfo($data);
        
        /*if($this->config['com_job_status']=="1"){
            $this->send_dingyue($nid,2);
        }*/
        
        $lastupdate  =  isset($p['lastupdate']) ? $p['lastupdate'] : time();
        $companyM->upInfo(array('uid'=>$uid),array('jobtime'=>$lastupdate));
        echo 1;die;
    }
    
    function add_com($p,$l)
    {
        $c_name=trim($p['com_name']);
        
        $companyM  =  $this->MODEL('company');
        $row  =  $companyM->getCompanyInfo(array('name'=>$c_name),'`uid`');
        
        if(is_array($row)){
            return $row['uid'];
        }else{
            $userid  =  $this->add_user($p,$l);
            $data['name']  	  =  trim($p['com_name']);
            $data['address']  =  trim($p['address']);
				if(!preg_match('/^1([0-9]{9})/',$p['linkphone'])){
					$data['linkphone']  =  trim($p['linkphone']);}
                else{
                    $data['linkphone']  =  "";
                    
                }
                $data['linkmail'] =  trim($p['email']);
                $data['zip']      =  trim($p['zip']);
                $data['linkman']  =  trim($p['linkman']);
                $data['linkjob']  =  trim($p['linkjob']);
                $data['linkqq']   =  trim($p['linkqq']);
                $data['linktel']  =  trim($p['moblie']);
                $data['website']  =  trim($p['website']);
                $data['x']        =  $p['mapx'];
                $data['y']        =  $p['mapy'];
                $data['logo']     =  trim($p['logo']);
                if($p['com_sdate']){
                    $data['sdate']  =  date("Y-m-d",strtotime(trim($p['com_sdate'])));
                }
                $money  =  str_replace(array("元","美元","￥","$"),"",trim($p['money']));
                if(!$money)$money=$l['locoy_com_money'];
                $data['money']   =  $money;
                $data['content'] = strip_tags(html_entity_decode($p['content'],ENT_NOQUOTES,"GB2312"),"<p> <br>");
                $data['lastupdate']  =  time();
                
                if (!empty($p['hy'])){
                    include(PLUS_PATH."industry.cache.php");
                    $data['hy']  =  $this->locoytostr($industry_name,$p['hy'],$l['locoy_rate']);
                }
                if(!isset($data['hy'])){
                    $data['hy']  =  $l['locoy_com_hy'];
                }
                
                if (!empty($p['pr'])){
                    $data['pr']  =  $this->locoytostr($this->get_com_type('pr'),$p['pr'],$l['locoy_rate']);
                }
                if(!isset($data['pr'])){
                    $data['pr']  =  $l['locoy_com_pr'];
                }
                if (!empty($p['mun'])){
                    $data['mun']  =  $this->locoytostr($this->get_com_type('mun'),$p['mun'],$l['locoy_rate']);
                }
                if(!isset($data['mun'])){
                    $data['mun']  =  $l['locoy_com_mun'];
                }
                $city_row  =  $this->get_city($p['city'],$l['locoy_rate']);
                if($city_row){
                    $i  =  1;
                    foreach($city_row as $v){
                        if($i==1)$data['provinceid']=$v;
                        if($i==2)$data['cityid']=$v;
                        $i++;
                    }
                }else{
                    $data['provinceid']  =  $l['locoy_com_province'];
                    $data['cityid']      =  $l['locoy_com_city'];
                }
                $nid  =  $companyM->upInfo($userid, array(),$data);
                return $userid;
        }
    }
    function add_user($p,$l)
    {
        $locoy_pwd  =  isset($l['locoy_pwd']) ? $l['locoy_pwd'] : '123456';
        $salt   =  substr(uniqid(rand()),-6);
        $pass   =  passCheck($locoy_pwd, $salt);
        $ip     =  fun_ip_get();
        $time   =  time();
        $username=$this->get_username($l);
        if($l['locoy_user_status']==1){
            $satus  =  1;
            $r_status  =  1;
        }else{
            $r_status  =  0;
        }
        
        if(!preg_match('/^1([0-9]{9})/',$p['linkphone'])){
            $data['linkphone']  =  trim($p['linkphone']);}
            else{
                $data['linkphone']   =  "";
            }
            $mData  =  array(
                'usertype'  =>  2,
                'username'  =>  $username,
                'password'  =>  $pass,
                'moblie'    =>  $p['moblie'],
                'email'     =>  $p['email'],
                'status'    =>  $satus,
                'salt'      =>  $salt,
                'reg_date'  =>  $time,
                'reg_ip'    =>  $ip,
                'source'    =>  6
            );
            $ratingM   =  $this->MODEL('rating');
            $sData     =  $ratingM->fetchRatingInfo();
            $uData     =  array(
                'name'         =>  $p['com_name'],
                'linkmail'     =>  $p['email'],
                'linktel'      =>  $p['moblie'],
                'address'      =>  $p['address'],
                'r_status'     =>  $r_status,
                'rating'       =>  $sData['rating'],
                'rating_name'  =>  $sData['rating_name'],
                'vipstime'     =>  $sData['vip_stime'],
                'vipetime'     =>  $sData['vip_etime']
            );
            $userinfoM  =  $this->MODEL('userinfo');
            $userid  	=  $userinfoM->addInfo(array('mdata'=>$mData,'udata'=>$uData,'sdata'=>$sData));
            return $userid;
    }
    function get_username($l){
        $row = array("a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","0","1","2","3","4","5","6","7","8","9");
        $va="";
        $length  =  isset($l['locoy_length']) ? $l['locoy_length'] : 10;
        for($i=0;$i<$length;$i++){
            $rand=mt_rand(0,61);
            $va.=$row[$rand];
        }
        $locoy_name  =  isset($l['locoy_name']) ? $l['locoy_name'] : 'locoy_';
        $data=$l['locoy_name'].$va;
        return $data;
    }
    
    function get_city($name,$locoy_rate){
        include(PLUS_PATH."city.cache.php");
        $name=str_replace(array("省","市","县","区"),"/",$name);
        $arr=explode("/",$name);
        if(is_array($arr)){
            foreach($arr as $v){
                $data[]=$this->locoytostr($city_name,$v,$locoy_rate);
            }
        }
        unset($data[3]);
        if($data[2]){
            $three_city=$arr[2];
            foreach($city_name as $key=>$vc){
                $namet=str_replace(array("省","市","县","区"),"",$vc);
                if($namet==$three_city){
                    $data[2]=$key;
                    break;
                }
            }
        }
        
        $city_type[0]=$city_index;
        $val=$this->get_all_city($city_type,$data,$locoy_rate);
        if(count($val)==1){
            $val[]=$this->get_once_city($city_type,$city_name,$val[0],$locoy_rate);
        }
        return $val;
    }
    function get_job_class($name,$locoy_rate){
        if (!empty($name)){
            include(PLUS_PATH."job.cache.php");
            $arr=explode("/",$name);
            if(is_array($arr)){
                foreach($arr as $v){
                    $data[]=$this->locoytostr($job_name,$v,$locoy_rate);
                }
            }
            $job_type[0]=$job_index;
            $val=$this->get_all_city($job_type,$data,$locoy_rate);
            if(count($val)==1){
                $val[]=$this->get_once_city($job_type,$job_name,$val[0],$locoy_rate);
            }
            return $val;
        }
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
    function get_com_type($cat){
        
        $data  =  array();
        include(PLUS_PATH."com.cache.php");
        if (isset($comdata["job_".$cat])){
            foreach($comdata["job_".$cat] as $v){
                $data[$v]=$comclass_name[$v];
            }
        }
        return $data;
    }
    function locoytostr($arr,$str,$locoy_rate="50"){
        foreach($arr as $key=>$value)
        {
            similar_text($str,$value,$percent);
            
            $rows[$percent]=$key;
            $aaa[$percent] = $value;
        }
        if (!empty($rows)){
            krsort($rows);
            foreach($rows as $k =>$v){
                if ($k>=$locoy_rate){
                    return $v;
                }else{
                    return false;
                }
            }
        }else{
            return false;
        }
    }
    function tostring($string){
        $length=strlen($string);
        $retstr=array();
        for($i=0;$i<$length;$i++) {
            $retstr[]=ord($string[$i])>127?$string[$i].$string[++$i]:$string[$i];
        }
        return $retstr;
    }
}
?>