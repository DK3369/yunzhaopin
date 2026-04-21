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
class logout_model extends model{

    /**
     * @desc   注销账号申请列表
     * @param  $whereData :查询条件
     * @param string[] $data
     * @return array|bool|false|string|void
     */
	public function getList($whereData , $data = array())
	{
        $data['field']  =   empty($data['field']) ? '*' : $data['field'];

		$List  =  $this->select_all('member_logout',$whereData, $data['field']);

		$List  =  $this->getDataList($List);


		return $List;
	}
	
	public function getInfo($where = array())
	{
	    $row  =  $this->select_once('member_logout',$where);
	    return $row;
	}

    /**
     * 注销账号申请
     * @param array $data
     * @param array $p
     * @return array
     */
	public function apply($data = array(),$p = array())
	{

        $return =   array();
        // 验证密码
        $user   =   $this->select_once('member', array('uid' => $data['uid']), '`uid`,`usertype`,`password`,`salt`,`username`,`moblie`');

        $res    =   passCheck($p['password'], $user['salt'], $user['password']);

        if (!$res) {

            $return =   array(
                'msg'       =>  '密码错误',
                'errcode'   =>  8
            );
        } else {

            $data['ctime']      =   time();
            $data['status']     =   1;
            $data['username']   =   $user['username'];
            $data['tel']        =   $user['moblie'];

            $nid = $this->insert_into('member_logout', $data);

            require_once('log.model.php');
            $logM = new log_model($this->db, $this->def);
            $logM->addMemberLog($user['uid'], $user['usertype'], '申请注销账号', 31, 1);

            $msg = '注销账号申请';

            $return['msg'] = $nid ? $msg . '成功' : $msg . '失败 ';
            $return['errcode'] = $nid ? 9 : 8;
        }
        return $return;
	}
    public function applymsg($data = array(),$p = array())
    {
        $mobile		=	$p['mobile'];
        $code		=	$p['code'];

        require_once('userinfo.model.php');
        require_once('company.model.php');
        $companyM = new company_model($this->db, $this->def);
        $userinfoM = new userinfo_model($this->db, $this->def);

        $info 		= 	$userinfoM->getInfo(array('moblie'=>$mobile),array("field"=>"`uid`,`username`,`moblie`"));
        $check		=	$info['moblie'];

        $cert 		= 	$companyM->getCertInfo(array("uid"=>$info['uid'],"type"=>"7","check"=>$check,'orderby'=>'ctime,desc'),array("field"=>"`uid`,`check2`,`ctime`,`id`"));

        if($code!=$cert['check2']){

            $return['msg']		=	"验证码错误";
            $return['error']	=	'8';

        }else{
            $user   =   $this->select_once('member', array('uid' => $data['uid']), '`uid`,`usertype`,`password`,`salt`,`username`,`moblie`');

            $data['ctime']      =   time();
            $data['status']     =   1;
            $data['username']   =   $user['username'];
            $data['tel']        =   $user['moblie'];

            $nid = $this->insert_into('member_logout', $data);

            require_once('log.model.php');
            $logM = new log_model($this->db, $this->def);
            $logM->addMemberLog($user['uid'], $user['usertype'], '申请注销账号', 31, 1);

            $msg = '注销账号申请';

            $return['msg'] = $nid ? $msg . '成功' : $msg . '失败 ';
            $return['errcode'] = $nid ? 9 : 8;
        }
        return $return;
    }
    /**
     * 注销账号申请处理
     * @param $where
     * @return array
     */
	public function status($where)
	{
	    $return  =  array();
	    if (!empty($where)){
	        
	        $row  =  $this->select_once('member_logout',$where);
	        
	        if (!empty($row)){
	            
	            $nid  =  $this->update_once('member_logout',array('status'=>2), $where);
	            
	            if ($nid){
	                
	                $member  =  $this->select_once('member',array('uid'=>$row['uid']),'`uid`,`username`,`moblie`,`email`');
	                
	                if (!empty($member)){
	                    
	                    // 用户名/手机号/邮箱，前后加字符串，地方绑定信息清空
	                    $str  =  'out';
	                    $arr  =  array(
	                        'username'     =>  createstr(16),
	                        'moblie'       =>  $str.'_'.$member['moblie'],
	                        'email'        =>  $str.'_'.$member['email'].'_'.$str,
	                        'status'       =>  2,
	                        'lock_info'    =>  '会员注销账号',
	                        'pwuid'        =>  0,
	                        'pw_repeat'    =>  0,
	                        'qqid'         =>  '',
	                        'qqunionid'    =>  '',
	                        'sinaid'       =>  '',
	                        'wxid'         =>  '',
	                        'wxopenid'     =>  '',
	                        'unionid'      =>  '',
	                        'wxname'       =>  '',
	                        'wxbindtime'   =>  '',
	                        'clientid'     =>  '',
	                        'deviceToken'  =>  '',
	                        'maguid'       =>  '',
	                        'qfyuid'       =>  ''
	                    );

                        $this->update_once('member',$arr, array('uid'=>$row['uid']));

                        $this->update_once('resume',array('telphone' => $str.'_'.$member['moblie'], 'email' => $str.'_'.$member['email'].'_'.$str), array('uid'=>$row['uid']));
                        $this->update_once('company',array('linktel' => $str.'_'.$member['moblie'], 'linkmail' => $str.'_'.$member['email'].'_'.$str), array('uid'=>$row['uid']));

	                    $this->commonLock($row['uid'], array('r_status' => 2));

	                    // 发送短信、邮件通知
	                    $sendData  =  array(
	                        'email'      =>  $member['email'],
	                        'mobile'     =>  $member['moblie'],
	                        'uid'        =>  $member['uid'],
	                        'username'   =>  $member['username'],
	                        'type'       =>  'logouted'
	                    );
	                    
	                    require_once ('notice.model.php');
	                    $noticeM   =  new notice_model($this->db, $this->def);
	                    $noticeM -> sendEmailType($sendData);
						$sendData['port']	=	'5';
	                    $noticeM -> sendSMSType($sendData);
	                }
	                
	                $return['msg']      =  '注销账号申请(ID：'.$row['id'].')处理成功';
	                $return['errcode']  =  9;
	                
	            }else{
	                
	                $return['msg']      =  '注销账号申请处理失败 ';
	                $return['errcode']  =  8;
	            }
	        }else{
	            
	            $return['msg']      =  '没有该注销申请';
	            $return['errcode']  =  8;
	        }
	    }else{
	        
	        $return['msg']      =  '请选择需要处理的申请';
	        $return['errcode']  =  8;
	    }
	    return $return;
	}

    private function commonLock($uid, $up)
    {
        $where = array('uid' => $uid);

        include_once('resume.model.php');
        $resumeM   =   new resume_model($this->db, $this->def);

        $this->update_once('resume', $up, $where);
        $this->update_once('company', $up, $where);


        $expectdata = $up;
        $resumeM->setExpectState($expectdata,$where);

        $this->update_once('company_job', $up, $where);
        $this->update_once('partjob', $up, $where);
    }

	/**
	 * 注销账号申请删除
	 */
	public function del($delId = null)
	{
	    if (empty($delId)) {
	        
	        $return  =  array('errcode' => 8, 'msg' => '请选择要删除的数据！');
	    } else {
	        
	        if (is_array($delId)) {
	            
	            $delId                =   pylode(',', $delId);
	            
	            $return['layertype']  =   1;
	        } else {
	            
	            $return['layertype']  =   0;
	        }
			 
	        $return['id']       =   $this -> delete_all('member_logout', array('id' => array('in', $delId)), '');
	        
	        $return['errcode']  =   $return['id'] ? 9 : 8;
	        
	        $msg                =   '注销账号申请（ID：'.pylode(',', $delId).'）';
	        
	        $return['msg']      =   $return['id'] ? $msg.'删除成功！' : '删除失败！';
	    }
	    return $return;
	}


    /**
     * 会员注销后台列表数据处理
     * @param $List
     * @return mixed
     */
    private function getDataList($List)
    {

        foreach ($List as $v) {

            $uids[] = $v['uid'];
        }

        $resume     =   $this->select_all('resume', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');
        $company    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`');

        foreach($List as $k => $v){

            foreach($resume as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['name']  =  $val['name'];
                }
            }
            foreach($company as $val){
                if($v['uid']==$val['uid']){

                    $List[$k]['comname']  =  $val['name'];
                }
            }
        }
        return $List;
    }

    /**
     *  查看
     */
    public function getListNumV1(){
        $arr    =   array();
        //会员总数
        $arr['count']      =   $this->select_num('member_logout');

        //锁定会员
        $arr['weishenhe']  =   $this->select_num('member_logout', array('status' => '1'));

        return $arr;
    }
}
?>