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
class company_controller extends common
{
    /**
     * 企业列表
     */
    function index_action()
    {
        $this->toLoginPage();
        $this -> get_moblie();
        
        $CacheM     =   $this -> MODEL('cache');
        
        $CacheList  =   $CacheM->GetCache(array('city', 'hy', 'com'));
        // 后台-页面设置-列表页区域默认设置。选择了一级城市
        if (!empty($this->config['sy_web_city_one'])) {
            $provinceid  =  $this->config['sy_web_city_one'];
            $CacheList['city_index']  =  array($provinceid);
            $this->yunset('nocityall', 1);
            // 选择了二级城市
            if (!empty($this->config['sy_web_city_two'])) {
                $cityid  =  $this->config['sy_web_city_two'];
            }
            if (!isset($_GET['provinceid']) && !isset($_GET['cityid'])){
                if (isset($provinceid)){
                    $_GET['provinceid'] = $provinceid;
                }
                if (isset($cityid)){
                    $_GET['cityid'] = $cityid;
                }
            }
            if (isset($_GET['three_cityid'])) {
                unset($_GET['provinceid']);
                unset($_GET['cityid']);
            }
        }
        $this -> yunset($CacheList);
        
        $searchUrlObj = array();
        foreach ($_GET as $k => $v) {
            
            if ($k != '') {
                
                $searchurl[] = $k.'='.$v;
                $searchUrlObj[$k]    = $v;
            }
        }
        
        $this -> seo('firm');
        
        $searchurl  =   @implode('&', $searchurl);
        
        $this -> yunset('searchurl', $searchurl);
        $this->yunset('searchUrlObj',json_encode($searchUrlObj));

        $cityChoosed = '';
        if ($_GET['threecityid']){
            $_GET['three_cityid']   =   $_GET['threecityid'];
            unset($_GET['threecityid']);
        }
        if($_GET['three_cityid']){
            $cityChoosed = $_GET['three_cityid'];
        }else if($_GET['cityid']){
            $cityChoosed = $_GET['cityid'];
        }else if($_GET['provinceid']){
            $cityChoosed = $_GET['provinceid'];
        }
        $this->yunset('cityChoosed', $cityChoosed);

        
        $this -> yunset('backurl', Url('wap'));
        $this -> yunset('topplaceholder', '请输入企业关键字,如：有限公司...');
        $this -> yunset('headertitle', '公司搜索');
        $this -> yuntpl(array('wap/company'));
    }

    /**
     * 企业详情
     */
    function show_action()
    {
        
        $this -> get_moblie();
		
        $companyM   =   $this -> MODEL('company');
        $jobM       =   $this -> MODEL('job');
        
        $uid        =   intval($this -> uid);
        $cuid       =   intval($_GET['id']);
        
        $CacheM     =   $this -> MODEL('cache');
        $CacheList  =   $CacheM->GetCache(array('job', 'com', 'city', 'hy'));
        $this -> yunset($CacheList);

        $companyM  -> upInfo($cuid, '', array('hits'=>array('+',1), 'expoure' => array('+', 1)));

        $row        =   $companyM -> getInfo($cuid, array('logo' => '1'));
        
        $show       =   $companyM -> getCompanyShowList(array('uid'=>$cuid, 'status'=>'0', 'orderby' => 'sort'));
       
        $this -> yunset('show', $show);

        if (!is_array($row)) {
            
            $this -> ACT_msg_wap($_SERVER['HTTP_REFERER'], '没有找到该企业！');
            
        } elseif ($row['r_status'] == 0 && $row['uid'] != $this->uid) {
            
            $this -> ACT_msg_wap($_SERVER['HTTP_REFERER'], '该企业正在审核中，请稍后查看！');
            
        } elseif ($row['r_status'] == 3 && $row['uid'] != $this->uid) {
            
            $this -> ACT_msg_wap($_SERVER['HTTP_REFERER'], '该企业未通过审核！');
            
        } elseif ($row['r_status'] == 2) {
            
            $this -> ACT_msg_wap($_SERVER['HTTP_REFERER'], '该企业暂被锁定，请稍后查看！');
            
        }

        $statisM    =   $this -> MODEL('statis');
        
        $rows       =   $statisM -> getInfo($cuid, array('usertype' => '2', 'field' => '`rating`'));
       
        $ratingM    =   $this -> MODEL('rating');
        
        $comrat     =   $ratingM -> getInfo(array('id'=> intval($rows['rating'])), array('pic'=>'1'));
        
        $row['lastupdate']  =   date('Y-m-d', $row['lastupdate']);

        // 解决通过Editor上传的图片路径问题
        $row['content']     =   str_replace(array('ti<x>tle','“','”','&nbsp;'), array('title','','',' '), $row['content']);
        $row['content']     =   htmlspecialchars_decode($row['content']);
        
        preg_match_all('/<img(.*?)src=("|\'|\s)?(.*?)(?="|\'|\s)/', $row['content'], $res);
        
        if (! empty($res[3])) {
            
            foreach ($res[3] as $v) {
                
                if (strpos($v, 'http:') === false && strpos($v, 'https:') === false) {
                    $ossv=checkpic($v);
                    $row['content'] = str_replace($v, $ossv, $row['content']);
                }
            }
        }

        $zpNum  =   $jobM->getJobNum(array('uid' => $_GET['id'], 'state' => 1, 'status' => 0, 'r_status' => 1,'is_depower'=>2));
        $row['zpNum']   =   $zpNum;
        
        $this -> yunset("row", $row);
        
        $this -> yunset("comrat", $comrat);
        
        if ($uid && $this->usertype == '1') {
            
            $atnM   =   $this -> MODEL('atn');
            
            $isatn  =   $atnM -> getAtnInfo(array('uid' => $uid, 'sc_uid' => $cuid));
            
            $this   ->  yunset('isatn', $isatn);
            
            $userid_job =   $jobM -> getSqJobInfo(array('uid' => $uid, 'com_id' => $cuid,'isdel'=>9));
            
            $this   ->  yunset('userid_job', $userid_job);
        }
        
        
        $data['company_name']       =   $row['name'];
        $data['company_name_desc']  =   $row['content'];
        $this -> data   =   $data;
        $this -> seo('company_index');
        
        $this -> yunset('headertitle', '公司详情');
        if($this->config['sy_h5_share']==1){
			$this -> yunset("shareurl", Url('wap', array('c' => 'company','a' => 'share','id' => $cuid)));
		}else{
			$this -> yunset("shareurl",Url('wap',array('c'=>'company','a'=>'show','id'=>$cuid)));	
		}

        $this->yunset(array('mapx' => 0, 'mapy' => 0));

        if ($this->config['sy_haibao_isopen'] == 1) {

            $WhbM       =   $this->MODEL('whb');

            $maxNum     =   $jobM->getJobNum(array('state' => 1, 'status' => 0, 'r_status' => 1, 'uid' => $cuid));
            $syComHb    =   $WhbM->getWhbList(array('type' => 2, 'isopen' => '1', 'num' => array('<=', $maxNum)));

            $this->yunset('hbNum', count($syComHb));

            if (!empty($syComHb)) {

                $hbids  =   array();
                foreach ($syComHb as $hk => $hv) {
                    $hbids[]    =   $hv['id'];
                }
                $this->yunset('hbids', $hbids);
            }
        }
        if (strpos($_SERVER['HTTP_REFERER'], 'company') === false) { // 不是从企业列表过来的才会直接赋值返回URL
            $backurl    =   Url('wap', array('c' => 'company'));

            $this->yunset('backurl', $backurl);
        }
        if($this->config['sy_wap_comtpl']==1){
            $this->yuntpl(array('wap/company_show_dark'));
        }else{
            $this->yunset('ball', 2);
            $this->yuntpl(array('wap/company_show'));
        }
    }

   

    function share_action()
    {
        
        $this   ->  get_moblie();

        $cuid       =   intval($_GET['id']);
        
        $comM       =   $this -> MODEL('company');
        
        $row        =   $comM -> getInfo($cuid, array('logo'=>'1'));

        $welfare    =   @explode(',', $row['welfare']);
        
        foreach ($welfare as $k => $v) {
        
            if (! $v) {
                
                unset($welfare[$k]);
            }
        }
        
        $row['welfare']     =   $welfare;
        
        $row['content']     =   strip_tags($row['content']);

        $row['content']     =   str_replace(array('&nbsp;'), array(' '), $row['content']);

        $this -> yunset('row', $row);
        
        $show   =   $comM -> getCompanyShowList(array('uid' => $cuid), array('field' => '`picurl`'));
         
        $this -> yunset('show', $show);
        
        $product    =   $comM -> getCompanyProductList(array('uid' => $cuid, 'status' => '1'));
        
        $this -> yunset('product', $product);
        
        $CacheM     = $this->MODEL('cache');
        
        $CacheList  = $CacheM->GetCache(array('job', 'com', 'city', 'hy'));

        $this -> yunset($CacheList);
        
        $data['company_name']       =   $row['name'];
        $data['company_name_desc']  =   $row['content'];
        $this -> data   =   $data;
        
        $this -> seo('company_index');
        $this -> yunset('company_style', $this->config['sy_weburl'] . '/app/template/wap/company');
        $this -> yuntpl(array('wap/company/index'));
    }

    // 企业微海报（选择海报）列表
    function whb_action()
    {

        $id =   $_GET['id'];

        $WhbM       =   $this->MODEL('whb');
        $comHb      =   $WhbM->getWhbList(array('type' => 2, 'isopen' => '1'), array('only' => 1));
        $this->yunset('comHb', $comHb);

        $jobM       =   $this->MODEL('job');
        // $jobList    =   $jobM->getHbJobList(array('uid' => $id, 'state' => 1, 'status' => 0, 'r_status' => 1), array('field' => '`id`,`name`,`minsalary`, maxsalary'));
        $jobList    =   $jobM->getList(
            array('uid' => $id, 'state' => 1, 'status' => 0, 'r_status' => 1),
            array('field' => '`id`,`name`,`minsalary`, maxsalary', 'salary_unit' => 1)
        );
        $jobList = !empty($jobList['list']) ? $jobList['list'] : array();
        $this->yunset('jobList', $jobList);

        if (isset($_GET['from'])){
            $backurl    =   Url('wap', array('c' => 'company', 'a' => 'show', 'id' => $id));
        }else{
            $backurl    =   Url('wap', array(), 'member');
        }
        $this->yunset('backurl', $backurl);

        $this->yunset('id', $id);

        $this->seo('whb');
        $this->yunset('headertitle', '企业微海报生成');
        $this->yuntpl(array('wap/hb/whb'));
    }
	/**
	 * 微信内上拉加载
	 */
	function ajaxLoad_action(){
	    
	    $param = array();
	    $searchurl = explode('&', $_POST['searchurl']);
	    foreach ($searchurl as $v){
	        $p = explode('=', $v);
	        $param[$p[0]] = $p[1];
	    }
	    
	    $where['name']		=	array('<>','');
	    $where['r_status']	=	1;
	    $keyword		    =	$this->stringfilter($param['keyword']);
	    $page				=	$_POST['page'];
	    $order				=	$param['order'];
	    $provinceid			=	(int)$param['provinceid'];
	    $cityid				=	(int)$param['cityid'];
	    $three_cityid		=	(int)$param['threecityid'];
	    $hy					=	(int)$param['hy'];
	    $pr					=	(int)$param['pr'];
	    $mun				=	(int)$param['mun'];
	    $rec				=	(int)$param['rec'];
	    
	    if($hy){//类别ID
	        $where['hy']			=	$hy;
	    }
	    if($provinceid){//类别ID
	        $where['provinceid']	=	$provinceid;
	    }
	    if($cityid){//类别ID
	        $where['cityid']		=	$cityid;
	    }
	    if($three_cityid){//类别ID
	        $where['three_cityid']	=	$three_cityid;
	    }
	    if($keyword){//关键字
	        $where['name']			=	array('like',$keyword);
	    }
	    if($rec==1){//名企
	        $where['rec']			=	1;
	        $where['hottime']		=	array('>',time());
	    }
	    if($pr){//企业性质
	        $where['pr']			=	$pr;
	    }
	    if($mun){//企业规模
	        $where['mun']			=	$mun;
	    }
	    // 处理分站查询条件
	    if ($this->config['sy_web_site'] == 1){
	        
	        if ($this->config['province'] > 0){
	            $where['provinceid']  =  $this->config['province'];
	        }
	        if ($this->config['cityid'] > 0){
	            $where['cityid']  =  $this->config['cityid'];
	        }
	        if ($this->config['three_cityid'] > 0){
	            $where['three_cityid']  =  $this->config['three_cityid'];
	        }
	        if ($this->config['hyclass'] > 0){
	            $where['hy']  =  $this->config['hyclass'];
	        }
	    }
	    if($order){//排序
	        $where['orderby']		=	$order;
	    }else{
	        $where['orderby']		=	'`lastupdate`,desc';
	    }
	    $limit = 20;
	    if($page){//分页
	        $pagenav				=	($page-1)*$limit;
	        $where['limit']			=	array($pagenav,$limit);
	    }else{
	        $where['limit']			=	$limit;
	    }
	    
	    $ComM   =  $this -> MODEL('company');
	    $field  =  '`uid`,`name`,`shortname`,`logo`,`logo_status`,`yyzz_status`,`hotstart`,`hottime`,`provinceid`,`cityid`,`three_cityid`,`pr`,`rating`,`mun`,`fact_status`';
	    $jfield =  '`id`,`uid`,`name`';
	    $rows   =  $ComM -> getList($where,array('field'=>$field,'jobfield'=>$jfield,'utype'=>'wxapp','logo'=>1,'url'=>1));
	    
	    $list	=  count($rows['list']) ? $rows['list'] : array();
	    
	    echo json_encode($list);die;
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