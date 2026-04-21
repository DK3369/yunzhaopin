<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class ask_model extends model{

    /**
     * @desc   查询member会员表
     * @param array $where
     * @param string $field
     * @return array|bool|false|string|void
     */
    private function getMemberList($where = array(), $field = '*'){
        
        $members  =   $this -> select_all('member', $where, $field);
        
        return $members;
    }

    /**
     * @desc    获取问答管理列表
     * @Desc    查训 question 表数据 ，多条数据查询
     * @param   $whereData :查询条件
     * @param   $data :自定义处理数组
     * @return array|bool|false|string|void
     */
	public function getList($whereData,$data=array())
    {
       
		$select   =   $data['field'] ? $data['field'] : '*';     
      
		$List     =   $this -> select_all('question', $whereData, $select);
		
		if(is_array($List) && $List ){
		    
		    if (isset($data['utype']) && $data['utype']=='admin') {
		        
		        $qids = array();

                foreach ($List as $v) {

                    $qids[] =   $v['id'];
                    $uids[] =   $v['uid'];   //作者id
                    $cids[] =   $v['cid'];   //类别id
                }

    			$awhere = array('qid'=>array('in', pylode(',', $qids)),'groupby'=>'qid');

    			if($data['utype']!='admin'){
    				$awhere['status'] = 1;
    			}

    			$alist	        =	$this->select_all('answer',$awhere,'qid,count(*) as num');
    			$memberList		=	$this->select_all('member',array('uid'=>array('in',pylode(',',$uids))));
    			$qclassList		=	$this->getQclassList(array('id'=>array('in',pylode(',',$cids))));
    			
    			foreach($List as $key => $val){
    				
    				$List[$key]['answer_num'] = 0;

    				if(!empty($alist)){
						foreach($alist as $av){
							if($av['qid'] == $val['id']){
								$List[$key]['answer_num']	=	$av['num'];	//	 参会企业
							}
						}
					}
					$List[$key]['pic']=checkpic($val['pic'],$this->config['sy_friend_icon']);
    				foreach($memberList as $v){
    					if($val['uid'] == $v['uid']){
    						$List[$key]['username']	=	$v['username'];
    					}
    				}
    				foreach($qclassList as $value){
    					if($val['cid'] == $value['id']){
    						$List[$key]['classname']	=	$value['name'];
    					}
    				}

    				isset($val['add_time']) && $List[$key]['add_time_n'] = date('Y-m-d H:i', $val['add_time']);
    				$List[$key]['ask_url'] = Url('ask', array('c' => 'content', 'look' => 'admin', 'id' => $val['id']));
    			}
		    }
			if(isset($data['utype']) && $data['utype'] == 'hot'){	// 推荐达人
				$recUser	=	$recUids	=	array();
				foreach ($List as $k => $v){
					$v['pic']	    =	checkpic($v['pic'], $this->config['sy_friend_icon']);
					if(!in_array($v['uid'], $recUids)){
						$recUids[]	=	$v['uid'];
						$recUser[]	=	$v;
					}
				}
				$List['recUser']	=	$recUser;
			}
			return $List;  
		}
	}

    /**
     * @desc    获取问答详情;查询 question 单条记录查询
     * @param $id
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getInfo($id, $data = array())
    {

        if (!empty($id)) {

            $where['id']    =   intval($id);
        } elseif ($data['where']) {

            $where          =   $data['where'];
        }

        $select     =   $data['field'] ? $data['field'] : '*';
        $question   =   $this->select_once('question', $where, $select);
        if ($question && is_array($question)) {
            $question['pic'] = checkpic($question['pic'], $this->config['sy_friend_icon']);
            $question['is_recom_n'] = $question['is_recom'] == 1? true:false;
        }
        return $question;
	}

    /**
     * @desc    修改问答详情;修改question表数据
     * @param $whereData
     * @param array $data
     * @return bool
     */
    public function upAskInfo($whereData, $data = array())
    {

        if (!empty($whereData)) {

            $PostData   =   array(

                'title'     =>  $data['title'],
                'cid'       =>  $data['cid'],
                'visit'     =>  $data['visit'],
                'is_recom'  =>  $data['is_recom'],
                'content'   =>  $data['content'],
                'ip'        =>  fun_ip_get()
            );
            $result     =   $this->update_once('question', $PostData, array('id' => $whereData['id']));
            return $result;
        }else{

            return false;
        }
    }

    /**
     * @desc 修改问答审核状态
     * @param $id
     * @param array $data
     * @return bool
     */
    public function upStatusInfo($id,$where=array(), $data = array())
    {

        if (!empty($id)) {
            if (is_array($id)) {

                $where['id'] = array('in', pylode(',', $id));
            } else {

                $where['id'] = intval($id);
            }

            $result =   $this->update_once('question', $data, $where);
            return $result;
        }else{
            return false;
        }
    }

    public function upRecommend($whereData, $data = array())
    {

        if (!empty($whereData)) {

            $result =   $this->update_once('question', $data, $whereData);
            return $result;
        }else{

            return false;
        }
    }

    /**
     * @desc    发布问答
     * @param array $data
     * @return mixed
     */
    public function addAskInfo($data = array())
    {

        $return =   array();

        if (!empty($data)) {

            if ($data['title'] == "") {

                $return['msg']      =   "标题不能为空！";
                $return['errcode']  =   8;
                return $return;
            }
            include_once('notice.model.php');
            $noticeM    =   new notice_model($this->db, $this->def);
            $result     =   $noticeM->jycheck($_POST['authcode'], '职场提问');
            if (!empty($result)) {

                $return['errcode']  =   8;
                $return['msg']      =   $result['msg'] ? $result['msg'] : '';
                if ($result['error'] == 107) {

                    $return['waperrcode'] = 4;
                } else {

                    $return['waperrcode'] = 0;
                }
                return $return;
            }

            //判断每日最多发布问答数配置项，判断当日已发布问答数
            if (isset($this->config['sy_day_ask_num']) && (int)$this->config['sy_day_ask_num'] > 0) {

                $dayWhere        =   array(
                    'uid'               =>  $data['uid'],
                    'PHPYUNBTWSTART_A'  =>  '',
                    'add_time'          =>  array(
                        0   =>  array('>=', strtotime(date('Y-m-d')), ''),
                        1   =>  array('<', time(), '')
                    ),
                    'PHPYUNBTWEND_A'    =>  '',
                );
                $dayAskNum      =   $this->getQuestionNum($dayWhere);

                if ($dayAskNum >= $this->config['sy_day_ask_num']) {

                    $return['msg']      =   "您今天已发布" . $dayAskNum . "个问答了，请明日再发！";
                    $return['errcode']  =   8;
                    return $return;
                }
            }
            if (isset($this->config['sy_ip_ask_num']) && (int)$this->config['sy_ip_ask_num'] > 0) {

                $ip             =   fun_ip_get();
                $ipWhere        =   array(
                    'PHPYUNBTWSTART_A'  =>  '',
                    'ip'                =>  array(
                        0   =>  array('<>', '', ''),
                        1   =>  array('=', $ip, '')
                    ),
                    'PHPYUNBTWEND_A'    =>  '',
                    'PHPYUNBTWSTART_B'  =>  '',
                    'add_time'          =>  array(
                        0   =>  array('>=', strtotime(date('Y-m-d')), ''),
                        1   =>  array('<', time(), '')
                    ),
                    'PHPYUNBTWEND_B'    =>  '',
                );
                $dayIpAskNum    =   $this->getQuestionNum($ipWhere);

                if ($dayIpAskNum >= $this->config['sy_ip_ask_num']) {

                    $return['msg']      =   "今日已发布".$dayIpAskNum."个问答了，请明日再发！";
                    $return['errcode']  =   8;
                    return $return;
                }
            }

            $auto   =   true;

            if ($data['usertype'] && $data['uid']) {

                include_once('userinfo.model.php');
                $userInfoM  =   new userinfo_model($this->db, $this->def);
                $mInfo      =   $userInfoM->getUserInfo(array("uid" => $data['uid']), array("usertype" => $data['usertype']));

                $addData['nickname']        =   trim($data['username']);

                if ($data['usertype'] == 2 || $data['usertype'] == 4) {

                    $addData['nickname']    =   $mInfo['name'];
                    $addData['pic']         =   $mInfo['logo'];
                } elseif ($data['usertype'] == 1 || $data['usertype'] == 5) {

                    $addData['nickname']    =   $mInfo['name'];
                    $addData['pic']         =   $mInfo['photo'];
                } elseif ($data['usertype'] == 3) {

                    $addData['nickname']    =   $mInfo['realname'];
                    $addData['pic']         =   $mInfo['photo'];
                }
            }

            $addData['title']   =   $data['title'];
            $addData['cid']     =   $data['cid'];
            $addData['content'] =   str_replace(array("&amp;", "background-color:#ffffff", "background-color:#fff", "white-space:nowrap;"), array("&", '', '', ''), $data["content"]);
            $addData['content'] =   strip_tags(trim($addData['content']), "<p> <br> <b>");
            $addData['uid']     =   $data['uid'];
            $addData['add_time']=   time();
            $addData['ip']      =   fun_ip_get();

            //后台开启问答审核，则默认为未审核；关闭审核，则默认为审核通过
            $addData['state']   =   $this->config['ask_check'] == 1 ? 0 : 1;

            //审核未通过的企业用户，发布问答为“未审核”状态
            if ($addData['state'] != 0 && $data['usertype'] == 2) {
                $member         =   $this->select_once('member', array('uid' => $data['uid']), '`status`');
                if ($member['status'] != 1) {
                    $addData['state'] = 0;
                }
            }

            $return['id']       =   $this->insert_into('question', $addData);

            if ($return['id']) {

                include_once('integral.model.php');
                $integralM  =   new integral_model($this->db, $this->def);
                $result     =   $integralM->max_time('发布问题', $data['uid'], $data['usertype']);
                if ($result == true || $auto == false) {

                    $integralM->company_invtal($data['uid'], $data['usertype'], $this->config['integral_question'], $auto, "发布问题", true, 2, 'integral');
                }

                $this->addMemberLog($data['uid'], $data['usertype'], "发布了问答《" . $data['title'] . "》", 19, 1);

                unset($_SESSION['authcode']);

                if ($addData['state'] == 0) {
                    $return['msg']      =   "已发布，正在审核中！";
                    $return['errcode']  =   9;
                } else {

                    $return['msg']      =   "发布成功！";
                    $return['errcode']  =   9;
                }
            } else {

                $return['msg']          =   "提问失败！";
                $return['errcode']      =   8;
            }
        }
        return $return;
    }


    /**
     * @desc   查询问答类别表
     * @param array $where
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getQclassList($where = array(), $data = array())
    {

        $select     =   $data['field'] ? $data['field'] : '*';
        $list    =   $this->select_all('q_class', $where, $select);

        if (!empty($list)) {
            foreach ($list as $key => $val) {
                if (!empty($data['utype']) && $data['utype'] == 'admin') {
                    isset($val['add_time']) && $list[$key]['add_time_n'] = date('Y-m-d', $val['add_time']);
                }
            }
        }

        return $list;
    }
	
	/**
	* @desc   获取问答类别详情
	*/
    public function getQclassInfo($id, $data = array())
    {

        if (!empty($id)) {

            $select     =   $data['field'] ? $data['field'] : '*';
            $QclassInfo =   $this->select_once('q_class', array('id' => intval($id)), $select);
            if ($QclassInfo['pic']) {

                $QclassInfo['pic']  =   checkpic($QclassInfo['pic']);
            }
        }
        return $QclassInfo;
    }

    /**
     * @desc   修改问答类别
     * @param $whereData
     * @param array $data
     * @return bool
     */
	public function upQclassInfo($whereData,$data = array())
    {
		
		if(!empty($whereData)) {
			
			$PostData	=	array(
				'name'		=>	$data['name'],
				'pid'		=>	$data['pid'],
				'sort'		=>	$data['sort'],
				'intro'		=>	$data['intro'],
				'content'	=>	$data['content'],
			);

			if($data['pic']){
				$PostData['pic']	=	$data['pic'];
			}
			
			if ($data && is_array($data)){
				
				 $nid  =  $this -> update_once('q_class', $PostData, array('id'=>$whereData['id']));
			}
		}
		return $nid;
	}

    /**
     * @desc   添加问答类别
     * @param array $data
     * @return bool
     */
    public function addQclassInfo($data = array())
    {

        $PostData   =   array(

            'name'      =>  $data['name'],
            'pid'       =>  $data['pid'],
            'sort'      =>  $data['sort'],
            'intro'     =>  $data['intro'],
            'content'   =>  $data['content'],
            'add_time'  =>  time()
        );

        if ($data['pic']) {
            $PostData['pic']    =   $data['pic'];
        }

        if ($data && is_array($data)) {

            $nid    =   $this->insert_into('q_class', $PostData);
        }
        return $nid;
    }
	
	/**
	* @desc   查询问答话题
	*/
	public function getAnswersList($where = array(), $data=array()){
		
	    $field  =	empty($data['field']) ? '*' : $data['field'];
	    
	    $answers  =   $this -> select_all('answer', $where, $field);
		if(is_array($answers) && $answers ){
			
			$aids = array();
			foreach($answers  as $v){
				
				$aids[]		=	$v['id'];

				$a_uid[]	=	$v['uid'];
				
				$qid[]		=	$v['qid'];
			
			}

			$arwhere = array('aid'=>array('in', pylode(',', $aids)),'groupby'=>'aid');

			if($data['utype']!='admin'){
				$arwhere['status'] = 1;
			}

			$arlist	=	$this->select_all('answer_review',$arwhere,'aid,count(*) as num');
			

			$awhere['uid']	=	array('in',pylode(',',$a_uid));
			
			$member		=	$this -> select_all('member',$awhere);
			$resume		=	$this -> select_all('resume',$awhere,'`uid`,`photo`');
			
			$company	=	$this -> select_all('company',$awhere,'`uid`,`logo`');
			
			$question	=	$this -> getList(array('id'=>array('in',pylode(',',$qid))),array('field'=>'`id` as `qid`, `title`'));
			
			foreach($answers as $key => $val){

				$answers[$key]['comment'] = 0;

				if(!empty($arlist)){
					foreach($arlist as $arv){
						if($arv['aid']==$val['id']){
							$answers[$key]['comment'] = $arv['num'];
						}
					}
				}

		        if(strlen($val['nickname'])>10){
		          $answers[$key]['nickname']   =  mb_substr($val['nickname'],0,10,'utf-8').'...';
		        }
				foreach($member as $v){
					
					if($val['uid'] == $v['uid']){
						
						$answers[$key]['username']	=	$v['username'];
						
						if(!$val['nickname']){
							
							$answers[$key]['nickname']	=	$v['username'];
						}
					
					}
				
				}
				foreach($question as $v){
					
					if($val['qid']==$v['qid']){
						
						$answers[$key]['title']	=	$v['title'];
						
						$answers[$key]['qid']	=	$v['qid'];
					}
				}
				$answers[$key]['content'] = str_replace("\r\n","<br />",$val["content"]);
				
				if($val['pic']){
					
					$pic=checkpic($val['pic']);
				}else{
					foreach($resume as $va){
						
						if($va['uid']==$val['uid']){
							
							$pic=checkpic($va['photo']);
								
						}
					}
					foreach($company as $va){
						
						if($va['uid']==$val['uid']){
							
							$pic=checkpic($va['logo']);
							
						}
					}
					
				}
				
				
				if($pic){
					
					$answers[$key]['pic']=$pic;
				}else{
					
					$answers[$key]['pic']=checkpic('',$this->config['sy_friend_icon']);
				}

                if ($data['utype'] == 'admin') {
                    isset($val['content']) && $answers[$key]['content_n'] = mb_substr(strip_tags($val['content']), 0, 40, 'utf-8');
                    isset($val['add_time']) && $answers[$key]['add_time_n'] = date('Y-m-d H:i:s', $val['add_time']);
                }
			}
		
		}

	    return $answers;
	
	}	
	
	/**
	* @desc   修改回答
	*/
	public function upAnswerInfo($whereData ,$data = array()){
		
		if(!empty($whereData)) {
			
			if ($data && is_array($data)){
				
				 $nid  =  $this -> update_once('answer', $data, $whereData);
			
			}
		
		}
		
		return $nid;
	}
	/**
	* @desc   发布回答
	*/
	public function addAnswerInfo($data = array()){
		
		if(!empty($data)) {
			if($data['content']&&$data['id']){
				if($data['utype']=='wap'){
					session_cache_limiter('private, must-revalidate');
					
					session_start();
					
					$authcode=md5(strtolower($data['authcode']));	
					
					if($authcode!=$_SESSION['authcode'] || empty($_SESSION['authcode'])){
						
						unset($_SESSION['authcode']);
						
						$arr['msg']		=	"验证码错误！";
						
						return $arr;	
					}
				}
				
				$auto=true;
				 
				$info	=	$this -> getInfo($data['id'],array('field'=>'`id`,`cid`,`uid`,`title`,`content`'));
				
				$content	=	$data["content"];
				
				$content	=	str_replace(array("&amp;","background-color:#ffffff","background-color:#fff","white-space:nowrap;"),array("&",'','',''),$content);
				
				$addData=array();
				
				if($data['usertype']&&$data['uid']){
					include_once ('userinfo.model.php');
					
					$userinfoM	=	new userinfo_model($this->db, $this->def);
					
					$minfo		=	$userinfoM -> getUserInfo(array("uid"=>$data['uid']),array("usertype"=>$data['usertype']));
					
					if($data['usertype']==2){
						
						$addData['nickname']	=	$minfo['name'];
						
						$addData['pic']			=	$minfo['logo'];
					}elseif($data['usertype']==1){
						
						$addData['nickname']	=	$minfo['name'];
						
						$addData['pic']			=	$minfo['photo'];
					}
				}
				
				$addData['usertype']			=	$data['usertype'];
				
				$addData['qid']		=	$data['id'];
				
				$addData['cid']		=	$info['cid'];
				
				$addData['content']	=	trim(strip_tags($content));
				
				$addData['uid']		=	$data['uid'];
				
				$addData['comment']	=	0;
				
				$addData['support']	=	0;
				
				$addData['oppose']	=	0;

				$addData['status']	=	isset($data['status']) ? $data['status'] : $this->config['answer_check'];
				
				$addData['add_time']=	time();
				
				$arr['id']=$this -> insert_into('answer', $addData);
				
				if($arr['id']){
                    include_once('integral.model.php');
                    $integralM = new integral_model($this->db, $this->def);
                    $result = $integralM->max_time('回答问题', $data['uid'], $data['usertype']);

                    if ($result == true || $auto == false) {
                        $integralM->company_invtal($data['uid'], $data['usertype'], $this->config['integral_answer'], $auto, "回答问题", true, 2, 'integral');
                    }
                    if ($addData['status'] == '1') {
                        $this->update_once('question', array('answer_num' => array('+', '1'), "lastupdate" => time()), array('id' => $info['id']));
                    }

                    $this->addMemberLog($data['uid'], $data['usertype'], "回答了问答《" . $info['title'] . "》", 19, 1);
					
					$arr['msg']		=	"回答成功！";
					
					$arr['errcode']	=	9;
				}else{
					$arr['msg']		=	"回答失败！";
					
					$arr['errcode']	=	8;
				}
			}else{
				$arr['msg']		=	"非法操作！";
				
				$arr['errcode']	=	8;
			}
			return $arr;
		}
	}
	
	/**
	* @desc   评论列表
	*/
	public function getCommentsList ($whereData,$data=array()) {       
		
		$select  =   $data['field'] ? $data['field'] : '*'; 
		
		$CommentList    =   $this -> select_all('answer_review',$whereData,$select);

		if(is_array($CommentList) && $CommentList ){
			
			foreach($CommentList  as $v){
				
				$uids[]		=	$v['uid'];
			
			}
			$where['uid']	=	array('in',pylode(',',$uids));
			
			$memberList		=	$this -> select_all('member',$where,"`username`,`uid`");
			
			$userList		=	$this -> select_all('resume',$where,"`photo`,`uid`,`name`");
			
			$comList		=	$this -> select_all('company',$where,"`logo`,`uid`,`name`");
			
			
			foreach($CommentList as $key => $val){
				
				if($val['usertype']==1){
					foreach($memberList as $v){
					
						if($v['uid']==$val['uid']){
							
							$CommentList[$key]['username']	=	$v['username'];
							
							// $CommentList[$key]['nickname']	=	$v['username'];
						}
					
					}
					foreach($userList as $v){
						
						if($v['uid']==$val['uid']){
							
							$CommentList[$key]['nickname']	=	$v['name'];
							
							$v['photo']?$CommentList[$key]['pic']=$v['photo']:$CommentList[$key]['pic']=$this->config['sy_friend_icon'];
						}
					}
				}
				
				if($val['usertype']==2){
					foreach($comList as $v){
						
						if($v['uid']==$val['uid']){
							$CommentList[$key]['nickname']	=	$v['name'];

							$v['logo']?$CommentList[$key]['pic']=$v['logo']:$CommentList[$key]['pic']=$this->config['sy_friend_icon'];
						}
					}
				}
				
                if ($data['utype'] == 'admin') {
                    isset($val['content']) && $CommentList[$key]['content_n'] = mb_substr(strip_tags($val['content']), 0, 40, 'utf-8');
                    isset($val['add_time']) && $CommentList[$key]['add_time_n'] = date('Y-m-d H:i:s', $val['add_time']);
                }
			}
		
		}
		
		return $CommentList;
	
	}
	
	/**
	* @desc   评论返回回答列表
	*/
	public function getCommentBackQuestion($id,$data=array()){
		
		if(!empty($id)){
			
			$select  		  =   $data['field'] ? $data['field'] : '*';	
			
			$QuestionInfo	  =	  $this -> select_once('answer', array('id'=>intval($id)), $select);
		
		}
		
		return $QuestionInfo;
	
	}
	public function statusAnswer($id, $upData = array())
    {
        if (!empty($id)) {
            $idstr      =   pylode(',', $id);

            $upData     =   array(
                'status'     =>  intval($upData['status']),
                'statusbody' =>  trim($upData['statusbody']),
            );

            $result     =   $this -> update_once('answer', $upData, array('id' => array('in', $idstr)));

            if ($result) {
                if($upData['status'] == 1 || $upData['status'] == 2){
                    $msg    =   array();
                    $uids   =   array();

                    $answers  =   $this -> select_all('answer',array('id' => array('in', $idstr)),'`id`,`uid`');

                    foreach ($answers as $v){

                        $uids[] =   $v['uid'];
                    }

                    foreach ($answers as $k => $v){

                        if ($upData['status'] == 2) {

                            $statusInfo         =   '您的问答回复审核未通过';

                            if ($upData['statusbody']) {
                                $statusInfo     .=  '，原因：'.$upData['statusbody'];
                            }

                            $msg[$v['uid']][]   =   $statusInfo;
                        }elseif ($upData['status'] == 1){

                            $msg[$v['uid']][]   =  '您的问答回复审核通过';
                        }
					}


                    //发送系统通知
                    require_once 'sysmsg.model.php';
                    $sysmsgM    =   new sysmsg_model($this->db, $this->def);
                    $sysmsgM -> addInfo(array('uid' => $uids,'usertype'=>2,'content'=>$msg));
                }
                
                $return['msg']      =   '问答回复(ID:'.$idstr.')审核成功';
				$return['errcode']  =  0;
			}else{
                $return['msg']      =  '审核回复(ID:'.$idstr.')审核失败';
                $return['errcode']  =  1;
            }
        }else {
            $return['msg']          =   '请选择需要审核的问答回复';
            $return['errcode']      =   1;
        }

        return $return;
    }
    public function statusAnswerReview($id, $upData = array())
    {
        if (!empty($id)) {
            $idstr      =   pylode(',', $id);

            $upData     =   array(
                'status'     =>  intval($upData['status']),
                'statusbody'=>  trim($upData['statusbody']),
            );

            $result     =   $this -> update_once('answer_review', $upData, array('id' => array('in', $idstr)));

            if ($result) {

                if($upData['status'] == 1 || $upData['status'] == 2){

                    $msg    =   array();
                    $uids   =   array();

                    $answers  =   $this -> select_all('answer_review',array('id' => array('in', $idstr)),'`id`,`uid`');

                    foreach ($answers as $v){

                        $uids[] =   $v['uid'];
                    }

                    foreach ($answers as $k => $v){

                        if ($upData['status'] == 2) {

                            $statusInfo         =   '您的回复评论审核未通过';

                            if ($upData['statusbody']) {
                                $statusInfo     .=  '，原因：'.$upData['statusbody'];
                            }

                            $msg[$v['uid']][]   =   $statusInfo;
                        }elseif ($upData['status'] == 1){

                            $msg[$v['uid']][]   =  '您的回复评论审核通过';
                        }
					}


                    //发送系统通知
                    require_once 'sysmsg.model.php';
                    $sysmsgM    =   new sysmsg_model($this->db, $this->def);
                    $sysmsgM -> addInfo(array('uid' => $uids,'usertype'=>2,'content'=>$msg));
                }
                
                $return['msg']      =  '审核回复评论(ID:'.$idstr.')设置成功';
				$return['errcode']  =  0;
			}else{
                $return['msg']      =  '审核回复评论(ID:'.$idstr.')设置失败';
                $return['errcode']  =  1;
            }
        }else {
            $return['msg']          =   '请选择需要审核的回复评论！';
            $return['errcode']      =   1;
        }

        return $return;
    }
	/**
	* @desc   评论数量
	*/
	public function getAnswersNum($where=array()){
		
		return $this -> select_num('answer', $where);
	
	}
	
	/**
	* @desc  获取评论详细信息
	*/
	public function getReviewInfo($id,$data=array()){
		
		if(!empty($id)){
			
			$select   	  =   $data['field'] ? $data['field'] : '*';	
			
			$ReviewInfo	  =	 $this -> select_once('answer_review', array('id'=>intval($id)), $select);
		
		}
		
		return $ReviewInfo;
	
	}
	
	/**
	* @desc   修改评论
	*/
	public function upReview($whereData ,$data = array()){
		
		if(!empty($whereData)) {
			
			$PostData	=	array(
				
				'content'	=>	$data['content'],
			
			);
			
			if ($data && is_array($data)){
				
				 $nid  =  $this -> update_once('answer_review', $PostData, array('id'=>$whereData['id']));
			
			}
		
		}
		
		return $nid;
	}
	/**
	* @desc   发布评论
	*/
	public function addReview($addData = array()){
		
		if(!empty($addData)) {
			
			$auto=true;
			 
			$data['aid']		=	(int)$addData['aid'];
			
			$data['qid']		=	(int)$addData['qid'];
			
			$data['content']	=	str_replace(array("&amp;","background-color:#ffffff","background-color:#fff","white-space:nowrap;"),array("&",'background-color:','background-color:','white-space:'),$addData['content']);
			 
			$data['uid']		=	$addData['uid'];
			
			$data['add_time']	=	time();

			$data['status']		=	isset($addData['status'])?$addData['status']:$this->config['answer_review_check'];
			
			$data['usertype']	=	$addData['usertype'];
			
			$new_id=$this -> insert_into('answer_review', $data);
			
			if($new_id){
				include_once ('integral.model.php');
				
				$integralM	=	new integral_model($this->db, $this->def);
				
				$result		=	$integralM -> max_time('评论问答',$addData['uid'],$data['usertype']);
				
				if($result==true||$auto==false){
					
					$integralM->company_invtal($data['uid'],$data['usertype'],$this->config['integral_answerpl'],$auto,"评论问答",true,2,'integral');
				}
				
				$this -> addMemberLog($addData['uid'],$addData['usertype'],"评论问答",19,1);
				if($data['status']=='1'){
					$this -> upAnswerInfo(array('id'=>$addData['aid']),array('comment'=>array('+','1')));
				}
				
				return '1';
			}else{
				return '0';
			}
		
		}
	}
	
	/**
	* @desc   删除评论
	*/
	public function delReview($delId){
		
		if(empty($delId)){
			
			return array(
				
				'errcode'	=>	8,
				
				'msg'		=>	'请选择要删除的数据！',
			
			);
		
		}else{
			
			if(is_array($delId)){
				
				$delId	=	pylode(',',$delId);
				
				$return['layertype']	=	1;
			
			}else{
				
				$return['layertype']	=	0;
			}
			
			$nid	=	$this -> delete_all('answer_review',array('id' => array('in',$delId)),'');
			
			if($nid){
			    $delNum =   count(explode(',', $delId));
				$this -> update_once("answer",array('comment'=>array('-', $delNum)),array('id'=>$_GET['aid']));
				
				$return['errcode']	=	$nid?'9':'8';
				
				$return['msg']		=	$nid ?'问答评论删除成功！' : '问答评论删除失败！';
			
			}
		
		}
		
		return	$return;
	
	}
	
	/**
	* @desc   删除试卷
	*/
	public function delquestion($delId,$data = array()){
		
		if($delId){
			if($data['utype'] = 'admin'){
				$delQuesWhere	=	array('id' => array('in',$delId));
				$delAnWhere		=	array('qid' => array('in',$delId));
			}else{
				$delQuesWhere	=	array('uid'=>$data['uid'],'id' => array('in',$delId));
				$delAnWhere		=	array('uid'=>$data['uid'],'qid' => array('in',$delId));
			}
			$result=$this -> delete_all('question' , $delQuesWhere,'');
			
			$this -> delete_all('answer_review' , $delAnWhere,'');
			
			$this -> delete_all('answer' , $delAnWhere,'');
			
		}
		return $result;
	}

    /**
     * @desc   删除问答类别
     * @param $delId
     * @return array
     */
    public function delQclass($delId)
    {
        if (empty($delId)) {
            return array(
                'errcode' => 1,
                'msg' => '请选择要删除的数据！',
            );
        }
        if (is_array($delId)) {
            $delId = pylode(',', $delId);
        }

        $qClass =   $this->select_all('q_class', array('id' => array('in', $delId), 'pid' => 0), 'id');
        $result =   $this->delete_all('q_class', array('id' => array('in', $delId)), '');
        if ($result) {
            if ($qClass){
                foreach ($qClass as $k => $v) {
                    $pidArr[]   =   $v['id'];
                }
                $this->delete_all('q_class', array('pid' => array('in', pylode(',', $pidArr))), '');
            }
        }

        $msg = '问答分类(ID:' . $delId . ')';
        $return['errcode'] = $result ? 0 : 1;
        $return['msg'] = $result ? $msg . '删除成功' : $msg . '删除失败';
        return $return;
    }
	
	public function delquestiongroups($delId){
		if($delId){
			
			$this -> delete_all('question' , array('id' => array('in',$delId)),'');
			
			$this -> delete_all('answer' , array('qid' => array('in',$delId)),'');
			
			$this -> delete_all('answer_review' , array('qid' => array('in',$delId)),'');
			
		}
	}
	
	/**
	* @desc   删除回答
	*/
	public function delAnswer($where=array(),$delId=''){
	 
		if($where){
			
			$result	=	$this -> delete_all('answer' , $where,'');
			
		}else{
			$result['layertype']		=	0;
			if(is_array($delId)){
				
				$delId	=	pylode(',',$delId);
				
				$result['layertype']	=	1;
			}
			$result['id']	=	$this -> delete_all('answer' , array('id'=>array('in',$delId)),'');
			
			$this -> delete_all("answer_review",array('aid'=>array('in',$delId)),'');
			
			$result['msg']		=	'用户回答(ID:'.$delId.')';
			
			$result['errcode']	=	$result['id']?9:8;
			
			$result['msg']		=	$result['id']?'删除成功！':'删除失败！';
		}
		return $result;
	}
	
	/**
	* @desc    获取问答数量
	* @desc    查询 question 数量
	*/
	public function getQuestionNum($wheredata = array()){
		
		return $this -> select_num('question',$wheredata);
	
	}
	
	/**
	 * @desc 查询关注信息表单条记录
	 * 
	 */
	public function getAtnInfo($whereData = array(), $data = array()){
	    
	    $select    =   $data['field'] ? $data['field'] : '*';
	    
	    $atnInfo   =   $this -> select_once('attention', $whereData, $select);
	    
	    return $atnInfo; 
	    
	}
	
	/**
	 * @desc   attention 表数据删除
	 * @param  array $whereData
	 * @return boolean
	 */
	function delAttention($whereData = array() ,$data = array()){
	    
	    if($data['type']=='one'){//单个删除
	        
	        $limit =	'limit 1';
	        
	    }
	    
	    if($data['type']=='all'){//多个删除
	        
	        $limit =	'';
	        
	    }
 
	    $result    =	$this	->	delete_all('attention', $whereData, $limit);
	    
	    return	$result;
	}
	
	/**
	 * @desc   修改attention表数据
	 * @param  array $whereData
	 * @param  array $data
	 * @return boolean
	 */
	function upAttention( $data = array(), $whereData = array()){
	    
	    if(!empty($whereData) && !empty($data)){
	        
	        $nid   =   $this -> update_once('attention', $data, $whereData);
	        
	    }
	    
	    return $nid;
	    
	}
	/**
	 * @desc   添加attention表数据
	 * @param  array $whereData
	 * @param  array $data
	 * @return boolean
	 */
	function addAttention($data = array()){
	    
	    if(!empty($data)){
	        
	        $nid   =   $this -> insert_into('attention', $data);
	        
	    }
	    
	    return $nid;
	    
	}
	
	function setAttention($data = array()){
		
		$isset  =   $this->getAtnInfo(array('uid'=>$data['uid'],'type'=>$data['type']));
		
		$ids    =   @explode(',',$isset['ids']);

        if ($data['type'] == '1') {
            $info       =   $this->getInfo($data['id'], array('field' => "`id`,`title`,`uid`,`atnnum`"));
            $gourl      =   Url('ask', array("c" => "content", "id" => $info['id']));
            $content    =   "关注了<a href=\"".$gourl."\" target=\"_blank\">《".$info['title']."》</a>。";
            $n_contemt  =   "取消了对<a href=\"".$gourl."\" target=\"_blank\">《".$info['title']."》</a>的关注。";
            $log        =   "关注了《".$info['title']."》";
            $n_log      =   "取消了对《".$info['title']."》的关注";
        } else {
            $info       =   $this->getQclassInfo($data['id'], array('field' => "`id`,`name`,`atnnum`"));
            $content    =   "关注了《".$info['name']."》。";
            $n_contemt  =   "取消了《".$info['name']."》。";
            $log        =   "关注了".$info['name'];
            $n_log      =   "取消了对".$info['name']."的关注。";
        }

		if($info['uid']==$data['uid']){

			$return['msg']		=	'不能关注自己发布的问题！';
			$return['errcode']	=	8;
			return $return;
		}else{

			$adata['uid']	=	$data['uid'];
			$adata['type']	=	$data['type'];

			$arr['isadd']	=	1;

            if ($ids[0] == '' || $ids[0] == '0') {

                $adata['ids'] = $data['id'];

                if (is_array($isset) && $isset) {

                    $nid = $this->upAttention($adata, array("id" => $isset['id']));
                } else {

                    $nid = $this->addAttention($adata);
                }

                $this->addMemberLog($data['uid'], $data['usertype'], $log, 5, 1);

            } else if (in_array($data['id'], $ids) && $ids[0]) {

                $nids   =   array();

                foreach ($ids as $val) {
                    if ($val != $data['id'] && $val && in_array($val, $nids) == false) {

                        $nids[] = $val;
                    }
                }
                $arr['isadd'] = 0;
                $adata['ids'] = pylode(',', $nids);
                $nid = $this->upAttention($adata, array("id" => $isset['id']));
                $this->addMemberLog($data['uid'], $data['usertype'], $n_log, 5, 1);
            } else if (in_array($data['id'], $ids) == false && $ids[0] > 0) {

                $ids[] = $data['id'];
                $adata['ids'] = pylode(',', $ids);
                $nid = $this->upAttention($adata, array("id" => $isset['id']));
                $this->addMemberLog($data['uid'], $data['usertype'], $log, 5, 1);
            } else if (in_array($data['id'], $ids) == false && $ids[0] < 1) {

                $nid = $this->upAttention(array("ids" => $ids), array("id" => $isset['id']));
                $this->addMemberLog($data['uid'], $data['usertype'], $log, 5, 1);
            }
			if($nid){
                if ($data['type'] == '1') {
                    if ($arr['isadd']) {
                        $atnnum = $info['atnnum'] + 1;
                        $this->upStatusInfo($info['id'], array("atnnum" => $atnnum));
                        $sql['content'] = $content;
                    } else {
                        $atnnum = $info['atnnum'] - 1;
                        $this->upStatusInfo($info['id'], array("atnnum" => $atnnum));
                        $sql['content'] = $n_contemt;
                    }
                }
                if ($data['type'] == '2') {
                    include(LIB_PATH . "cache.class.php");
                    $cacheclass =   new cache(PLUS_PATH, $this->obj);
                    $makecache  =   $cacheclass->ask_cache("ask.cache.php");
                }

				$return['id']		=	$nid;
				$return['msg']		=	'操作成功！';
				$return['errcode']	=	9;
				$return['atnnum']	=	$atnnum > 0 ? $atnnum : 0;
				$return['isadd']	=	$arr['isadd'];
				return $return;
			}else{

				$return['msg']		=	'操作失败！';
				$return['errcode']	=	8;
				return $return;
			}
		}
	}
	
	function upSupportInfo($data=array()){
		if(!empty($data)){
			
			$cookid	=	explode(',', $_COOKIE['support']);
			
			if(in_array($data['aid'],$cookid)){
				
				echo '2';die;
			}else{
				$id	=	$this -> upAnswerInfo(array('id'=>$data['aid']),array('support'=>array('+','1')));
				
				if($id){
					$this -> addMemberLog($data['uid'],$data['usertype'],"给问题回答点赞",19,1);
					
					$sendid		=	array();
					
					$sendid[]	=	$data['aid'];
					
					if($_COOKIE['support']){
						$support=	$_COOKIE['support'].','.pylode(',',$sendid);
					}else{
						$support=	pylode(',',$sendid);
					}
					require_once ('cookie.model.php');
					
					$cookieM	=	new cookie_model($this->db, $this->def);
					
					$cookieM -> SetCookie("support",$support,time() + 86400);
					
					echo '1';
				}else{
					echo '0';die;
				}
			}
		}
	}
	
    /**
     * @desc   引用log类，添加用户日志   
     */
    private function addMemberLog($uid,$usertype,$content,$opera='',$type='') {
        
        require_once ('log.model.php');
        
        $LogM = new log_model($this->db, $this->def);
        
        return  $LogM -> addMemberLog($uid,$usertype,$content,$opera,$type); 
        
    }
	
}
?>