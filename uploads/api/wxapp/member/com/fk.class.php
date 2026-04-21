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
class fk_controller extends com_controller{
    /**
     * @desc 会员套餐、增值服务、单项购买页面
     */
    function server_action(){
        
        include(CONFIG_PATH.'db.data.php');
      
        $sy_only_price  =   @explode(',', $this->config['sy_only_price']);
        $ratingM        =   $this->MODEL('rating');
        if (!empty($this->comInfo['package'])){

            $ratingList =   $ratingM -> getList(array('id' => array('in', $this->comInfo['package']), 'orderby' => array('type,asc', 'sort,desc')));
        }else{

            $ratingList =   $ratingM -> getList(array('display' => 1, 'orderby' => array('type,asc', 'sort,desc')));
        }
        $rating_1   =   $rating_2   =   $raV    =   array();
        
        if (!empty($ratingList)) {
            
            foreach ($ratingList as $ratingV) {
                
                $raV[$ratingV['id']]    =   $ratingV;
                
                if ($ratingV['category'] == 1) {
                    //有效期处理
                    if($ratingV['service_time']>0){
                        
                        $ratingV['service_time']	=	$ratingV['service_time'].'天';
                        
                    }else{
                        $ratingV['service_time']	=	'永久';
                    }
                    //有效期处理end
                    
                    //价格显示处理
                    if($ratingV['time_start'] < time() && $ratingV['time_end'] > time()){
                        if($this->config['com_integral_online']==3 && !in_array('vip', $sy_only_price)){
                            $ratingV['service_price_n']	=	intval($ratingV['service_price'] * $this->config['integral_proportion']);
                            $ratingV['yh_price_n']		=	intval($ratingV['yh_price'] * $this->config['integral_proportion']).$this->config['integral_pricename'];
                        }else{
                            $ratingV['service_price_n']	=	$ratingV['service_price'];
                            $ratingV['yh_price_n']		=	'￥'.$ratingV['yh_price'];
                        }
                    }else{
                        if($this->config['com_integral_online']==3 && !in_array('vip', $sy_only_price)){
                            $ratingV['service_price_n']	=	intval($ratingV['service_price'] * $this->config['integral_proportion']).$this->config['integral_pricename'];
                        }else{
                            $ratingV['service_price_n']	=	'￥'.$ratingV['service_price'];
                        }
                        
                        unset($ratingV['yh_price']);
                    }
                    //价格显示处理end
                    $ratingV['explains']    	=   $ratingV['explains']?$ratingV['explains']:$ratingV['name'];
                    if ($ratingV['integral_buy']>0){
                        $ratingV['explains']	.=	' 赠送'.$ratingV['integral_buy'].$this->config['integral_pricename'];
                    }

                    if ($ratingV['type'] == 1) {
                        
                        $rating_1[]     =   $ratingV;
                    } elseif ($ratingV['type'] == 2) {
                        //套餐详细处理
                        if ($ratingV['interview'] > 0 || $ratingV['resume'] > 0){
                            $ratingV['interview_resume']	=	'';
                            if($ratingV['interview'] > 0){
                                $ratingV['interview_resume'].='面试邀请: '.$ratingV['interview'].'次；';
                            }
                            if($ratingV['resume'] > 0){
                                $ratingV['interview_resume'].='简历下载: '.$ratingV['resume'].'次；';
                            }
                        }
                        if ($ratingV['breakjob_num'] > 0){
                            $ratingV['job_breakjob']    =   '';
                            $ratingV['job_breakjob'].='刷新职位: '.$ratingV['breakjob_num'].'份；';
                        }
                        if($ratingV['zph_num'] > 0){
                            $ratingV['zph'].='招聘会报名 : '.$ratingV['zph_num'].'次';
                        }
                        //套餐详细处理
                        
                        $rating_2[]     =   $ratingV;
                    }
                }
            }
        }
        if (isset($this->comInfo['noPermission']) && $this->comInfo['noPermission'] == 1 && !empty($this->config['com_package_open'])){
            $data['noPermission']   =   1;  //  无可浏览会员
        }
        if ($this->comInfo['r_status'] == 4){
            $data['isPaused']       =   1;  //  已暂停
        }
        if ((int)$this->comInfo['crm_uid'] > 0){

            $crmInfo    =   $this->obj->select_once('admin_user', array('uid' => $this->comInfo['crm_uid']), '`name`,`moblie`,`ewm`');
            $data['consultant'] =   array('name' => $crmInfo['name'], 'mobile' => $crmInfo['moblie'], 'wechat' => !empty($crmInfo['ewm']) ? checkpic($crmInfo['ewm']): '');
        }

        $data['rating_1']	=	array_values($rating_1);//套餐会员
        $data['rating_2']	=	array_values($rating_2);//时间会员
        
        $comStatis     		=   $this->company_statis($this->member['uid']);

        $statis             =   array();
        
        if(!empty($comStatis)){
            
            $discount           	=   isset($raV[$comStatis['rating']]) ? $raV[$comStatis['rating']] : array();
            if($discount['service_discount'] > 0){
                $statis['zk']       =   $discount['service_discount'] * 0.01 ;
                $statis['zk_n']     =   $discount['service_discount'] * 0.1 ;
            }
            if (isVip($comStatis['vip_etime'])){
                $statis['notOvertime']  =  1;
            }else{
                $statis['notOvertime']  =  0;
            }
            $statis['rating_type']  =  $comStatis['rating_type'];
            $statis['rating_name']  =  $comStatis['rating_name'];
            $statis['integral']     =  $comStatis['integral'];
            
            $data['statis']		=	$statis;//当前已有的服务信息
        }

        if ($this->member['usertype'] == 2) {
            
            $add        	=   $ratingM->getComServiceList(array('display' => 1 , 'orderby' => array('sort,desc')), array('detail' => 'yes'));
        }
        if(!empty($add)){
            foreach ($add as $k => $v) {
                foreach ($v['detail'] as $dk => $dv) {
                    //价格显示处理
                    if($this->config['com_integral_online']==3 && !in_array('pack', $sy_only_price)){
                        
                        if($statis['zk']){
                            $add[$k]['detail'][$dk]['service_price_n']	=	intval($dv['service_price'] * $this->config['integral_proportion']);
                            
                            $add[$k]['detail'][$dk]['yh_price_n']		=	intval($dv['service_price'] * $this->config['integral_proportion'] * $statis['zk']).$this->config['integral_pricename'];
                            $add[$k]['detail'][$dk]['yh_price']			=	sprintf('%.2f',$dv['service_price'] * $statis['zk']);
                        }else{
                            
                            $add[$k]['detail'][$dk]['service_price_n']	=	intval($dv['service_price'] * $this->config['integral_proportion']).$this->config['integral_pricename'];
                        }
                        
                    }else{
                        
                        if($statis['zk']){
                            $add[$k]['detail'][$dk]['service_price_n']	=	$dv['service_price'];
                            $add[$k]['detail'][$dk]['yh_price_n']		=	'￥'.$dv['service_price'] * $statis['zk'];
                            $add[$k]['detail'][$dk]['yh_price']			=	sprintf('%.2f',$dv['service_price'] * $statis['zk']);
                        }else{
                            $add[$k]['detail'][$dk]['service_price_n']	=	'￥'.$dv['service_price'];
                        }
                        
                    }
                    //价格显示处理end
                }
            }
        }
        $data['add']  =  $add;//增值列表
        
        $server  =  trim($_POST['server']);
        $com_single_can = explode(',', $this->config['com_single_can']);

        $serverCheck = $server;
        if($server=='sxpart'||$server=='sxjob'){
            $serverCheck = 'sxjob';
        }
        // 判断后台是否设置可以单项购买
        if($serverCheck && ($serverCheck=='autojob' || in_array($serverCheck,$com_single_can))){
            $data['sigle_show'] = 1;
        }
        switch($server){
            case 'issuejob':
                $single_price	  =	 $this->config['integral_job'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次上架职位';
                break;
            case 'jobtop':
                $single_price	  =	 $this->config['integral_job_top'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次职位置顶';
                break;
            case 'jobrec':
                $single_price	  =	 $this->config['com_recjob'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次职位推荐';
                break;
            case 'joburgent':
                $single_price	  =	 $this->config['com_urgent'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次职位紧急招聘';
                break;
            case 'sxjob':
                
                if ($_POST['id'] == 'all'){
                    $jobM  =  $this->MODEL('job');
                    $jobs  =  $jobM -> getList(array('uid'=>$this->member['uid'],'state' => 1, 'status' => 0,'r_status' => 1),array('field'=>'`id`'));
                    foreach ($jobs['list'] as $v){
                        $jobid[]  =  $v['id'];
                    }
                }else{
                    $jobid        =   array($_POST['id']);
                }
                $num    	      =  count($jobid) - $statis['breakjob_num'];
                $single_price	  =	 $this->config['integral_jobefresh'] * $num;
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次刷新职位';
                
                break;
            case 'sxpart':
                $single_price	  =	 $this->config['integral_jobefresh'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次刷新职位';
                
                break;
            case 'downresume':
                $resumeM    	  =  $this->MODEL('resume');
                $id         	  =  intval($_POST['id']);
                $price      	  =  $resumeM -> setDayprice($id);
                
                $single_price	  =	 $price;
                $single_integral  =  $price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次下载简历';
                
                break;
            case 'invite':
                $single_price	  =	 $this->config['integral_interview'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次邀请面试';
                
                break;
            case 'zph':
                $zphM       	  =   $this -> MODEL('zph');
                $id         	  =   intval($_POST['bid']);
                $space      	  =   $zphM -> getZphSpaceInfo(array('id' => $id));
                
                $single_price	  =	 $space['price'] / $this->config['integral_proportion'];
                $single_integral  =  $space['price'];
                $single_msg		  =	'本次报名招聘会';
                
                break;
            case 'autojob':
                $single_price	  =	 $this->config['job_auto'];
                $single_integral  =  $single_price * $this->config['integral_proportion'];
                $single_msg		  =	 '本次设置自动刷新';
                break;
        }
        
        $config  =  array(
            'com_vip_type'			=>	$this->config['com_vip_type'],
            'com_integral_online'	=>	$this->config['com_integral_online'],
            'integral_proportion'	=>	$this->config['integral_proportion'],
            'integral_pricename'	=>	$this->config['integral_pricename'],
            'integral_min_recharge'	=>	$this->config['integral_min_recharge'],

            'meal_vip'              =>  in_array('vip', $sy_only_price) ? 1 : 0,
            'meal_pack'             =>  in_array('pack', $sy_only_price) ? 1 : 0,
            'sy_comwebtel'          =>  $this->config['sy_comwebtel'],
			'sy_only_price'         =>  $this->config['sy_only_price']
        );
        
        $config['fktype']	      =	 $this->fktype();//支付方式
        $data['config']			  =	 $config;
        $data['single_price']	  =	 $single_price;
        $data['single_integral']  =	 $single_integral;
        $data['single_msg']		  =	 $single_msg;
        $data['vip_etime']        =  $comStatis['vip_etime'];
        $data['time']             =  strtotime('today');
        
        $this->render_json(0,'',$data);
    }
    function dkzf_action(){
        
        $data               =   $this   ->  post_trim($_POST);

        $data['uid']        =   $this   ->  member['uid'];
        $data['username']   =   $this   ->  member['username'];
        $data['usertype']   =   $this   ->  member['usertype'];
    
        $M                  =   $this   ->  MODEL('jfdk');
        
        $return             =   $M      ->  dkBuy($data);
        
        $this->render_json($return['error'],$return['msg']);
        
    }
     
    /**
     * 生成订单
     */
    function getOrder_action()
    {
        $_POST  =  $this -> post_trim($_POST);
        
        if($_POST){

            $arr['uid']		   =  $this->member['uid'];
            $arr['did']		   =  $this->member['did'];
            $arr['usertype']   =  $this->member['usertype'];
            $arr['username']   =  $this->member['username'];

            $arr['price_int']  =  $_POST['price_int'];
            $arr['dkjf']       =  $_POST['integral_dk'];
            if ($this->comInfo['crm_uid']){
                $arr['crm_uid']  =   $this->comInfo['crm_uid'];
            }
            if ($_POST['fktype'] == 'fkal'){
                $arr['paytype']  =  'alipay';
            }
            if ($_POST['from'] == 'wap'){
                $arr['paytype'] = $_POST['paytype'];
            }
            
            $compayM  =  $this -> MODEL('compay');
            
            if($_POST['server']=='autojob'){
                
                $arr['days']        =  $_POST['days'];
                $arr['jobautoids']  =  $_POST['single_id'];
                $return  =	$compayM -> buyAutoJob($arr);
                
            }elseif ($_POST['server']=='jobtop'){
                
                $arr['days']     =  $_POST['days'];
                $arr['zdjobid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyZdJob($arr);
                
            }elseif ($_POST['server']=='joburgent'){
                
                $arr['days']    =  $_POST['days'];
                $arr['ujobid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyUrgentJob($arr);
                
            }elseif ($_POST['server']=='jobrec'){
                
                $arr['days']      =  $_POST['days'];
                $arr['recjobid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyRecJob($arr);
                
            }elseif ($_POST['server']=='sxjob'){
                
                $arr['sxjobid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyRefreshJob($arr);
                
            }elseif ($_POST['server']=='sxpart'){
                
                $arr['sxpartid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyRefreshPart($arr);
                
            }elseif ($_POST['server']=='partrec'){

                $arr['days']      =  $_POST['days'];
                $arr['recpartid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyRecPart($arr);

            }elseif ($_POST['server']=='issuejob'){
                
                $return  =  $compayM -> buyIssueJob($arr);
                
            }elseif ($_POST['server']=='downresume'){
                
                $arr['eid']   =  $_POST['single_id'];
                $return =	$compayM -> buyDownresume($arr);
                
            }elseif ($_POST['server']=='invite') {

                $return = $compayM->buyInviteResume($arr);

            }elseif($_POST['server'] == 'zph'){
                
                $arr['bid']    =  $_POST['bid'];
                $arr['zid']    =  $_POST['single_id'];
                $arr['jobid']  =  $_POST['jobid'];
                $return  =	$compayM -> buyZph($arr);
                
            }elseif($_POST['server'] == 'vip'){
                
                $arr['ratingid']  =  $_POST['single_id'];
                $return  =	$compayM -> buyVip($arr);
                
            }elseif($_POST['server'] == 'pack'){

                $arr['tcid']   =  $_POST['single_id'];
                $return  =  $compayM -> buyPackOrder($arr);
                
            }
            
            if($return['order']['order_id'] && $return['order']['id']){
                $dingdan	=	$return['order']['order_id'];
                $price		=	$return['order']['order_price'];
                $id			=	$return['order']['id'];
                
                $result  =  array(
                    'id'   =>  $id
                );
                if ($_POST['from'] == 'wap') {
                    if ($_POST['paytype'] == 'alipay') {

                        $result['url']  =   $this->config['sy_weburl'].'/api/wapalipay/alipayto.php?dingdan=' . $dingdan . '&dingdanname=' . $dingdan . '&alimoney=' . $price;
                    } 
                }
                
                $this->render_json(0,'ok',$result);
                
            }else{
                
                // 生成失败 返回具体原因
                $this->render_json(1,$return['error']);
            }
        }else{
            
            $this->render_json(1,'参数错误，请重试！');
        }
    }
}