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
class zph_model extends model{

    /**
	 * @desc   获取招聘会列表
	 * @param  $whereData:查询条件
	 * @param  $data:自定义处理数组
	 */
	public function getList($whereData , $data = array('field'=>null,'utype'=>null)) {

		$select		=	$data['field'] ? $data['field'] : '*';

		$List		=   $this -> select_all('zhaopinhui',$whereData,$select);

		$time  		=	time();

		if($List&&is_array($List)){

		    if ($data['utype'] == 'admin'){

		        foreach($List as $key => $val){

		            $zid[]	=	$val['id'];

		            $List[$key]['comnum']	=	'0';
		            $List[$key]['booking']	=	'0';

		        }

		        $all	=	$this -> getZphCompanyList(array('zid'=>array('in',pylode(',', $zid)),'groupby'=>'zid'),array('field'=>'zid,count(id) as num'));

		        $status	=	$this -> getZphCompanyList(array('zid'=>array('in',pylode(',', $zid)),'status'=>'0','groupby'=>'zid'),array('field'=>'`zid`,count(`id`) as num'));

		        foreach($List as $key => $v){

		            foreach($all as $val){

		                if($v['id'] == $val['zid']){
		                    $List[$key]['comnum']	=	$val['num'];
		                }
		            }
		            foreach($status as $val){

		                if($v['id'] == $val['zid']){
		                    $List[$key]['booking']	=	$val['num'];
		                }
		            }
		            if($v['did']<1){
		                $List[$key]['did']	=	'0';
		            }
		            
		        }
		    }
		    if ($data['utype'] == 'app'){

		    	foreach($List as $key => $val){

		            $zid[]	=	$val['id'];

		            $List[$key]['comnum']	=	0;
		            $List[$key]['jobnum']	=	0;

		        }

		    	$all	=	$this -> getZphCompanyList(array('zid'=>array('in',pylode(',', $zid)),'status'=>1),array('field'=>'zid,uid,jobid'));

                
		    	$arr_uid=	array();
		    	$job_ids=	array();
		    	foreach($all as $va){
                    
					$arr_uid[]	 =	$va['uid'];

					if($va['jobid']){

                        $job_ids = 	array_unique(array_merge($job_ids,@explode(",",$va['jobid'])));

                    }
                    
				}

		        $arr_uid  =  array_unique($arr_uid);

		        if(!empty($arr_uid)){
                    
                    $jobwhere	=	array(
                    	'uid'		=>	array('in',pylode(',',$arr_uid)),
                    	'state'		=>	1,
                    	'status'	=>	0,
                    	'r_status'	=>	1,
                    	'groupby'	=>	'uid'
                    );
                    //企业的所有职位
                    $joblist = $this->select_all("company_job",$jobwhere,"`uid`,count(*) as `num`,sum(`zp_num`) as `zpnum`");

                    $comalljobnum = array();

                    $comalljobUsernum = array();

                    foreach($joblist as $val){
                        
                        $comalljobnum[$val['uid']] = $val['num'];
                        $comalljobUsernum[$val['uid']] = $val['zpnum'];
                    }

                    $jobidwhere	=	array(
                    	'id'		=>	array('in',pylode(',',$job_ids)),
                    	'state'		=>	1,
                    	'status'	=>	0,
                    	'r_status'	=>	1,
                    );
                    //企业参会的所有职位
                    $jobidlist = $this->select_all("company_job",$jobidwhere,"`id`,`zp_num`");

                    $jidarr =   array();
                    $jusernumarr=	array();
                    foreach($jobidlist as $jidv){
                        $jidarr[] = $jidv['id'];
                        $jusernumarr[$jidv['id']] = $jidv['zp_num'];
                    }

                    foreach($all as $k=>$v){

                        $all[$k]['jobnum'] = 0;
                        $all[$k]['zpnum'] = 0;
                        if($v["jobid"]){

                            $jobidarr = @explode(",",$v["jobid"]);

                            foreach($jobidarr as $jv){

                                if(in_array($jv,$jidarr)){
                                	$zp_num = !empty($jusernumarr[$jv])?$jusernumarr[$jv]:0;
                                    $all[$k]['jobnum']++;
                                    $all[$k]['zpnum']+=$zp_num;
                                }

                            }

                        }else{

                            $all[$k]['jobnum']  =  	$comalljobnum[$v['uid']] ? $comalljobnum[$v['uid']] : 0; 
                            $all[$k]['zpnum']	=	$comalljobUsernum[$v['uid']] ? $comalljobUsernum[$v['uid']] : 0;

                        }
                    }
                }
                
		        foreach($List as $key => $val){
		        	$List[$key]['comnum']	  =  0;
		        	$List[$key]['jobnum']     =  0;
		        	$List[$key]['zpnum']	  =  0;
		        	$List[$key]["zphtype"] 	  =  'zph';
		            $List[$key]['wapurl']     =  Url('wap', array('c'=>'zph','a'=>'show','id'=>$val['id']));
		            $List[$key]['starttime_timestamp']  =  strtotime($val['starttime']);
                    $List[$key]['endtime_timestamp']  	=  strtotime($val['endtime']);
		            $List[$key]['starttime']  =  date('Y-m-d',strtotime($val['starttime']));
		            $List[$key]['endtime']    =  date('Y-m-d',strtotime($val['endtime']));
		            $List[$key]['stime']      =  strtotime($val['starttime'])-$time;
		            $List[$key]['etime']      =  strtotime($val['endtime'])-$time;
		            $List[$key]['is_themb_n'] =  checkpic($val['is_themb'],$this->config['sy_zph_icon']);
		            $List[$key]['is_themb_wap_n'] =  checkpic($val['is_themb_wap'],$List[$key]['is_themb_n']);
					$List[$key]['banner_n']  =  checkpic($val['banner'], $this->config['sy_zphbanner_icon']);
                    $List[$key]['banner_wap_n']= checkpic($val['banner_wap'],$List[$key]['banner_n']);
                    $List[$key]['pic_n']      =  $List[$key]['is_themb_wap_n'];
                    
		            if (!empty($all)){
		                
		                foreach($all as $aval){
		                    if($val['id'] == $aval['zid']){
		                        $List[$key]['comnum']++;
		                        $List[$key]['jobnum']+=$aval['jobnum'];
		                        $List[$key]['zpnum']+=$aval['zpnum'];
		                    }
		                }
		            }
		        }
		    }
			if (empty($data['utype'])) {
                foreach($List as $key => $val){
                    $List[$key]['is_themb_n'] =  checkpic($val['is_themb'],$this->config['sy_zph_icon']);
                    $List[$key]['is_themb_wap_n'] =  checkpic($val['is_themb_wap'],$List[$key]['is_themb_n']);
                }
            }
            if ($data['utype'] == 'admin') {
                foreach($List as $key => $val){
                    $List[$key]['url']     =  Url('zph', array('c'=>'show','id'=>$val['id']));
                    $List[$key]['banner_n']   =  checkpic($val['banner'], $this->config['sy_zphbanner_icon']);
                    $List[$key]['banner_wap_n']= checkpic($val['banner_wap'],$List[$key]['banner_n']);
                    $List[$key]['is_themb_n'] =  checkpic($val['is_themb'],$this->config['sy_zph_icon']);
                    $List[$key]['is_themb_wap_n'] =  checkpic($val['is_themb_wap'],$List[$key]['is_themb_n']);
                }
            }
		}
		return $List;
	}
	/**
     * 招聘会与网络招聘会合并 只显示未过期数据
     * @param null[] $data
     */
    public function getZphList($data = ['field' => null, 'utype' => null])
    {
        $select    = $data['field'] ? $data['field'] : '*';
        $time      = date("Y-m-d H:i:s", time());
        $whereData = [
            'endtime' => ['>=', $time], //查询未过期招聘会
            'is_open' => 1,
            'orderby' => ['endtime, asc'],
        ];

        $zphList   = $this->select_all('zhaopinhui', $whereData, $select); //现场招聘会

        $zphIds = array();

        if ($zphList) {
            foreach ($zphList as $key => $value) {
                $zphIds[] = $value["id"];
            }
            $zphStatisticsData = $this->getZphStatistics($zphIds, 'zph');
            $jobNum            = $zpNum = [];
            foreach ($zphList as $key => $val) {
                $zphList[$key]['jobnum']  = 0;
                $zphList[$key]['comnum']  = 0;
                $zphList[$key]['zpnum']   = 0;
                $zphList[$key]['usernum'] = 0; // 直播招聘会未记录查看人数 默认为0
                if (isset($zphStatisticsData[$val['id']])) {
                    foreach ($zphStatisticsData[$val['id']] as $k => $v) {
                        $jobNum[$val['id']][] = $v['jobnum'];
                        $zpNum[$val['id']][]  = $v['zpnum'];
                    }
                    $zphList[$key]['jobnum'] = array_sum($jobNum[$val['id']]);
                    $zphList[$key]['zpnum']  = array_sum($zpNum[$val['id']]);
                    $zphList[$key]['comnum'] = count($zphStatisticsData[$val['id']]);
                }
                $zphList[$key]['cover']     = checkpic($val['is_themb'], $this->config['sy_zph_icon']);
                $zphList[$key]['cover_wap'] = checkpic($val['is_themb_wap'], $this->config['sy_zph_icon']);
                $zphList[$key]['url']       = Url('zph',array('c'=>'show','id'=>$val['id']));
                $zphList[$key]['url_wap'] = "index.php?c=zph&a=show&id=".$val['id'];
            }
        }

        $endtime = $starttime = [];
        $result = $zphList;
        if ($result) {
            foreach ($result as $key => $value) {
                $result[$key]['starttime'] = date('Y-m-d H:i', strtotime($value['starttime']));
                $result[$key]['endtime']   = date('Y-m-d H:i', strtotime($value['endtime']));
                $result[$key]['stime']     = strtotime($value['starttime']) - time();
                $result[$key]['etime']     = strtotime($value['endtime']) - time();
                $endtime[$key]             = $value['endtime'];
                $starttime[$key]           = $value['starttime'];
            }
            //合并招聘会与网络招聘会 按照技术日期排序
            array_multisort($starttime, SORT_ASC, $result);
        }
        
        return $result;
    }

    /**
     * 获取正在【进行中/未开始】的【招聘会/网络招聘会】
     * @return array|false|mixed|string|void
     */
    public function getNewZph()
    {

        $zpInfo     =   array();
        $whereData  =   [
            'endtime'   =>  array('unixtime', '>=', time()),
            'is_open'   =>  1,
            'orderby'   =>  ['unix_timestamp(`starttime`), asc']
        ];

        $data       =   $this->select_all('zhaopinhui', $whereData, "*");



        foreach ($data as $dk => $dv) {
            if (strtotime($dv['starttime']) <= time() && empty($zpInfo)){   //  正在进行的现场招聘会

                $zpInfo     =   $dv;
            }
        }
        if (empty($zpInfo)) {
            foreach ($data as $dk => $dv) {
                if (strtotime($dv['starttime']) > time() && empty($zpInfo)) {   //  即将开始的现场招聘会

                    $zpInfo = $dv;
                }
            }
        }

        if (!empty($zpInfo)){
            $zpInfo['cover']    =   checkpic($zpInfo['banner'],$this->config['sy_zphbanner_icon']);
            $zpInfo['cover_wap']=   checkpic($zpInfo['banner_wap'],$zpInfo['cover']);
            $zpInfo['url']      =   "index.php?m=zph&c=show&id=" . $zpInfo['id'];
            $zpInfo['url_wap']  =   "index.php?c=zph&a=show&id=" . $zpInfo['id'];
        }
        return $zpInfo;
    }

    public function getZphStatistics($zIds, $type = 'zph')
    {
        $where = array(
            'zid'    => array('in', pylode(',', $zIds)),
            'status' => 1
        );


        $zphComList = $this->select_all("zhaopinhui_com", $where, 'zid,uid,jobid');


        $companyIds = [];
        $jobids	= $jobuid = array();
        foreach ($zphComList as $comk => $comv) {
            $companyIds[] = $comv["uid"];

            if($comv['jobid']){

                $jobarr 	=	@explode(',',$comv['jobid']);
                $jobids		=	array_merge($jobids,$jobarr);

            }else{

                if(!in_array($comv['uid'] , $jobuid)){
                    $jobuid[]	=	$comv['uid'];
                }
            }
        }
        $companyIds = array_unique($companyIds);
        if (!empty($companyIds)) {
            $newCompanyIds = [];
            $companyList   = $this->select_all("company", array('uid' => array('in', pylode(',', $companyIds)), 'r_status' => 1), "`uid`");
            foreach ($companyList as $key => $value) {
                $newCompanyIds[] = $value['uid'];
            }
            foreach ($zphComList as $ck => $cv) {
                // 过滤未审核的企业状态
                if (!in_array($cv['uid'], $newCompanyIds)) {
                    unset($zphComList[$ck]);
                }
            }

            $comJobWhere = array(
                'state'    => 1,
                'r_status' => 1,
                'status'   => 0,
            );
            if(!empty($jobuid)){

                $comJobWhere['PHPYUNBTWSTART'] = 1;
                $comJobWhere['uid']	=	array('in',pylode(',',$jobuid));
                $comJobWhere['id']	    =	array('in',pylode(',',$jobids),'OR');
                $comJobWhere['PHPYUNBTWEND'] = 1;
            }else{
                $comJobWhere['id']	    =	array('in',pylode(',',$jobids));
            }
            // 筛选企业已审核已通过职位
            $comJobList = $this->select_all("company_job", $comJobWhere, "`id`, uid,zp_num");
            $jobTemp    = array();
            foreach ($comJobList as $key => $jidv) {
                $jobTemp[$jidv['uid']]['id'][$key] = $jidv['id'];
                if($jobTemp[$jidv['uid']]['zp_num']){
                    $jobTemp[$jidv['uid']]['zp_num'] += $jidv['zp_num'];
                }else {
                    $jobTemp[$jidv['uid']]['zp_num'] = $jidv['zp_num'];
                }
            }
        }

        $newCompList = [];
        foreach ($zphComList as $key => $value) {
            $zphComList[$key]['jobnum']       = isset($jobTemp[$value['uid']]) ? count($jobTemp[$value['uid']]['id']) : 0;
            $zphComList[$key]['zpnum']        = isset($jobTemp[$value['uid']]) ? $jobTemp[$value['uid']]['zp_num'] : 0;
            $newCompList[$value['zid']][$key] = $zphComList[$key];
        }

        return $newCompList;
    }
	/**
	* @desc 获取招聘会列表详细信息
	*/
	public function getInfo($whereData,$data = array()){
        $select     =   $data['field'] ? $data['field'] : '*';

        $Info       =   $this -> select_once('zhaopinhui', $whereData, $select);

        if (!empty($Info)) {

            if ($Info['starttime']) {
                $Info["stime"]          =   strtotime($Info['starttime']) - time();
                $Info['starttime_n']    =   date('Y-m-d H:i', strtotime($Info['starttime']));
            }

            if ($Info['endtime']) {
                $Info['etime']          =   strtotime($Info['endtime']) - time();
                $Info['endtime_n']      =   date('Y-m-d H:i', strtotime($Info['endtime']));
            }

            if ($Info['reserved']) {
                $Info['reserved_n']     =   @explode(',', $Info['reserved']);
            }
            if (!empty($Info['body'])){

                $Info['body']  =  $this->publicHtmlChar($Info['body']);
            }
            if (!empty($Info['media'])){

                $Info['media']  =  $this->publicHtmlChar($Info['media']);
            }
            if (!empty($Info['packages'])){

                $Info['packages']  =  $this->publicHtmlChar($Info['packages']);
            }
            if (!empty($Info['booth'])){

                $Info['booth']  =  $this->publicHtmlChar($Info['booth']);
            }
            if (!empty($Info['participate'])){

                $Info['participate']  =  $this->publicHtmlChar($Info['participate']);
            }
            if (!empty($data['pic'])) { // 缩略图
                $Info['is_themb_n']  =  checkpic($Info['is_themb'], $this->config['sy_zph_icon']);
                $Info['is_themb_wap_n']  =  checkpic($Info['is_themb_wap'], $Info['is_themb_n']);
            }

            if (!empty($data['banner'])) { // 横幅
                $Info['banner_n']  =  checkpic($Info['banner'], $this->config['sy_zphbanner_icon']);
                $Info['banner_wap_n']  =  checkpic($Info['banner_wap'],$Info['banner_n']);
            }

			$Info['comnum']		=	$this->getZphComNum(array('zid'=>$Info['id'],'status'=>1));

			$com		=	$this->select_all('zhaopinhui_com',array('zid'=>$Info['id'],'status'=>1));

			$jobnum		=	0;
            $zpnum      =   0;
            $sqnum      =   0;
			$job_ids	=	array();
            $zcuid_arr  =   array();

			foreach($com as $key=>$val){

                if($val['uid'] && !in_array($val['uid'],$zcuid_arr)){

                    $zcuid_arr[] =  $val['uid'];

                }
                
                if($val['jobid']){

					$job_ids = 	array_unique(array_merge($job_ids,@explode(",",$val['jobid'])));

				}else{
					$uids[]	=	$val['uid'];
				}

			}
			if(!empty($job_ids)){
				$jobidwhere	=	array(
	            	'id'		=>	array('in',pylode(',',$job_ids)),
	            	'state'		=>	1,
	            	'status'	=>	0,
	            	'r_status'	=>	1,
	            );
	            $jobid_count = $this->select_all("company_job",$jobidwhere,"count(*) as `num`,sum(`zp_num`) as `zpnum`");
			}
			$jobnum = !empty($jobid_count[0]['num'])?$jobid_count[0]['num']:0;
            $zpnum = !empty($jobid_count[0]['zpnum'])?$jobid_count[0]['zpnum']:0;
			if(!empty($uids)){
				$jobwhere	=	array(
					'uid'		=>	array('in',pylode(',',$uids)),
					'state'		=>	1,
					'status'	=>	0,
					'r_status'	=>	array('<>',2),
				);
            
				$job_count = $this->select_all("company_job",$jobwhere,"count(*) as `num`,sum(`zp_num`) as `zpnum`");
			}
			$allchoosejobnum = !empty($job_count[0]['num'])?$job_count[0]['num']:0;
			$allchoosezpnum = !empty($job_count[0]['zpnum'])?$job_count[0]['zpnum']:0;
			$Info['jobnum']	 =	$jobnum+$allchoosejobnum;

            $Info['zpnum']   =  $zpnum+$allchoosezpnum;

            if(!empty($zcuid_arr)){

                $sqWhere = array(
                    'com_id'    =>  array('in',pylode(',',$zcuid_arr)),
                    'datetime'  =>  array('>',$Info['ctime'])
                );

                $sqnum  = $this->select_num('userid_job',$sqWhere);
            }

            $Info['sqnum']   =  $sqnum;



		}
        return $Info;
    }

    public function publicHtmlChar($key){
    	$info  =  str_replace(array('&quot;','&nbsp;','<>'), array('','',''), $key);
        $info  =  htmlspecialchars_decode($info);

        preg_match_all('/<img(.*?)src=("|\'|\s)?(.*?)(?="|\'|\s)/',$info,$res);
        if(!empty($res[3])){
            foreach($res[3] as $v){
                if(strpos($v,'http:')===false && strpos($v,'https:')===false){
                    $info  =  str_replace($v,$this->config['sy_ossurl'].$v,$info);
                }
            }
        }
        return $info;
    }
	/**
	* @desc 添加招聘会
	*/
	public function addInfo($data	=	array()){

		$AddData	=	array(

			'title'			=>	$data['title'],

			'sid'			=>	$data['sid'],

			'address'		=>	$data['address'],

			'traffic'		=>	$data['traffic'],

			'phone'			=>	$data['phone'],

			'organizers'	=>	$data['organizers'],

			'user'			=>	$data['user'],

			'starttime'		=>	$data['starttime'],

			'endtime'		=>	$data['endtime'],

			'body'			=>	$data['body'],

			'media'			=>	$data['media'],

			'packages'		=>	$data['packages'],

			'booth'			=>	$data['booth'],

			'participate'	=>	$data['participate'],

			'ctime'			=>	time(),

			'status'		=>'0',

			'did'	        =>	$data['did'],

			'reserved'		=>	$data['reserved'],
			'is_open'		=>	$data['is_open'],
			'is_themb'		=>	$data['is_themb'],
			'banner'		=>	$data['banner'],
			'is_themb_wap'	=>	$data['is_themb_wap'],
			'banner_wap'	=>	$data['banner_wap'],

            // 'sort'          =>  $data['sort']
		);
		
		if ($AddData && is_array($AddData)){

			$nid	=	$this -> insert_into('zhaopinhui',$AddData);

		}

		return $nid;

	}

	/**
	* @desc 修改招聘会
	*/
	public function upInfo($whereData, $data = array()){

		if(!empty($whereData)) {

			$PostData	=	array(

				'title'			=>	$data['title'],

				'sid'			=>	$data['sid'],

				'address'		=>	$data['address'],

				'traffic'		=>	$data['traffic'],

				'phone'			=>	$data['phone'],

				'organizers'	=>	$data['organizers'],

				'user'			=>	$data['user'],

				'starttime'		=>	$data['starttime'],

				'endtime'		=>	$data['endtime'],

				'body'			=>	$data['body'],

				'media'			=>	$data['media'],

				'packages'		=>	$data['packages'],

				'booth'			=>	$data['booth'],

				'participate'	=>	$data['participate'],

				'did'			=>	$data['did'],

				'reserved'		=>	$data['reserved'],

				'is_open'		=>	$data['is_open'],
				'is_themb'		=>	$data['is_themb'],
				'banner'		=>	$data['banner'],
				'is_themb_wap'	=>	$data['is_themb_wap'],
				'banner_wap'	=>	$data['banner_wap']

                // 'sort'          =>  $data['sort']
			);
			
			if ($PostData && is_array($PostData)){

				$nid	=	$this -> update_once('zhaopinhui',$PostData,array('id'=>$whereData['id']));

			}

			return $nid;

		}

	}

	/**
	* @desc 删除招聘会
	*/
	public function delZph($delId){

        if (empty($delId)) {

            $return     =   array( 'errcode' => 8, 'msg' => '请选择要删除的数据！');

        } else {

            if (is_array($delId)) {

                $delId  =   pylode(',', $delId);

                $return['layertype']    =   1;

            } else {

                $return['layertype']    =   0;
            }

            $delid      =   $this -> delete_all('zhaopinhui', array('id' => array('in', $delId)), '');

            if ($delid) {

                $this -> delZphPic(array( 'zid' => array( 'in', $delId)));

                $this -> delete_all('zhaopinhui_com', array('zid' => array('in', $delId)), '');

                $return['msg']      =   '招聘会';

                $return['errcode']  =   $delid ? '9' : '8';

                $return['msg']      =   $delid ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
            }

        }

        return $return;
    }
    /**
    * @desc 获取参会职位列表
    */
    public function getZphJobList($whereData , $data=array()){

        $zselect     =   $data['zfield'] ? $data['zfield'] : '*';
        $zwhereData =   $whereData['zwhereData'];

        $List       =   $this -> select_all('zhaopinhui_com' , $zwhereData , $zselect);

        $return    =   array('list'=>array(),'zjnum'=>0);
        
        if($List && is_array($List)){

            include_once('job.model.php');
            $jobM  =  new job_model($this->db, $this->def);

            $jobids = $jobuid = $uid  =  $zid  =  array();

            foreach($List as $v){

                if($v['uid'] && !in_array($v['uid'] , $uid)){
                    $uid[]  =  $v['uid'];
                }

                if($v['zid'] && !in_array($v['zid'],$zid)){
                    $zid[]  =  $v['zid'];
                }

                if($v['jobid']){

                    $jobarr     =   @explode(',',$v['jobid']);
                    $jobids     =   array_merge($jobids,$jobarr);

                }else{

                    if(!in_array($v['uid'] , $jobuid)){
                        $jobuid[]   =   $v['uid'];
                    }
                }
            }

            $company  =  $this -> select_all('company', array('uid'=>array('in',pylode(',',$uid))),'`uid`,`name`,`logo`,`logo_status`');

            $jwhereData =   $whereData['jwhereData'];

            
            $jobwhere['PHPYUNBTWSTART'] = 1;
            $jobwhere['uid']    =   array('in',pylode(',',$jobuid));
            $jobwhere['id'] =   array('in',pylode(',',$jobids),'OR');
            $jobwhere['PHPYUNBTWEND'] = 1;

            $jobwhere = array_merge($jobwhere,$jwhereData);

            

            $jobsdata = $jobM->getList($jobwhere,array('isurl'=>'yes','field'=>'`id`,`uid`,`com_name`,`name`,`minsalary`,`maxsalary`,`urgent`,`urgent_time`,`provinceid`,`cityid`,`three_cityid`,`exp`,`edu`,`welfare`'));
            $joblist    = $jobsdata['list'];

            unset($jobwhere['limit']);
            $zjnum = $jobM->getJobNum($jobwhere);

            $return['list'] = $joblist;
            $return['zjnum'] = $zjnum;
            
        }
        return $return;
    }
	/**
	* @desc 获取参会企业列表
	*/
	public function getZphCompanyList($whereData , $data=array()){

		$select		=	$data['field'] ? $data['field'] : '*';

		$List		=	$this -> select_all('zhaopinhui_com' , $whereData , $select);

		if($List && is_array($List)){

			include_once('job.model.php');
            $jobM  =  new job_model($this->db, $this->def);

            include_once('company.model.php');
            $comM  =  new company_model($this->db, $this->def);

		    $jobids = $jobuid = $uid  =  $zid  =  array();

			foreach($List as $v){

				if($v['uid'] && !in_array($v['uid'] , $uid)){
					$uid[]  =  $v['uid'];
				}

				if($v['zid'] && !in_array($v['zid'],$zid)){
					$zid[]  =  $v['zid'];
				}

				if($v['jobid']){

					$jobarr 	=	@explode(',',$v['jobid']);
					$jobids		=	array_merge($jobids,$jobarr);

				}else{

					if(!in_array($v['uid'] , $jobuid)){
						$jobuid[]	=	$v['uid'];
					}
				}
			}

			$comdata  =  $comM->getList(array('uid'=>array('in',pylode(',',$uid))),array('url'=>'1','field'=>'`uid`,`name`,`logo`,`logo_status`,`hy`,`pr`,`mun`,`linkmail`','logo'=>'1'));
            $company  =  $comdata['list'];
			
			$jobwhere =	 array(
				'state' 	=>	1,
				'status' 	=>	0,
				'r_status'	=>	1
			);
			$jobwhere['PHPYUNBTWSTART'] = 1;
			$jobwhere['uid']	=	array('in',pylode(',',$jobuid));
			$jobwhere['id']	=	array('in',pylode(',',$jobids),'OR');
			$jobwhere['PHPYUNBTWEND'] = 1;
			$jobsdata = $jobM->getList($jobwhere,array('isurl'=>'yes','field'=>'`id`,`uid`,`name`,`minsalary`,`maxsalary`,`urgent`,`urgent_time`'));
            $listA    = $jobsdata['list'];
			
			$zph	  =  $this -> select_all('zhaopinhui',array('id'=>array('in',pylode(',',$zid))),'`id`,`title`,`address`,`starttime`,`endtime`');

			$space	  =  $this -> getZphSpaceList("zhaopinhui_space");

			foreach($space as $val){
				$spacename[$val['id']]	=	$val['name'];
			}
			foreach($List as $k => $v){
			    $List[$k]['wapurl'] = Url('wap', array('c'=>'zph','a'=>'show','id'=>$v['zid']));
			    
				foreach($zph as $val){

					if($v['zid'] == $val['id']){
						$List[$k]['zphname']	=	$val['title'];
						$List[$k]['title']		=	$val['title'];
						$List[$k]['address']	=	$val['address'];
						$List[$k]['starttime']	=	$val['starttime'];
						$List[$k]['endtime']	=	$val['endtime'];
						if(strtotime($val['starttime'])>time()){
							$List[$k]['notstart']	=	1;
						}
					}
				}
				if ($spacename[$v['sid']]){
				    $List[$k]['sidname']        =   $spacename[$v['sid']];
				}
				if ($spacename[$v['cid']]){
				    $List[$k]['cidname']        =   $spacename[$v['cid']];
				}
				if ($spacename[$v['bid']]){
				    $List[$k]['bidname']	    =	$spacename[$v['bid']];
				}

				foreach($company as $val){

					if($v['uid'] == $val['uid']){
						$List[$k]['comname']=	$val['name'];
                        $List[$k]['logo']   =   $val['logo'];
                        $List[$k]['hy_n']   =   $val['hy_n'];
                        $List[$k]['pr_n']   =   $val['pr_n'];
                        $List[$k]['mun_n']  =   $val['mun_n'];
                        $List[$k]['comwapurl'] =   $val['wapurl'];
						$List[$k]['linkmail']  =   $val['linkmail'];
						if($v['status']!=1){
							//控制取消按钮
							$List[$k]['notstart']	=	1;
						}
					}
				}

 				$jobname	=	array();
 				$jobid		=	array();
 				$jidarr		=	array();
 				if($v['jobid']){
 					$jidarr	=	@explode(',',$v['jobid']);
 				}

				foreach($listA as $val){

					if($v['jobid']){

						if(in_array($val['id'],$jidarr)){
							// 控制职位显示数量
					        if (count($jobname) < 20){
					            $List[$k]['job'][]		=	$val;
					            $jobname[]		    	= 	$val['name'];
					            $List[$k]['jobname']    =	@implode(",",$jobname);

					            $jobid[]				= 	$val['id'];
					            $List[$k]['jobid']    	=	@implode(",",$jobid);
					        }
						}

					}else{

						if ($v['uid'] == $val['uid']) {
	                        // 控制职位显示数量
					        if (count($jobname) < 20){
					            $List[$k]['job'][]		=	$val;
					            $jobname[]		    	= 	$val['name'];
					            $List[$k]['jobname']    =	@implode(",",$jobname);

					            $jobid[]				= 	$val['id'];
					            $List[$k]['jobid']    	=	@implode(",",$jobid);
					        }
					    }

					}

				}
				$List[$k]['bmctime_n']	=	date('Y-m-d',$v['ctime']);
			}
		}
		return $List;
	}

	/**
	* @desc 获取参会企业详细信息
	*/
	public function getZphComInfo($whereData,$data	=	array()){

		$select		=	$data['field'] ? $data['field'] : '*';

		$ZComInfo	=	$this -> select_once('zhaopinhui_com', $whereData, $select);

		return $ZComInfo;

	}
	/**
	* @desc 获取参会企业数量
	*/
	public function getZphComNum($whereData=array()){

		return $this -> select_num('zhaopinhui_com', $whereData);

	}

	/**
	* @desc 添加参会企业
	*/
	public function addZCom($data	=	array()){

		$AddData	=	array(

			'uid'		=>	$data['comid'],

			'zid'		=>	$data['zphid'],

			'ctime'		=>	time(),

			'status'	=>	isset($data['status']) ? $data['status'] : 1 ,

			'sid'		=>	$data['sid'],

			'cid'		=>	$data['cid'],

			'bid'		=>	$data['bid'],
            
		    'jobid'     =>  $data['jobid']
		);

		if ($AddData && is_array($AddData)){

			$nid	=	$this -> insert_into('zhaopinhui_com',$AddData);

		}

		return $nid;

	}

	/**
	 * @desc 获取招聘会场地
	 */
	public function getZphSpaceList($whereData="" , $data=array()) {

        $select     =   $data['field'] ? $data['field'] : '*';

        $List       =   $this -> select_all('zhaopinhui_space', $whereData, $select);
        if ($data['utype'] == 'admin') {
            foreach ($List as $k => $v) {
                $List[$k]['pic_n'] = $v['pic'] ? checkpic($v['pic']) : '';
            }
        }

        if ($data['utype'] == 'index') {

            if (is_array($List)) {

                foreach ($List as $v) {
                    $keyid[] = $v['id'];
                }

                $keyid          =   pylode(',', $keyid);
                $spacelistall   =   $this -> getZphSpaceList(array('keyid' => array('in', $keyid), 'orderby' => 'sort,asc'));

                if (!empty($data['id'])){
                    // 查询后台设定的不可预订展位
                    $zph  =  $this->getInfo(array('id'=>$data['id']),array('field'=>'reserved'));

                    if (!empty($zph)){

                        $reserved  =  explode(',', $zph['reserved']);
                    }

                    $comlist        =   $this -> getZphCompanyList(array('zid' => $data['id']));

                    if (is_array($comlist)) {

                        foreach ($comlist as $val) {
                            $uids[]     =   $val['uid'];
                        }

                        $companylist    =   $this -> select_all('company', array('uid' => array('in', pylode(',', $uids))), '`uid`,`name`,`shortname`');

                        foreach ($comlist as $k => $v) {
                            foreach ($companylist as $val) {
                                if ($v['uid'] == $val['uid']) {
                                    if ($val['shortname']) {
                                        $comlist[$k]['name']    =   $val['shortname'];
                                    } else {
                                        $comlist[$k]['name']    =   $val['name'];
                                    }
                                }
                            }
                        }
                        foreach ($spacelistall as $k => $v) {
                            $spacelistall[$k]['comstatus']      =   '-1';

                            if ($v['price'] > 0){
                                if ($this->config['com_integral_online'] == 3 && !in_array('zph', explode(',', $this->config['sy_only_price']))){
                                    $spacelistall[$k]['price_n']    =   $v['price'];
                                    $spacelistall[$k]['unit']       =   $this->config['integral_priceunit'].$this->config['integral_pricename'];
                                }else{
                                    $spacelistall[$k]['price_n']    =   $v['price']/$this->config['integral_proportion'];
                                    $spacelistall[$k]['unit']       =   '元';
                                }
                            }else{
                                $spacelistall[$k]['price_n']    =   '免费';
                            }
                            if (!empty($reserved) && in_array($v['id'], $reserved)){
                                $spacelistall[$k]['comstatus']  =   3;
                            }
                            foreach ($comlist as $val) {
                                if ($v['id'] == $val['bid']) {
                                    $spacelistall[$k]['comstatus']  =   $val['status'];
                                    $spacelistall[$k]['uid']        =   $val['uid'];
                                    $spacelistall[$k]['comname']    =   $val['name'];
                                    $spacelistall[$k]['joblist']    =   $val['joblist'];
                                }
                            }
                        }
                    }
                    foreach ($List as $k => $v) {
                        foreach ($spacelistall as $val) {
                            if ($v['id'] == $val['keyid']) {
                                $List[$k]['list'][] = $val;
                            }
                        }
                    }
                }
            }
        }
        return $List;
    }

	/**
	* @desc 添加招聘会场地
	*/
	public function addZphSpaceInfo($data = array()){


	    $name  =   array();

	    foreach ($data['name'] as $val){
	        if ($val) {
	            $name[]    =   $val;
	        }
	    }

	    $addData   =   array();

	    foreach ($name as $k => $v){
	        $addData[$k]['keyid']      =   $data['keyid'];
	        $addData[$k]['price']      =   $data['price'];
	        $addData[$k]['name']       =   $v;
	        $addData[$k]['sort']       =   $data['sort'];
	        $addData[$k]['content']    =   $data['content'];
	        if($data['pic']){
	            $addData[$k]['pic']	   =   $data['pic'];
	        }
	    }

	    $nid   =   $this->DB_insert_multi('zhaopinhui_space', $addData);

		return $nid;

	}

	/**
	* @desc 修改招聘会场地
	*/
	public function upZphSpaceInfo($whereData, $data = array()){

		if(!empty($whereData)) {

			$PostData	=	array(

				'keyid'		=>	$data['keyid'],

				'price'		=>	$data['price'],

				'name'		=>	$data['name'],

				'sort'		=>	$data['sort'],

				'content'	=>	$data['content'],

			);
			if($data['pic']){
				$PostData['pic']	=	$data['pic'];
			}
			if ($PostData && is_array($PostData)){

				$nid	=	$this -> update_once('zhaopinhui_space',$PostData,array('id'=>$whereData['id']));

			}

			return $nid;

		}

	}

	function upZphSpaceInfos($whereData, $data = array()){

		if(!empty($whereData)){

			$nid	=	$this -> update_once('zhaopinhui_space',$data,$whereData);

		}

		return $nid;

	}

	/**
	* @desc 删除招聘会场地
	*/
	public function delZphSpace($delId){

		if(empty($delId)){

			return	array(

			  'errcode' => 8,

			  'msg' 	=> '请选择要删除的数据！',
            );

		}else{
			if(is_array($delId)){

				$delId	=	pylode(',',$delId);

				$return['layertype']	=	1;

			}else{

				$return['layertype']	=	0;
			}

			$delid	=	$this -> delete_all('zhaopinhui_space',array('id' => array('in',$delId), 'keyid' => array('in', $delId, 'OR')),'');

			if($delid){

				$return['msg']		=	'招聘会场地';

				$return['errcode']	=	$delid ? '9' :'8';

				$return['msg']		=	$delid ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';

			}

		}

		return $return;

	}

	/**
	 * @desc 获取招聘会场地详细信息
	 */
	public function getZphSpaceInfo($whereData , $data = array('field'=>null)){

	    $select =   $data['field'] ? $data['field'] : '*';

        $Info   =   $this -> select_once('zhaopinhui_space', $whereData, $select);

        if (!empty($Info)) {

            if ($Info['content']) {

                $subject            =   strip_tags($Info['content']); // 去除html标签
                $pattern            =   '/\s/'; // 去除空白
                $Info['content_n']  =   preg_replace($pattern, '', $subject);
            }

            if (!empty($data['pic'])) { // 缩略图
                $Info['pic_n']      =   checkpic($Info['pic']);
            }
        }

        return $Info;
    }

	/**
	 * @desc   修改参会企业表信息（审核，分站）
	 */
	public  function upZphCom($id , $data = array()){
        $where  =   array();

        if (! empty($id)) {

            $where['id']    =   array( 'in', pylode(',', $id));

            if ($data['status']) {

                $updata     =   array('status' => $data['status'],'statusbody' => $data['statusbody']);
            }else if ($data['did']) {

                $updata     =   array('did' => $data['did']);
            }

            $nid            =   $this->update_once('zhaopinhui_com', $updata, $where);

            if (!empty($data['status'])) {

                $List           =   $this -> getZphCompanyList($where, array('field' => '`zid`,`uid`,`com_name`'));

                /* 消息前缀 */
                $tagName        =   '参会企业';

                if (! empty($List)) {
					
					$uids = $zids = array();

                    foreach ($List as $v) {

                        $uids[] =   $v['uid'];
						
						$zids[]  = $v['zid'];

                        if ($updata['status'] == 2) {

                            $statusInfo         =   $tagName . ':<a href="zphtpl,'.$v['zid'].'">'.$v['com_name'].'</a>,审核未通过';

                            if ($updata['statusbody']) {

                                $statusInfo     .= ' , 原因：' . $updata['statusbody'];
                            }

                            $msg[$v['uid']]     =   $statusInfo;
                        } elseif ($updata['status'] == 1) {
							
                            $msg[$v['uid']]     =   $tagName . ':<a href="zphtpl,'.$v['zid'].'">'.$v['com_name'].'</a>,已审核通过';
                        }
                    }
					//邮箱通知
                    if (!empty($zids)){
                        $zids = array_unique($zids);
                        require_once 'notice.model.php';
                        $noticeM = new notice_model($this->db, $this->def);
                        
                        $zphs = $this->select_all('zhaopinhui', array('id'=>array('in', pylode(',', $zids))),'id,title');
                        
                        foreach ($List as $k=>$v){
                            foreach ($zphs as $val){
                                if ($v['zid'] == $val['id'] && !empty($v['linkmail'])){
                                    $sendData['type']           =   $updata['status']=='2'?'zphshwtg':'zphshtg';
                                    $sendData['uid']            =   $v['uid'];
                                    $sendData['email']          =   $v['linkmail'];
                                    $sendData['zphname']        =   $val['title'];
                                    $sendData['date']           =   date('Y-m-d H:i:s');
                                    $noticeM->sendEmailType($sendData);
                                }
                            }
                        }
                    }
                    // 发送系统通知
                    include_once ('sysmsg.model.php');
                    $sysmsgM    = new sysmsg_model($this->db, $this->def);
                    $sysmsgM -> addInfo(array('uid' => $uids,'usertype'=>2, 'content' => $msg));
                }
            }

            return $nid;
        }
    }

    /**
     * 修改招聘前台显示
    */

    public function upIsOpen($id,$is_open){
        if (!empty($id)) {
            $result  =  $this->update_once('zhaopinhui',array('is_open' => $is_open), array('id' => $id));

            if ($result){
                $return['msg']          =   '修改成功！';
                $return['errcode']      =   9;
            }else{
                $return['msg']          =   '修改失败！';
                $return['errcode']      =   8;
            }
        }else{
            $return['msg']          =   '参数错误！';
            $return['errcode']      =   8;
        }
        return $return;
    }

	/**
	 * @desc   修改参会企业表信息排序
	 */
	public  function upZphComSort($id , $data = array()){
        $where  =   array();

        if (! empty($id)) {

            $where['id']    =   $id;

            $nid            =   $this->update_once('zhaopinhui_com', $data, $where);

            return $nid;
        }
    }

	/**
	* @desc  删除参会企业
	*/
	public function delZphCom($delId = null, $data = array()){

        if (empty($delId)) {

            $return         =   array('errcode' => 8, 'msg' => '请选择要删除的数据！');
        } else {

            if (is_array($delId)) {

                $delId                  =   pylode(',', $delId);

                $return['layertype']    =   1;
            } else {
				
                $return['layertype']    =   0;
            }

			if($data['utype'] != 'admin'){

				$delWhere	=	array('id' => array('in', $delId),'uid'=>$data['uid']);
			}else{
				$delWhere	=	array('id' => array('in', $delId));
			}
			$comlist           =   $this -> getZphCompanyList(array('id' => array('in', $delId)));
			$tagName        =   '参会企业';
			if (!empty($comlist)) {
				foreach ($comlist as $v) {
					$uids[] =   $v['uid'];
					if ($v['status'] == 2) {
                        $statusInfo         =   $tagName . ':<a href="zphtpl,'.$v['zid'].'">'.$v['com_name'].'</a>,审核未通过，站长已删除您的报名，请重新报名~';
                        if ($v['statusbody']) {
                            $statusInfo     .= ' , 原因：' . $v['statusbody'];
                        }
                        $msgs[$v['uid']]     =   $statusInfo;
						include_once ('sysmsg.model.php');
						$sysmsgM    = new sysmsg_model($this->db, $this->def);
						$sysmsgM -> addInfo(array('uid' => $uids,'usertype'=>2, 'content' => $msgs));
                    }
				}
			}
            $return['id']       =   $this -> delete_all('zhaopinhui_com', $delWhere, '');
           
            $return['errcode']  =   $return['id'] ? 9 : 8;

            $msg                =   '招聘会参会企业（ID：'.$delId.'）';

            $return['msg']      =   $return['id'] ? $msg.'删除成功！' : '删除失败！';

        }

        return $return;
    }

	/**
	* @desc  招聘会图片
	*/
	public function getZphPicList($whereData,$data=array()) {

		$select		=	$data['field'] ? $data['field'] : '*';

		$List	=   $this -> select_all('zhaopinhui_pic',$whereData,$select);

		if($List&&is_array($List)){
			foreach($List as $key => $v){
				if ($v['pic']) { // 缩略图
					$List[$key]['pic_n']     =   checkpic($v['pic']);
				}
			}
		}
		return $List;
	}

	/**
	* @desc  招聘会图片详细信息
	*/
	public function getZphPicInfo($whereData,$data	=	array()){

		$select		=	$data['field'] ? $data['field'] : '*';

		$Info	=	$this -> select_once('zhaopinhui_pic', $whereData, $select);

		return $Info;

	}

	/**
	* @desc  添加招聘会图片
	*/
	public function addZphPicInfo($data	=	array()){

		$AddData	=	array(

			'title'		=>	$data['title'],

			'sort'		=>	$data['sort'],

			'zid'		=>	$data['zid'],

			'did'		=>	$data['did'],

		);
		if($data['pic']){
			$AddData['pic']	= $data['pic'];
		}
		if ($AddData && is_array($AddData)){

			$nid	=	$this -> insert_into('zhaopinhui_pic',$AddData);

		}

		return $nid;

	}

	/**
	* @desc  修改招聘会图片
	*/
	public function upZphPicInfo($whereData, $data = array()){

		if(!empty($whereData)) {

			$PostData	=	array(

				'title'		=>	$data['title'],

				'sort'		=>	$data['sort'],

			    'did'		=>	$data['did']

			);
			if($data['pic']){
                $PostData['pic']	= $data['pic'];
			}
			if ($PostData && is_array($PostData)){

				$nid  =  $this -> update_once('zhaopinhui_pic',$PostData,array('id'=>$whereData['id']));

			}

			return $nid;

		}

	}

	/**
	* @desc  删除招聘会图片
	*/
	public function delZphPic($where=array()){
        $delid  =   0;

        if (! empty($where)) {

            $delid  =   $this -> delete_all('zhaopinhui_pic', $where, '');
        }

        return $delid;
    }

	/**
	* @desc  招聘会设置缩略图
	*/
	public function getSetThembInfo($whereData,$data = array()){

		$select				=	$data['field'] ? $data['field'] : '*';

		$SetThembInfo		=	$this -> select_once('zhaopinhui_pic', $whereData, $select);

		if($SetThembInfo['pic']){

				$this -> update_once('zhaopinhui_pic',array('is_themb'=>''),array('zid'=>$SetThembInfo['zid']));

				$pic='.'.$SetThembInfo['pic'];

				$this -> update_once('zhaopinhui',array('is_themb'=>$pic),array('id'=>$SetThembInfo['zid']));

				$this -> update_once('zhaopinhui_pic',array('is_themb'=>'1'),array('id'=>$SetThembInfo['id']));

		}

		return $SetThembInfo;

	}

	/**
	 * @desc   招聘会预定
	 */
	public function ajaxZph($data = array())
	{

	    $uid        =   intval($data['uid']);
		$jobid		=   $data['jobid'];
	    $usertype   =   intval($data['usertype']);

	    $did        =   $data['did'] ? intval($data['did']) : $this->config['did'];

	    $zid        =   $data['zid'] ? intval($data['zid']) : '';
	    $id         =   $data['id'] ? intval($data['id']) : '';

	    $return     =   array();

	    require_once ('statis.model.php');
	    $statisM    =   new statis_model($this->db, $this->def);

	    require_once ('company.model.php');
	    $companyM   =   new company_model($this->db, $this->def);

	    $online     =   $this->config['com_integral_online'];

	    //判断后台是否开启该单项购买
        $single_can =   @explode(',', $this->config['com_single_can']);
        $serverOpen =   1;
        if(!in_array('zph',$single_can)){
            $serverOpen =   0;
        }

	    if (empty($uid) || empty($usertype)) {

	        $return['login']    =   1;
	    } else if ($usertype != 2) {

	        $return['msg']      =   '企业用户才可以预定招聘会！';
	    } else {

	        $comInfo  =  $this->select_once('company',array('uid' => $uid), '`name`,`r_status`');

	        if(empty($comInfo['name'])){

	            $return['msg']	=	'请先完善基本资料！';
    	   }else{

    	        $zph    =   $this->getInfo(array('id' => $zid), array('field' => '`starttime`,`endtime`'));

    	        if (strtotime($zph['starttime']) < time()) {

    	            $return['msg']  =   '招聘会已经开始！';
    	        } else if (strtotime($zph['endtime']) < time()) {

    	            $return['msg']  =   '招聘会已经结束！';
    	        } else {
    	            if ($comInfo['r_status'] != 1) {
                        if ($comInfo['r_status'] == 4) {
                            $return['msg'] = '当前账户会员权益已暂停，请联系客服开启服务~';
                        }else{
                            $return['msg'] = '您的账号未通过审核，请联系管理员加速审核进度！';
                        }
    	            } else {

    	                // 判断当天是否已达到最大报名次数
    	                $result     =   $companyM->comVipDayActionCheck('zph', $uid);

    	                if ($result['status'] != 1) {
    	                    return $result;
    	                }
    	                $zphzw  =  $this->getZphComInfo(array('bid' => $id, 'zid' => $zid));

    	                if (!empty($zphzw)){

    	                    $return['status']  =  3;
    	                    $return['msg']     =  '展位已有其他企业报名，请重新选择展位';

    	                    return $return;
    	                }

    	                $zphcom  =   $this->getZphComInfo(array('uid' => $uid, 'zid' => $zid));

    	                if (!empty($zphcom)) {

    	                    if ($zphcom['status'] == 2) {

    	                        $return['msg'] = '您的报名未通过审核，请联系管理员！';
    	                    } else {

    	                        $return['msg'] = '您已报名该招聘会！';
    	                    }

    	                    return $return;
    	                } else {

    	                    $suid   =   $uid;

    	                    $statis =   $statisM -> getInfo($suid, array('usertype' => $usertype, 'field' => '`rating_type`,`vip_etime`,`zph_num`,`integral`'));

    	                    $space  =   $this -> getZphSpaceInfo(array('id' => $id));

    	                    $com    =   $companyM -> getInfo($uid, array('field' => '`name`'));

    	                    $zData              =   array();
    	                    $zData['uid']       =   $uid;
    	                    $zData['usertype']  =   $usertype;
    	                    $zData['com_name']  =   $com['name'];
    	                    $zData['status']    =   1;
    	                    $zData['did']       =   $did;
							$zData['jobid']		=   $jobid;
    	                    $zData['id']        =   $id;
    	                    $zData['zid']       =   $zid;

    	                    $zData['fuid']      =   $uid;

    	                    $price  =   floatval($space['price'] / $this->config['integral_proportion']); // 展位价格

                            if (isVip($statis['vip_etime'])) {
                                if ($statis['rating_type'] == 1) {
                                    if ($statis['zph_num'] == 0) {
                                        if ($price == 0 && $serverOpen == 1) {

                                            $return =   $this->reserveZph($zData);
                                            return $return;
                                        } else {

                                            if ($online != 4) {
                                                if ($online == 3 && !in_array('zph', explode(',', $this->config['sy_only_price']))) { // 积分消费

                                                    if ($serverOpen) {

                                                        $return['msg']  =   "您的等级特权已经用完，继续报名将消费 <span style=color:red;>".$space['price']."</span> ".$this->config['integral_pricename']."，是否继续？";
                                                    } else {

                                                        $return['msg']  =   "您的等级特权已经用完，可以<a href=\"".$this->config['sy_weburl']."/wap/member/index.php?c=rating\" style=\"color:red;cursor:pointer;\">购买会员</a>！";
                                                    }

                                                    $return['url']      =   $this->config['sy_weburl'].'wap/member/index.php?c=getserver&id='.$uid.'&server=15';
                                                    $return['jifen']    =   $space['price'];
                                                    $return['integral'] =   intval($statis['integral']);
                                                    $return['pro']      =   $this->config['integral_proportion'];
                                                } else {

                                                    $return['url']      =   $this->config['sy_weburl'].'wap/member/index.php?c=getserver&id='.$uid.'&server=15';
                                                    if ($serverOpen) {

                                                        $return['msg']  =   "您的等级特权已经用完，继续报名将消费 <span style=color:red;>".$price."</span> 元，是否继续？";
                                                    } else {

                                                        $return['msg']  =   "您的等级特权已经用完，可以<a href=\"".$this->config['sy_weburl']."/wap/member/index.php?c=rating\" style=\"color:red;cursor:pointer;\">购买会员</a>！";
                                                    }
                                                    $return['price']    =   $price;
                                                }
                                            } else {
                                                $return['price']=   $price;
                                                $return['url']  =   $this->config['sy_weburl'].'wap/member/index.php?c=rating';
                                                $return['msg']  =   "您的等级特权已经用完，可以<a href=\"".$this->config['sy_weburl']."/wap/member/index.php?c=rating\" style=\"color:red;cursor:pointer;\">购买会员</a>！";
                                            }
                                            $return['zid']      =   $zid;
                                            $return['bid']      =   $id;
                                            $return['status']   =   2;

                                            return $return;
                                        }
                                    } else {

                                        $statisM->upInfo(array('zph_num' => array('-', 1)), array('uid' => $suid, 'usertype' => $usertype));

                                        $payDetail = '报名招聘会，扣除招聘会套餐数量：1';
                                        $statisM->addStatisDetail(array('uid' => $uid, 'type' => 8, 'num' => 1, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));

                                        $return =   $this->reserveZph($zData);
                                        return $return;
                                    }
                                } else if ($statis['rating_type'] == 2) {

                                    $payDetail = '报名招聘会，消耗今日招聘会套餐数量：1';
                                    $statisM->addStatisDetail(array('uid' => $uid, 'type' => 8, 'num' => 1, 'detail' => $payDetail, 'uri' => $_SERVER['REQUEST_URI']));

                                    $return =   $this->reserveZph($zData);
                                    return $return;
                                }
                            } else { // 过期会员

                                if ($online != 4) {

                                    if ($online == 3 && !in_array('zph', explode(',', $this->config['sy_only_price']))) { // 积分消费

                                        $return['jifen'] = $space['price'];
                                        $return['integral'] = intval($statis['integral']);
                                        $return['pro'] = $this->config['integral_proportion'];
                                    } else {
                                        $return['price'] = $price;
                                    }
                                    $return['msg'] = "你的会员已到期，请先购买会员！";
                                } else {
                                    $return['url'] = $this->config['sy_weburl'] . 'wap/member/index.php?c=rating';
                                    $return['msg'] = "你的会员已到期，你可以<a href='" . $this->config['sy_weburl'] . "/wap/member/index.php?c=rating' style='color:red;cursor:pointer;'>购买会员</a>！";
                                }
                                $return['zid'] = $zid;
                                $return['bid'] = $id;
                                $return['status'] = 2;

                                return $return;
                            }
    	                }
    	            }
    	        }
    	    }
	    }
	    return $return;
	}

	/**
	 * @Desc   招聘会报名
	 *
	 * @param array $data
	 */
    private function reserveZph($data = array())
    {

        $bid    =   intval($data['id']);    // 展位ID
        $space  =   $this->getZphSpaceInfo(array('id' => $bid));
        $sid    =   $this->getZphSpaceInfo(array('id' => $space['keyid']));

	    $sql    =   array(

	        'uid'          =>  $data['uid'],
	        'com_name'     =>  $data['com_name'],
	        'zid'          =>  $data['zid'],
	        'ctime'        =>  time(),
	        'status'       =>  0,
	        'did'          =>  $data['did'],
			'jobid'		   =>  $data['jobid'],
	        'sid'          =>  $sid['keyid'],
	        'cid'          =>  $space['keyid'],
	        'bid'          =>  $bid
	    );

	    $nid    =   $this->insert_into('zhaopinhui_com', $sql);

	    $return =   array();
        if ($nid) {
            require_once('log.model.php');
            $logM   =   new log_model($this->db, $this->def);
            $logM->addMemberLog($data['fuid'], $data['usertype'], '报名招聘会（ID:' . $data['zid'] . '，展位：' . $bid . '）', 14, 1);

            $return['status'] = 1;
            $return['msg'] = '报名成功,等待管理员审核！';

        } else {
            $return['msg'] = '报名失败,请稍候重试！';
        }

	    return $return;
	}
}
?>