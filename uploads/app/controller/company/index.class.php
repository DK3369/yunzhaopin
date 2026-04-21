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
class index_controller extends common{

    /**
     * 找企业
     */
	function index_action(){

        $this->toLoginPage();
		$CacheM       =   $this -> MODEL('cache');

		$CacheList    =   $CacheM -> GetCache(array('city','com','hy','job'));

		$this         ->  yunset($CacheList);

		if($_GET['city']){//城市匹配

		    $city					=	explode("_",$_GET['city']);

		    $_GET['provinceid']		=	$city[0];
		    $_GET['cityid']			=	$city[1];
		    $_GET['three_cityid']	=	$city[2];
		}

		if ($this->config['sy_web_city_one']) {
	        $_GET['provinceid'] = 	$this->config['sy_web_city_one'];
	    }
	    if ($this->config['sy_web_city_two']) {
	        $_GET['cityid'] 	= 	$this->config['sy_web_city_two'];
	    }

	    if($this->config['province']){
			$_GET['provinceid'] 	= 	$this->config['province'];
		}
		if($this->config['cityid']){
			$_GET['cityid'] 		= 	$this->config['cityid'];
		}
		if($this->config['three_cityid']){
			$_GET['three_cityid']	= 	$this->config['three_cityid'];
		}

		$this         ->  yunset(array('gettype' => $_SERVER['QUERY_STRING'], 'getinfo' => $_GET));

 		include PLUS_PATH.'keyword.cache.php';

		if(is_array($keyword)){

			foreach($keyword as $k=>$v){

				if($v['type']=='4' && $v['tuijian']=='1'){

					$comkeyword[]  =   $v;

				}
			}
		}
 		$this->yunset("comkeyword", $comkeyword);

        //关键字显示end
 		$this->seo('firm');
		$this->yun_tpl(array('index'));
	}

	// 企业信息查询和判断公用方法
	function public_action(){

		$JobM         =   $this -> MODEL("job");
		$CompanyM     =   $this -> MODEL('company');
		$StatisM      =   $this -> MODEL('statis');
		$RatingM      =   $this -> MODEL('rating');

		$uid          =   intval($_GET['id']);

		$num          =   $JobM -> getJobNum(array('uid'=>$uid, 'r_status'=>1, 'status'=>0, 'state'=>1));
        $this->yunset("num", $num);

		$userinfo     =   $CompanyM -> getInfo($uid, array('logo' => '1'));

		$userstatis   =   $StatisM -> getInfo($uid, array('usertype'=>2));

		$comrat       =   $RatingM -> getInfo(array('id'=>$userstatis['rating']),array('pic'=>'1'));
        $this->yunset("comrat", $comrat);

		$row          =   array_merge($userinfo, $userstatis);
		//企业状态判断
        if ($_GET['look'] != 'admin') {
            if ($row['uid'] != $this->uid) {
                if ($row ['r_status'] == 0) {

                    $this->ACT_msg($this->config['sy_weburl'] . '/member', '企业正在审核中！');
                } elseif ($row ['r_status'] == 2) {

                    $this->ACT_msg($this->config['sy_weburl'] . '/member', '企业暂被锁定，请稍后查看！');
                } elseif ($row ['r_status'] == 3) {

                    $this->ACT_msg($this->config['sy_weburl'] . '/member', '企业审核暂未通过！');
                }
            }
        }

		if(!is_array($row)){

            $this->ACT_msg($this->config['sy_weburl'],"没有找到该企业！");

		}elseif($row['r_status'] == 0 && $row['uid'] != $this->uid){

            session_start();
            if(empty($_SESSION['auid'])){
                $this->ACT_msg($this->config['sy_weburl'],"该企业正在审核中，请稍后查看！");
            }

		}elseif($row['r_status'] == 3 && $row['uid'] != $this->uid){

            session_start();
            if(empty($_SESSION['auid'])){
                $this->ACT_msg($this->config['sy_weburl'],"该企业未通过审核！");
            }

        }elseif($row['r_status'] == 2){

            session_start();
            if(empty($_SESSION['auid'])){
                $this->ACT_msg($this->config['sy_weburl'],"该企业暂被锁定，请稍后查看！");
            }
        }

		$CacheM=$this->MODEL('cache');
        $CacheList=$CacheM->GetCache(array('city','com','hy'));
        $this->yunset($CacheList);

        $city_name          =   $CacheList['city_name'];
        $comclass_name      =   $CacheList['comclass_name'];
        $industry_name      =   $CacheList['industry_name'];

        $city               =   array(

            'provinceid'    =>  $row['provinceid'],
            'cityid'        =>  $row['cityid'],
            'three_cityid'  =>  $row['three_cityid']

        );
        $this -> yunset("city" , $city);

        $row['provinceid']  =   $city_name[$row['provinceid']];
        $row['cityid']      =   $city_name[$row['cityid']];
        $row['mun_info']    =   $comclass_name[$row['mun']];
        $row['pr_info']     =   $comclass_name[$row['pr']];
        $row['hy_info']     =   $industry_name[$row['hy']];

        if($row['sdate']){
        	$date = explode('-',$row['sdate']);
        }

        $row['sdate']=$date[0];
        $row['comqcode']	=	checkpic($row['comqcode']);
        $welfare            =   @explode(",",$row['welfare']);


		$row['content']       =   str_replace(array("ti<x>tle","“","”"),array("title"," "," "),$row['content']);
		$this -> yunset("com",  $row);

		$banner               =   $CompanyM   ->  getBannerInfo('', array('where'=>array('uid'=>$uid,'status'=>0,)));
		$this -> yunset("banner",$banner);

		$NewsList             =   $CompanyM   ->  getCompanyNewsList(array('uid'=>$uid, 'status'=>1));
		$this -> yunset('NewsList' , $NewsList);

		$ProductList          =   $CompanyM   ->  getCompanyProductList(array('uid'=>$uid, 'status'=>1));
		$this->yunset('ProductList' , $ProductList);

		$shows                =   $CompanyM   ->  getCompanyShowList(array('uid'=>$uid, 'status'=>0,'orderby'=>'sort,desc'));
		$this->yunset('shows' , $shows);

		$data                 =   array(

		    'company_name'        =>  $row['name'],
		    'company_name_desc'   =>  strip_tags($row['content']),
		    'industry_class'      =>  $row['hy_info']

		);

 		if($this->uid && $this->usertype=='1'){

 		    $AtnM    =   $this->MODEL('atn');
 		    $isatn   =   $AtnM->getatnInfo(array('uid' => $this->uid, 'sc_uid' => (int)$_GET['id']));
 		    $this -> yunset('isatn', $isatn);

		}

		$invite_resume    =   $JobM -> getYqmsNum(array('fid'=>$uid,'isdel'=>9));
		$this -> yunset("invite_resume" , $invite_resume);

		$de_resume        =   $JobM -> getSqJobNum(array('com_id'=>$uid, 'is_browse'=>'1','isdel'=>9));
		$des_resume       =   $JobM -> getSqJobNum(array('com_id'=>$uid,'isdel'=>9));
		if (!empty($des_resume) && !empty($de_resume)){

		    $pre          =   round((1 - $de_resume / $des_resume) * 100);
		}else{

		    $pre          =   100;
		}
		$this -> yunset("pre",$pre);

		$time             =   strtotime("-14 day");
		$this -> yunset("id", $uid);

 		return $data;

	}

	/**
	 * @desc 企业黄页
	 */
    function show_action(){

    	$data				=	$this->public_action();

		$uid				=   intval($_GET['id']);

		$CompanyM			=   $this -> MODEL('company');

		$jobM				=   $this -> MODEL('job');

        $CompanyM->upInfo($uid, '', array('hits' => array('+', 1), 'expoure' => array('+', 1)));

        if ($_GET['style']) {

            $urlmsg = Url('company', array('c' => 'msg', 'id' => $uid, 'style' => $_GET['style']));
        } else {

            $urlmsg = Url('company', array('c' => 'msg', 'id' => $uid));
        }
        $this->yunset("urlmsg",$urlmsg);

		$JobM=$this->MODEL('job');
		$num         =   $JobM -> getJobNum(array('uid' => $uid, 'r_status'=>array('<>','2'), 'status' => array('<>','1'), 'state'=>1));
        $maxpage     =   intval(ceil($num/5));
        $pgarr = array();
        for ($i=1;$i<=$maxpage;$i++){
            $pgarr[] = $i;
        }
        $this->yunset("pgarr",$pgarr);
		if ($this->usertype==1){

		    $userid_job = $JobM->getSqJobInfo(array('uid'=>$this->uid,'com_id'=>$uid,'isdel'=>9));

            $this->yunset('userid_job', $userid_job);

            // 查询求职咨询信息
            $msgM	=	$this -> MODEL('msg');
            $isMsg	=	$msgM -> getMsgNum(array('uid' => $this->uid,'status'=>1,'job_uid' => $uid));

            if(intval($isMsg) > 0){
                $msgList	=	$msgM -> getList(array('job_uid' => $uid,'status'=>1, 'del_status'=>0,'orderby' => 'datetime,desc', 'limit' => 5));

                $this -> yunset('msgList', $msgList['list']);
            }

        }
        // 查询对最近一次对申请简历的操作时间
        $Info       =   $JobM -> getList(array('uid'=>$uid, 'state'=>'1', 'operatime'=>array('<>','')), array('field'=>'operatime,id'));


        $i=0;$ids=array();
        $times  =   0;
        foreach($Info as $k=>$v){

            if($v['operatime']){

        		$times+=$v['operatime'];
        		$i++;

        	}

        	$ids[]=$v['id'];

        }

        if($i>0){

            $times=$times/$i;

        }

        $Infojob    =   $JobM -> getSqJobList(array('com_id'=>$uid,'isdel'=>9,'job_id'=>array('in',pylode(',', $ids))),array('field'=>'datetime'));

        $i=0;
        $timejobs = 0;
        foreach($Infojob as $k=>$v){
        	if($v['datetime']){
        		$timejobs+=$v['datetime'];
        		$i++;
        	}
        }
        if($i>0){
        	$timejobs=$timejobs/$i;
        }
        $operatime=$times-$timejobs;

		if(!$times or !$timejobs){
			$times='0';
		}else if($operatime<3600){
			$times='一小时以内';
		}else if($operatime>=3600&&$operatime<86400){
			$times=floor($operatime/3600).'小时';
		}else if($operatime>=86400){
			$times=floor($operatime/86400).'天';
		}
 
		$this->yunset('operatime', $times);
		$this->data=$data;

        $WhbM       =   $this->MODEL('whb');
        $syComHb    =   $WhbM->getWhbList(array('type' => 2, 'isopen' => 1), array('only' => 1));
        $this->yunset('hbNum', count($syComHb));
        $this->yunset('comHb', $syComHb);

        $jobwhere['uid']		=   $uid;
        $jobwhere['state']		=	1;
        $jobwhere['r_status']	=	1;
        $jobwhere['status']		=	0;
        $comJobs	=   $this->obj->select_all('company_job', $jobwhere, '`id`,`name`');//招聘中职位
        
        $this->yunset('jobs', $comJobs);
        $this->yunset('job_cnt', $comJobs ? count($comJobs) : 0);

        if(!empty($syComHb)){
            $hbids  =   array();
            foreach ($syComHb as $hk => $hv) {
                $hbids[] = $hv['id'];
            }
            $this->yunset('hbids', $hbids);
        }
        $this->yunset('hb_uid', $uid);
		
		$this->seo("company_index");
    	$this->comtpl("index");
    }

	/**
	 * @desc 企业黄页模板选取
	 */
	function comtpl($tpl){

        if ($_GET['style'] && !preg_match('/^[a-zA-Z]+$/',$_GET['style'])){
            exit();
        }
        if ($_GET['tp'] && !preg_match('/^[a-zA-Z]+$/',$_GET['tp'])){
            exit();
        }

        $uid        =   intval($_GET['id']);

        $statisM    =   $this -> MODEL('statis');

        $statis     =   $statisM -> getInfo($uid, array('usertype'=>'2'));

		if($statis['comtpl'] && $statis['comtpl']!="default" && !$_GET['style']){

            $tplurl =   $statis['comtpl'];

        }else{

            $tplurl =   "default";

        }

		if($_GET['style']){

            $tplurl =   $_GET['style'];

        }

		$this->yunset(array("com_style"=>$this->config['sy_weburl']."/app/template/company/".$tplurl."/","comstyle"=>TPL_PATH."company/".$tplurl."/"));
		
		$this->yuntpl(array('company/'.$tplurl."/".$tpl));

	}


	/**
	 * @desc 企业产品Show
	 */
	function productshow_action(){

		$CompanyM =   $this -> MODEL("company");

		$jobM     =   $this -> MODEL('job');

		$uid      =   intval($_GET['id']);

		$sq_num   =   $jobM -> getSqJobNum(array('com_id'=>$uid,'isdel'=>9));

		$this     ->  yunset("sq_num",$sq_num);
 
		$id       =   intval($_GET['pid']);

		session_start();

        if(!is_numeric($_SESSION['auid']) && $uid != $this->uid){

            $where  =   array('id' => $id , 'status' => '1');

        }else {

            $where  =   array('id' => $id);
        }

        $ProductInfo            =   $CompanyM -> getComProductInfo($where);

        $ProductInfo['body']    =   str_replace(array("ti<x>tle","“","”"), array("title"," "," "), $ProductInfo['body']);

        $this -> yunset('ProductInfo', $ProductInfo);

        $data   =   $this -> public_action();

        $data['company_product']    =   $ProductInfo['title'];

        $this -> data   =   $data;

        $this -> seo("company_productshow");

		$this -> comtpl("productshow");

	}

	/**
	 * @desc 企业新闻show
	 */
	function newsshow_action(){

	    $CompanyM  =   $this -> MODEL('company');

	    $JobM      =   $this -> MODEL('job');

	    $uid       =   intval($_GET['id']);

	    $sq_num    =   $JobM -> getSqJobNum(array('com_id'=>$uid,'isdel'=>9));

	    $this      ->  yunset('sq_num', $sq_num);

	    $id        =   intval($_GET['nid']);

		session_start();

        if(!is_numeric($_SESSION['auid']) && (int)$_GET['id']!=$this->uid){

            $where  =   array('id'=>$id, 'status'=>'1');

        }else{

            $where  =   array('id'=>$id);
        }

        $NewsInfo				=	$CompanyM -> getCompanyNewsInfo($where);

		$NewsInfo['body']		=	str_replace(array("ti<x>tle","“","”"),array("title"," "," "),$NewsInfo['body']);

		$data					=	$this->public_action();

		$data['company_news']	=	$NewsInfo['title'];

		$this -> data =	$data;

		$this -> yunset('NewsInfo',$NewsInfo);

		$this -> seo("company_newsshow");

		$this -> comtpl("newsshow");

	}

	/**
	 * @desc ajax 加载企业职位信息
	 */
	function prestr_action(){

 	    if($_POST['page']){

			$CacheM      =   $this->MODEL('cache');
			$CacheList   =   $CacheM->GetCache(array('city','com','hy','job'));
			$this->yunset($CacheList);

			$JobM        =   $this -> MODEL('job');

			$page        =   intval($_POST['page']);

			$uid         =   intval($_POST['id']);

			$num         =   $JobM -> getJobNum(array('uid' => $uid, 'r_status'=>array('<>','2'), 'status' => array('<>','1'), 'state'=>1));

			$data['num'] =   $num;

			$limit       =   intval($_POST['limit']);

			$maxpage     =   intval(ceil($num/$limit));

			if ($page > $maxpage){
			    $page    =   $maxpage;
	        }

	        $start       =     $page * $limit;

	        if (intval($_POST['updown'])==1){

	            $compage   =   max(1,($page-1));

	        }else if (intval($_POST['updown'])==2){

	            $compage   =   min($maxpage,($page+1));

	        }else{
	            $compage = min($maxpage,$_POST['updown']);
            }

			if($compage == 1){

				$start  =  0;

			}else{

			    $start   =   ($compage-1)*$limit;
			}

			$joblistA    =   $JobM -> getList(array('uid' => $uid, 'status' => '0', 'state' => '1', 'orderby' => 'lastupdate,desc', 'limit'=>array($start, $limit)),array('isurl'=>'yes'));

			$job_list    =   $joblistA['list'];

			$joblist     =   "";


			if ($_POST['style'] == '1') {


			    foreach($job_list as $v){

			        $v['url']   =   $v[joburl];

			        $joblist    .=  "<div class='firm_post'>";

			        $joblist    .=  "<div class='fpc_name'><a href='".$v[url]."' target='_blank'>".$v[name]."</a></div>";

			        $joblist    .=  "<div class='firm_post_joblist'>";

			        if($v[job_salary]){
			            $joblist .=  "<span class='comshow_job_xz'><i>".$v[job_salary]."</i></span>";
			        }
			        if($v[job_city_two]){
			            $joblist  .= "<span class='comshow_job_city'>".$v[job_city_two]."</span>";
			        }
			        if($v['job_exp']){
			            $joblist  .= "<span class='comshow_job_jy'>".$v['job_exp']."经验</span>";
			        }
			        $joblist    .=  "</div>";

			        if($v[lastupdate]){

			            $v[lastupdate]     =   date('Y-m-d', $v[lastupdate]);

			            $joblist           .=  "<div class='firm_post_jobtime'>".$v[lastupdate]."</div>";
			        }

			        $joblist    .=  "<a href='".$v[url]."' target='_blank'  class='firm_post_jobbth'>立即查看</a></div>";

			        $joblist      .=  "</div>";

			    }

			}else {


    			foreach($job_list as $v){

    				$v['url']   =   $v[joburl];

    				$joblist    .=  "<div class='firm_post'><div class='com_details_com_otherjob_l'><div class='com_details_com_otherjob_name'><a href='".$v[url]."' target='_blank'>".$v[name]."</a></div>";
    				$joblist    .=  "<div class='com_details_com_otherjob_info'>";

    				if($v['job_exp']){
     					$joblist  .= $v['job_exp']."经验";
    				}
    				if($v['job_edu'] && $v['job_exp']){
     					$joblist  .= "<span class='com_details_line'>|</span>";
    				}
    				if($v['job_exp']){
     					$joblist  .= $v['job_edu']."学历";
    				}
    				$joblist    .=  "</div></div>";

    				$joblist    .=  "<div class='com_details_com_otherjob_c'><div class='com_details_com_otherjob_xz'>";

    				if($v[job_salary]){

    				    $joblist    .=  $v[job_salary];

    				}

    				$joblist    .=  "</div><div class='com_details_com_otherjob_city'>";

    				if($v[job_city_two]){
     					$joblist  .= $v[job_city_two];
    				}

    				$joblist    .=  "</div></div>";


    				$joblist    .=  "<div class='com_details_com_otherjob_r'>";

    				if($v[lastupdate]){

    					$v[lastupdate]     =   date('Y-m-d', $v[lastupdate]);

    					$joblist           .=  "<div class='com_details_com_otherjob_time'>".$v[lastupdate]."</div>";
    				}

    				$joblist    .=  "<a href='".$v[url]."' target='_blank'  class='com_details_com_otherjob_sq'>申请</a></div>";

    				$joblist      .=  "</div>";

    			}

			}

			$joblist .="<div class='pages' style='margin-top:20px'>
                            <a href=\"javascript:void(0);\" onclick=\"page('$_POST[id]','$compage','5','1');\">上一页</a>
                            ";
                        for ($i=0;$i<$maxpage;$i++) {
                            $a = $i+1;
                            if ($a == $compage) {
                                $joblist .= "<a href=\"javascript:void(0);\" class='selected' onclick=\"page('$_POST[id]','$i','5','$a');\">$a</a>";
                            }else{
                                $joblist .= "<a href=\"javascript:void(0);\" onclick=\"page('$_POST[id]','$i','5','$a');\">$a</a>";
                            }
                        }
            $joblist .= "<a href=\"javascript:void(0);\" onclick=\"page('$_POST[id]','$compage','5','2');\">下一页</a>
                            <a href=\"javascript:void(0);\">总页码".$maxpage."</a>
                        </div>";
			$data['joblist']=$joblist;

	        echo json_encode($data);
	    }
	}
	//微信扫码查看联系方式
	function telQrcode_action(){
	    
	    $WxM	=	$this -> MODEL('weixin');
	    
	    $qrcode =	$WxM->pubWxQrcode('comtel',$_GET['id']);
	    if(isset($qrcode)){
	        
	        $imgStr  =	CurlGet($qrcode);
	        
	        header("Content-Type:image/png");
	        
	        echo $imgStr;
	    }
	}
}
?>