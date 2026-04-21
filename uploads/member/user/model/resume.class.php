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
class resume_controller extends user{
	//简历管理
	function index_action(){

		$this->public_action();
        
        $ResumeM        =     $this->MODEL('resume');
        
        $where['uid']   =     $this->uid;
        
        if (!empty($this->config['user_number'])){
            
            $num		=     $ResumeM->getExpectNum($where);
            $maxnum		=  $this->config['user_number'] - $num;
            
            if($maxnum <= 0){$maxnum='0';}
        }else{

            $maxnum		=  '';
        }
        $this->yunset("maxnum",$maxnum);
		
        $rows  =  $ResumeM -> getSimpleList($where,array('field'=>'id,name,lastupdate,doc,tmpid,integrity,hits,height_status,statusbody,`topdate`,`is_entrust`,`top`,`state`,`status`'));
    
        if($rows&&is_array($rows)){
		
            foreach($rows as $key=>$val){
    				
                if($val['topdate']>1){
        				
                $rows[$key]['topdate']=date("Y-m-d",$val['topdate']);
        
                $rows[$key]['topdatetime']=$val['topdate']-time();
        	
                }else{
                  
    				$rows[$key]['topdate']='未设置';
          
    			}
    		}
		}
       
		$this->yunset("rows",$rows);

        $row    =    $ResumeM->getResumeInfo($where,array('field'=>'def_job,idcard_status,idcard_pic'));

        $resume                       =     $ResumeM->getExpectByUid($this->uid,array('member'=>1,'needCache'=>1,'field'=>'id,uid,integrity,statusbody,sex,whour,jobstatus,report,status,exp'));

        $this->yunset("expectStatus",$resume);
        $integrity                    =     $resume['integrity'];
        
        $renumwhere['integrity']      =     array('>',$integrity);
        
        $resumepm                     =     $ResumeM->getExpectNum($renumwhere);
    
		$this->yunset("resumepm",$resumepm);//简历完整度排名

		$this->yunset("def_job",$row['def_job']);
		$this->user_tpl('resume');
	}
	//删除简历
	function del_action(){
    
        $ResumeM          =       $this->MODEL('resume');
        
        $id               =       $_GET['id'];
        
        $return           =       $ResumeM->delResume($id,array('uid' => $this -> uid));
        
        $this -> layer_msg($return['msg'], $return['errcode'], $return['layertype'], $_SERVER['HTTP_REFERER']);
	}
    //删除简历附表
	function publicdel_action()
    {
        $ResumeM = $this->MODEL('resume');
        
        $uid = $this->uid;
        
        if ($_POST['id'] && $_POST['table']) {
            
            $tables = array("skill","work","project","edu","training","cert","other");
            
            if (in_array($_POST['table'], $tables)) {
                
                $table  =  "resume_" . $_POST['table'];
                $eid    =  (int) $_POST['eid'];
                $id     =  (int) $_POST['id'];
                $url    =  $_POST['table'];
                
                $nid  =  $ResumeM->delFb($_POST['table'], array('id'=>$id,'uid'=>$uid,'eid'=>$eid), array('utype' => 'user'));
                
                if ($nid) {
                    
                    $expect = $ResumeM->getExpect(array('id'=>$eid,'uid'=>$uid),array('field'=>'integrity'));
                    $fbrow  = $ResumeM->getUserResumeInfo(array('eid'=>$eid,'uid'=>$uid));
                    $data   = array(
                        'num'        =>  $fbrow[$url],
                        'integrity'  =>  $expect['integrity']
                    );
                    
                    echo json_encode($data);die();
                } else {
                    echo 0;die();
                }
            } else {
                echo 0;die();
            }
        }
    }
    //购买简历置顶
	function resumeOrder_action(){
		if($_POST){
      
  			$M=$this->MODEL('userpay');
			
			$_POST['uid']		=	$this->uid;
			$_POST['usertype']	=	$this->usertype;
			$_POST['did']		=	$this->userdid;
        
			if ($_POST['resumeid']){
        
				$return = $M->buyZdresume($_POST);
        
				$msg="购买简历置顶";
        
			}
			if($return['order']['order_id'] && $return['order']['id']){
        
				//订单生成成功
                echo json_encode(array('error' => 0, 'orderid' => $return['order']['order_id'], 'id' => $return['order']['id']));

			}else{
				//生成失败 返回具体原因
        
				echo json_encode(array('error'=>1,'msg'=>$return['error']));
        
			}
		}else{
      
			echo json_encode(array('error'=>1,'msg'=>'参数错误，请重试！'));
      
		}
    
	}
	/**
	 * 单个简历刷新
	 */
	function refresh_action()
	{
    
        $resumeM  =  $this->MODEL('resume');
        
        $data     =  array('lastupdate'=>time());
        
        $nid      =  $resumeM -> upInfo(array('id'=>(int)$_GET['id'],'uid'=>$this->uid),array('eData'=>$data));

        if ($nid) {

            $resumeM->upResumeInfo(array('uid' => $this->uid), array('rData' => $data, 'port' => 1));
        }
            
        $nid?$this->layer_msg('刷新成功！',9,0):$this->layer_msg('刷新失败！',8,0);
    
 	}
	
	/**
	 * 会员中心首页提醒弹出框刷新
	 */
	function resumerefresh_action()
	{
	    $resumeM  =  $this->MODEL('resume');
	    
	    $data     =  array(
	        'lastupdate'  =>  time(),
	        'jobstatus'   =>  $_POST['jobstatus']
	    );
	    
	    $nid  =  $resumeM -> upInfo(array('id'=>(int)$_POST['id'],'uid'=>$this->uid),array('eData'=>$data));
    		
        if($nid){
            
            $resumeM -> upResumeInfo(array('uid'=>$this->uid),array('rData'=>array('lastupdate'=>time()), 'port' => 1));
    			
            echo 1;die;
        }
    }
 	
 	
   
	//设置默认简历，TODO:仅个人会员中心
    function defaultresume_action()
    {
     
        $resumeM  =  $this->MODEL('resume');
        
        $resumeM -> defaults(array('id'=>(int)$_GET['id'],'uid'=>$this->uid));
    
		$this->layer_msg('操作成功！',9,0,"index.php?c=resume");
	}
}
?>