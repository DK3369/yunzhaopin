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
class resumeout_controller extends user{
    //简历外发列表
    function index_action(){

		$resumeM				=	$this->MODEL('resume');
		$where['uid']			=  $this -> uid;
		$urlarr['c']			=	$_GET['c'];
		$urlarr['page']			=	'{{page}}';
	    $pageurl				=	Url('member',$urlarr);

	    $pageM					=	$this   ->  MODEL('page');
	    $pages					=	$pageM  ->  pageList('resumeout',$where,$pageurl,$_GET['page']);
	    
	    if($pages['total'] > 0){
	        $where['orderby']	=	'id';
	        $where['limit']		=	$pages['limit'];
	        
	        $List   =  $resumeM  ->  getResumeOutList($where);
	    }
		
        $rows		=	$resumeM -> getSimpleList(array('uid'=>$this->uid),array('field'=>"id,name"));
        
        $outmsg =   '';
        $recomM =   $this->MODEL('recommend');
        if (isset($this->config['sy_resumeout_day_num']) && $this->config['sy_resumeout_day_num'] > 0) {

            
            $num    =   $recomM->getRecommendNum(array('uid' => $this->uid,'rec_type'=>3));
            
            if ($num > $this->config['sy_resumeout_day_num']) {
                
                $outmsg ='每天最多外发'.$this->config['sy_resumeout_day_num'].'次简历！';

            }
            
        }else{
            $outmsg ='外发简历功能已关闭！';
        }
        if(isset($this->config['sy_resumeout_interval']) && $this->config['sy_resumeout_interval'] > 0){
    
            $recrow                 =   $recomM -> getInfo(array('uid' =>$this->uid,'rec_type'=>3, 'orderby' => 'addtime'));

            if(!empty($recrow['addtime']) && (time() - $recrow['addtime']) < $this->config['sy_resumeout_interval']){
                $needTime = $this->config['sy_resumeout_interval'] - (time() - $recrow['addtime']);
                if($needTime > 60){
                    $h              =   floor(($needTime % (3600*24)) / 3600);
                    $m              =   floor((($needTime % (3600*24)) % 3600) / 60);
                    $s              =   floor((($needTime % (3600*24)) % 3600 % 60));
                    if($h != 0){
                        $needTime   =   $h.'时';
                    }else if($m != 0){
                        $needTime   =   $m.'分';
                    }
                }else{
                    $needTime       =   $needTime.'秒';
                }

                $recs               =   $this->config['sy_resumeout_interval'];
                if($recs>60){
                    $h              =   floor(($recs % (3600*24)) / 3600);
                    $m              =   floor((($recs % (3600*24)) % 3600) / 60);
                    $s              =   floor((($recs % (3600*24)) % 3600 % 60));
                    if($h != 0){
                        $recs       =   $h.'时';
                    }else if($m != 0){
                        $recs       =   $m.'分';
                    }
                }else{
                    $recs           =   $recs.'秒';
                }
                $outmsg ='外发简历间隔不得少于'.$recs.'，请'.$needTime.'后再外发！';
            }
        }
        $this->yunset('outmsg',$outmsg);
        $this->public_action();
        $this->yunset('rows',$rows);
        $this->yunset('out',$List);
        $this->user_tpl('resumeout');
    }
    //发送简历外发
    function send_action() {
        $resumeM	=	$this -> MODEL('resume');
        $_POST		=	$this->post_trim($_POST);
		
        $data		=	array(
            'uid'         =>  $this->uid,
            'resume'	  =>  $_POST['resume'],
            'email'		  =>  $_POST['email'],
            'comname'	  =>  $_POST['comname'],
            'jobname'	  =>  $_POST['jobname'],
            'resumename'  =>  $_POST['resumename']
        );
        
        $return		=	$resumeM  ->  addResumeOut($data,array('uid'=>$this->uid));
        $this -> ACT_layer_msg($return['msg'],$return['errcode'],$_SERVER['HTTP_REFERER']);
    }
    //删除简历外发记录
    function del_action(){
       $resumeM	=	$this->MODEL('resume');
        if($_GET['id']){
          $return		=	$resumeM  ->  delResumeOut($_GET['id'],array('uid'=>$this->uid,'usertype'=>$this->usertype));
          $this -> layer_msg($return['msg'],$return['errcode']);
        }
    }
}
?>