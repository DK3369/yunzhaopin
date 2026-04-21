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
class resume_controller extends user_controller{
    //添加简历页面，判断是否满足条件并返回基本信息
	function addresume_action()
	{
		$resumeM	=   $this		->	MODEL('resume');
		
		$return		=	$resumeM	->	addResumePage(array('uid'=>$this->member['uid']), 'wxapp');

		if($return['error']['err']!=0){

			$error	=	$return['error']['err'];
			$msg	=	$return['error']['msg'];
			$this->render_json($error,$msg);
		}else{
			$error	=	$return['error']['err'];
			$data	=	$return['setarr'];
			$data['config'] = array(
			    'sy_rname_num'       =>  !empty($this->config['sy_rname_num']) ? $this->config['sy_rname_num'] : 10,
			    'sy_resumename_num'  =>  !empty($this->config['sy_resumename_num']) ? $this->config['sy_resumename_num'] : 0
			);
			$this->render_json($error,'',$data);
		}
		
	}
	//简历管理
	function resume_action()
	{
		$resumeM	=	$this->MODEL('resume');

		if (!empty($_POST['eid'])){
			$eid	=	(int)$_POST['eid'];
		}else{
		    $expect	=	$resumeM->getExpectByUid($this->member['uid']);
			
			$eid	=	$expect['id'];
		}

        $return		=	$resumeM->getInfo(array(
			'uid'		=>	$this->member['uid'],
            'eid'		=>	$eid,
            'tb'		=>	'all',
            'needCache'	=>	1
        ));
		$rewhere['uid']		=	$this->member['uid'];
		$rewhere['orderby']	=	array('lastupdate,desc');
		$rows  =  $resumeM->getSimpleList($rewhere,array('field'=>'id,name,defaults'));
		$elist = array();
		foreach($rows as $key=>$val){
		    $r['id']  =  $val['id'];
			if($val['defaults']){
			    $r['name']	=  $val['name'].'(默认)';
			}else{
			    $r['name']	=  $val['name'];
			}
			$elist[] = $r;
		}
        //判断简历最大数量
        if (!empty($this->config['user_number'])){

            $num     =  $resumeM->getExpectNum(array('uid'=>$this->member['uid']));

            $maxnum  =  $this->config['user_number'] - $num;

            if($maxnum < 0){$maxnum='0';}
        }else{
            $maxnum  =  0;
        }
	    $data  =  array(
			'elist'		=>	$elist,
	    	'expect'	=>	$return['expect'],
	    	'work'		=>	$return['work'],
	    	'edu'		=>	$return['edu'],
	    	'project'	=>	$return['project'],
	    	'training'	=>	$return['training'],
	    	'skill'		=>	$return['skill'],
	    	'other'		=>	$return['other'],
			'show'		=>	$return['show'],
	    	'resume'	=>	$return['resume'],
            'fktype'    =>  $this->fktype(),
            'top_price' =>  $this->config['integral_resume_top'],
            'maxnum'    =>  $maxnum
	    );

		$this->render_json(1,'', $data);
	}
    //添加简历前检查
	function isaddresume_action()
	{
	    $resumeM	=	$this		->	MODEL('resume');
	    $return		=	$resumeM	->	addResumeRequireCheck(array('uid'=>$this->member['uid']), 'wxapp');
		$this->render_json($return['err'],$return['msg']);
	}
	/*wxapp意向职位修改页面显示*/

	function expectedit_action()
	{
		
		if($_POST['eid']){
		    $resumeM  =  $this->MODEL('resume');
			$return		=	$resumeM->getInfo(array(
				'uid'		=>	$this->member['uid'],
				'eid'		=>	intval($_POST['eid']),
				'tb'		=>	'all',
				'needCache'	=>	1
			));

			$return['expect']['sy_rname_num']   =   $this->config['sy_rname_num'] ? $this->config['sy_rname_num'] : 10;
            $return['expect']['salaryList'] = $resumeM->salary();
		}
		$this->render_json(1,'',$return['expect']);
	}
	function rinfo_action()
	{
		$ResumeM		=	$this->MODEL('resume');
		$UserinfoM		=	$this->MODEL('userinfo');

		$table			=	$_POST['table'];
		
		$list			=	$ResumeM->getFbList($table,array('eid'=>(int)$_POST['eid'],'uid'=>$this->member['uid']));

		$list			=	count($list) ? $list : array();
		
		$this->render_json(1,'',$list);
	}
	
	/*wxapp工作经历、培训经历。。。修改页面*/
	function addresumeson_action()
	{
		if(!in_array($_POST['table'],array('expect','desc','cert','doc','edu','other','project','show','skill','training','work'))){

			unset($_POST['table']);
		}
		
		$ResumeM  		=  $this->MODEL('resume');
		
		if($_POST['table']=='desc'){

		    $desc		=	$ResumeM->getResumeInfo(array('uid'=>$this->member['uid']),array('field'=>'description,tag'));
			

			if($desc['tag']){
				$tag	=	@explode(',',$desc['tag']);
			}

			$desc['tag']=$tag?$tag:array();

			//系统的标签
			$cacheM		=  $this->MODEL('cache');
		
			$cacheList	=  $cacheM -> GetCache('user');
			foreach ($cacheList['userdata']['user_tag'] as $v){
			    if(!in_array($cacheList['userclass_name'][$v], $tag) && $cacheList['userclass_name'][$v]){
			        $usertag[]	=	$cacheList['userclass_name'][$v];
			    }
			}
			$desc['usertag']	=	$usertag?$usertag:array();
			
			$data['list']	=	$desc;
			
			include PLUS_PATH."introduce.cache.php";
			
			if(!empty($introduce_index)){
				$nid			=	$introduce_index[0];
				
				$info['name']	=	$introduce_name[$nid];
			
				$info['id']		=	$nid;
				
				$introduce_con    =   str_replace(array('&quot;','&nbsp;','<>'), array('','',''),$introduce_content[$nid]);
                $introduce_con    =   htmlspecialchars_decode($introduce_con);

				$info['content']=	$introduce_con;
				
			}
			$data['info']	=	!empty($info) ? $info : array();

		}elseif ($_POST['table'] == 'show'){
		    
		    $data['list']	=	$ResumeM->getResumeShowList(array('eid'=>$_POST['eid'],'uid'=>$this->member['uid']));
		    
		}else{
		    
		    $id  	=	$_POST['id'];
		    
		    $table	=	'resume_'.$_POST['table'];
		    
		    $row	=	$ResumeM -> getFb($table, $id ,$this->member['uid']);

		    $data['list']	=	!empty($row) ? $row : array();
		}
		$this->render_json(1,'',$data);
	}
    /*wxapp保创建简历保存*/
    function saveaddresume_action()
    {

        $resumeM    =	$this->MODEL('resume');

        $rInfo    	=   $resumeM->getResumeInfo(array('uid'=>$this->member['uid']),array('field'=>'uid,photo,defphoto,sex'));

        $_POST  	=   $this->post_trim($_POST);

        $rData	    =	array(
            'name'			=>	$_POST['realname'],
            'sex'			=>	$_POST['sex'],
            'birthday'		=>	$_POST['birthday'],
            'edu'			=>	$_POST['edu'],
            'exp'			=>	$_POST['exp'],
            'telphone'		=>	$_POST['telphone'],
            'living'		=>	$_POST['living'],
            'email'			=>	$_POST['email'],
        );

        $eData      =   array(
            'lastupdate'	 =>  time(),
            'height_status'  =>	 0,
            'uid'		     =>	 $this->member['uid'],
            'ctime'			 =>	 time(),
            'name'			 =>	 $_POST['name'],
            'hy'			 =>	 $_POST['hy'],
            'job_classid'	 =>	 $_POST['jobclassid'],
            'city_classid'   =>  $_POST['cityclassid'],
            'minsalary'		 =>	 $_POST['minsalary'],
            'maxsalary'		 =>	 $_POST['maxsalary'],
            'type'			 =>	 $_POST['type'],
            'report'		 =>	 $_POST['report'],
            'jobstatus'		 =>	 $_POST['jobstatus'],
            'state'		 	 =>	 resumeTimeState($this->config['resume_status']),
            'r_status'		 =>	 1,
            'edu'            =>  $_POST['edu'],
            'exp'            =>  $_POST['exp'],
            'sex'            =>  $_POST['sex'],
            'birthday'       =>	 $_POST['birthday'],
            'source'         =>  $_POST['source'],
        );

        /**************************简历是否必填工作经历*************************************************/
        foreach ($_POST as $pk => $pv) {
            if (strpos($pk, '_') != false) {

                $parr   =   @explode('_', $pk);

                if (count($parr) > 1) {

                    $_POST[$parr[0]][$parr[1]]  =   $pv;
                }
            }
        }
        if ($this->config['resume_create_exp'] == '1' || $this->config['resume_create_edu'] == '1' || $this->config['resume_create_project'] == '1') {
            if (isset($_POST['presave']) && empty($_POST['eid'])){
                // 第一步预创建简历
                $eData['content']   =   '简历创建未完成，中途退出，信息不完整';     // 简历备注为未完善
                $eData['state']     =   0;              // 简历状态为未审核
            } else {

                $eData['content']   =   '';             // 取消 简历未备注
            }
        }

        $workData   =   array();
        if($this->config['resume_create_exp']=='1'&&$_POST['iscreateexp']!='2'){

            for ($i=0; $i < count($_POST['workname']); $i++) {
                $workData[$i]   =   array(
                    'uid'       =>  $this->member['uid'],
                    'name'      =>  $_POST['workname'][$i],
                    'sdate'     =>  strtotime($_POST['worksdate'][$i]),
                    'edate'     =>  $_POST['totoday'][$i] ? 0 : $_POST['workedate'][$i] ? strtotime($_POST['workedate'][$i]) : 0,
                    'title'     =>  $_POST['worktitle'][$i],
                    'content'   =>  $_POST['workcontent'][$i]
                );
            }
        }

        /**************************简历是否必填教育经历*************************************************/
        $eduData    =   array();
        if ($this->config['resume_create_edu'] == '1' && $_POST['iscreateedu'] != '2') {
            for ($i = 0; $i < count($_POST['eduname']); $i++) {
                $eduData[$i]    =   array(

                    'uid'       =>  $this->member['uid'],
                    'name'      =>  $_POST['eduname'][$i],
                    'sdate'     =>  strtotime($_POST['edusdate'][$i]),
                    'edate'     =>  strtotime($_POST['eduedate'][$i]),
                    'specialty' =>  $_POST['specialty'][$i],
                    'education' =>  $_POST['eduid'][$i]
                );
            }
        }

        /**************************简历是否必填项目经历*************************************************/
        $proData    =   array();
        if ($this->config['resume_create_project'] == '1' && $_POST['iscreatepro'] != '2') {
            for ($i = 0; $i < count($_POST['projectname']); $i++) {

                $proData[$i]    =   array(
                    'uid'       =>  $this->member['uid'],
                    'name'      =>  $_POST['projectname'][$i],
                    'sdate'     =>  strtotime($_POST['projectsdate'][$i]),
                    'edate'     =>  strtotime($_POST['projectedate'][$i]),
                    'title'     =>  $_POST['projecttitle'][$i],
                    'content'   =>  $_POST['projectcontent'][$i]
                );
            }
        }

        if (!$rInfo['photo'] || ($rInfo['defphoto'] == 2 && $rData['sex'] != $rInfo['sex'])) {

            $deflogo    =  $resumeM->deflogo($rData);

            if ($deflogo != '') {

                $rData['photo']         =   $deflogo;
                $rData['defphoto']      =   2;
                $rData['photo_status']  =   0;
            }
        }

        if ($_POST['eid']) {
            $eData['id']    =   intval($_POST['eid']);
        }

        $addArr     =   array(
            'uid'       =>  $this->member['uid'],
            'rData'     =>  $rData,
            'eData'     =>  $eData,
            'workData'  =>  $workData,
            'eduData'   =>  $eduData,
            'proData'   =>  $proData,
            'utype'     =>  'user',
            'source'	=>	$_POST['source']
        );

        if (isset($_POST['presave']) && empty($_POST['eid'])){

            $addArr['presave']  =   1;

            $userinfoM  =   $this->MODEL("userinfo");
            $data       =   array('moblie' => $_POST['telphone']);
            $return     =   $userinfoM->addMemberCheck($data, $this->member['uid']);
            if ($return['error']) {

                $this->render_json(2, $return['msg']);
            }
        }

        if($this->member['usertype'] == 0){

            $userinfoM  =   $this->MODEL("userinfo");
            $userinfoM->activUser($this->member['uid'], $this->member['usertype']);
        }
        if ($_POST['eid']) {

            $return         =   $resumeM->updatePreResume($addArr);
            $return['id']   =   $_POST['eid'];
        } else {

            $return         =   $resumeM->addInfo($addArr);
        }

        $data['error']	=	$return['errcode'] == 9 ? 1 : 2;
        $data['msg']	=	$return['msg'];
        $data['eid']	=	$return['id'];

        $this->render_json($data['error'],$data['msg'],$data);
    }
	//意向职位保存
	function saveexpect_action()
	{
	    $_POST	  =  $this->post_trim($_POST);
		
		$resumeM  =  $this->MODEL('resume');
	    
	    $eid      =  (int)$_POST['eid'];
	        
		if($eid){

		    if (strlen($_POST['name']) > $this->config['sy_rname_num'] * 3 && (int)$this->config['sy_rname_num']>0){
                $this->render_json(2, '求职意向最多'.$this->config['sy_rname_num'].'个字');
            }
			
			$expectDate  =  array(
				'name'			 =>	 $_POST['name'],
				'job_classid'	 =>	 $_POST['job_classid'],
				'city_classid'	 =>	 $_POST['city_classid'],
				'minsalary'      =>	 $_POST['minsalary'],
				'type'			 =>	 $_POST['type'],
				'report'		 =>	 $_POST['report'],
				'jobstatus'	     =>	 $_POST['jobstatus'],
				'lastupdate'	 =>	 time()
			);

			foreach ($expectDate as $k=>$v){
				if (empty($v)){
					$data['error']=3;
					$data['msg']='请完善信息！！';
                    $this->render_json($data['error'],$data['msg']);
				}
			}
			$expectDate['hy']  		  =  $_POST['hy'];
			$expectDate['maxsalary']  =  $_POST['maxsalary'];
            if(resumeTimeState($this->config['user_revise_state']) == '0'){
                $expectDate['state'] = 0 ;
            }
			$return  =  $resumeM -> upInfo(array('id'=>$eid,'uid'=>$this->member['uid']), array('eData'=>$expectDate,'utype'=>'user','source'=>$_POST['source'], 'sxlog' => 1, 'port' => $this->port));
			
			$data['error']	=	$return['errcode']  == 9?1:2;
		}
		$this->render_json($data['error'],$return['msg']);
	}
	//保存简历分表 skill,work,project,edu,training,other，desc分表的保存统一使用saveresumeson
	function saveresumeson_action()
	{
		$ResumeM	=	$this->MODEL('resume');


		$_POST	=	$this->post_trim($_POST);
			
		if($_POST['table']=="resume"){

			if($_POST['tag']){

				$tag	=	array_unique(@explode(',',$_POST['tag']));

				foreach($tag as $value){

					$tagLen	=	mb_strlen($value);

					if($tagLen>=2 && $tagLen<=8){
						
						$tagList[]	=	$value;
					}
					if(count($tagList)>=5){
						break;
					}
				}
				$tagStr = implode(',',$tagList);
			}

			$rData	=	array(
				'tag'			=>	$tagStr,
				'description'	=>	$_POST['description'],
				'lastupdate'	=>	time()
			);
            if(resumeTimeState($this->config['user_revise_state']) == '0'){
                $rData['state'] = 0 ;
            }
			$return	=	$ResumeM->upResumeInfo(array('uid'=>$this->member['uid']),array('rData'=>$rData,'utype'=>'user','source'=>$_POST['source']));
			
			$data['error']	=	$return['errcode']==9 ? 1 : 2;
			$data['msg']	=	$return['errcode']==9 ?"保存成功！":"保存失败！";
			$this->render_json($data['error'],$data['msg']);
		}

		if($_POST['eid']>0){

			if(!in_array($_POST['table'],array('expect','edu','exp','other','project','skill','training','work'))){
				
				$data['error']	=	2;
				$data['msg']	=	"参数错误";
				$this->render_json($data['error'],$data['msg']);
			} 
			$table	=	"resume_".$_POST['table'];

			$id		=	(int)$_POST['id'];
			$url	=	$_POST['table'];

			unset($_POST['submit']);
			unset($_POST['table']);
			unset($_POST['id']);

			$_POST['sdate']	=	strtotime($_POST['sdate']);
			
			if(intval($_POST['totoday'])=='1'){

				unset($_POST['totoday']);
				$_POST['edate']	=	'';
			}else{
				$_POST['edate']	=	strtotime($_POST['edate']);
			}
			if($table=='resume_skill'){

				//查询修改
				$resume		 =	$ResumeM->getResumeSkill(array('id'=>$id,'eid'=>$_POST['eid']),'pic');
                if($_POST['wappic']==1){
                    if ($_POST['preview']){

                        $upArr   =  array(
                            'dir'      =>  'user',
                            'type'     =>  'skill',
                            'base'     =>  $_POST['preview'],
                        );
                        $result  =  $this -> upload($upArr);

                        if (!empty($result['msg'])){

                            $data['msg']   =  $result['msg'];

                        }elseif (!empty($result['picurl'])){

                            $_POST['pic']  =  $result['picurl'];

                        }
                    }else{
                        $_POST['pic']=$resume['pic'];
                    }
                }else {
                    if ($_FILES['photos']['tmp_name']) {

                        $UploadM = $this->MODEL('upload');

                        $upArr = array(
                            'file' => $_FILES['photos'],
                            'dir' => 'user',
                            'type' => 'skill',
                            'base' => $_POST['base'],
                            'preview' => $_POST['preview']
                        );
                        $result = $UploadM->newUpload($upArr);

                        if (!empty($result['msg'])) {
                            $data['error'] = 2;
                            $data['msg'] = $result['msg'];

                        } elseif (!empty($result['picurl'])) {

                            $_POST['pic'] = $result['picurl'];

                        }
                    } else {
                        $_POST['pic'] = $resume['pic'];
                    }
                }
			}

			if($url=='work'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'title'		=>	$_POST['title'],
					'sdate'		=>	$_POST['sdate'],
					'edate'		=>	$_POST['edate'],
					'totoday'	=>	$_POST['totoday'],
					'content'	=>	$_POST['content']
				);
			}elseif($url=='edu'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'title'		=>	$_POST['title'],
					'sdate'		=>	$_POST['sdate'],
					'edate'		=>	$_POST['edate'],
					'education'	=>	$_POST['education'],
					'specialty'	=>	$_POST['specialty']
				);
			}elseif($url=='project'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'title'		=>	$_POST['title'],
					'sdate'		=>	$_POST['sdate'],
					'edate'		=>	$_POST['edate'],
					'content'	=>	$_POST['content']
				);
			}elseif($url=='training'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'title'		=>	$_POST['title'],
					'sdate'		=>	$_POST['sdate'],
					'edate'		=>	$_POST['edate'],
					'content'	=>	$_POST['content']
				);
			}elseif($url=='skill'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'longtime'	=>	$_POST['longtime'],
					'ing'		=>	$_POST['ing'],
					'pic'		=>	$_POST['pic']
				);
			}elseif($url=='other'){
				$adata	=	array(
					'name'		=>	$_POST['name'],
					'content'	=>	$_POST['content']
				);
			}
			
			$reusmeData[$url.'Data'][] = $adata;

			if($id){
				$nid			=	$ResumeM->upResumeTable($table,array('id'=>$id,'uid'=>$this->member['uid']),$_POST,array('utype'=>'user'));
			}else{
				$adata['uid']	=	$this->member['uid'];
				$adata['eid']	=	$_POST['eid'];
				$nid			=	$ResumeM->upResumeTable($table,'',$adata,array('utype'=>'user'));
				
				if($url=='work'){
					$udata['work']		=	array('+',1);
				}elseif($url=='skill'){
				    $udata['skill']		=	array('+',1);
				}elseif($url=='project'){	
				    $udata['project']	=	array('+',1);
				}elseif($url=='edu'){	
				    $udata['edu']		=	array('+',1);
				}elseif($url=='training'){
				    $udata['training']	=	array('+',1);
				}
				
				$ResumeM->upUserResume($udata,array('eid'=>(int)$_POST['eid'],'uid'=>$this->member['uid']));
			}

			if($table=='resume_work'){
				//计算
				$workList	=	$ResumeM->getResumeWorks(array('eid'=>(int)$_POST['eid'],'uid'=>$this->member['uid']));

				$whour		=	0;
				$hour		=	array();
				foreach($workList as $value){
					//计算每份工作时长(按月)
					if ($value['edate']){

						$workTime	=	ceil(($value['edate']-$value['sdate'])/(30*86400));
					}else{
						$workTime	=	ceil((time()-$value['sdate'])/(30*86400));
					}
					$hour[]			=	$workTime;
					$whour			+=	$workTime;
				}
				//更新当前简历时长字段
				$avghour = ceil($whour/count($hour));

				$ResumeM->upInfo(array('id'=>(int)$_POST['eid'],'uid'=>$this->member['uid']),array('eData'=>array('whour'=>$whour,'avghour'=>$avghour)));
			}

			$nid?$data['msg']	=	'保存成功！':$data['msg']='保存失败！';
			$nid?$data['error']	=	'1' : $data['error'] = '2';
		}
		$this->render_json($data['error'],$data['msg']);
	}
	/*wxapp简历管理页面刷新*/
	function refresh_resume_action()
	{
		$id						=	(int)$_POST['id'];

        $upexpectData           =   array('lastupdate' => time());

		$upexpectWhere['id']	=	$id;
		$upexpectWhere['uid']   =   $this->member['uid'];

		$ResumeM				=	$this->MODEL('resume');
		$nid					=   $ResumeM->upInfo($upexpectWhere, array('eData' => $upexpectData, 'port' => $this->port, 'sxlog' => 1));
		$data['error']			=	$nid? 1 : 2;
		$data['msg']			=	$nid? '刷新成功！' : '刷新失败！';
		$this->render_json($data['error'],$data['msg']);
	}
	/*wxapp简历管理页面设置默认*/
	function default_resume_action()
	{
		$id				=	(int)$_POST['id'];

		$ResumeM		=	$this		->	MODEL('resume');

		$return			=	$ResumeM	->	defaults(array('id'=>$id,'uid'=>$this->member['uid']));
		$data['error']	=	$return['errcode'];
		$data['msg']	=	$return['msg'];
		$this->render_json($data['error'],$data['msg']);
	}
	/*wxapp简历管理页面设置是否公开*/
	function status_resume_action()
	{
        $status =   $_POST['status'];
        $resumeM=   $this->MODEL('resume');
        $return =   $resumeM->upInfo(array('uid' => $this->member['uid']), array('eData' => array('status' => $status)));

        if ($return['errcode'] == 9) {

            $resumeM->upResumeInfo(array('uid' => $this->member['uid']), array('rData' => array('status' => $status)));

            $logM       =   $this->MODEL('log');
            $logContent =   '简历更新：设置隐私状态';
            $logDetail  =   '更新简历隐私状态：公开';
            $logM->addMemberLog($this->member['uid'], 1, $logContent, 2, 2, $logDetail);
        }
        $data['error']  =   $return['errcode'] == 9 ? 1 : 2;
        $data['msg']    =   $return['errcode'] == 9 ? '设置成功！' : '设置失败！';
        $this->render_json($data['error'], $data['msg']);
	}
	/*wxapp简历管理页面删除*/
	//原本的del_resume既可以删除整个简历也可以删除简历附表，现参照wap分开，删除附表为delResumeFb
	function del_resume_action()
	{
		$ResumeM  =  $this->MODEL('resume');
        
        $id       =  $_POST['id'];
        $uid = $this->member['uid'];
        $return   =  $ResumeM->delResume($id,array('uid'=>$uid));
        $error	  =  $return['errcode']==9 ? 1 : 2;
		$this->render_json($error,$return['msg']);
	}
	
	// 删除简历附表,小程序内需要修改为此方法
	function delResumeFb_action()
	{
	    $table			=	$_POST['table'];
	    
	    $fbwhere['eid']	=	intval($_POST['eid']);
	    $fbwhere['id']	=	$_POST['id'];
	    $fbwhere['uid']	=	$this->member['uid'];
	    
	    $ResumeM		=	$this->MODEL('resume');
	    
	    $return			=	$ResumeM->delFb($table,$fbwhere,array('utype'=>'user'));
		$error	  =  $return['errcode']==9 ? 1 : 2;
		$this->render_json($error,$return['msg']);
	}
	/*wxapp谁看过我记录*/
	function look_resume_action()//简历浏览记录
	{
		$LookResumeM		=	$this->MODEL('lookresume');
		$page				=	$_POST['page'];
		$limit				=	$_POST['limit'];
		$limit				=	!$limit?20:$limit;
		$where['uid']		=	$this->member['uid'];
		$where['status']	=	0;
		$where['usertype']	=	2;
		$where['orderby']	=	array('id,desc');
		$total = $LookResumeM->getLookNum($where);
		if($page){
			$pagenav		=	($page-1)*$limit;
			$where['limit']	=	array($pagenav,$limit);
		}else{
			$where['limit']	=	array('',$limit);
		}
		$looknew			=	$LookResumeM->getList($where, array('uid' => $this->member['uid'], 'usertype' => $this->member['usertype']));
		$rows				=	$looknew['list'];
		if(is_array($rows)&&!empty($rows)){
			$data			=	count($rows)?$rows:array();
			$error			=	1;
		}else{
			$error			=	2;
		}
		$this -> render_json($error,'',$data,$total);
	}
	/*wxapp谁看过我记录删除*/
	function look_resume_del_action()//删除简历浏览记录
	{
		$del	      	=  $_POST['ids'];

        $lookResumeM  	=  $this->MODEL('lookresume');
        
        $return       	=  $lookResumeM -> delInfo(array('id'=>$del,'uid'=>$this->member['uid'],'usertype'=>$this->member['usertype']));
        $data['error']	=  $return['errcode']==9 ? 1 : 2;
		$this -> render_json($data['error'],$return['msg'],'');
	}
	//检查简历创建基本信息
	function checkMember_action(){
		$userinfoM	=	$this->MODEL("userinfo");
		$data	=	array(
			'moblie'	=> $_POST['telphone'],
			'email'		=> $_POST['email']
		);
		$return		=	$userinfoM->addMemberCheck($data,$this->member['uid']);
		
		if($return['error']){
			$this -> render_json(2,$return['msg']);
		}else{
			$this -> render_json(1,'');
		}
		
	}
	//wap简历置顶
	function topCheck_action(){
		$data	=	array(
			'eid'	=>	$_POST['eid'],
			'uid'	=>	$this->member['uid']
		);
		$resumeM	=	$this -> MODEL('resume');
		
		$return		=	$resumeM -> topResumeCheck($data);
		
		$this -> render_json('0',$return['msg']);
	}
	//个人评价看看别人怎么写
	function getIntroduceInfo_action(){
		include PLUS_PATH."introduce.cache.php";
		
		if($_POST['introduceid']){
			
			$id			=	intval($_POST['introduceid']);
			
			foreach($introduce_index as $key=>$val){
				if($val==$id){
					unset($introduce_index[$key]);
				}
				
			}
		}	
		
		$keyid	=	array_rand($introduce_index);
		
		$nid	=	$introduce_index[$keyid];
		
		if(!empty($nid)){
			
			$data['name']	=	$introduce_name[$nid];
			
			$data['id']		=	$nid;

			$introduce_con    =   str_replace(array('&quot;','&nbsp;','<>'), array('','',''), $introduce_content[$nid]);
            $introduce_con    =   htmlspecialchars_decode($introduce_con);
			
			$data['content']	=	$introduce_con;

		}else{
			
			$data['name']	=	$introduce_name[$nid];
			
			$data['id']		=	'';
			
			$data['content']	=	'暂无示例';

		}
		
		$this -> render_json(1,'',$data);
		
	} 
	//简历发布成功
	function rcomplete_action(){
		$data		=	array(
			'id'	=>	$_POST['eid'],
			'uid'	=>	$this->member['uid'],
			'limit'	=>	5
		);
		$resumeM	=	$this->MODEL('resume');
		$list		=	$resumeM->likeJob($data);
		
		if(empty($list)){
			$jobM	=	$this->MODEL('job');
			$jwhere =	array(
				'state'		=>	1,
				'r_status'	=>	1,
				'status'	=>	0,
				'limit'		=>	5,
			    'lastupdate'=>  array('>', strtotime('-30 day'))
			);
			$jlist	=	$jobM->getList($jwhere);
			$list	=	$jlist['list'];
		}
		
		$this -> render_json(1,'',$list);
	}
    //保存个人作品
	function showSave_action(){
		
		$ResumeM	=	$this->MODEL('resume');
		if($_POST['wappic']==1){
            if ($_POST['uimage']){

                $upArr   =  array(
                    'dir'      =>  'user',
                    'type'     =>  'show',
                    'base'     =>  $_POST['uimage'],
                );
                $result  =  $this -> upload($upArr);

                if (!empty($result['msg'])){

                    $data['error'] = 2;
                    $data['msg'] = $result['msg'];
                    $this->render_json($data['error'], $data['msg']);

                }elseif (!empty($result['picurl'])){

                    $_POST['picurl']  =  $result['picurl'];

                }
            }else{
                $this->render_json(2, '请上传图片');
            }
        }else {
            if ($_FILES['show']['tmp_name']) {

                $UploadM = $this->MODEL('upload');

                $upArr = array(
                    'file' => $_FILES['show'],
                    'dir' => 'user',
                    'type' => 'show'
                );
                $result = $UploadM->newUpload($upArr);

                if (!empty($result['msg'])) {
                    $data['error'] = 2;
                    $data['msg'] = $result['msg'];
                    $this->render_json($data['error'], $data['msg']);

                } elseif (!empty($result['picurl'])) {

                    $_POST['picurl'] = $result['picurl'];

                }
            } else {
                $this->render_json(2, '请上传图片');
            }
        }

		$adata	=	array(
			'picurl'	=>	$_POST['picurl'],
			'status'  	=>  $this->config['rshow_photo_status']
		);
		$reusmeData['showData'][] = $adata;
		
		$adata['ctime']	=	time();
		$adata['uid']	=	$this->member['uid'];
		$adata['eid']	=	$_POST['eid'];
		$nid			=	$ResumeM->upResumeTable('resume_show','',$adata,array('utype'=>'user'));
		
		$list  =  $ResumeM->getResumeShowList(array('uid'=>$this->member['uid'],'eid'=>(int)$_POST['eid']));
		
		$udata['show']	=  count($list);
		$ResumeM->upUserResume($udata,array('eid'=>(int)$_POST['eid'],'uid'=>$this->member['uid']));
		$nid?$data['msg']	=	'保存成功！':$data['msg']='保存失败！';
		$nid?$data['error']	=	'1' : $data['error'] = '2';
		$this->render_json($data['error'],$data['msg']);
	}
    /**
     * 处理单个图片上传
     * @param file/需上传文件; dir/上传目录; type/上传图片类型; base/需上传base64; preview/pc预览即上传
     */
    private function upload($data = array('file'=>null,'dir'=>null,'type'=>null,'base'=>null,'preview'=>null)){

        $UploadM  =  $this	  ->	MODEL('upload');
        $upArr    =  array(
            'file'     =>  $data['file'],
            'dir'      =>  $data['dir'],
            'type'     =>  $data['type'],
            'base'     =>  $data['base'],
            'preview'  =>  $data['preview']
        );
        $return  =  $UploadM -> newUpload($upArr);

        return $return;
    }

}