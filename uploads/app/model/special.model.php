<?php
/*
* $Author ：PHPYUN开发团队
*
* 官网: http://www.phpyun.com
*
* 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
*
* 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class special_model extends model{
	/**
     * @desc   引用company类，查询company列表信息   
     */
    
    private function getComList($whereData = array(), $data = array()) {

        require_once ('company.model.php');
        
        $CompanyM = new company_model($this->db, $this->def);
        
        return  $CompanyM   ->  getList($whereData , $data);
        
    }
	/**
     * @desc   获取缓存数据
     *
     * @param   array $options
     * @return  array $cache
     */
    private function getClass($options)
    {
             
        include_once ('cache.model.php');
        
        $cacheM     =   new cache_model($this->db, $this->def);
        
        $cache      =   $cacheM -> GetCache($options);
        
        return $cache;
     
    }

    function getSpecialNum($whereData = array())
    {
        return intval($this->select_num('special', $whereData));
    }

    function getSpecial($whereData=array(),$data=array()){
		$ListNew		=	array();
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		$List			=	$this -> select_all('special',$whereData,$data['field']);
		
		if(!empty( $List )){
			
			$ListNew['list']	=	$List;
		}

		return	$ListNew;
    }
	
	function getSpecialOne($whereData=array(),$data=array()){
		if($whereData){
			$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
			$List			=	$this -> select_once('special',$whereData,$data['field']);
			if(!empty($List)){
				$List['ctime_n']=date('Y年-m月-d日',$List['ctime']);
				$List['etime_n']=date('Y年-m月-d日',$List['etime']);
                $List['intro']=str_replace(array("&amp;","background-color:#ffffff","background-color:#fff","white-space:nowrap;"),array("&",'','',''),$List["intro"]);
				if($List['pic']){
					$List['pic']		=	checkpic($List['pic']);
				}
				if($List['background']){
					$List['background']	=	checkpic($List['background']);
				}
				if($List['wappic']){
				    $List['wappic']	    =	checkpic($List['wappic']);
				}
				if($List['wapback']){
				    $List['wapback']	=	checkpic($List['wapback']);
				}
				if($List['limit']>$List['num']){
					$List['apply']		=	'1';
				}
			}
		}
		return $List;
    }
	
    function addSpecial($setData=array()){
		if(!empty($setData)){
			
			$nid	=	$this -> insert_into('special',$setData);
			
		}

		return $nid;
    }
	
    function upSpecial($whereData=array(),$data=array()){
		if(!empty($whereData)){
			
			$nid	=	$this -> update_once('special',$data,$whereData);
			
		}
		return $nid;
    }
	
	function addSpecialCom($setData=array()){
		if(!empty($setData)){
			
			$nid	=	$this -> insert_into('special_com',$setData);
			
		}

		return $nid;
    }

    /**
     * 添加多个参会企业
     * @param array $data
     * @return array
     */
    public function addSpecialMutiCom($data = array())
    {

        $sid = $data['sid'];
        $uid = $data['uid'];

        if ($sid && $uid) {

            $row    =   $this->select_all('special_com', array('sid' => $sid), '`uid`');

            $s_uid_arr = array();

            foreach ($row as $k => $v) {
                $s_uid_arr[] = $v['uid'];
            }

            $uid_arr = @explode(',', $uid);

            $AddData = array(
                'sid' => $sid,
                'time' => time(),
                'status' => isset($data['status']) ? $data['status'] : 1,
            );

            if (!empty($uid_arr)) {

                $comlist    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uid_arr))), '`uid`,`r_status`');
                $comdata    =   array();
                foreach ($comlist as $ck => $cv) {
                    $comdata[$cv['uid']] = $cv;
                }

                foreach ($uid_arr as $cv) {
                    if (!in_array($cv, $s_uid_arr) && $comdata[$cv]['r_status'] == 1) {
                        $AddData['uid'] =   $cv;
                        $this->insert_into('special_com', $AddData);
                    }
                }

                $return['error'] = 0;
                $return['msg'] = '添加成功';

            } else {
                $return['error'] = 8;
                $return['msg'] = '请选择要添加的企业';
            }

        } else {
            $return['error'] = 8;
            $return['msg'] = '参数错误请重试';
        }

        return $return;
    }
	
	function upSpecialCom($whereData=array(),$data=array()){
		if(!empty($whereData)){
			
			$nid	=	$this -> update_once('special_com',$data,$whereData);
			
		}
		return $nid;
    }
	
	function getSpecialComOne($whereData=array(),$data=array()){
		if($whereData){
			$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		
			$List			=	$this -> select_once('special_com',$whereData,$data['field']);
		}

		return $List;
    }
	
	//删除专题
	function delSpecial($whereData,$data){
		
		if($data['type']=='one'){
			$limit		=	'limit 1';
		}
		if($data['type']=='all'){
			$limit		=	'';
		}

		$special	=	$this->getSpecialList(array('id'=>$whereData['id']),array('field'=>'`pic`,`background`'));
		

		$result		=	$this -> delete_all('special',$whereData,$limit);

		if(is_array($special['list']) && $result){
			
			$this -> delSpecialCom(array('sid'=>$whereData['id']),$data['type']);
		}
		
		
		return	$result;
		
    }
	
	//删除专题商家
    function delSpecialCom($whereData,$data){
		
		if($data['type']=='one'){
			$limit	=	'limit 1';
		}
		
		if($data['type']=='all'){
			$limit	=	'';
		}
		if($data['uid']){
		
			$getWhere	=	array('uid'=>$data['uid'],'id'=>$whereData['id'],'status'=>0);
		}else{
			
			$getWhere	=	array('id'=>$whereData['id'],'status'=>0);
		}

		$rows	=	$this->getSpecialComList($getWhere,array('field'=>'`uid`,`integral`'));
		
		if(is_array($rows['list'])){
			
			require_once ('integral.model.php');
        
			$IntegralM 	= 	new integral_model($this->db, $this->def);
			
			foreach($rows['list'] as $val){
				
				if($val['integral']>0){
					
					$IntegralM->company_invtal($val['uid'],2,$val['integral'],true,"取消专题招聘报名，退还".$this->config['integral_pricename'],true,2,'integral');
				}
			
			}
		}
		
		$result	=	$this	->	delete_all('special_com',$whereData,$limit);
		
		return	$result;
		
    }
	
	//获取专题商家列表
	function getSpecialComList($whereData=array(),$data=array()){

 		$ListNew		=	array();
		$data['field']  =	empty($data['field']) ? '*' : $data['field'];
		$List			=	$this -> select_all('special_com',$whereData,$data['field']);
 		$jobinfo=array();
		if(!empty( $List )){
			foreach($List as $val){
				
			    if (!empty($val['uid'])) {
				    $uid[]			=	$val['uid'];
				}
				if (!empty($val['sid'])) {
				    $sid[]			=	$val['sid'];
				}
			}
			if (!empty($uid)){
			    $comWhere['uid']	=	array('in',pylode(',',$uid));
			    $company			=	$this->getComList($comWhere,array('field'=>'`uid`,`name`,`hy`,`logo`,`mun`,`rec`,`hotstart`,`hottime`'));
			    $companyMaps		=	array();
                foreach ($company['list'] as $valCompany) {
                    $companyMaps[$valCompany['uid']] = $valCompany;
                }
			    if (isset($data['utype']) && $data['utype']=='wxapp') {
					$job			=	$this->select_all("company_job",array('uid'=>array('in',pylode(',',$uid)),'state'=>1,'status'=>0,'orderby'=>'lastupdate,desc'),'`id`,`uid`,`name`,`minsalary`,`maxsalary`,`exp`,`edu`');
			    }elseif(isset($data['utype']) && $data['utype']=='admin') {
					$job			=	$this->select_all("company_job",array('uid'=>array('in',pylode(',',$uid)),'state'=>1,'orderby'=>'lastupdate,desc'),'`id`,`uid`,`name`,`minsalary`,`maxsalary`,`exp`,`edu`');
				}
			}
			if (!empty($sid) && $data['utype'] == 'user'){
			    $special			=	$this->getSpecial(array('id'=>array('in',pylode(',',$sid))),array('field'=>'id,title,intro'));
                $specialMaps		=	array();
                foreach ($special['list'] as $valSpecial){
                    $specialMaps[$valSpecial['id']] = $valSpecial;
                }
			}
			if(!empty($job)){
			    $cache  =  $this -> getClass(array('hy','com'));
			    
				foreach($job as $k=>$v){
					$v['edu_n']=$cache['comclass_name'][$v['edu']];
					$v['exp_n']=$cache['comclass_name'][$v['exp']];
					$v['job_salary'] = salaryUnit($v['minsalary'], $v['maxsalary']);
					$jobinfo[$v['uid']][]=$v;
				}
			}
			foreach($List as $key=>$val){
			    if (isset($data['utype']) && $data['utype']=='admin') {
					
                    if (isset($companyMaps[$val['uid']])) {
                        $v = $companyMaps[$val['uid']];
						$List[$key]['name']		=	$v['name'];
                        $List[$key]['comUrl']	=	Url('company', array('c' => 'show', 'id' => $v['uid'], 'look' => 'admin'));
                    }
				}
				if (isset($data['utype']) && ($data['utype']=='wxapp' || $data['utype'] == 'gl')) {
				    $List[$key]['hyname']	=	'';
				    $List[$key]['mun_n']	=	'';
                    if (isset($companyMaps[$val['uid']])) {
                        $v = $companyMaps[$val['uid']];
                        $List[$key]['name_n']	=	$v['name'];
                        $List[$key]['hyname']	=	$v['hy_n'];
                        $List[$key]['logo'] 	=   checkpic($v['logo'],$this->config['sy_unit_icon']);
                        $List[$key]['mun_n']	=	$v['mun_n'];
                        $List[$key]['url']	    =	Url('company', array('c'=>'show','id'=>$v['uid']));
                        $List[$key]['wapurl']	=	Url('wap', array('c'=>'company','a'=>'show','id'=>$v['uid']));
					}
				}
				if (isset($data['utype']) && $data['utype'] == 'user'){
                    if (isset($specialMaps[$val['sid']])) {
                        $v = $specialMaps[$val['sid']];
                        $List[$key]['title']=$v['title'];
                        $List[$key]['intro']=$v['intro'];
				    }
				}
				if (!empty($jobinfo)){
				    $List[$key]['jobs'] = $jobinfo[$val['uid']];
				}
				if (isset($val['time'])){
				    $List[$key]['spetime_n'] = date('Y-m-d',$val['time']);
				}
			}
			
			$ListNew['list']	=	$List;
		}

		return	$ListNew;
    }
	
	
	//获取专题列表
	function getSpecialList($whereData=array(),$data=array()){
		$ListNew	=	array();
		$List		=	$this -> getSpecial($whereData,$data);
		
		if(!empty( $List )){
			
			foreach($List['list'] as $key=>$val){
				$zid[]	=	$val['id'];
				$List['list'][$key]['ctime_n']	=	date('Y-m-d',$val['ctime']);
				if(!empty($val['pic'])) {
                    $List['list'][$key]['pic_n'] = checkpic($val['pic']);
                }else{
                    $List['list'][$key]['pic_n'] = "";
                }
                if(!empty($val['wappic'])){
                    $List['list'][$key]['wappic_n'] = checkpic($val['wappic']);
                }else{
                    $List['list'][$key]['wappic_n'] = "";
                }
			}
			
			if ($data['utype']=='admin') {
				$oneWhere['sid']		=	array('in',pylode(",",$zid));
				$oneWhere['groupby']	=	'sid';
				$all					=	$this->getSpecialComList($oneWhere,array('field'=>'`sid`,count(id) as num'));
				
				$twoWhere['sid']		=	array('in',pylode(",",$zid));
				$twoWhere['status']		=	'0';
				$twoWhere['groupby']	=	'sid';
				$status					=	$this->getSpecialComList($twoWhere,array('field'=>'`sid`,count(id) as num'));
				foreach($List['list'] as $key=>$v){
					foreach($all['list'] as $val){
						if($v['id']	==	$val['sid']){
							$List['list'][$key]['comnum']	=	$val['num']>=0	?	$val['num']:0;
						}
					}
					foreach($status['list'] as $val){
						if($v['id']	==	$val['sid']){
							$List['list'][$key]['booking']	=	$val['num']>=0	?	$val['num']:0;
						}
					}
                    $List['list'][$key]['title_href'] = Url('special', array('c' => 'show', 'id' => $v['id']));
                    $List['list'][$key]['display_switch'] = $v['display'] == 1 ? true : false;
				}
           
		   }
			
			$ListNew['list']	=	$List['list'];
		}
		
		return	$ListNew;
		
	}
	
	 
	public function getSpecialComNum($Where=array()){
		
        return intval($this->select_num('special_com',$Where));
    }
	
	public function addSpecialComInfo($data=array()){
		
		$id			=	(int)$data['id'];
		
		if($data['uid']&&$data['usertype']=='2'){
			
			$info	=	$this->getSpecialOne(array("id"=>$id));
			
			if($info['com_bm']!='1'){
				return array('msg'=>'该专题禁止报名！','errcode'=>8);
			}else if($info['etime']<time()){
				return array('msg'=>'该专题报名已结束！','errcode'=>8);
			}
			require_once ('statis.model.php');
			$statisM	= new statis_model($this->db, $this->def);
			$statis		=	$statisM->getInfo($data['uid'],array("usertype"=>'2','field'=>'integral,`rating`'));
			
			$isapply	=	$this->getSpecialComNum(array("uid"=>$data['uid'],"sid"=>$id));
			$applynum 	=	$this->getSpecialComNum(array("sid"=>$id));
			

			if($isapply){
				return array('msg'=>'您已报名该专题，请等待管理员审核！','errcode'=>8);
			}
			
			if($info['rating']){
				$rating	=	@explode(',',$info['rating']);
			}
			
			$jobnum		=	$this->select_num('company_job',array("uid"=>$data['uid'],"state"=>'1','sdate'=>array('<',time())));
			
			if($info['limit']<=$applynum){
				return array('msg'=>'报名已满，请下次提前报名！','errcode'=>8);
			}
			if($jobnum<1){
				return array('msg'=>'您暂无公开且合适职位！','errcode'=>8);
			}  
			if($rating&&is_array($rating)){ 
				
				if(!in_array($statis['rating'],$rating)){
					require_once ('rating.model.php');
					$ratingM		= 	new rating_model($this->db, $this->def);
					
					$ratings		=	$ratingM->getList(array("display"=>1,'category'=>1,'id'=>array('in',$info['rating'])),array("field"=>"`id`,`name`"));
					
					$rname			=	array();
					foreach($ratings as $val){
						$rname[]	=	$val['name'];
					}
					return array('msg'=>'只有'.@implode('、',$rname).'才能报名该专题！','errcode'=>8);
				}
			}
			if($statis['integral']<$info['integral']){
				return array('msg'=>$this->config['integral_pricename'].'不足，请先充值！','errcode'=>8);
			}
			require_once ('integral.model.php');
			$integralM		= 	new integral_model($this->db, $this->def);
					
			$nid	=	$integralM->company_invtal($data['uid'],2,$info['integral'],false,"报名专题招聘",true,2,'integral',9);
			if($nid){
				
				$this->insert_into('special_com',array("sid"=>$id,"uid"=>$data['uid'],'integral'=>$info['integral'],'status'=>'0','time'=>time()));
				$cominfo = $this->select_once('company',array('uid'=>$data['uid']),'`name`');
				return array('msg'=>'报名成功，请耐心等我们工作人员审核！','errcode'=>9,'url'=>$_SERVER['HTTP_REFERER']);
			}else{
				return array('msg'=>'报名失败，请稍后重试！','errcode'=>8,'url'=>$_SERVER['HTTP_REFERER']);
			}
		}else{
			return array('msg'=>'只有企业用户才能报名！','errcode'=>8);
		}
    }	
    // gl模板所需的参会企业行业
    function getSpecialHy($uid)
    {
        $cn  = array();
        $cw  = array('uid'=>array('in',pylode(',', $uid)),'r_status'=>1,'hy'=>array('>',0),'groupby'=>'hy');
        // 查职位，将没有职位的企业排除
        $joblist = $this->select_all('company_job', array('uid'=>array('in',pylode(',',$uid)),'r_status'=>1,'state'=>1,'status'=>0),'`uid`');
        
        if (!empty($joblist)){
            foreach ($joblist as $v){
                $juid[] = $v['uid'];
            }
            $juid = array_unique($juid);
            $cw['uid'] = array('in', pylode(',', $juid));
            
            $com = $this->select_all('company', $cw,'hy,count(hy) as hynum');
            
            foreach ($com as $v){
                $cn[] = $v['hynum'];
            }
            // 按行业对应企业数量，进行降序排列
            array_multisort($cn,SORT_DESC,SORT_NUMERIC,$com);
        }
        
        $jn  = array();
        $jw  = array('uid'=>array('in',pylode(',', $uid)),'r_status'=>1,'state'=>1,'status'=>0,'hy'=>array('>',0),'groupby'=>'hy');
        $job = $this->select_all('company_job', $jw,'hy,count(hy) as hynum');
        foreach ($job as $v){
            $jn[] = $v['hynum'];
        }
        // 按行业对应企业数量，进行降序排列
        array_multisort($jn,SORT_DESC,SORT_NUMERIC,$job);
        
        $cache  =  $this -> getClass('hy');
        $hyname =  $cache['industry_name'];
        
        $ch = $jh = array();
        if (!empty($com)){
            foreach ($com as $v){
                $ch[] = array('id'=>$v['hy'],'name'=>$hyname[$v['hy']]);
            }
        }
        if (!empty($job)){
            foreach ($job as $v){
                $jh[] = array('id'=>$v['hy'],'name'=>$hyname[$v['hy']]);
            }
        }
        return array('comhy'=>$ch, 'jobhy'=>$jh);
    }
    // gl模板精选企业
    function glComList($sid, $hy, $page, $numb)
    {
        $sp = $this -> select_all('special_com',array('sid'=>$sid, 'status'=>1),'uid');
        if (!empty($sp)){
            foreach ($sp as $v){
                $uid[] = $v['uid'];
            }
            // page从0开始
            if(!empty($page)){
                $pagenav  =  ($page)*$numb;
                $limit	  =	 array($pagenav,$numb);
            }else{
                $limit  =  array('',$numb);
            }
            $where = array('uid'=>array('in',pylode(',',$uid)));
            if (!empty($hy)){
                $where['hy'] = $hy;
            }
            // 先查出所有企业
            $comlist  =  $this->select_all('company', $where, '`uid`');
            
            if (!empty($comlist)){
                foreach ($comlist as $v){
                    $cuid[] = $v['uid'];
                }
                // 在查出所有企业中，有职位的企业
                $joblist = $this->select_all('company_job', array('uid'=>array('in',pylode(',',$cuid)),'r_status'=>1,'state'=>1,'status'=>0),'`uid`');
                if (!empty($joblist)){
                    foreach ($joblist as $v){
                        $juid[] = $v['uid'];
                    }
                    $juid = array_unique($juid);
                    
                    $where['uid']  =  array('in', pylode(',', $juid));
                    
                    $num = $this->select_num('company', $where);
                    
                    $where['limit']   = $limit;
                    // 排序按企业登录时间
                    $where['orderby'] = 'login_date';
                    $company  =  $this->getComList($where, array('logo'=>1,'url'=>1,'field'=>'`uid`,`name`,`hy`,`logo`,`logo_status`,`mun`,`content`'));
                    
                    if (!empty($company)){
                        foreach ($company['list'] as $v){
                            $cuid[] = $v['uid'];
                        }
                        $jobs = $this->select_all('company_job', array('uid'=>array('in',pylode(',',$cuid)),'r_status'=>1,'state'=>1,'status'=>0),'`uid`');
                        
                        foreach ($company['list'] as $k=>$v){
                            $content = strip_tags($v['content']);
                            $content = str_replace('&nbsp;', '', $content);
                            $company['list'][$k]['content'] = mb_substr($content, 0,30);
                            foreach ($jobs as $val){
                                if ($v['uid'] == $val['uid']){
                                    $company['list'][$k]['joblist'][] = $val;
                                }
                            }
                        }
                        foreach ($company['list'] as $k=>$v){
                            if (empty($v['joblist'])){
                                unset($company['list'][$k]);
                            }
                        }
                    }
                }
            }
        }
        $return['list']  = isset($company['list']) ? $company['list'] : array();
        $return['count'] = isset($num) ? ceil($num/$numb) : 1;
        
        return	$return;
    }
    // gl模板精选职位
    function glJobList($data = array() )
    {
        $sid     = $data['sid'];
        $hy      = intval($data['hy']);
        $page    = intval($data['page']);
        $numb    = intval($data['numb']);
        $keyword = trim($data['keyword']);
        $type    = trim($data['type']);
        
        $sp = $this -> select_all('special_com',array('sid'=>$sid, 'status'=>1),'uid');
        if (!empty($sp)){
            foreach ($sp as $v){
                $uid[] = $v['uid'];
            }
            // page从0开始
            if(!empty($page)){
                $pagenav  =  ($page)*$numb;
                $limit	  =	 array($pagenav,$numb);
            }else{
                $limit  =  array('',$numb);
            }
            $where = array('uid'=>array('in',pylode(',',$uid)),'r_status'=>1,'state'=>1,'status'=>0);
            if (!empty($hy)){
                $where['hy'] = $hy;
            }
            $num = $this->select_num('company_job', $where);
            
            if (!empty($keyword)){
                if ($type == 'qy'){
                    $where['com_name'] = array('like', $keyword);
                }elseif ($type == 'zw'){
                    $where['name'] = array('like', $keyword);
                }
            }
            $where['limit'] = $limit;
            // 排序按企业登录时间
            $where['orderby'] = 'lastupdate';
            
            require_once ('job.model.php');
            $JobM  =  new job_model($this->db, $this->def);
            $job   =  $JobM->getList($where, array('isurl'=>'yes','utype'=>'wxapp'));
            
        }
        $return['list']  = isset($job['list']) ? $job['list'] : array();
        $return['count'] = isset($num) ? ceil($num/$numb) : 1;
        
        return	$return;
    }
    /**
     * gl模板名企
     */
    public function glFamous($where = array())
    {
        $where['famous']  =  1;
        $where['status']  =  1;
        $list = $this->select_all('special_com', $where, '`uid`');
        
        foreach ($list as $v){
            
            $uid[]=$v['uid'];
        }
        
        $com  =  $this->getComList(array('uid'=>array('in',pylode(',', $uid)),'r_status'=>1), array('logo'=>1));
        
        $hjwhere['uid']	     =	array('in',pylode(',', $uid));
        $hjwhere['state']	 =	1;
        $hjwhere['status']	 =	0;
        $hjwhere['r_status'] =	1;
        
        require_once ('job.model.php');
        $jobM    =  new job_model($this->db, $this->def);
        $hjrows  =  $jobM -> getList($hjwhere);
        
        foreach ($list as $k => $v){
            
            foreach ($com['list'] as $val){
                if ($v['uid'] == $val['uid']){
                    if($val['shortname']){
                        $list[$k]['name']  =  $val['shortname'];
                    }else{
                        $list[$k]['name']  =  $val['name'];
                    }
                    $list[$k]['hot_pic']  =  $val['logo'];
                    $list[$k]['citystr']  =  $val['citystr'];
                    $list[$k]['hy_n']     =  $val['hy_n'];
                    $list[$k]['pr_n']     =  $val['pr_n'];
                    $list[$k]['mun_n']    =  $val['mun_n'];
                    $list[$k]['content']  =  mb_substr(strip_tags($val['content']), 0,100);
                }
            }
            $list[$k]['joblist'] = array();
            foreach ($hjrows['list'] as $val){
                if ($v['uid'] == $val['uid']){
                    $list[$k]['joblist'][] = $val;
                }
            }
        }
        foreach ($list as $k => $v){
            if (empty($v['joblist'])){
                unset($list[$k]);
            }else{
                foreach ($v['joblist'] as $jk=>$jv){
                    if($jk > 4){
                        unset($v['joblist'][$jk]);
                    }
                }
                $list[$k]['joblist'] = $v['joblist'];
            }
        }
        $list = array_values($list);
        return $list;
    }
}
?>