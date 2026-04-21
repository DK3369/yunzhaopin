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
class userEntrust_model extends model{
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
     * @desc   引用log类，添加用户日志
     */
    private function addMemberLog($uid,$usertype,$content,$opera='',$type='') {
        
        require_once ('log.model.php');
        
        $LogM = new log_model($this->db, $this->def);
        
        return  $LogM -> addMemberLog($uid,$usertype,$content,$opera,$type);
        
    }

    /**
     * 获取简历推送记录
     * $whereData 	查询条件
     * $data 		自定义处理数组
     */
    
    public function getRecordInfo($whereData,$data=array()){
        
        $select     =   $data['field'] ? $data['field'] : '*';
        
        $info		=	$this -> select_once('user_entrust_record',$whereData,$select);
        
        if (is_array($info)) {
            
            return	$info;
            
        }    
    }
    
    /**
     * 简历推送记录列表 查询数量
     * @param $whereData
     * @return int
     */
    public function getRecordNum($whereData = array())
    {
        return intval($this->select_num('user_entrust_record', $whereData));
    }
    
    /**
    * 获取简历推送记录列表
    * $whereData 	查询条件
    * $data 		自定义处理数组
    */
    
    public function getRecordList($whereData,$data=array()){
        $ListNew	=	array();
        $List		=	$this -> select_all('user_entrust_record',$whereData);
        
        if(!empty( $List )){
            $List  =  $this -> getDataRecord($List, $data);
            
            $ListNew['list']	=	$List;
        }
        
        return	$ListNew;
    }
    /**
     * 简历记录管理-简历推送记录-删除推送
     */
    public function delRecord($id, $data = array()){
        
        $limit                =  'limit 1';
        if (!empty($id)){
            
            if(is_array($id)){
                
                $id  =  pylode(',', $id);
                $return['layertype']  =  1;
                $limit                =  '';
            }else{
				$return['layertype']  =  0;
			}
			if($data['uid']){
				if($data['usertype']==2){
					$delWhere = array('id'=>array('in',$id),'comid'=>$data['uid']);
				}else{
					$delWhere = array('id'=>array('in',$id),'uid'=>$data['uid']);
				}
			}else{
				$delWhere = array('id'=>array('in',$id));
			}
            $nid      =  $this->delete_all('user_entrust_record',$delWhere,$limit);
            
            if ($nid){
                $this -> addMemberLog($data['uid'],$data['usertype'],"悬赏推荐：删除推送的人才",25,3);
				
                $return['msg']      =  '简历推送(ID:'.$id.')删除成功';
                $return['errcode']  =  '9';
            }else{
                $return['msg']      =  '简历推送(ID:'.$id.')删除成功';
                $return['errcode']  =  '8';
            }
        }else{
            $return['msg']      =  '请选择您要删除的简历推送';
            $return['errcode']  =  '8';
        }
        return $return;
    }

    /**
    * 推送简历列表,后台显示数据处理
    */
    private function getDataRecord($List, $data = array('utype' => '')){
        
        foreach($List as $v){
            if ($v['uid']){
                $uids[]    =  $v['uid'];
            }
            if ($v['eid']){
                $eids[]    =  $v['eid'];
            }
            if ($v['comid']){
                $comids[]  =  $v['comid'];
            }
            if ($v['jobid']){
                $jobids[]  =  $v['jobid'];
            }
        }
		//  查询个人姓名
		$rWhere['uid']    =   array('in', pylode(',', $uids));
        $rData['field']   =   '`uid`,`name`,`nametype`,`sex`,`def_job`';
        
        $resume           =   $this -> getResumeList($rWhere, $rData);
		
		 //  查询个人简历名称
        $reWhere['id']    =   array('in', pylode(',', $eids));
        $reData['field']  =   '`id`,`name`,`job_classid`,`minsalary`,`maxsalary`,`exp`';
        
        $expect  =   $this -> getResumeExpectList($reWhere, $reData);
        
        $com     =   $this -> select_all('company',array('uid'=>array('in',pylode(',', $comids))),'`uid`,`name`');
        
        $job     =   $this -> select_all('company_job',array('id'=>array('in',pylode(',', $jobids))),'`id`,`name`');
		
		$userid_msg  =	$this -> select_all("userid_msg",array('fid'=>array('in',pylode(',', $comids)),'uid'=>array('in',pylode(",",$uids))),'`uid`');
		
        foreach($List as $k=>$v){
            foreach($resume as $val){
	            
	            if($v['uid'] == $val['uid']){
	                $List[$k]['user_name']      =  $val['name_n'];
	                $List[$k]['username_n']     =  $val['username_n'];
	            }
	        }
			foreach($expect['list'] as $val){
	            
	            if( $v['eid'] == $val['id']){
					$List[$k]['exp']			=	$val['exp_n'];
					$List[$k]['salary']			=	$val['salary'];
					if ($val['job_classid'] != "") {
	                    $List[$k]['jobclassidname']	= $val['job_classname'];
	                }
	                $List[$k]['resume_name']	=  $val['name'];
	            }
	        }
            foreach($com as $val){
                
                if($val['uid'] == $v['comid']){
                    
                    $List[$k]['com_name']  =  $val['name'];
                    $com_url_arr = array('c' => 'show', 'id' => $v['comid']);
                    if ($data['utype'] == 'admin') {
                        $com_url_arr['look'] = 'admin';
                    }
                    $com_url = Url('company', $com_url_arr);
                    $List[$k]['com_url'] = $com_url;
                }
            }
            foreach($job as $val){
                
                if($val['id'] == $v['jobid']){
                    
                    $List[$k]['job_name']  =  $val['name'];
                    $job_url_arr = array('c' => 'comapply', 'id' => $v['jobid']);
                    if ($data['utype'] == 'admin') {
                        $job_url_arr['look'] = 'admin';
                    }
                    $List[$k]['job_url'] = Url('job', $job_url_arr);
                }
            }
			foreach($userid_msg as $val){
				if($v['uid']==$val['uid']){
					$List[$k]['userid_msg']		=	1;
				}
			}
            $List[$k]['ctime_n'] = $v['ctime'] ? date('Y-m-d H:i', $v['ctime']) : '';
        }
        return $List;
    }

	function getEntrustNum($where=array()){
		return $this->select_num('user_entrust',$where);
	}
}
?>