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
class atn_model extends model{
	//获取简历信息列表resume_expect
    private function getResumeExpectList($whereData, $data = array()){
        
        require_once ('resume.model.php');
        
        $resumeM    =   new resume_model($this->db, $this->def);
        
        return  $resumeM   ->  getList($whereData , $data);
    }
	//获取简历信息列表resume
    private function getResumeList($whereData, $data = array()){
        
        require_once ('resume.model.php');
        
        $resumeM    =   new resume_model($this->db, $this->def);
        
        return  $resumeM   ->  getResumeList($whereData , $data);
    }
    /**
     * 设置简历头像展示
     */
    private function setResumePhotoShow($data = array()){
        require_once ('resume.model.php');

        $resumeM    =   new resume_model($this->db, $this->def);

        return  $resumeM   ->  setResumePhotoShow($data);
    }
	//获取职位信息列表company_job
    private function getJobList($whereData, $data = array()){
        
        require_once ('job.model.php');
        
        $jobM    =   new job_model($this->db, $this->def);
        
        return  $jobM   ->  getList($whereData , $data);
    }
	//获取职位信息列表company
    private function getComList($whereData, $data = array()){
        
        require_once ('company.model.php');
        
        $companyM    =   new company_model($this->db, $this->def);
        
        return  $companyM   ->  getList($whereData , $data);
    }
    private function addMemberLog($uid, $usertype, $content, $opera = '', $type = '', $detail = '')
    {
        require_once('log.model.php');
        $LogM = new log_model($this->db, $this->def);
        return $LogM->addMemberLog($uid, $usertype, $content, $opera, $type, $detail);
    }
    /**
     * 关注总数
    */
    public function getantnNum($whereData){
        return $this->select_num('atn',$whereData);
    }
     /**
      * 查询全部信息
      * @param 表：atn
      * @param 功能说明：获取atn表里面信息
      * @param 引用字段：$whereData：条件  2:$data:查询字段
      *
     */
    public function getatnList($whereData,$data=array()){
        $field              =   $data['field'] ? $data['field'] : '*';
        
        $atnlist            =   $this   ->  select_all('atn', $whereData, $field);
        
        foreach($atnlist as $k=>$v){
        	if($v['time']){
        		$atnlist[$k]['time_n']	=	date('Y-m-d',$v['time']);
        	}
        }
        if(is_array($atnlist) && $atnlist){
          if($data['utype']=='user'){
            //关注企业
            $atnlist	=	$this -> getAtnUserDataList($atnlist,$data);
            
          }
          if($data['utype']=='company'){
            //关注企业
            $atnlist	=	$this -> getAtnComDataList($atnlist,$data);
          }
        }
		
        return $atnlist; 
    }
	/**
     * @desc    关注我的人才
     *
     * @param   array $List
     */
    private function getAtnUserDataList($List,$data=array()) {
        $uids   =   array();
        foreach($List as $v){
			if($v['uid'] && !in_array($v['uid'],$uids)){
				$uids[]	=	$v['uid'];
			}
		}
		//  查询个人姓名
		$rWhere['uid']            	=   array('in', pylode(',', $uids));
		$rData['field']             =   '`uid`,`name`,`nametype`,`sex`,`telphone`,`def_job`, `photo_status`,`defphoto`, `phototype`';

		$resumeList                	=   $this -> getResumeList($rWhere, $rData);

		//  查询个人简历
		$reWhere['uid']             =   array('in', pylode(',', $uids));
		$reWhere['defaults']        =   '1';
		$reData['field']            =   '`id`,`uid`,`name`,`job_classid`,`minsalary`,`maxsalary`,`height_status`,`exp`,`edu`,`birthday`,`status`';

		$expectList                 =   $this -> getResumeExpectList($reWhere, $reData);
		
		$userid_msg		=	$this -> select_all("userid_msg",array('fid'=>$data['uid'],'uid'=>array('in', pylode(',', $uids))),"uid");
		
		$userid_job		=   $this -> select_all('userid_job',array('com_id'=>$data['uid'],'uid'=>array('in',pylode(',',$uids))),'`uid`,`is_browse`');
				
		foreach($List  as $k=>$v){
			foreach($resumeList as $val){
					
				if($v['uid'] == $val['uid']){
					$List[$k]['name_n']		=   $val['name'];
					$List[$k]['telphone_n']	=   $val['telphone'];
					$List[$k]['username']    =	$val['name_n'];
					$List[$k]['telphone']    =	$val['telphone'];
                    $List[$k]['photo']		=   $this -> setResumePhotoShow(array(
                        'photo' => $val['photo'],
                        'photo_status'=>$val['photo_status'],
                        'phototype' => $val['phototype'],
                        'sex' => $val['sex']
                    ));
				}
			}
			foreach ($expectList['list'] as $val){
				
				if ($v['uid']   ==  $val['uid']) {
                    $List[$k]['waprurl']	=   Url('wap',array('c'=>'resume','a'=>'show','id'=>$val['id']));
                    $List[$k]['eid']		=   $val['id'];
					$List[$k]['exp']		=   $val['exp_n'];
					$List[$k]['edu']		=   $val['edu_n'];
					$List[$k]['age']		=   $val['age_n'];
					$List[$k]['resume']		=   $val['name'];
					$List[$k]['resume_status'] = $val['status'];
					if ($val['job_classid'] != "") {
						$List[$k]['jobname'] = $val['job_classname'];
					}
				}
			}
			foreach($userid_msg as $val)
			{
				if($val['uid']==$v['uid'])
				{
					$List[$k]['userid_msg']=1;
				}
			}
			
			foreach($userid_job as $val){
			    
				if($v['uid']==$val['uid']){
					$List[$k]['is_browse']		=	$val['is_browse'];
				}
			}
		}
        return $List;
    }
	
/**
     * @desc 关注企业，查询company、company_job相关数据
     * @param array $List
     * @param array $data
     * @return array
     */
    private function getAtnComDataList($List, $data = array())
    {

        $sids   =   array();
        foreach ($List as $v) {
            if ($v['sc_uid'] && !in_array($v['sc_uid'], $sids)) {

                $sids[] =   $v['sc_uid'];
            }
        }

        // 职位信息
        $jobWhere['uid']    =   array('in', pylode(',', $sids));
        $jobWhere['status'] =   array('<>', 1);
        $jobWhere['state']  =   1;
        $jobData['field']   =   '`uid`,`name`,`id`,`pr`,`mun`';
        $joblist            =   $this->getJobList($jobWhere, $jobData);
        foreach ($joblist['list'] as $v) {
            if ($data['wap']) {

                $url = Url('wap', array("c" => 'job', 'a' => 'comapply', "id" => $v['id']));
            } else {

                $url = Url('job', array("c" => 'comapply', "id" => $v['id']));
            }
            $jobname[$v['uid']][]   =   "<a href='".$url."' target='_bank'>".$v['name']."</a>";
        }

        //  企业信息
        $comWhere['uid']    =   array('in', pylode(',', $sids));
        $comData['field']   =   '`uid`,`name`,`sdate`,`ant_num`,`logo`,`hy`,`pr`,`mun`,`logo_status`,`provinceid`,`cityid`,`three_cityid`';
        $comData['logo']    =   '1';
        $comlist            =   $this->getComList($comWhere, $comData);

        foreach ($List as $k => $v) {

            $List[$k]['time_n'] =   date('Y-m-d H:i', $v['time']);

            foreach ($comlist['list'] as $val) {
                if ($v['sc_uid'] == $val['uid']) {

                    $List[$k]['com_name']   =   $val['name'];
                    $List[$k]['hy_n']       =   $val['hy_n'];
                    $List[$k]['pr_n']       =   $val['pr_n'];
                    $List[$k]['mun_n']      =   $val['mun_n'];
                    $List[$k]['logo']       =   $val['logo'];
                    $List[$k]['city_n']     =   $val['citystr'];
                    $sdate                  =   explode('-', $val['sdate']);
                    $List[$k]['com_sdate']  =   $sdate[0];
                    $List[$k]['wapcom_url'] =   Url('wap', array('c' => 'company', 'a' => 'show', 'id' => $v['sc_uid']));
                }
            }

            foreach ($joblist['list'] as $val) {
                if ($v['sc_uid'] == $val['uid']) {

                    $List[$k]['com_pr']     =   $val['job_pr'];
                    $List[$k]['com_mun']    =   $val['job_mun'];

                    $List[$k]['joblist_wx'][$val['id']] = $val['name'];
                }
            }

            foreach ($jobname as $kk => $val) {
                if ($v['sc_uid'] == $kk) {

                    $List[$k]['jobnum'] =   count($val);

                    $i = 0;
                    foreach ($val as $value) {
                        if ($i < 2) {
                            $joblist[$kk][] = $value;
                        }
                        $i++;
                    }
                    $List[$k]['jobname']=   @implode(",", $joblist[$kk]);
                }
            }
        }
        return $List;
    }


    /**
     * @desc 取消关注企业
     * @param 表：atn
     * @param array $data :字段 sc_usertype : 1个人、2企业
     * @return array
     */
    public function delAtnAll($id, $data = array())
    {
     
		if(!empty($id)){
 
		    $return   =   array();
		    
		    if ($data['type'] == 'admin') {   // 后台删除院校关注人
		        
		        if(is_array($id)){
		        
		            $ids    				=	$id;
		            $return['layertype']	=	1;
		        }else{
		            
		            $ids    				=   @explode(',', $id);
		            $return['layertype']	=	0;
		        }
		        $id       =   pylode(',', $ids);
				
		        
                $result             =   $this->delete_all('atn', array('id' => array('in', $id)), '');
		        
                $return['msg']      =   $result ? '删除成功' : '删除失败';
                $return['errcode']  =   $result ? 9 : 8;
		        
		        return $return;
		        
		    }else{
		        
		        $return['layertype']	=	0;

                $id       =   intval($id);
                $uid      =   intval($data['uid']);
                $usertype =   intval($data['usertype']);

                $field      =   '`sc_uid`';
                $where      =   array('id' => $id, 'uid' => $uid, 'usertype' => $usertype, 'sc_usertype' => 2);
                $atnInfo    =   $this->select_once('atn', array('id' => $id), $field);
                $result     =   $this -> delete_all('atn', $where, '');

                if($result){
                    $comInfo    =   $this->select_once('company', array('uid' => $atnInfo['sc_uid']), '`name`,`atn_num`');
                    if ((int)$comInfo['atn_num'] > 0){
                        $this->update_once('company', array('ant_num' => array('-', 1)), array('uid' => intval($data['cuid'])));
                    }
                    $logDetail  =   '取消关注企业：《'.$comInfo['name'].'》';
                    $this->addMemberLog($data['uid'], $data['usertype'], '取消关注：招聘企业（UID：'.$atnInfo['sc_uid'].'）', 5, 3, $logDetail);

                    $return['msg']      =  '取消成功';
                    $return['errcode']  =  '9';
                } else{
                    $return['msg']      =  '取消失败';
                    $return['errcode']  =  '8';
                }
		    }
			
		}else{

			$return['msg']      	=  '系统繁忙';
			$return['errcode']  	=  '8';
			$return['layertype']	=	0;
		}
		return $return;  
	}
	
    public function getatnInfo($where = array(), $data = array())
    {
       
          $select     =   $data['field'] ? $data['field'] : '*';
               
          $atnInfo    =   $this -> select_once('atn', $where, $select);
               
          return $atnInfo;
     }
    /**
     * @desc 添加信息
     * @param array $data
     * @return bool
     */
    public function addAtnInfo($data = array())
    {

        return $this->insert_into('atn', $data);
    }

    /**
     * @desc 关注数量
     * @param array $whereData
     * @return array|bool|false|string|void
     */
    function getAtnNum($whereData = array())
    {

        return $this->select_num('atn', $whereData);
    }
	
	/**
	 * @desc 关注
	 * @param array $data
	 * @return array
	 */
    public function addAtnLt($data = array())
    {
        $return =   array();
        $id     =   (int)$data['id'];

        if ($id > 0) {
            if ($data['uid'] && $data['username']) {

                if ($data['usertype'] != '1') {

                    $return['msg'] = '只有个人用户才可以关注！';
                    $return['errcode'] = 2;
                }

                if ($_POST['id'] == $data['uid']) {

                    $return['msg'] = '自己不能关注自己！';
                    $return['errcode'] = 2;
                }
                
                $where                  =   array();
                $where['uid']           =   $data['uid'];   // 关注人UID
                $where['sc_uid']        =   $id;            // 被关注人UID
                
                $where['sc_usertype']   =   $data['sc_usertype'];
                
               	if ($data['utype'] == 'teacher') {
                   
                    $table          =   'company';
                    $where['xjhid'] =   '0';
                    $company        =   $this->select_once('company', array('uid' => $id), "`name`");
                    $name           =   $company['name'];
                    $logContent     =   '关注：招聘企业（UID：'.$id.'）';
                    $logDetail      =   '关注招聘企业：《'.$name.'》';
                    $utype          =   2;
                }

                $atninfo = $this->select_once('atn', $where, '`id`');  // 查询已关注信息

                if (is_array($atninfo) && !empty($atninfo)) {

                    $this->delete_all('atn', $where, '');

                    if (!empty($table)) {

                        $this->update_once($table, array('ant_num' => array('-', 1)), array('uid' => $id, 'atn_num' => array('>', 0)));
                    }

                    if (!empty($utype)) {
                        include_once('sysmsg.model.php');
                        $sysmsgM    =   new sysmsg_model($this->db, $this->def);

                        $userurl    =   '<a href="usertpl,'.$data['uid'].'">'.sub_string($data['username']).'</a>';
                        $content    =   "用户 ".$userurl." 取消了对您（".$name."）的关注！";
                        $sysmsgM->addInfo(array('uid' => $id, 'usertype' => $utype, 'content' => $content));
                    }

                    $this->addMemberLog($data['uid'], $data['usertype'], '取消'.$logContent, 5, 3, '取消'.$logDetail);

                    $return['msg']      =   '取消关注成功！';
                    $return['cancel']   =   1;
                    $return['errcode']  =   1;
                } else {
                    
                    $adata      =   array(
                        'uid'       =>  $data['uid'],
                        'sc_uid'    =>  $id,
                        'usertype'  =>  $data['usertype'],
                        'time'      => time()
                    );
                    $adata['sc_usertype'] = $data['sc_usertype'];
                    $this->insert_into('atn', $adata);
                    if (!empty($table)) {
                        $this->update_once($table, array('ant_num' => array('+', 1)), array('uid' => $id));
                    }
                    if (!empty($utype)) {
                        include_once('sysmsg.model.php');
                        $sysmsgM = new sysmsg_model($this->db, $this->def);
                        $userurl = '<a href="usertpl,' . $data['uid'] . '">' . sub_string($data['username']) . '</a>';
                        $content = "用户 " . $userurl . " 关注了你(" . $name . ")";
                        $sysmsgM->addInfo(array('uid' => $id, 'usertype' => $utype, 'content' => $content));
                    }

                    $this->addMemberLog($data['uid'], $data['usertype'], $logContent, 5, 1, $logDetail);

                    $return['msg'] = '关注成功！';
                    $return['errcode'] = 1;
                }
            } else {

                $return['msg'] = '您还没有登录！';
                $return['errcode'] = 2;
            }
        }
        return $return;
    }
}
?>