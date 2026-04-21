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

class statis_model extends model{

	/**
	 * @desc   获取账户套餐数据信息
	 * 
	 * @param int $uid
	 * @param array $data
	 * @return $statis
	 */
	function getInfo($uid, $data = array())
    {
        if (! empty($uid)) {
            
            $uid    =   intval($uid);

            $uType  =   intval($data['usertype']);

            $TBList = array(
                1 => 'member_statis',
                2 => 'company_statis'
            );

            $tb = $TBList[$uType];

            $select = $data['field'] ? $data['field'] : '*';
            $statis = $this -> select_once($tb, array('uid' => $uid), $select);
            if ($statis && is_array($statis)) {
				$statis['packfk'] 		=   sprintf('%.2f', $statis['packpay']);
                $statis['freeze_n']		=   sprintf('%.2f', $statis['freeze']);
                $statis['vip_stime_n']	=   date('Y-m-d', $statis['vip_stime']);
                $statis['vip_etime_n']	=   $statis['vip_etime'] > 0 ? date('Y-m-d', $statis['vip_etime']): '不限';
                if($uType=='2'){
                    $statis['max_time_n']  =   $statis['max_time'] > 0 ? date('Y-m-d', $statis['max_time']): '不限';
                }
                return $statis;
            }
        }
    }
	
	/**
	 * @desc   获取账户套餐数据信息列表
	 * 
	 * @param array $whereData
	 * @param array $data
	 * @return $statis
	 */
	function getList($whereData = array(), $data = array())
    {
        if (! empty($whereData)) {

            $uType = $data['usertype'] ? intval($data['usertype']) : 2;

            $TBList = array(
                1 => 'member_statis',
                2 => 'company_statis'
            );

            $tb = $TBList[$uType];

            $select = $data['field'] ? $data['field'] : '*';

            $statis = $this -> select_all($tb, $whereData, $select);

            if ($statis && is_array($statis)) {
                if(in_array($uType, array(2, 3))){
                    foreach ($statis as $sk => $sv) {
                        if(isset($sv['vip_stime']) && $sv['vip_stime'] > 0){
                            $statis[$sk]['vip_stime_str']    =   date('Y-m-d', $sv['vip_stime']);
                        }
                        if($sv['vip_etime'] > 0){
                            
                            $statis[$sk]['vip_etime_str']    =   date('Y-m-d', $sv['vip_etime']);
                        }else{
                            
                            $statis[$sk]['vip_etime_str']    =   '永久';
                        }
                        if($sv['rating_type'] == 1){
                            
                            $statis[$sk]['rating_type_name'] =   '套餐模式';
                        }elseif ($sv['rating_type'] == 2) {
                            
                            $statis[$sk]['rating_type_name'] =   '时间模式';
                        }
                    }
                }

                return $statis;
            }
        }
    }

    /**
     * @desc   更新账户套餐数据信息
     *
     * @param array $data 更新数据
     * @param array $whereData
     * @return bool
     */
	function upInfo($data = array(), $whereData = array())
    {
        if (!empty($data) && $whereData) {

            $uid        = intval($whereData['uid']);
            $uType      = intval($whereData['usertype']);
            $ratingid   = intval($data['rating']);
            
            $TBList = array(
                1 => 'member_statis',
                2 => 'company_statis'
            );

            $tb = $TBList[$uType];

            /* 更新职位表会员等级字段，名企数据  */
            if($uType == 2 && $ratingid){
                
                if (empty($data['rating_name']) || empty($data['vip_etime'])) {

                    require_once 'rating.model.php';
                    $ratingM    =   new rating_model($this->db, $this->def);
                    $ratingInfo =   $ratingM -> ratingInfo($ratingid, $uid);
                    
                    $data['rating_name']    =   $ratingInfo['rating_name'];

                    $data['vip_etime']      =   $ratingInfo['vip_etime'];
                    
                }
                $this -> update_once('company_job', array('rating' => $ratingid), array('uid' => $uid));
				$this -> update_once('company', array('rating' => $ratingid,'rating_name'=>$data['rating_name'],'vipstime'=>time(),'vipetime'=>$data['vip_etime']), array('uid' => $uid));
                
            }
            
            /* 管理员修改企业，变更会员等级，记录订单数据 */
            if (isset($whereData['adminedit']) && isset($whereData['info'])) {
                
                $rinfo  =   $whereData['info'];
                
                if($rinfo['time_start'] < time() && $rinfo['time_end'] > time()){
                    $price = $rinfo['yh_price'];
                }else{
                    $price = $rinfo['service_price'];
                }
                
                if($price > 0 && ($uType == 2||$uType == 3)){
                    
                    $statisInfo =   $this -> select_once($tb, array('uid' => $uid));
                    
                    if ($statisInfo['rating'] != $rinfo['id']) {
                        
                        $rinfo['usertype']	=	$uType;
                        $this -> addComOrder($rinfo, $uid, $ratingid);
                    }
                }
                
                $hData = array(
                    
                    'rating_id'     =>  $ratingid,
                    'rating'        =>  $data['name'],
                    'servide_price' =>  $price,
                    'time_start'    =>  time(),
                    'time_end'      =>  $rinfo['service_time'] == 0 ? strtotime("2029-12-30") : $rinfo['service_time'],
                );
                
                $this -> update_once('hotjob', $hData, array('uid' => $uid));
            }

            $nid = $this -> update_once($tb, $data, array('uid'=>$uid));
            
			return $nid;
        }
    }
    
    /**
     * @desc   管理员修改会员等级，记录订单 
     * 
     * @param array $data
     * @param int $uid 
     * @param int $rating 会员等级
     */
    private function addComOrder($data = array(), $uid = null, $rating = null) {
        
        if($data['time_start'] < time() && $data['time_end'] > time()){
            
            $price  =   floatval($data['yh_price']);
        }else{
            
            $price  =   floatval($data['service_price']);
        }

        $comInfo    =   $this->select_once('company', array('uid' => $uid), '`crm_uid`');
        $ratingInfo = $this->select_once('company_rating', array('id' => $rating),'`name`,`integral_buy`');
        $dingdan    =   time().rand(10000,99999);
        
        $order      =   array(
            'order_id'      =>  $dingdan,
            'order_price'   =>  $price,
            'type'          =>  1,
            'order_time'    =>  time(),
            'order_state'   =>  '2',
            'order_remark'  =>  '管理员修改会员套餐'.$ratingInfo['name'],
            'uid'           =>  intval($uid),
			'usertype'      =>  $data['usertype'],
            'did'           =>  $this->config['did'],
            'rating'        =>  intval($rating),
            'order_type'    =>  'adminpay'
        );
        if ((int)$comInfo['crm_uid'] > 0){
            $order['crm_uid']   =   $comInfo['crm_uid'];
        }
        
        $this ->insert_into('company_order', $order);
        
        // 后台变更会员赠送积分
        if ($data['usertype'] == 2 && $ratingInfo['integral_buy'] > 0) {
            require_once('integral.model.php');
            $IntegralM = new integral_model($this->db, $this->def);
            $IntegralM->insert_company_pay($ratingInfo['integral_buy'], 2, $uid, 2, '管理员修改会员套餐赠送积分', 1, 2, true);
        }
    }
	
	/**
	 * @desc   新增账户，添加账户套餐数据信息
	 * 
	 * @param array $data
	 * @param array $where
	 */ 
	function addInfo($data = array(), $where = array())
    {
        if (!empty($data)) {

            $uType = intval($where['usertype']);

            $TBList = array(
                1 => 'member_statis',
                2 => 'company_statis'
            );

            $tb = $TBList[$uType];

            return $this -> insert_into($tb, $data);
        }
    }
    /**
     * @desc 企业 会员套餐过期检测，并处理
     */
    public function vipOver($uid, $usertype = null){
    	
        $statis     =   $this -> getInfo($uid, array('usertype' => $usertype));//查询会员信息
        
        $cominfo    =   $this -> select_once('company',array('uid'=>$uid));
        $vipover  = false;

        if($usertype==2){

            if(!isVip($statis['vip_etime'])){

            	$statis['remind']=1;
            }

            $max_time = intval($statis['max_time']);
            
            if($max_time>0 && !isVip($max_time) && $cominfo['r_status']==4){
                $vipover = true;
            }
        }


        if($vipover || ($statis['vip_etime'] && !isVip($statis['vip_etime']) && $cominfo['r_status']!=4)){ //已过期非暂停会员
            //会员到期，变为过期会员
            if($this->config['com_vip_done'] == '0'){
                // 会员等级未清0的才需要处理，不加此判断，过期的企业，调用会重复处理
                if ($statis['rating'] > 0){
                    $data = array(
                        'job_num'       =>  '0',
                        'breakjob_num'  =>  '0',
                        'down_resume'   =>  '0',
                        'invite_resume' =>  '0',
                        'zph_num'       =>  '0',
                        'top_num'       =>  '0',
                        'rec_num'       =>  '0',
                        'urgent_num'    =>  '0',
                        'oldrating_name'=>  $statis['rating_name'],
                        'rating_name'   =>  '过期会员',
                        'rating_type'   =>  '0',
                        'rating'        =>  '0',
                        'suspend_num'   =>  '0',
                        'max_time'      =>  '0',
                    );
                    $this -> upInfo($data, array('uid' => $uid,'usertype' => $usertype));
                    
                    $this -> update_once('company', array('rating'=>$data['rating'], 'rating_name'=>$data['rating_name']), array('uid' => $uid));
                    $upJobArr['rating'] = 0;
                    
                    //会员到期职位下架
                    if($this -> config['jobunder'] == '1'){

                        if (!isset($this->config['job_under_delay']) || empty($this->config['job_under_delay'])){

                            $upJobArr['status'] = 1;
                        }
                    }
                    $this -> update_once('company_job', $upJobArr, array('uid' => $uid));

                    
                }
                //  过期会员，会员模式、会员等级清0
                $statis['rating_name']  =   '过期会员';
                $statis['rating_type']  =   '0';
                $statis['rating']       =   '0';
                
            }else{
                //会员到期，会员等级保留
                //修改会员等级
                require_once 'rating.model.php';
                $ratingM    =  new rating_model($this->db, $this->def);
                $rat_value  =  $ratingM -> ratingInfo($this->config['com_vip_done'], $uid);
                
                $this -> upInfo($rat_value,array('uid' => $uid, 'usertype' => $usertype));
                
                $this -> update_once('company_job', array('rating' => $this->config['com_vip_done']), array('uid' => $uid));
                $this -> update_once('company', array('rating' => $this->config['com_vip_done'],'rating_name' => $rat_value['rating_name'],'vipetime'=>$rat_value['vip_etime']), array('uid' => $uid));
            }
        }
        
        //会员未到期，永久会员（含过期会员）
        if(isVip($statis['vip_etime']) && !$vipover){
            
            if($statis['rating_type'] == '2'){//时间会员
                // 时间会员职位数同套餐会员的逻辑一致
                $zpjobnum   =   $this->select_num('company_job', array('uid' => $uid, 'status' => 0));
                $zppartnum  =   $this->select_num('partjob', array('uid' => $uid, 'status' => 0));
                $zpnum      =   $zpjobnum + $zppartnum;
                // 已上架数量超限的，发布前要进行提示
                if ($zpnum >= $statis['job_num']) {
                    $addjobnum  =   2;
                } else {
                    $addjobnum  =   1;
                }

            }else if($statis['rating_type'] == '1'){
                // 套餐会员，发布职位不需要扣除发布数量，需要查询上架职位数量，与套餐量进行对比20220326
                $zpjobnum  = $this->select_num('company_job', array('uid'=>$uid, 'status'=>0));
                $zppartnum = $this->select_num('partjob', array('uid'=>$uid, 'status'=>0));
                $zpnum     = $zpjobnum + $zppartnum;
                // 已上架数量超限的，发布前要进行提示
                if ($zpnum >= $statis['job_num']){
                    $addjobnum  =  2;
                }else{
                    $addjobnum  =  1;
                }
            }else{  //  过期
                
                $addjobnum      =  '0';
            }
        }else { //  过期
            
            $addjobnum          =  '0';
        }
        
        if($statis['rating'] > 0){
            if($statis['vip_etime'] && isVip($statis['vip_etime'])){

                $statis['days']     =  round(($statis['vip_etime']-time())/3600/24) ;
            }
        }
        $ratingInfo =   $this->select_once('company_rating', array('id' => $statis['rating']),'`freerefresh_num`');
        if ((int)$ratingInfo['freerefresh_num'] > 0){

            $freeArr        =   $this->select_all('job_refresh_log', array('uid' => $uid, 'free' => 1, 'r_time' => array('>=', strtotime('today'))), 'sum(free_num) as `num`');   //  今日免费刷新数目
            $freeNum        =   $freeArr[0]['num'];

            $statis['free_num']     =   intval($ratingInfo['freerefresh_num']) - $freeNum > 0 ? intval($ratingInfo['freerefresh_num']) - $freeNum : 0;
        }else{

            $statis['free_num']     =   0;
        }

        $statis['upJobNum']         =   $statis['breakjob_num']+$statis['free_num'];

        $statis['addjobnum']        =  $addjobnum;
        $statis['pay_format']       =  number_format($statis['pay'],2);
        $statis['integral_format']  =  number_format($statis['integral']);
        
        return $statis;
    }


    /**
     * @desc发布职位、上架职位，套餐处理
     * @param array $data
     * @return array
    */
    function getCom($data=array())
    {
        
        $uid        =   intval($data['uid']);
        $usertype   =   intval($data['usertype']);
        $jobnum   =   intval($data['jobnum'])?intval($data['jobnum']):1;
        $statis		=	$this -> getInfo($uid,array('usertype' =>$usertype));
        
        if ($statis['uid'] == '') {
            
            $statis =   $this->vipOver($uid, 2);
        }
        
        if ($statis['rating_type'] == '' && $statis['rating']) {
            
            $rating =   $this -> select_once('company_rating',array('id'=>$statis['rating']),'`type`');
            $this -> upInfo(array('rating_type' => $rating['type']), array('usertype' => $usertype, 'uid' => $uid));
            $statis['rating_type']  =   $rating['type'];
        }
        
        if ($statis['rating_type'] && $statis['rating']) {
            if (isVip($statis['vip_etime'])) {

                // 套餐会员，发布职位数限制的是企业上架的职位数量，不是企业发布的职位数量。 20220326
                // 发布职位不需要扣除发布数量，需要查询上架职位数量，与套餐量进行对比
                $zpjobnum  = $this->select_num('company_job', array('uid'=>$uid, 'status'=>0));
                $zppartnum = $this->select_num('partjob', array('uid'=>$uid, 'status'=>0));
                $zpnum      =   $zpjobnum + $zppartnum;
                // 待发布或待上架的数量加上现有上架职位数量，大于等于套餐量的，无法继续执行
                if (($jobnum + $zpnum) > $statis['job_num']){
                    return array('msg'=>'你的套餐已用完！','errcode' => 8);
                }
            } else {
                
                return array('msg'=>'你的套餐已用完！','errcode' => 8);
            }
        } else {
            
            return array('msg'=>'你的会员已经到期，请先购买会员！','errcode'=>8);
        }
    }

	/**
     * 计划任务：会员到期自动下架职位
     * @param array $data data['unit'] all  传入此值时，更新全部会员
     */
	function setVipedupjob($data = array())
    {

        $time       =   date('Y-m-d', strtotime('-7 day'));

        $endTime    =   strtotime($time . ' 23:59:59');

        $statisWhere['PHPYUNBTWSTART_A'] = '';

        if (isset($data['unit']) && $data['unit'] == 'all') {

            $statisWhere['vip_etime'][] = array('>', 0, 'AND');
        } else {

            $statisWhere['vip_etime'][] = array('>=', $endTime, 'AND');
        }
        
        $statisWhere['vip_etime'][]         =   array('<', strtotime('today'), 'AND');
        
        $statisWhere['PHPYUNBTWEND_A']      =   '';

		$num = $this -> select_num('company_statis', $statisWhere);
        
		if ($num>0){
			
			$comstatis = $this -> select_all('company_statis', $statisWhere ,'`uid`,`vip_etime`,`rating_name`');

			if(is_array($comstatis)){
				
				foreach($comstatis as $key=>$value){
					if($this->config['jobunder']==1){
						if (isset($this->config['job_under_delay']) && (int)$this->config['job_under_delay'] > 0){

							if ($value['vip_etime'] + (int)$this->config['job_under_delay'] * 86400 < strtotime(date('Y-m-d 23:59:59'))){

								$uid[] = $value['uid'];
							}
						}else{

							$uid[] = $value['uid'];
						}
					}
                }

                // 职位
                $where              =   array();
				$where['uid']		=	array('in',pylode(',', $uid));
				$where['status']	=	array('<>', 1);
                $where['r_status']  =   array('<>', 4);
				
				$jobnum = $this -> select_num('company_job', $where);

				for($i=1; $i<=ceil($jobnum/500); $i++){
					
                    $where['limit']	=	array((($i-1)*500),($i*500));
					$jobids = array();
					$joblist = $this -> select_all('company_job', $where, '`id`,`uid`,`name`');
					foreach($joblist as $k=>$v){

						$jobids[]=$v['id'];
                    }

					$this -> update_once('company_job', array('status' => 1), array('id'=>array('in', pylode(',',$jobids))));
				}
				// 兼职
				$where              =   array();
				$where['uid']		=	array('in',pylode(',', $uid));
				$where['status']	=	0;
				$where['r_status']  =   array('<>', 4);
				
				$jobnum = $this -> select_num('partjob', $where);
				
				for($j=1; $j<=ceil($jobnum/500); $j++){
				    
				    $where['limit']	=	array((($j-1)*500),($j*500));
				    $jobids = array();
				    $joblist = $this -> select_all('partjob', $where, '`id`');
				    foreach($joblist as $k=>$v){
				        $jobids[]=$v['id'];
				    }
				    $this -> update_once('partjob', array('status' => 1), array('id'=>array('in', pylode(',',$jobids))));
				}
			}
		}
    }
    
     
    /**
     * 计算金额
     */
    private function calculateMoney($payMoney, $payType){
        if($payType == 'integral'){
            $payMoney   =   bcmul($payMoney, $this -> config['integral_proportion'], 2);
        }
        return floatval($payMoney);
    }
    
    public function getStatisTotal($timeBegin, $timeEnd,$userType){
      
        //调用MODEL
        require_once ('companyorder.model.php');

        $CompanyorderM          =           new companyorder_model($this->db, $this->def);

        require_once ('pack.model.php');

        $PackM                  =           new pack_model($this->db, $this->def);
 
        if($userType){

            require_once ('userinfo.model.php');

            $UserinfoM                    =           new userinfo_model($this->db, $this->def);

            $memberwhere['usertype']      =           $userType;

            $member                       =           $UserinfoM->getList($memberwhere,array('field'=>'`uid`'));
          
            foreach($member  as $val){
                
                $uidArr[]              =           $val['uid'];

            }

            $uidStr 				    = 			implode(',', $uidArr);

            $where['uid']				=			array('in',pylode(',',$uidStr));
    
            $mwhere['uid']				=			array('in',pylode(',',$uidStr));
           

        }

        $where['order_state']   =           2;

        if($timeBegin !=''){

			$where['order_time'][]         		=   			array('>=', $timeBegin);
		
			$where['order_time'][]         		=           	array('<=', $timeEnd,'AND');
        }
       
        $field 			= 				'sum(order_price) as `num`';

        $row		    =				$CompanyorderM->getList($where,array('field'=>$field));

        $in             =               isset($row[0]['num']) && $row[0]['num'] > 0 ? $row[0]['num'] : 0;

        $mwhere['order_state']					=				1;
       
        if($timeBegin != ''){

			$mwhere['time'][]         			=   			array('>=', $timeBegin);
			
			$mwhere['time'][]         			=           	array('<=', $timeEnd,'AND');

        }
        
        $mfield 								= 				'sum(order_price) as `num`';

        $mrow									=				$PackM->getList($mwhere,array('field'=>$mfield));
        
        $out 									= 				isset($mrow[0]['num']) && $mrow[0]['num'] > 0 ? $mrow[0]['num'] : 0;

        $net_income 							= 				$in - $out;
        
        return array($in, $out, $net_income);
      
    }
    /**
     * 计划任务：会员到期自动变更会员等级
     * data['unit'] all  传入此值时，更新全部会员
     */
    function setViped()
    {

        $today = strtotime('today');
        
        $statisWhere['PHPYUNBTWSTART_A']    =   '';
        $statisWhere['vip_etime'][]         =   array('>', 0, 'AND');
        $statisWhere['vip_etime'][]         =   array('<',$today);
        $statisWhere['PHPYUNBTWEND_A']      =   '';
        $statisWhere['rating']              =   array('>', 0);

        $num    =   $this->select_num('company_statis', $statisWhere);

        if ($num > 0) {

            $comstatis  =   $this->select_all('company_statis', $statisWhere, '`uid`,`max_time`');

            if (is_array($comstatis)) {

                foreach ($comstatis as $value) {
                    $uid[]  =   $value['uid'];
                }

                // 查询暂停的企业,没有超过最大有效期的以及旧数据 不需要处理
                $cuid       =   array();
                $comList    =   $this->select_all('company', array('uid' => array('in', pylode(',', $uid))), '`uid`,`r_status`');

                foreach ($comList as $v) {
                    if ($v['r_status'] == 4) {

                        $cuid[] =   $v['uid'];
                    }
                }

                if (!empty($cuid)) {
                    foreach ($comstatis as $key => $value) {
                        if (in_array($value['uid'], $cuid) 
                            && (empty($value['max_time']) || ($value['max_time']>0 && $value['max_time']>$today)) 
                        ) {

                            unset($comstatis[$key]);
                        }
                    }
                }
                
                // 查询并清掉暂停的企业 end

                if (!empty($comstatis)) {

                    $euid       =   array();

                    foreach ($comstatis as $value) {
                        $euid[] =   $value['uid'];
                    }

                    // 提前获取要批量设置的会员等级
                    if ($this->config['com_vip_done'] != '0') {

                        $rating =   $this->select_once('company_rating', array('id' => $this->config['com_vip_done'], 'category' => 1));
                    }

                    $cnum   =   count($comstatis);

                    $limit  =   200;

                    $where  =   array('uid' => array('in', pylode(',', $euid)));

                    for ($i = 1; $i <= ceil($cnum / $limit); $i++) {

                        $where['limit'] =   array((($i - 1) * $limit), ($i * $limit));

                        $suids          =   array();
                        $ratingArr      =   array();
                        $statislist     =   $this->select_all('company_statis', $where, '`uid`,`rating_name`,`rating`');

                        foreach ($statislist as $val) {
                            $suids[]    =   $val['uid'];
                            if (!in_array($val['rating'], $ratingArr)){

                                $ratingArr[]    = array('rating' => $val['rating'], 'rating_name' => $val['rating_name']);
                            }
                        }

                        // 过期会员清空批量处理
                        if ($this->config['com_vip_done'] == '0') {

                            $data       =   array(
                                'job_num'       =>  '0',
                                'breakjob_num'  =>  '0',
                                'down_resume'   =>  '0',
                                'invite_resume' =>  '0',
                                'zph_num'       =>  '0',
                                'top_num'       =>  '0',
                                'rec_num'       =>  '0',
                                'urgent_num'    =>  '0',
                                'rating_name'   =>  '过期会员',
                                'rating_type'   =>  '0',
                                'rating'        =>  '0',
                                'suspend_num'   =>  '0',
                                'max_time'      =>  '0',
                            );

                            foreach ($ratingArr as $rk => $rv){

                                $data['oldrating']      =   $rv['rating'];
                                $data['oldrating_name'] =   $rv['rating_name'];

                                $this->update_once('company_statis', $data, array('rating' => $rv['rating'], 'uid' => array('in', pylode(',', $suids))));
                            }

                            $this->update_once('company', array('rating' => $data['rating'], 'rating_name' => $data['rating_name']), array('uid' => array('in', pylode(',', $suids))));

                            $upJobArr   =   array('rating' => 0);
                            //会员到期职位下架
                            if ($this->config['jobunder'] == '1') {

                                if (!isset($this->config['job_under_delay']) || empty($this->config['job_under_delay'])){

                                    $upJobArr['status'] = 1;
                                }
                            }
                            $this->update_once('company_job', $upJobArr, array('uid' => array('in', pylode(',', $suids))));
                        } elseif (!empty($rating)) {

                            $rat_value  =   array(
                                'job_num'       =>  $rating['job_num'],
                                'breakjob_num'  =>  $rating['breakjob_num'],
                                'down_resume'   =>  $rating['resume'],
                                'invite_resume' =>  $rating['interview'],
                                'zph_num'       =>  $rating['zph_num'],
                                'top_num'       =>  $rating['top_num'],
                                'rec_num'       =>  $rating['rec_num'],
                                'urgent_num'    =>  $rating['urgent_num'],
                                'rating_name'   =>  $rating['name'],
                                'rating_type'   =>  $rating['type'],
                                'rating'        =>   $rating['id'],
                                'vip_stime'     =>  time(),
                                'suspend_num'   =>  $rating['suspend_num'],
                                'max_time'      =>  strtotime("+ $rating[max_time] day"),
                            );

                            // 会员等级到期时间。服务有效时间为0的是永久，不需要到期时间
                            if ($rating['service_time'] > 0) {
                                $rat_value['vip_etime'] = time() + 86400 * $rating['service_time'];
                            } else {
                                $rat_value['vip_etime'] = 0;
                            }

                            foreach ($ratingArr as $rk => $rv) {

                                $rat_value['oldrating']     =   $rv['rating'];
                                $rat_value['oldrating_name']=   $rv['rating_name'];

                                $this->update_once('company_statis', $rat_value, array('rating' => $rv['rating'], 'uid' => array('in', pylode(',', $suids))));
                            }

                            $this->update_once('company_job', array('rating' => $rating['id']), array('uid' => array('in', pylode(',', $suids))));
                            $this->update_once('company', array('rating' => $rating['id'], 'rating_name' => $rating['name'],'vipetime'=>$rat_value['vip_etime']), array('uid' => array('in', pylode(',', $suids))));
                        }
                    }
                }
            }
        }
    }

    /**
     * @desc 套餐消耗明细
     * @param array $data
     */
    public function addStatisDetail($data = array())
    {

        $valData    =   array(
            'uid'   =>  $data['uid'],
            'type'  =>  $data['type'],
            'num'   =>  $data['num'],
            'detail'=>  $data['detail'],
            'time'  =>  time(),
            'uri'   =>  $data['uri'],
            'ip'    =>  fun_ip_get()
        );
        $this->insert_into('company_statis_detail', $valData);
    }

    public $typeN = array('1' => '上架|发布 职位', '2' => '刷新职位', '3' => '下载简历', '4' => '邀请面试', '5' => '职位推荐', '6' => '紧急招聘', '7' => '职位置顶', '8' => '招聘会报名', '10' => '直聊', '11' => '视频面试');

    public function getStatisDetail($where = array(), $data = array())
    {

        $field  =   $data['field'] ? $data['field'] : '*';
        $list   =   $this->select_all('company_statis_detail', $where, $field);

        foreach ($list as $k => $v){

            $list[$k]['type_n'] =   $this->typeN[$v['type']];
            $list[$k]['time_n'] =   date('Y-m-d H:i', $v['time']);
        }

        return $list;
    }

    public function delStatisDetail($id, $data = array('utype' => ''))
    {

        if (!empty($id)) {
            if (is_array($id)) {

                $ids = $id;
                $return['layertype'] = 1;
            } else {

                $ids = @explode(',', $id);
                $return['layertype'] = 0;
            }

            $result =   $this->delete_all('company_statis_detail', array('id' => array('in', pylode(',', $ids))), '');

            $return['msg']      =   '套餐记录(ID:'.$id.')';
            $return['errcode']  =   $result ? '9' : '8';
            $return['msg']      =   $result ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
        } else {

            $return['msg']      =   '请选择您要删除的记录！';
            $return['errcode']  =   8;
        }

        return $return;
    }
}
?>