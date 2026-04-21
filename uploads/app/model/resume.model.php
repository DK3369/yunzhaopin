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
class resume_model extends model{

    private $resume_tables = array(
        'resume_expect',
        'resume_work',
        'resume_edu',
        'resume_training',
        'resume_skill',
        'resume_project',
        'resume_other',
        'resume_doc',
        'resume_show',
    );

    public $resume_state_arr = array(
            '0'=>'未审核',
            '1'=>'已审核',
            '2'=>'被举报',
            '3'=>'未通过'
        );
    public $resume_status_arr = array(
            '1'=>'公开',
            '2'=>'隐藏',
            '3'=>'仅投递企业可见'
        );
    public $resume_rstatus_arr = array(
            '0'=>'未审核',
            '1'=>'已审核',
            '2'=>'账户被锁定',
            '3'=>'未通过'
        );
	/* 查询数量 */
	function getResumeNum($Where=array()){
	    return $this->select_num('resume',$Where);
	}
	/* 查询简历数 */
	function getExpectNum($Where=array()){
	    return $this->select_num('resume_expect',$Where);
	}
    /*查询简历投递数*/
    function getUseridJobNum($Where=array()){
        return $this->select_num('userid_job',$Where);
    }
    /*查询简历刷新数*/
    function getRefExpectNum($Where=array()){
        return $this->select_num('resume_refresh_log',$Where);
    }
    /*查询简历浏览数*/
    function getLookResumeNum($Where=array()){
        return $this->select_num('look_resume',$Where);
    }
	/**
	 * 通过uid查询单条简历。有默认简历返回默认简历；没有默认简历，返回最近更新的简历
	 */
    function getExpectByUid($uid = '', $data = array()){

        $expect = $this->getExpect(array('uid'=>$uid,'defaults'=>1), $data);

        if (empty($expect)){

            $expect = $this->getExpect(array('uid'=>$uid,'orderby'=>'`id`'), $data);
        }

        return $expect;
    }
    private function addErrorLog($uid,$type='',$content) {

        require_once ('errlog.model.php');

        $ErrlogM = new errlog_model($this->db, $this->def);

        return  $ErrlogM -> addErrorLog($uid, $type, $content);

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
	 * 批量查个人基本信息
	 * @param array $whereData
	 * @param string $field
	 */
	function getResumeList($whereData, $data = array('field' => null, 'utype'=>null)){
        $field  =   $data['field'] ? $data['field'] : '*';

        $List   =   $this->select_all('resume', $whereData, $field);
        $cache  =   $this->getClass(array('user'));

        //所获列表数据是否要对下载后的简历信息处理
        $downuids   =	  array();

        if(!empty($data['downresume_where'])){

        	$dresume	=	$this -> select_all('down_resume',$data['downresume_where'],'`uid`');

        	foreach($dresume as $dv){

        		$downuids[]	  =	  $dv['uid'];
        	}
        }

        foreach ($List as $k => $v) {

            $photoArr  =  array('photo' => $v['photo'],'defphoto' => $v['defphoto'], 'phototype'=> $v['phototype'], 'photo_status' => $v['photo_status'], 'sex' => $v['sex']);

            if($v['photo']){
                $List[$k]['photo_o']   =   $v['photo'];
            }

            if ($data['utype'] == 'admin'){
                $List[$k]['cert_time_n'] = $v['cert_time'] ? date('Y-m-d H:i', $v['cert_time']) : '';
                $List[$k]['photo']   =   checkpic($v['photo']);
            }else{

                $List[$k]['photo']   =   $this -> setResumePhotoShow($photoArr);
            }
            //已下载过的简历呈现给商家时为原名
            if(!empty($data['downresume_where']) && !empty($downuids) && in_array($v['uid'], $downuids)){

                $List[$k]['name_n']  =   $v['name'];

            }else{
            	$nameArr             =   array('nametype' => $v['nametype'], 'name' => $v['name'], 'eid' => $v['def_job'], 'sex' => $v['sex']);

            	$List[$k]['name_n']  =   $this -> setUsernameShow($nameArr);
            }

            $List[$k]['username_n']  =   $v['name'];

            unset($List[$k]['name']);

            if ($v['marriage']) {
                $List[$k]['marriage_n'] =   $cache['userclass_name'][$v['marriage']];
            }
            if ($v['sex']) {
                $List[$k]['sex_n']      =   $cache['user_sex'][$v['sex']];
            }
            if ($v['edu']) {
                $List[$k]['edu_n']      =   $cache['userclass_name'][$v['edu']];
            }
            if ($v['exp']) {
                $List[$k]['exp_n']      =   $cache['userclass_name'][$v['exp']];
            }
            if ($v['birthday']) {
                $List[$k]['age_n']      =   date('Y') - date('Y',strtotime($v['birthday']));
            }
            if ($v['idcard_pic']) {
                $List[$k]['idcard_pic'] =	checkpic($v['idcard_pic']);
            }
            if ($v['tag']) {
                $List[$k]['tag_arr'] =   @explode(',',$v['tag']);
            }
            $List[$k]['email_status_n']		=			$v['email_status'];
            $List[$k]['moblie_status']		=			$v['moblie_status'];
            $List[$k]['idcard_status']		=			$v['idcard_status'];
            $List[$k]['idcard']				=			$v['idcard'];
            $List[$k]['description']        =           str_replace(array('\r','\n'), array('<br/>','<br/>'), strip_tags($v['description'],'\r,\n'));
        }
        if ($data['utype']=='admin') {

            $List   =   $this->getDataUserList($List);

        }
        return $List;
    }

    /**
     * @desc 获取resume_expect 列表，简单的查询返回，复杂的，访问getList方法
     *
     * @param $whereData    查询条件
     * @param array $data   自定义处理数组
     * @return array|bool|false|string|void
     */
    public function getSimpleList($whereData, $data = array())
    {

        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $List           =   $this->select_all('resume_expect', $whereData, $data['field']);
        foreach ($List as $k => $v) {
            
            if (isset($v['lastupdate'])){
                $List[$k]['lastupdate'] = date('Y-m-d H:i:s', $v['lastupdate']);
            }
        }

        return $List;
    }
    //搜索筛选-简历列表
    public function searchList($data){

        $uid        = $data['uid'];
        $usertype   = $data['usertype'];
        unset($data['uid']);
        unset($data['usertype']);
    	$_POST	=	$data;
        $where 	= 	"a.`state`=1 AND a.`r_status`=1 AND a.`defaults`=1 AND a.`status`=1";

        $page	=	$_POST['page'];
        $limit	=	$_POST['limit'];
        $time	=	time();
        $CacheArr	=	$this -> getClass(array('user','city'));
        if($_POST['exp']){
            // 简历搜索，排序比搜索小的都符合条件。如搜“五年”，类别排序大于等于“五年”排序的（要排除不限）都符合
            if (stripos($CacheArr['userclass_name'][$_POST['exp']], '应届生') !== false){
                // 应届生特殊，只搜应届生
                $where .= " AND a.`exp`=".$_POST['exp'];
            }else{
                $expArr  = $CacheArr['userdata']['user_word'];
                $expSort = 0;
                $expIds  = array();
                foreach ($expArr as $k => $v) {
                    if ($v == $_POST['exp'] && $CacheArr['userclass_name'][$v] != '不限'){
                        $expSort = $k;
                        break;
                    }
                }
                foreach ($expArr as $k => $v) {
                    if ($k >= $expSort && $CacheArr['userclass_name'][$v] != '不限'){
                        $expIds[] = $v;
                    }
                }
                if ($expIds) {
                    $where .= " AND a.`exp` in (".implode(",", $expIds).")";
                }
            }
        }


        //性别
        if($_POST['sex']){
            $where .= " AND a.`sex`='".$_POST['sex']."'";
        }
        if($_POST['edu']){
            // 简历搜索，排序比搜索大的都符合条件。如搜“硕士”，类别排序大于等于“硕士”排序的（要排除不限）都符合
            $eduArr  = $CacheArr['userdata']['user_edu'];
            $eduSort = 0;
            $eduIds  = array();
            foreach ($eduArr as $k => $v) {
                if ($v == $_POST['edu'] && $CacheArr['userclass_name'][$v] != '不限'){
			        $eduSort = $k;
			        break;
			    }
			}
			foreach ($eduArr as $k => $v) {
			    if ($k >= $eduSort && $CacheArr['userclass_name'][$v] != '不限'){
			        $eduIds[] = $v;
			    }
			}
            if ($eduIds) {
                $where .= " AND a.`edu` in (".implode(",", $eduIds).")";
            }
        }
        if($_POST['type']){
            $where .= " AND a.`type`='".$_POST['type']."'";
        }
        if($_POST['report']){
            $where	.=	" AND a.`report`='".$_POST['report']."'";
        }
        if($_POST['rec_resume']){
            $where .=   " AND a.`rec_resume`=1";
        }
        if($_POST['integrity']){

            include(CONFIG_PATH.'db.data.php');

            $integrity_val  =   $arr_data['integrity_val'];

            $integrity		=   $integrity_val[$_POST['integrity']];

            $where  .=  " AND a.`integrity`>='".$integrity."'";
        }
        if($_POST['tag']){
            $tagname=$CacheArr['userclass_name'][$_POST['tag']];
            $rwhere['def_job']	=	array('>',0);
            $rwhere['r_status']	=	1;
            $rwhere['status']	=	1;
            $rwhere['tag']		=	array('findin',$tagname);
            $tag	=	$this->select_all("resume",$rwhere,"`def_job`");
            if(is_array($tag)){
                foreach($tag as $v){
                    $tagid[]=$v['def_job'];
                }
            }
            $where	.=	" AND a.`id` in (".pylode(",",$tagid).")";
        }
        if($_POST['uptime']){
            if($_POST['uptime']==1){
                $beginToday =	strtotime(date('Y-m-d 00:00:00'));
                $where     .=	" AND a.`lastupdate`>$beginToday";
            }else{
                $uptime     =	time()-($_POST['uptime']*86400);
                $where     .=	" AND a.`lastupdate`>$uptime";
            }
        }else{
            if($this->config['sy_datacycle']>0){
                // 后台-页面设置-数据周期
                $uptime = strtotime('-'.$this->config['sy_datacycle'].' day');
                $where.=" AND a.`lastupdate`>$uptime";
            }
        }
        if($_POST['hy']){
            $where .= " AND a.`hy`='".$_POST['hy']."'";
        }

        $cjwhere = '1';
        $job_col = $city_col = '';
        if($_POST['three_cityid']){
            $city_col = "three_cityid";
            $cjwhere .= " AND `$city_col`= $_POST[three_cityid]";
        }elseif($_POST['cityid']){
            $city_col = "cityid";
            $cjwhere .= " AND `$city_col`= $_POST[cityid]";
        }elseif($_POST['provinceid']){
            $city_col = "provinceid";
            $cjwhere .= " AND `$city_col`= $_POST[provinceid]";
        }
        if($_POST['job_post']){
            $job_col = "job_post";
            $cjwhere .= " AND `$job_col`= $_POST[job_post]";
        }elseif($_POST['job1_son']){
            $job_col = "job1_son";
            $cjwhere .= " AND `$job_col`= $_POST[job1_son]";
        }elseif($_POST['job1']){
            $job_col = "job1";
            $cjwhere .= " AND `$job_col`= $_POST[job1]";
        }
        if ($_POST['keyword']){
            $keyword	=	$_POST['keyword'];
            $where1[]	=	"a.`name` LIKE '%$keyword%'";
            $where1[]	=	"a.`uname` LIKE '%$keyword%'";
            $cityid		=	array();
            foreach($CacheArr['city_name'] as $k=>$v){
                if(strpos($v,$keyword)!==false){
                    $cityid[]	=	$k;
                }
            }
            //只取匹配到的第一个城市，已选省则匹配省下面的城市、未选择城市则按关键字匹配城市
            if(!empty($cityid)){

				$ckwhere['PHPYUNBTWSTART_A']  =   '';
                $ckwhere['cityid']            =   array('in',pylode(',',$cityid),'OR');
                $ckwhere['three_cityid']      =   array('in',pylode(',',$cityid),'OR');
                $ckwhere['PHPYUNBTWEND_A']    =   '';

				$cityresume	=	$this->select_all("resume_cityclass",$ckwhere);
				if($cityresume){
				    $rc = array();
					foreach ($cityresume as $v){
				        $rc[] = $v['eid'];
					}
					$where1[]  =  'a.`id` IN ('.pylode(',', $rc).')';
				}
			}
			//搜索工作经历
			$workList = $this->select_all('resume_work', array('title'=>array('like',$keyword), 'content'=>array('like',$keyword, 'OR'),'orderby'=>'id', 'limit'=>500), 'eid');
			if(!empty($workList)){
				$workId = array();
				foreach($workList as $v){
					$workId[] = $v['eid'];
				}
				$where1[]  =  'a.`id` IN ('.pylode(',', $workId).')';
			}
            $where	.=	" AND (".@implode(" or ",$where1).")";
        }

        if(!empty($_POST['zdids'])){
            $where	.=	" AND a.`id` not in (".pylode(',',$_POST['zdids']).")";
        }

        $order	=	" order by a.`lastupdate` desc";
        if($page){//分页
            $pagenav	=	($page-1)*$limit;
            $resumelimit=" limit $pagenav,$limit";
        }else{
            $resumelimit=" limit $limit";
        }
        // 拼接唯一标识字段
        if($city_col || $job_col){
            if($city_col && $job_col){
                $cjwhere .= " AND `{$city_col}_{$job_col}_num`= 1";
            }elseif($city_col){
                $cjwhere .= " AND `{$city_col}_num`= 1";
            }elseif($job_col){
                $cjwhere .= " AND `{$job_col}_num`= 1";
            }
        }
        $pagewhere	=	"";
        $joinwhere	=	"";
        if($cjwhere != '1'){
            $pagewhere	.=	" ,(select `eid` from `".$this->def."resume_city_job_class` where ".$cjwhere." group by `eid`) cj";
            $joinwhere	.=	" a.`id`=cj.`eid` and ";
        }
        $select	=	"a.`id`,a.`uid`,a.`name`,a.`hy`,a.`job_classid`,a.`city_classid`,a.`jobstatus`,a.`type`,a.`photo`,a.`report`,a.`lastupdate`,a.`rec`,a.`rec_resume`,a.`ctime`,a.`uname`,a.`idcard_status`,a.`minsalary`,a.`maxsalary`";
        if($pagewhere!=""){
            $sql	=	"select ".$select." from `".$this->def."resume_expect` a ".$pagewhere." where ".$joinwhere.$where.$order.$resumelimit;
        }else{
            $sql	=	"select ".$select." from `".$this->def."resume_expect` a where ".$where.$order.$resumelimit;
        }

        $rows		=	$this->DB_query_all($sql,'all');
        if($rows&&is_array($rows)){
            if ($page==1){
            	$zdids		=	array();
            	$select     .=  ',a.`top`,a.`topdate`';
                $where		.=	' AND a.`top`=1 AND a.`topdate`>"'.$time.'"';
                if($pagewhere!=""){
                    $sql 	= "select ".$select." from `".$this->def."resume_expect` a ".$pagewhere." where ".$joinwhere.$where.' ORDER BY rand() limit 5';
                }else{
                    $sql 	= "select ".$select." from `".$this->def."resume_expect` a where ".$where.' ORDER BY rand() limit 5';
                }
                $topresume	=	$this->DB_query_all($sql,'all');
                if ($topresume){
                    foreach($rows as $k=>$v){
                        foreach ($topresume as $val){
                            if($v['id']==$val['id']){
                                unset($rows[$k]);
                            }
                        }
                    }
                    foreach ($topresume as $v){
                    	$zdids[]	=	$v['id'];
                        array_unshift($rows,$v);
                    }
                }
            }
        }
        $data['row']=$rows;
        $data['zdids']=$zdids;
        return $data;
    }
	/**
	 * 简历列表
	 * @param array $whereData
	 * @param array $data
     * field:查询字段;
     * 'utype':列表显示位置，不同的位置可能有不同的需求 ;
     * 'cache':页面需要传回缓存 多条数据查询合并，
     * withResumeFiled:联合resume表 多条数据查询合并时查询resume表的字段
     * downresume_where：在联合resume表查询时，查询down_resume表的条件
     * look_where：查询是否浏览过简历
     * search：列表搜索的post值,主要用于列表搜索筛选
	 */
	public function getList($whereData,$data=array('utype'=>'')){

	    $ListNew	=	array();

	    $field      =   $data['field'] ? $data['field'] : '*';

  	    if(!empty($data['search'])){

  	    	$rdata	=	$this	->	searchList($data['search']);
  	    	$List	=	$rdata['row'];
  	    	$zdids	=	$rdata['zdids'];
        }elseif(!empty($data['sql'])){

  	        $List = $this->DB_query_all($data['sql'], 'all');
        }else{
            $List	=	$this	->	select_all('resume_expect',$whereData,$field);
        }

	    if(!empty($List)){
	        //真正的列表查询，需要处理列表数据，一般的简单查询，不需要处理
	        $time    =   time();
	        $cache   =   $this -> getClass(array('user','city','hy','job'));

	        foreach($List as $k=>$v){

	            $euids[]				=  $v['uid'];

	            $List[$k]['age_n']		=  date('Y') - date('Y',strtotime($v['birthday']));

	            $List[$k]['hy_n']		=  $cache['industry_name'][$v['hy']] ? $cache['industry_name'][$v['hy']] : '不限';

	            $List[$k]['sex_n']		=  $cache['user_sex'][$v['sex']];

	            $List[$k]['edu_n']		=  $cache['userclass_name'][$v['edu']];

	            $List[$k]['exp_n']		=  $cache['userclass_name'][$v['exp']];

	            $List[$k]['report_n']	=  $cache['userclass_name'][$v['report']];

	            $List[$k]['type_n']		=  $cache['userclass_name'][$v['type']];

                $List[$k]['label_n']	=  $cache['userclass_name'][$v['label']];

                $List[$k]['content_n']	=  str_replace(array("\r\n","\r","\n"), array("","",""), strip_tags($v['content'],'\r\n'));
                if($v['lastupdate']){
                    $ltime				=  $v['lastupdate'];

                    if ($data['utype'] == 'admin') {
                        $List[$k]['lastupdate_n']	=	date("Y-m-d H:i", $ltime);
                    } else {
                        //今天开始时间戳
                        $beginToday			=  strtotime('today');
                        //昨天开始时间戳
                        $beginYesterday		=  strtotime('yesterday');
                        //一周内时间戳
                        if($ltime>$beginYesterday && $ltime<$beginToday){
                            $List[$k]['lastupdate_n']	=	"昨天";
                        }elseif($ltime>$beginToday){
                            $List[$k]['lastupdate_n']	=	lastupdateStyle($ltime);
                        }else{
                            $List[$k]['lastupdate_n']	=	date("Y-m-d",$ltime);
                        }
                    }
                }
                if(isset($v['ctime'])){
                    $todaystart =   strtotime('today');
                    $beforeYesterday    =   $todaystart - 86400 * 2;
                    if($v['ctime']>$beforeYesterday){
                        $List[$k]['newtime']  =  1;
                    }
                    $List[$k]['ctime_n'] = date('Y-m-d H:i',$v['ctime']);
                }

	            //处理薪资
	            $List[$k]['salary']  =  salaryUnit($v['minsalary'], $v['maxsalary']);
	            //处理职位类别id
	            if ($v['job_classid'] && $cache){

	                $job_classid = @explode(',',$v['job_classid']);

	                if(is_array($job_classid)){

	                    $jobclassid     =  array();
	                    $job_classname  =  array();

	                    foreach($job_classid as $jv){

	                        if($cache['job_name'][$jv]){

	                            $jobclassid[]     =  $jv;

	                            $job_classname[]  =  $cache['job_name'][$jv];
	                        }
	                    }
	                    $List[$k]['job_classid']    =  pylode(',',$jobclassid);

	                    $List[$k]['job_classname']  =  @implode(',',$job_classname);
	                }
	            }
	            //处理城市类别id
	            if ($v['city_classid'] && $cache){

	                $city_classid = @explode(',',$v['city_classid']);

	                if(is_array($city_classid)){

	                    $cityclassid     =  array();
	                    $city_classname  =  array();

	                    foreach($city_classid as $cv){

	                        if($cache['city_name'][$cv]){

	                            $cityclassid[]     =  $cv;

	                            $city_classname[]  =  $cache['city_name'][$cv];
	                        }
	                    }
	                    $List[$k]['city_classid']    =  pylode(',',$cityclassid);

	                    $List[$k]['city_classname']  =  @implode(' ',$city_classname);
	                }
	            }
	            //处理置顶天数
	            if($v['topdate'] > $time){

	                $List[$k]['top_day']	=	ceil(($v['topdate']-$time)/86400);

	            }else{

	                $List[$k]['top_day'] = '0';
	            }

                if ($v['topdate']) {
                    $List[$k]['topdate_n'] = date('Y-m-d', $v['topdate']);
                }

	            if (isset($v['photo'])){

	                $icon  =  $v['sex'] == 1 ? $this->config['sy_member_icon'] : $this->config['sy_member_iconv'];

	                $List[$k]['photo_n']  =  checkpic($v['photo'], $icon);
	            }

	            if (isset($data['url'])){

	                $List[$k]['wapurl']  =  Url('wap', array('c'=>'resume','a'=>'show','id'=>$v['id']));
	            }
	        }
	        //后台处理简历多城市/用户名、审核状态
	        if ($data['utype'] == 'admin'){

	            $List  =   $this -> getDataList($List,$cache);
	        }

            //联合resume表 多条数据 查询合并
            if ($data['withResumeField']) {

                $resumeList =   $this->getResumeList(array('uid' => array('in', pylode(',', $euids))), array('field' => $data['withResumeField'], 'downresume_where' => $data['downresume_where']));

                foreach ($List as $lk => $lv) {
                    foreach ($resumeList as $rk => $rv) {
                        if ($rv['uid'] == $lv['uid']) {

                            $List[$lk] = array_merge($lv, $rv);
                        }
                    }
                }
            }

	        //浏览简历列表类查询个人姓名使用
	        if ($data['utype'] == 'user'){

	            $List  =   $this -> getJlData($List);
	        }

            if($data['workexp']){//学历与经验简化显示

                $List  =   $this -> getWorkexpData($List);
            }

            //  浏览简历
            if (isset($data['look_where']) && !empty($data['look_where'])){
                $List[$k]['isLookEd']   =   0;
                $lookResumeList         =   $this->select_all('look_resume', $data['look_where'], '`resume_id`');
                if (!empty($lookResumeList)){
                    $lookResumeIdArr    =   array();
                    foreach ($lookResumeList as $lv){
                        $lookResumeIdArr[]  =   $lv['resume_id'];
                    }
                    foreach ($List as $k=>$v){
                        if (in_array($v['id'], $lookResumeIdArr)){
                            $List[$k]['isLookEd']   =   1;
                        }
                    }
                }
            }

	        $ListNew['list']	=	$List;

	        //app简历列表返回第一页的置顶简历id数组集
	        if(isset($zdids) && !empty($zdids)){
	        	$ListNew['zdids']	=	$zdids;
	        }

	        //需要缓存的
	        if ($data['cache']){

	            $ListNew['cache']  =   $cache;
	        }
	    }
	    return	$ListNew;
	}

    function getWorkexpData($List){

        $cache          =   $this -> getClass(array('user'));

        $userclass_name =   $cache['userclass_name'];

        $eids           =   array();

        foreach ($List as $lv){

            $eids[]  =  $lv['id'];
        }

        if(!empty($eids)){

            $eduList    = $this-> select_all("resume_edu",array('eid'=>array('in',pylode(',',$eids))));
            $workList   = $this-> select_all("resume_work",array('eid'=>array('in',pylode(',',$eids))));
        }

        if(!empty($eduList)){
            foreach($eduList as $key=>$value){
                $eduListNew[$value['eid']][] = $value;
            }
            foreach($eduListNew as $k=>$eduv){
                $edumin             =   0;
                $edumax             =   0;
                $edutitle           =   array();
                $education          =   array();
                foreach($eduv as $v){

                    $edumin         =   $v['sdate'];

                    if($v['edate']==0 ){
                        $edumax     =   0;
                    }elseif($edumax<$v['edate']){
                        $edumax     =   $v['edate'];
                    }
                    $education[]    =   $userclass_name[$v['education']];
                    $edutitle[]     =   $v['specialty'];
                }
                $return             =   array();
                $return['edumin']   =   date('Y.m',$edumin);
                $return['edumax']		=		$edumax  == 0 ?  '至今': date('Y',$edumax);
                $return['education']=   @implode(',',$education);
                $return['edu_time'] =   $return['edumin']."-".$return['edumax'];
                $return['eduspecialty']		=		@implode('、',$edutitle);
                if($return['eduspecialty']){
                    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · '.$return['eduspecialty'].' · 毕业于'.$return['edumax'].'年';
                }else{
                    $workexpList[$k]['edu_content']	=	$return['education'].'学历 · 毕业于'.$return['edumax'].'年';
                }
            }
        }

        if(!empty($workList)){
            foreach($workList as $key=>$value){
                $workListNew[$value['eid']][] = $value;
            }
            foreach($workListNew as $k=>$workv){

                $whour          =   0;
                $hour           =   array();
                $time           =   time();
                $workmin        =   0;
                $workmax        =   0;
                $worknum        =   count($workv);
                $wtitle         =   array();
                foreach($workv as $v){

                    if ($v['edate']){
                        $workTime   =   ceil(($v['edate']-$v['sdate'])/(30*86400));
                    }else{
                        $workTime   =   ceil(($time - $v['sdate'])/(30*86400));
                    }
                    /* 计算每份工作时长(按月) */
                    $workmin        =   $v['sdate'];

                    if($v['edate']==0 ){
                        $workmax    =   0;
                    }elseif($workmax<$v['edate']){
                        $workmax    =   $v['edate'];
                    }

                    $wtitle[]   =   $v['title'];

                    $hour[]     =   $workTime;
                    $whour      +=  $workTime;
                }

                $workavg    =   ceil($whour/count($hour));
                $return     =   array();
                $return['worknum']      =   $worknum  > 0 ?  $worknum:0;

                $return['workavg']      =   $workavg  > 11 ?  avgToYm($workavg) : $workavg;

                $return['workmin']      =   date('Y.m',$workmin);
                $return['workmax']      =   $workmax  == 0 ?  '至今': date('Y.m',$workmax);
                $return['worktit']      =   @implode(',',$wtitle);
                $return['work_time']    =   $return['workmin'].'-'.$return['workmax'];
                if($return['workavg']>0){
                    if($return['worktit']!=''){
                        $workexpList[$k]['work_content']  =  ' 参加过'.$return['worknum'].'份工作 · 涉及'.$return['worktit'].'等岗位';
                    }else{
                        $workexpList[$k]['work_content']  =   '参加过'.$return['worknum'].'份工作';
                    }
                }
            }
        }

        if(!empty($workexpList)){

            foreach ($List as $k=>$v){

                $List[$k]['work_content']   =   '';
                $List[$k]['edu_content']    =   '';

                foreach($workexpList as $wk=>$wv){

                    if($wk==$v['id']){

                        $List[$k]['work_content']   =   $wv['work_content'] ? $wv['work_content'] : '';
                        $List[$k]['edu_content']    =   $wv['edu_content'] ? $wv['edu_content'] : '';
                    }
                }
            }
        }

        return $List;
    }

    /**
     * @desc    获取resume    详情 ; 简单的查询返回，复杂的，访问getInfo方法
     * @param $whereData    查询条件
     * @param null[] $data  自定义处理数组
     * @return array|bool|false|string|void
     */
    public function getResumeInfo($whereData, $data = array('logo'=>null))
    {

        $data['field']  =  empty($data['field']) ? '*' : $data['field'];
        $resumeInfo		=  $this -> select_once('resume', $whereData, $data['field']);

		if(!empty($resumeInfo)){

			$cache  =  $this -> getClass('user');

			$userclass_name =   $cache['userclass_name'];

			if($resumeInfo['birthday']){
                if (strlen($resumeInfo['birthday'] > 7)) { // 原来数据生日格式为年月日，如果是旧格式，改为年月
                    
					$resumeInfo['birthday'] = date('Y-m',strtotime($resumeInfo['birthday']));
                }
				$a					        =	date('Y',strtotime($resumeInfo['birthday']));
                $resumeInfo['birthYear']    =   date('Y',strtotime($resumeInfo['birthday']));
				$resumeInfo['age']	        =	date("Y")-$a;
			}
            $resumeInfo['wapurl']             =   Url('wap', array('c'=>'resume','a'=>'show','id'=>$resumeInfo['def_job']));
			$resumeInfo['sex_n']	        =	$resumeInfo['sex'] == 1?'男':'女';

            if ($resumeInfo['nametype'] == 1) {

                $resumeInfo['nametype_n']   =   '完全公开';
            } elseif ($resumeInfo['nametype'] == 2) {

                $resumeInfo['nametype_n']   =   '显示编号(例:NO.11)';
            } elseif ($resumeInfo['nametype'] == 3) {

                $resumeInfo['nametype_n']   =   '性别称呼(例:X先生)';
            } else {

                $resumeInfo['nametype_n']   =   '完全公开';
            }
            if ($resumeInfo['marriage']) {

                $resumeInfo['marriage_n']   =   $userclass_name[$resumeInfo['marriage']];
            }
            if ($resumeInfo['edu']) {

                $resumeInfo['edu_n']        =   $userclass_name[$resumeInfo['edu']];
            }
            if ($resumeInfo['exp']) {

                $resumeInfo['exp_n']        = $userclass_name[$resumeInfo['exp']];
            }

			if (!empty($resumeInfo['tag'])){
				$resumeInfo['arrayTag']		=	explode(',', $resumeInfo['tag']);
			}

		    if (!empty($whereData['uid'])){

		        if(isset($data['setname']) && $data['setname'] == 1){

					require_once ('userinfo.model.php');
					$UserinfoM  =  new userinfo_model($this->db, $this->def);
					$info	    =	$UserinfoM -> getInfo(array('uid'=> $whereData['uid']),array('setname'=>'1'));

					$resumeInfo['setname']  =   $info['setname']=='1'?1:0;
				}

		        if($data['logo'] == 1){
		            // 个人查看
		            $icon  =  $resumeInfo['sex'] == 1 ? $this->config['sy_member_icon'] : $this->config['sy_member_iconv'];

		            $resumeInfo['photo_n']	   =  $resumeInfo['photo'];
		            $resumeInfo['photo']	   =  checkpic($resumeInfo['photo'],$icon);
		            $resumeInfo['wxewm']	   =  checkpic($resumeInfo['wxewm']);
		            $resumeInfo['idcard_pic']  =  checkpic($resumeInfo['idcard_pic']);

		        }elseif($data['logo'] == 2){
		            // 非个人查看
		            if (empty($resumeInfo['photo_status']) || empty($resumeInfo['phototype']) || empty($resumeInfo['sex'])){

		                $resume  =  $this -> select_once('resume', $whereData, '`photo_status`,`phototype`,`sex`');

		                $resumeInfo['photo_status']  =  $resume['photo_status'];
		                $resumeInfo['phototype']     =  $resume['phototype'];
		                $resumeInfo['sex']           =  $resume['sex'];
		            }
		            $resumeInfo['photo']  =  $this -> setResumePhotoShow(array('photo' => $resumeInfo['photo'],'defphoto' => $resumeInfo['defphoto'], 'photo_status'=>$resumeInfo['photo_status'],'phototype' => $resumeInfo['phototype'], 'sex' => $resumeInfo['sex']));
                    $resumeInfo['wxewm_n']	   =  checkpic($resumeInfo['wxewm']);
		        }
		    }
		}
        return $resumeInfo;
	}

    /**
     * @desc 更新个人基本信息操作
     * @param array $whereData
     * @param null[] $data | mData member表要修改数据;  rData resume表要修改数据;  utype 修改操作类型：admin-后台，user-会员中心; port 刷新简历操作端
     * @return array|string[]
     */
    public function upResumeInfo($whereData = array(), $data = array('mData' => null, 'rData' => null, 'utype' => null, 'port' => 1))
    {

        $return =   array();

        require_once('warning.model.php');
        $warningM       =   new warning_model($this->db, $this->def);

        if (is_array($whereData) && !empty($whereData)) {

            $mData  =   $data['mData'];
            $rData  =   $data['rData'];

            $rField =   '`living`,`lastupdate`,`moblie_status`,`email_status`,`idcard_status`';
            $resume =   $this->select_once('resume', array('uid' => $whereData['uid']), $rField);

            //会员操作的修改，需要判断手机号、邮件是否已绑定,身份证是否已验证，绑定的不能修改
            if ($data['utype'] == 'user') {

                if (!empty($resume)) {

                    if ($resume['moblie_status'] == '1') {

                        if (!empty($mData) && $mData['moblie']) {

                            unset($mData['moblie']);
                        }
                        if (!empty($rData) && $rData['moblie']) {

                            unset($rData['moblie']);
                        }
                    }
                    if ($resume['email_status'] == '1') {

                        if (!empty($mData) && $mData['email']) {

                            unset($mData['email']);
                        }
                        if (!empty($rData) && $rData['email']) {

                            unset($rData['email']);
                        }
                    }
                    if ($resume['idcard_status'] == '1') {

                        if (!empty($rData) && $rData['name']) {
                            unset($rData['name']);
                        }
                    }
                }
            }

            // 处理会员基本信息
            if (!empty($mData)) {

                require_once('userinfo.model.php');
                $UserinfoM  =   new userinfo_model($this->db, $this->def);
                $ckresult   =   $UserinfoM->addMemberCheck($mData, $whereData['uid'], $data['utype']);

                if (isset($ckresult) && $ckresult['msg']) {

                    $ckresult['errcode'] = 8;

                    return $ckresult;
                }
            }

            // 处理个人基本信息
            if (!empty($rData)) {
                //处理手机站个人二维码
                if (!empty($rData['preview']) || !empty($rData['ewmFromWx'])) {

                    $upArr  =   array(
                        'base'  =>  $rData['preview'],
                        'dir'   =>  'user',
                        'file'  =>  $rData['ewmFromWx']
                    );

                    $result =   $this->upload($upArr);

                    if (!empty($result['msg'])) {

                        $return['msg']      =   $result['msg'];
                        $return['errcode']  =   8;
                        return $return;
                    } else {

                        $rData['wxewm']     =   $result['picurl'];
                    }
                    unset($rData['preview']);
                    unset($rData['ewmFromWx']);
                }

                $return['id']   =   $this->update_once('resume', $rData, $whereData);

                if ($return['id']) {

                    //处理求职意向相关数据
                    $expect     =   $this->setExpectInfo($whereData['uid'], $rData, $port = $data['port']);

                    //处理聊天头像数据
                    $chatmember	=	$this->select_all('chat_member', array('uid'=>$whereData['uid'], 'usertype'=>1));
                    if($rData['photo']&&!empty($chatmember)){
                        $this -> update_once('chat_member',array('avatar'=>$rData['photo']), array('uid'=>$whereData['uid'], 'usertype'=>1));
                    }

                    //处理用户提交内容
                    if ($data['utype'] == 'user') {

                        require_once('cookie.model.php');
                        $cookieM    =   new cookie_model($this->db, $this->def);
                        $cookieM->SetCookie('delay', '', time() - 60);

                        // 会员日志
                        $this->addMemberLog($whereData['uid'], 1, '基本信息：完善基本信息', 7, 2);

                        // 首次完善获取积分
                        if ($resume['name'] == '' || $resume['living'] == '') {

                            require_once('integral.model.php');
                            $IntegralM = new integral_model($this->db, $this->def);
                            $IntegralM->invtalCheck($whereData['uid'], 1, 'integral_userinfo', '首次填写基本资料', 25);
                        }

                        $status         =   '1';

                    }
                    // 基本信息更新，调整完善度
                    $urlist = $this->select_all('user_resume', array('uid'=>$whereData['uid'], 'info'=>0));
                    if (!empty($urlist)){
                        $eids = array();
                        foreach ($urlist as $v){
                            $eids[] = $v['eid'];
                        }
                        $this->update_once('user_resume', array('info'=>1), array('eid'=>array('in', pylode(',', $eids))));
                        
                        $elist = $this->select_all('resume_expect', array('id'=>array('in', pylode(',', $eids))),'id,integrity');
                        foreach ($elist as $v){
                            if ($v['integrity'] < 80){
                                $this->update_once('resume_expect', array('integrity'=>array('+',10)), array('id'=>$v['id']));
                            }
                        }
                    }

                    $return['msg']      =   "基本资料(UID:{$whereData['uid']})修改成功";
                    $return['errcode']  =   9;

                    if ($expect == false) {

                        $return['url']  =   'index.php?c=expect&act=add';
                    } else {

                        $return['url']  =   $_SERVER['HTTP_REFERER'];
                    }
                } else {

                    $status             =   '2';
                    $return['msg']      =   '基本资料修改失败';
                    $return['errcode']  =   8;
                }
            } else {
                $return['msg']          =   '没有要修改的信息';
                $return['errcode']      =   8;
            }
        } else {

            $return['msg']      =   '请选择要修改的个人';
            $return['errcode']  =   8;
        }

        if (isset($status)){
            $return['status']   =   $status;
        }

        return $return;
    }

    /**
     * 修改个人基本信息同步修改简历相关信息
     * @param $uid
     * @param $rData
     * @param $port
     * @return bool
     */
    private function setExpectInfo($uid, $rData, $port)
    {

        $eField =   '`sex`,`birthday`,`exp`,`edu`,`uname`,`r_status`,`photo`';
        $expect =   $this->select_once('resume_expect', array('uid' => $uid), $eField);

        // 有简的比较各修改项，有改动的才修改
        if (!empty($expect)) {

            $isChange = false;

            if (!empty($rData['lastupdate'])) {

                $eData['lastupdate']    =   $rData['lastupdate'];
            }
            if (!empty($rData['photo']) && $expect['photo'] != $rData['photo']) {

                $eData['photo']         =   $rData['photo'];
                $isChange = true;
            }
            if (!empty($rData['sex']) && $expect['sex'] != $rData['sex']) {

                $eData['sex']           =   $rData['sex'];
                $isChange = true;
            }
            if (!empty($rData['birthday']) && $expect['birthday'] != $rData['birthday']) {

                $eData['birthday']      =   $rData['birthday'];
                $isChange = true;
            }
            if (!empty($rData['exp']) && $expect['exp'] != $rData['exp']) {

                $eData['exp']           =   $rData['exp'];
                $isChange = true;
            }
            if (!empty($rData['edu']) && $expect['edu'] != $rData['edu']) {

                $eData['edu']           =   $rData['edu'];
                $isChange = true;
            }
            if (!empty($rData['name']) && $expect['uname'] != $rData['name']) {

                $eData['uname']         =   $rData['name'];
                $isChange = true;
            }
            if (isset($rData['r_status']) && $expect['r_status'] != $rData['r_status']) {

                $eData['r_status']      =   $rData['r_status'];
                $isChange = true;
            }
            if (!empty($eData)) {

                $this->update_once('resume_expect', $eData, array('uid' => $uid));
            }

            if (isset($rData['state'])) {
                    
                if ($rData['state'] == 0){
					$expectS    =   $this->select_all('resume_expect', array('uid' => $uid), 'id, state, uname, name');

                    $eidArr =   array();

                    foreach ($expectS as $ek => $ev) {

                        $eidArr[]           =   $ev['id'];

                         
                    }
                    $this->update_once('resume_expect', array('state' => 0), array('id' => array('in', pylode(',', $eidArr))));
                    $this->update_once('userid_job', array('resume_state' => 0), array('eid' => array('in', pylode(',', $eidArr))));
                }
            }else if ($isChange){

                $expectS    =   $this->select_all('resume_expect', array('uid' => $uid, 'state' => 3), 'id, uname, name, statusbody');

                if (!empty($expectS)){

                    $eidArr =   $eidSArr = array();
                    foreach ($expectS as $ek => $ev) {

                        $eidArr[]       =   $ev['id'];

                        if (empty($ev['statusbody'])){
                            $eidSArr[]  =   $ev['id'];
                        }
                    }

                    if (!empty($eidSArr)){

                        $this->update_once('resume_expect', array('state' => 0, 'statusbody' => '审核未通过简历，重新修改提交审核'), array('id' => array('in', pylode(',', $eidSArr))));
                    }

                    $this->update_once('userid_job', array('resume_state' => 0), array('eid' => array('in', pylode(',', $eidArr))));
                }
            }

            if (isset($eData['lastupdate'])){

                $resumeExpect   =   $this->select_all('resume_expect', array('uid' => $uid), 'id,lastupdate');

                $logSxData      =   array();

                foreach ($resumeExpect as $rk => $rv) {

                    $logSxData[$rk]['uid']          =   $uid;
                    $logSxData[$rk]['resume_id']    =   $rv['id'];
                    $logSxData[$rk]['r_time']       =   $rv['lastupdate'];
                    $logSxData[$rk]['port']         =   $port;
                    $logSxData[$rk]['ip']           =   fun_ip_get();
                }

                require_once 'log.model.php';
                $logM = new log_model($this->db, $this->def);
                $logM->addResumeSxLogS($logSxData);
            }

            return true;
        } else {

            return false;
        }
    }

    /**
     * 修改询简历信息时的查询
     * @param array $data uid=>用户id; eid=>简历id; rfield=>基本信息查询字段; needCache=>需要调用缓存; tb=>查询表范围
     * @return mixed
     */
	public function getInfo($data = array('uid'=>null,'eid'=>null,'needCache'=>null,'tb'=>null))
	{

	    if ($data['needCache'] == 1){
	        $cache				=  $this -> getClass(array('user','city','job','hy','introduce'));
	        $return['cache']	=  $cache;
	    }
	    $resume	=	$expect = $edu = $other = $project = $skill = $training = $work = array();

	    $resume	 =	$this -> getResumeInfo(array('uid'=>$data['uid']),array('logo'=>1));

	    //查询简历信息
	    if ($resume['uid'] && !empty($data['eid'])){
	        $eid     =  intval($data['eid']);

	        $expect  =  $this -> getExpect(array('id'=>$eid,'uid'=>$resume['uid']),array('cache'=>$cache,'needCache'=>$data['needCache']));

	        //查询所有附表信息
	        if ($expect['id'] && $data['tb'] == 'all'){
	            $edu       =    $this -> getResumeEdus(array('eid'=>$eid,'uid'=>$resume['uid'],'orderby'=>'sdate,desc'),'*',$cache['userclass_name']);

	            $other     =    $this -> getResumeOthers((array('eid'=>$eid,'uid'=>$resume['uid'])));

	            $project   =    $this -> getResumeProjects(array('eid'=>$eid,'uid'=>$resume['uid'],'orderby'=>'sdate,desc'));

	            $skill     =    $this -> getResumeSkills(array('eid'=>$eid,'uid'=>$resume['uid']),'*',$cache['userclass_name']);

	            $training  =    $this -> getResumeTrains(array('eid'=>$eid,'uid'=>$resume['uid'],'orderby'=>'sdate,desc'));

	            $work      =    $this -> getResumeWorks(array('eid'=>$eid,'uid'=>$resume['uid'],'orderby'=>'sdate,desc'));

				$show      =    $this -> getResumeShowList(array('eid'=>$eid,'uid'=>$resume['uid'],'orderby'=>'sort,desc'));
	        }
	    }
		$expectjob =   $this->getInfoByEid(array('eid' => $eid));
        $expect['expectjob']=$expectjob['expectjob'];
	    $return['resume']  	=	$resume;
	    $return['expect']  	=	$expect;
	    $return['edu']     	=   $edu;
	    $return['other']   	=   $other;
	    $return['project'] 	=   $project;
	    $return['skill']   	=   $skill;
	    $return['training']	=	$training;
	    $return['work']    	=	$work;
		$return['show']    	=	$show;

	    return $return;
	}

    /**
     * 通过eid查询简历信息，用作简历详情页展示
     * @param array $data : uid => 查看用户id; eid=>简历id; field=>基本信息查询字段; cache=>返回包含的缓存数据
     * @return array|bool|false|string|void
     */
	public function getInfoByEid($data = array())
	{
		$res						=	array();
		$eid     					=	intval($data['eid']);
		$uid						=	intval($data['uid']);       //查看简历的用户uid
		$usertype					=	intval($data['usertype']);  //查看简历的用户usertype

		if(empty($eid)){
			return $res;
		}

		$cache						=	$this -> getClass(array('user', 'city', 'job', 'hy', 'industry'));

	    //查询简历信息
		$expect  					=	$this -> getExpect(array('id' => $eid), array('cache' => $cache,'needCache'=>1));

		if(empty($expect)){
			return $res;
		}
		$euid						=	$expect['uid'];
		$resume						=	$this -> getResumeInfo(array('uid' => $euid));
		if(empty($resume)){
			return $res;
		}
        //查询账户信息
        $memberinfo = $this->select_once('member',array('uid'=>$euid),'uid,source,reg_date,lock_info');
		$resume['source'] = $memberinfo['source'];
		$resume['reg_date'] = $memberinfo['reg_date'];
		$resume['reg_date_n'] = date('Y-m-d',$memberinfo['reg_date']);
        $resume['lock_info'] = $memberinfo['lock_info'];
		//拼凑相关的字段
		if(empty($resume['sex'])){
			$resume['sex']			=	$expect['sex'];
		}
		if($resume['sex'] == 152){
			$resume['sex']			=	2;
		}elseif ($resume['sex'] == 153){
			$resume['sex']			=	1;
		}
		//简历名称
		$resume['customjob']		=	$expect['name'];
		//名称显示处理
		$resume['username_n']		=	$this -> setUsernameShow(array('nametype' => $resume['nametype'], 'name' => $resume['name'], 'eid' => $eid, 'sex' => $resume['sex']));
		//图片显示处理
		$resume['photo']		    =	$this -> setResumePhotoShow(array('photo' => $resume['photo'],'defphoto' => $resume['defphoto'], 'photo_status'=>$resume['photo_status'],'phototype' => $resume['phototype'], 'sex' => $resume['sex']));
		//年龄
		if(!empty($resume['birthday'])){
			$resume['age']			=	date('Y') - date('Y', strtotime($resume['birthday']));
		}

		$resume['user_exp']			=	$cache['userclass_name'][$resume['exp']];
		$resume['user_marriage']	=	$cache['userclass_name'][$resume['marriage']];
		$resume['useredu']			=	$cache['userclass_name'][$resume['edu']];
		$resume['job_sex']			=	$cache['user_sex'][$resume['sex']];
		$resume['minsalary']		=	$expect['minsalary'];
		$resume['job_classid']		=	$expect['job_classid'];
		$resume['city_classid']		=	$expect['city_classid'];
		$resume['maxsalary']		=	$expect['maxsalary'];
		$resume['salary']		    =	$expect['salary'];
		$resume['jobstatus']		=	$expect['jobstatus_n'];
		$resume['report']			=	$expect['report_n'];
		$resume['type']				=	$expect['type_n'];
		$resume['hy']				=	$expect['hy_n'];
		$resume['lastupdate']		=	lastupdateStyle($expect['lastupdate']);
		$resume['r_name'] 			=	$expect['name'];
		$resume['doc'] 				=	$expect['doc'];
		$resume['hits']				=	$expect['hits'];
		$resume['dnum']				=	$expect['dnum'];
		$resume['r_status']			=	$expect['r_status'];
		$resume['state']			=	$expect['state'];
        $resume['statusbody']       =	$expect['statusbody'];
        $resume['expect_audit']     =	$expect['content'];
		$resume['content']	        =	$expect['content'];
        $resume['uname']            =   $expect['uname'];
        $resume['label_n']			=	!empty($expect['label']) ? $cache['userclass_name'][$expect['label']] : '';
        $resume['label_id']			=	$expect['label'];
		$resume['height_status']	=	$expect['height_status'];
        $resume['id']				=	$eid;
        $resume['login_date_n']		=	$resume['login_date'] > 0 ? date('Y-m-d', $resume['login_date']) : '<font color="red">从未登录</font>';
		$resume['wxewm']	        =	checkpic($resume['wxewm']);

		if (!empty($resume['tag'])){
			$resume['arrayTag']		=	explode(',', $resume['tag']);
		}else{
            $resume['arrayTag'] = array();
        }

		if (!empty($expect['whour'])){
			$resume['whour'] 		=	$expect['whour'];
			if(($expect['whour']/12) >= 1){
				$whour 				=	floor($expect['whour']/12).'年';
			}
			if(($expect['whour']%12) >= 1){
				$whour 				=	floor($expect['whour']%12).'个月';
			}
			$resume['whourInfo']	=	$whour;
		}

		if (!empty($expect['avghour'])){
			$resume['avghour']		=	$expect['avghour'];
            $avghour = '';
			if(($expect['avghour']/12) >= 1){
				$avghour 			.=	floor($expect['avghour']/12).'年';
			}
			if(($expect['avghour']%12) >= 1){
				$avghour 			.=	floor($expect['avghour']%12).'个月';
			}
			$resume['avghourInfo']	=	$avghour;
		}else{
			$resume['avghourInfo']	=	'1个月内';
		}

		$resume['jobname']			=	$expect['job_classname'];
		$resume['expectjob']		=	@explode(',', $expect['job_classname']);

		$resume['cityname']			=	$expect['city_classname'];
		$resume['expectcity']		=	@explode(',', $expect['city_classname']);
		if($expect['doc'] == 1){
			$user_doc				=	$this -> select_once('resume_doc', array('eid' => $eid));
		}else{
			$user_edu				=	$this -> getResumeEdus(array('eid' => $eid, 'orderby' => 'sdate, desc'), '*', $cache['userclass_name']);
			$user_training			=	$this -> getResumeTrains(array('eid' => $eid, 'orderby' => 'sdate,desc'));
			$user_work				=	$this -> getResumeWorks(array('eid' => $eid, 'orderby' => 'sdate,desc'));
			$user_other				=	$this -> getResumeOthers(array('eid' => $eid));
			$user_skill				=	$this -> getResumeSkills(array('eid' => $eid), '*', $cache['userclass_name']);
			$user_xm				=	$this -> getResumeProjects(array('eid' => $eid, 'orderby' => 'sdate,desc'));
			if ($usertype == 2) {
			    $user_show          =   $this -> getResumeShowList(array('eid' => $eid, 'status' => 0));
			}else {
			    $user_show			=	$this -> getResumeShowList(array('eid' => $eid));
			}
		}

		//获取是否可以查看
		if($usertype == 2){

		    $userid_job  =  $this->select_once('userid_job',array('com_id' => $uid,'isdel'=>9,'eid' => $eid,'orderby'=>'id,desc'),'`id`,`job_name`,`datetime`');

		    if (!empty($userid_job)){
		        $resume['sqInfo']   =   array(
		            'sq_job_name'   =>  $userid_job['job_name'],
                    'sq_job_time'   =>  date('Y-m-d', $userid_job['datetime'])
                );

                $resume['userid_job']   =   1;
            }
			$resume['rdayprice']	=	$this -> setDayprice($eid);
		}

		if($uid == $euid){
			$resume['m_status']		=	1;
			$resume['username_n']	=	$resume['name'];
			$resume['diy_status']	=	1;
		}else{
			$resume['diy_status']	=	2;
		}

		//获取是否下载过此简历
		if($usertype==2){
			
			$tmpField		=	'`vip_etime`,`down_resume`, `rating_type`,`rating`';
			$suid           =   $uid;

            require_once ('statis.model.php');
            $statisM			=	new statis_model($this->db, $this->def);
            $statis 			=	$statisM -> getInfo($suid, array('usertype' => $usertype, 'field' => $tmpField));
            //未过期会员判断是否有免费查看投递简历下载记录
            if (isVip($statis['vip_etime'])){
                $freeisDown = $this->select_once('freedown_resume',array('eid' => $eid, 'comid' => $uid, 'usertype' => $usertype));
                if ($usertype == 2 && !empty($userid_job)){
                    $ratingInfo     =   $this->select_once('company_rating', array('id' => $statis['rating']), '`freelook_num`');
                    if ($ratingInfo['freelook_num'] > 0) {

                        $freeDownNum    =   $this->select_num('freedown_resume', array('comid' => $uid, 'usertype' => 2));
                        if ($ratingInfo['freelook_num'] > $freeDownNum) {
                            $resume['free_look']    =   1;
                        }
                    }
                }
            }
			$row     =	$this -> select_once('down_resume',array('eid' => $eid, 'comid' => $uid,'usertype'=>$usertype));
			if(!empty($row) || (isset($freeisDown) && !empty($freeisDown))){
			    //已下载
				$resume['downresume']	=	1;
				$resume['m_status']		=	1;
				$resume['username_n']	=	$resume['name'];
			}else{

				//此处获取每日最大限制
				
				$row 				=	$statisM -> getInfo($suid, array('usertype' => $usertype, 'field' => $tmpField));

				if($row['rating_type'] == 2){

					require_once ('company.model.php');
					$comM			=	new company_model($this->db, $this->def);
					$today			=	strtotime('today');
					$nid			=	$comM -> comVipDayActionCheck('resume', $uid);
					$dnum			=	$this -> select_num('down_resume', array('comid' => $uid, 'downtime' =>	array('>',$today)));

					if($nid != 1){
						$row['down_resume']	=	0;
					}else{
						$row['down_resume']	=	$row['down_resume'] - $dnum;	// 时间会员今日已下载
					}
				}

                if(isVip($row['vip_etime']) && $row['down_resume'] > 0){

                    if($this->config['com_lietou_job'] == 1){

                        if($usertype == 2){
                            $jobNum     =     $this->select_num('company_job',array('uid' => $uid, 'state' => 1,'r_status' =>1, 'status' => 0));
                            $listNum    =     $jobNum;
                        }                       

                        if(intval($listNum) > 0){
                            $resume['showcontactflag']  =   true;
                            $resume['downresumes']      =   $row['down_resume']; // 剩余下载量（时间会员每人剩余/套餐会员总剩余量）
                        }
                    }else {

                        $resume['showcontactflag']  =   true;
                        $resume['downresumes']      =   $row['down_resume']; // 剩余下载量（时间会员每人剩余/套餐会员总剩余量）
                    }
                }
			}
		}

        //获取是申请过简历
        if($usertype == 2){
            $sqdata     =   $this -> select_all('userid_job',array('uid' => $euid, 'com_id' => $uid, 'isdel'=>9)); // 没投递过当前简历，投递过其它简历，也显示真实名字
            if(!empty($sqdata)){
                $resume['username_n']   =   $resume['name'];
            }
        }

		if(isset($data['reward'])){
			if($data['reward']>1 && $data['reward']!=20){
				$resume['m_status']		=	1;
			}elseif($data['reward']==1){
				$resume['m_status']		=	2;
			}
		}
		$tj	=	$this->getTj(array('edu'=>$user_edu,'train'=>$user_training,'work'=>$user_work,'xm'=>$user_xm,'skill'=>$user_skill , 'show'=>$user_show));

		$resume['per']				=	sprintf('%.2f%%',($expect['dnum']/$expect['hits'])*100);

		$resume['description']		=	str_replace(array('\r','\n'), array('<br/>','<br/>'), strip_tags($resume['description'],'\r,\n'));
		$resume['user_doc']			=	isset($user_doc) ? $user_doc : array();
		$resume['user_edu']			=	$user_edu;
		$resume['user_tra']			=	$user_training;
		$resume['user_work']		=	$user_work;
		$resume['user_other']		=	$user_other;
		$resume['user_xm']			=	$user_xm;
		$resume['user_skill']		=	$user_skill;
		$resume['user_show']		=	$user_show;
		$resume['tj']				=	$tj;

	    return $resume;
	}
	/**
	 * 企业通过uid查看用户姓名
	 */
	function getUnameByUid($uid, $data = array())
	{
	    $name = '';
	    if (empty($data['comid']) || empty($data['usertype'])){
	        return $name;
	    }
	    // 首先按照后台、个人设置的姓名显示方式处理
	    $resume = $this->select_once('resume',array('uid'=>$uid),'name,sex,nametype,def_job');
	    $setarr = array('name'=>$resume['name'],'sex'=>$resume['sex'],'eid'=>$resume['def_job'],'nametype'=>$resume['nametype']);
	    $name   = $this->setUsernameShow($setarr);
	    // 有下载记录，返回真实姓名
	    $down = $this->select_num('down_resume', array('uid'=>$uid, 'comid'=>$data['comid'], 'usertype'=>$data['usertype']));
	    if ($down){
	        $name = $resume['name'];
	    }
	    return $name;
	}
    //个人随机默认头像
    public function deflogo($data = array()){

        $logo = '';
        $sex = $data['sex'];

        if($sex=='2'){
            $icon_arr = $this->config['sy_member_iconv_arr'];
        }else{
            $icon_arr = $this->config['sy_member_icon_arr'];
        }

        if(!empty($icon_arr)){
            $key  = array_rand($icon_arr,1);
            $logo = $icon_arr[$key];
        }

        return $logo;
    }

    /**
     * 创建简历
     * @param null[] $data
     * @return array|string[]
     */
	public function addInfo($data=array('uid'=>null,'eData'=>null,'rData'=>null,'workData'=>null,'eduData'=>null,'proData'=>null,'utype'=>null)){
	    // 字段检测

	    $check  =  $this->setFieldCheck($data);

	    $uid    =  $data['uid'];

	    if ($check['msg']){
	        return $check;
	    }

	    // 保存基本信息
	    if (!empty($data['rData']) && !empty($uid)){

	        $rData = $data['rData'];
	        if (!empty($rData['email']) || !empty($rData['telphone'])){
	            $mData  =  array(
	                'email'   =>  $rData['email'],
	                'moblie'  =>  $rData['telphone']
	            );
	        }else{
	            $mData  =  array();
	        }

	        $resumeInfo	=	$this->select_once('resume',array('uid'=>$uid),'name,living,resumetime');

            if ($resumeInfo) {

                $result =   $this->upResumeInfo(array('uid' => $uid), array('mData' => $mData, 'rData' => $rData, 'utype' => 'user', 'source' => $data['source']));
            } else {

                $this->insert_into('resume', array('uid' => $uid));

                $result =   $this->upResumeInfo(array('uid' => $uid), array('mData' => $mData, 'rData' => $rData, 'utype' => 'user', 'source' => $data['source']));
            }

	        if ($result['errcode'] == 8){

	            return $result;
	        }
	    }

	    // 添加求职意向
	    if (!empty($data['eData'])){
	        $eData  =  $data['eData'];
	        //先检查后台设置的最大简历数量
	        $enum   =  $this -> getExpectNum(array('uid'=>$uid));

			//用户操作的，判断简历是否超限
	        if ($data['utype'] == 'user'){

	            if($this->config['user_number'] && $enum >= $this->config['user_number']){

	                return array('errcode'=>8,'msg'=>'您最多可以拥有'.$this->config['user_number'].'份简历');
	            }
	        }
	        if (isset($result['r_status'])){

	            // 根据上面 upResumeInfo 返回的参数进行修改账号审核状态
	            $eData['r_status']  =   $result['r_status'];
	        }
	        // 拥有简历数量为0，新添加的简历设为默认简历
	        $eData['defaults']      =   $enum == 0 ? 1 : 0;
			$ip = fun_ip_get();
            $eData['add_ip'] = $ip;
            $eData['ip_address'] = getIpAddress($ip);

	        $return['id']           =   $this -> insert_into('resume_expect',$eData);
			
	        if ($return['id']){

	            // 处理简历多职位和多城市
	            $this->city_job($return['id'], $uid, $eData['lastupdate'], $eData['city_classid'], $eData['job_classid']);

	            $this -> update_once('member_statis',array('resume_num'=>$enum + 1),array('uid'=>$uid));
                // 基础平均工作时长
                $whour      =  0;
                // 处理user_resume表数据
                $uResume    =  array(
                    'uid'     =>  $uid,
                    'eid'     =>  $return['id'],
                    'work'	  =>  0,
                    'edu'	  =>  0,
                    'project' =>  0
                );
                // 创建简历基础完成度
                if ($data['source'] == 28){
                    // 来源是求职顾问
                    if (isset($resumeInfo) && (!empty($resumeInfo['name']) || !empty($resumeInfo['living']))){
                        // 基本信息已完善过的
                        $integrity  =  45;
                        $uResume['info'] = 1;
                    }else{
                        $integrity  =  35;
                        $uResume['info'] = 0;
                    }
                    $uResume['expect'] = 0;
                }else{
                    $integrity  =  55;
                    $uResume['info']   = 1;
                    $uResume['expect'] = 1;
                }

                // 处理工作经历
                if (!empty($data['workData'])){
                    // 处理平均工作时长
                    $whour      =   0;
                    $hour       =   array();

                    foreach ($data['workData'] as $key => $value) {

                        $data['workData'][$key]['eid']  =  $return['id'];
                        $this -> insert_into('resume_work',$data['workData'][$key]);
                        $uResume['work']++;
                        // 计算每份工作时长(按月)
                        if ($value['edate']) {

                            $workTime = ceil(($value['edate'] - $value['sdate']) / (30 * 86400));
                        } else {
                            $workTime = ceil((time() - $value['sdate']) / (30 * 86400));
                        }
                        $hour[] = $workTime;
                        $whour += $workTime;
                    }
                    // 更新当前简历时长字段
                    $avghour = ceil($whour / count($hour));


                    $integrity  +=  10;

                }
                // 处理教育经历
                if (!empty($data['eduData'])){

                    foreach ($data['eduData'] as $key => $value) {

                        $data['eduData'][$key]['eid']  =  $return['id'];

                        $this -> insert_into('resume_edu',$data['eduData'][$key]);

                        $uResume['edu']++;

                    }

                    $integrity  +=  10;

                }
                // 处理项目经历
                if (!empty($data['proData'])){

                    foreach ($data['proData'] as $key => $value) {

                        $data['proData'][$key]['eid']  =  $return['id'];

                        $this -> insert_into('resume_project',$data['proData'][$key]);

                        $uResume['project']++;

                    }

                    $integrity  +=  8;

                }

                if ($data['utype'] == 'user'){

                    // 记录会员日志
                    $this -> addMemberLog($uid, 1, '简历：新建简历(ID:'.$return['id'].')', 2, 1);

                    //预警处理
                    require_once('warning.model.php');
                    $warningM  =  new warning_model($this->db, $this->def);
                    $warningM -> warning(3, $uid);

                    require_once ('cookie.model.php');
                    $cookieM  =  new cookie_model($this->db, $this->def);
                    $cookieM -> SetCookie('delay', '', time() - 60);
                }

	            // 创建简历，修改默认简历
	            if ($enum == 0){

	                $this -> update_once('resume',array('def_job'=>$return['id'],'lastupdate'=>$eData['lastupdate'],'resumetime'=>$eData['ctime']),array('uid'=>$uid));

	                if ($data['utype'] == 'user' && empty($resumeInfo['resumetime'])){ // 第一次创建简历发红包
	                    // 创建首份简历加积分，先要检测是否已获得过此积分
	                    $ipay  =  $this -> select_once('company_pay',array('com_id'=>$uid,'pay_remark'=>'发布简历'));

                        if (empty($ipay)) {

                            require_once('integral.model.php');
                            $integralM                      =   new integral_model($this->db, $this->def);

                            $integralM->invtalCheck($uid, 1, 'integral_add_resume', '发布简历');
                        }
	                }
	            }else {

	                $this -> update_once('resume',array('resumetime'=>$eData['ctime']),array('uid'=>$uid));
	            }
	            // 增加简历附表数据
	            $this -> insert_into('user_resume', $uResume);
	            // 向简历表中更新基本信息中已有数据
	            $resume  =  $this->select_once('resume',array('uid'=>$uid),'`name`,`idcard_status`,`status`,`photo`,`defphoto`, `phototype`,`did`');

	            $upExpect  =  array(
	                'did'			 =>	 $resume['did'],
	                'integrity'		 =>	 $integrity,
	                'whour'			 =>	 $whour,
	                'avghour'		 =>	 $avghour,
	                'uname'          =>  $resume['name'],
	                'status'         =>  $resume['status'],
	                'photo'          =>  $resume['photo'],
                    'defphoto'       =>  $resume['defphoto'],
	                'phototype'      =>  $resume['phototype'],
	                'idcard_status'  =>  $resume['idcard_status']
	            );


                if(isset($upExpect['state'])){
                    $this->setExpectState($upExpect,array('id'=>$return['id'], 'uid'=> $uid));
                }else{
                    $this->update_once('resume_expect', $upExpect, array('id'=>$return['id'], 'uid'=> $uid));
                }

	            // 管理员操作记录管理员日志
	            if ($data['utype'] == 'admin'){

	                require_once 'log.model.php';
	                $logM  =  new log_model($this->db, $this->def);
	                $logM -> addAdminLog('添加简历(ID:'.$return['id'].')');
				}
				$return['errcode']  =  9;
	            $return['msg']  	=  '保存成功';

	        }else {

				$return['errcode']  =  8;
	            $return['msg']  	=  '保存失败';
                $this->addErrorLog($uid,2,'简历保存失败');
	        }
	    }
	    return $return;
	}
	
    /**
     * 简历预创建后，更新简历信息
     * @param null[] $data
     * @return array|string[]
     */
    public function updatePreResume($data=array('uid'=>null,'eData'=>null,'rData'=>null,'workData'=>null,'eduData'=>null,'proData'=>null,'utype'=>null)){
        // 字段检测
        $check = $this->setFieldCheck($data);
        $uid = $data['uid'];
        if ($check['msg']){
            return $check;
        }
        // 保存基本信息
        if (!empty($data['rData']) && !empty($uid)){
            $mData = array(
                'email' => $data['rData']['email'],
                'moblie' => $data['rData']['telphone']
            );
            $resumeInfo	= $this->select_once('resume',array('uid'=>$uid));
            if ($resumeInfo) {
                $result = $this->upResumeInfo(array('uid' => $uid), array('mData' => $mData, 'rData' => $data['rData'], 'utype' => 'user', 'source' => $data['source']));
            } else {
                $this->insert_into('resume', array('uid' => $uid));
                $result = $this->upResumeInfo(array('uid' => $uid), array('mData' => $mData, 'rData' => $data['rData'], 'utype' => 'user', 'source' => $data['source']));
            }
            if ($result['errcode'] == 8){
                return $result;
            }
        }
        // 添加求职意向
        if (!empty($data['eData'])){
            $eData = $data['eData'];
            $eid = $eData['id'];
            unset($eData['id']);
            //先检查后台设置的最大简历数量
            $enum = $this->getExpectNum(array('uid'=>$uid));
            if (isset($result['r_status'])){
                // 根据上面 upResumeInfo 返回的参数进行修改账号审核状态
                $eData['r_status'] = $result['r_status'];
            }
            // 拥有简历数量为0，新添加的简历设为默认简历
            $eData['defaults'] = $enum <= 1 ? 1 : 0;
            $r = $this->update_once('resume_expect',$eData,array('id'=>$eid));
            if ($r) {
                $oldresume  =   $this->getExpect(array('uid'=>$uid),array('field'=>'`city_classid`,`job_classid`,`state`'));

                // 处理简历多职位和多城市
                $oldCity    =   isset($oldresume['city_classid']) ? $oldresume['city_classid'] : '';
                $oldJob     =   isset($oldresume['job_classid']) ? $oldresume['job_classid'] : '';
                $this->city_job($eid, $uid, $eData['lastupdate'], $eData['city_classid'], $eData['job_classid'],$oldCity,$oldJob);
                $this->update_once('member_statis',array('resume_num' => $enum + 1),array('uid' => $uid));
                // 创建简历基础完成度
                $integrity = 55;
                // 基础平均工作时长
                $whour = 0;
                // 处理user_resume表数据
                $uResume    =  array(
                    'uid'     =>  $uid,
                    'eid'     =>  $eid,
                    'expect'  =>  1,
                    'work'	  =>  0,
                    'edu'	  =>  0,
                    'project' =>  0
                );
                if ($data['utype'] == 'user'){
                    // 处理工作经历
                    if (!empty($data['workData'])){
                        // 处理平均工作时长
                        $whour = 0;
                        $hour = array();
                        foreach ($data['workData'] as $key => $value) {
                            $data['workData'][$key]['eid'] = $eid;
                            $this->insert_into('resume_work',$data['workData'][$key]);
                            $uResume['work']++;
                            // 计算每份工作时长(按月)
                            if ($value['edate']) {
                                $workTime = ceil(($value['edate'] - $value['sdate'])/(30 * 86400));
                            } else {
                                $workTime = ceil((time() - $value['sdate'])/(30 * 86400));
                            }
                            $hour[] = $workTime;
                            $whour += $workTime;
                        }
                        // 更新当前简历时长字段
                        $avghour = ceil($whour/count($hour));
                        $integrity += 10;
                    }
                    // 处理教育经历
                    if (!empty($data['eduData'])){
                        foreach ($data['eduData'] as $key => $value) {
                            $data['eduData'][$key]['eid'] = $eid;
                            $this -> insert_into('resume_edu',$data['eduData'][$key]);
                            $uResume['edu']++;
                        }
                        $integrity += 10;
                    }
                    // 处理项目经历
                    if (!empty($data['proData'])){
                        foreach ($data['proData'] as $key => $value) {
                            $data['proData'][$key]['eid'] = $eid;
                            $this->insert_into('resume_project',$data['proData'][$key]);
                            $uResume['project']++;
                        }
                        $integrity  +=  8;
                    }
                    // 记录会员日志
                    $this->addMemberLog($uid, 1, '简历：更新简历信息(ID:'.$eid.')', 2, 1);
                    //预警处理
                    require_once('warning.model.php');
                    $warningM = new warning_model($this->db, $this->def);
                    $warningM->warning(3, $uid);
                    require_once ('cookie.model.php');
                    $cookieM = new cookie_model($this->db, $this->def);
                    $cookieM->SetCookie('delay', '', time() - 60);
                }
                $this->update_once('resume',array('resumetime'=>$eData['ctime']),array('uid'=>$uid));
                // 增加简历附表数据
                if ($eid) {
                    $this->update_once('user_resume',$uResume,array('uid'=>$uid,'eid'=>$eid));
                } else {
                    $this->insert_into('user_resume', $uResume);
                }
                // 向简历表中更新基本信息中已有数据
                $resume = $this->select_once('resume',array('uid'=>$uid),'`name`,`idcard_status`,`status`,`photo`,`defphoto`, `phototype`,`did`');
                $upExpect = array(
                    'did' => $resume['did'],
                    'integrity' => $integrity,
                    'whour' => $whour,
                    'avghour' => $avghour,
                    'uname' => $resume['name'],
                    'status' => $resume['status'],
                    'photo' => $resume['photo'],
                    'defphoto' => $resume['defphoto'],
                    'phototype' => $resume['phototype'],
                    'idcard_status' => $resume['idcard_status']
                );
                
                $this->update_once('resume_expect', $upExpect, array('id'=>$eid, 'uid'=> $uid));
                
                $return['errcode'] = 9;
                $return['msg'] = '保存成功';
            
            }else {
                $return['errcode']  =  8;
                $return['msg']  	=  '保存失败';
                $this->addErrorLog($uid,2,'简历保存失败');
            }
        }
        return $return;
    }
	/**
	 * 创建简历字段检测
	 */
	private function setFieldCheck($data = array()){

		$return    =  array();

	    $rData     =  $data['rData'];
	    $eData     =  $data['eData'];
	    $workData  =  $data['workData'];
	    $eduData   =  $data['eduData'];
		$proData   =  $data['proData'];

		$return['errcode']	=	8;

	    if (!empty($rData)){

	        if($rData['name']=='')      {$return['msg']  == '请填写真实姓名';}

	        if($rData['sex']=='')       {$return['msg']  == '请选择性别';}

	        if($rData['birthday']=='')  {$return['msg'] == '请选择出生年月';}

	        if($rData['living']=='')    {$return['msg'] == '请填写现居住地';}

	        if($rData['edu']=='')       {$return['msg'] == '请选择最高学历';}

	        if($rData['exp']=='')       {$return['msg'] == '请选择工作经验';}

	        if($rData['telphone']=='')  {$return['msg'] == '请填写手机号码';}
	    }

	    if (!empty($eData)){

	        if($eData['name']=='')      {$return['msg'] == '请填写期望职位';}

	        if($eData['hy']=='')        {$return['msg'] == '请选择从事行业';}

	        if($eData['job_class']=='') {$return['msg'] == '请选择期望职位';}

	        if($eData['minsalary']=='') {$return['msg'] == '请填写期望薪资';}

	        elseif($eData['maxsalary']>0 && (int)$eData['minsalary'] > (int)$eData['maxsalary']){$return['msg'] == '最高薪资必须大于最低薪资';}

	        if($eData['city_class']==''){$return['msg'] == '请选择工作地区';}

	        if($eData['type']=='')      {$return['msg'] == '请选择工作性质';}

	        if($eData['report']=='')    {$return['msg'] == '请选择到岗时间';}

	        if($eData['jobstatus']=='') {$return['msg'] == '请选择求职状态';}
	    }

	    if (!empty($workData)){

	        if($workData['workname']=='') {$return['msg'] == '请填写公司名称';}

	        if($workData['worksdate']==''){$return['msg'] == '请填写工作时间';}

	        if($workData['worktitle']==''){$return['msg'] == '请填写公司职务';}

	        if($workData['workedate'] < $workData['worksdate']){$return['msg'] == '工作时间不合理';}
	    }

	    if (!empty($eduData)){

	        if($eduData['eduname']=='')  {$return['msg'] == '请填写学校名称';}

	        if($eduData['edusdate']=='') {$return['msg'] == '请填写在校时间';}

	        if($eduData['eduedate']=='') {$return['msg'] == '请填写离校或预计离校时间';}

	        if($eduData['education']==''){$return['msg'] == '请选择相关学历';}

	        if($eduData['eduspec']=='')  {$return['msg'] == '请填写相关专业';}

	        if($eduData['eduedate'] < $eduData['edusdate']){$return['msg'] == '在校时间不合理';}
	    }

	    if (!empty($proData)){

	        if($proData['proname']=='') {$return['msg'] == '请填写项目名称';}

	        if($proData['prosdate']==''){$return['msg'] == '请填写项目开始时间';}

	        if($proData['proedate']==''){$return['msg'] == '请填写项目结束时间';}

	        if($proData['protitle']==''){$return['msg'] == '请填写项目担任职务';}

	        if($eduData['proedate'] < $eduData['prosdate']){$return['msg'] == '项目时间不合理';}
	    }
	    return $return;
    }

    /**
     * 添加修改粘贴简历
     * @param $post
     * @return array
     */
	public function addDocInfo($post)
    {

		$eid		=	intval($post['eid']);
		$uid		=	intval($post['uid']);
		$usertype	=	intval($post['usertype']);
		$expect		=	$post['expect'];//插入到resume_expect数据

		$doc		=	$post['doc'];
		$doctype	=	$post['doctype'];//判断是否是粘贴简历

		$expect['lastupdate']	=	time();

		if($doctype){

			$expect['doc']		=	1;
		}

		if(!$eid){

            //判断简历数是否超过后台设置
            $num  		=   $this -> getExpectNum(array('uid'=>$uid));
            if($this->config['user_number'] && $num >= $this->config['user_number']){

                return array('errcode'=>1,'msg'=>'您最多可以拥有'.$this->config['user_number'].'份简历');
            }

            //拥有简历数量为0，新添加的简历设为默认简历
            $expect['defaults'] =  $num == 0 ? 1 : 0;

			$expect['ctime']	=	time();

			$nid	=	$this -> insert_into('resume_expect',$expect);
			$eid	=	$nid;

			if($doctype){
				$this->insert_into('resume_doc',array('uid'=>$uid,'eid'=>$eid,'doc'=>$doc));
			}
			if($nid){
				$qdata	=	array(
					'uid'=>$uid,
					'eid'=>$eid,
					'expect'=>1
				);
				$this->insert_into('user_resume',$qdata);

				$this->update_once('member_statis',array('resume_num'=>array('+',1)),array('uid'=>$uid));
				if($num==0){
					$this->update_once('resume',array('def_job'=>$eid),array('uid'=>$uid));
				}
			}
			$etype		=	1;
			$msg		=	'添加';
		}else{

		    $oldresume  =   $this->getExpect(array('uid'=>$uid),array('field'=>'`city_classid`,`job_classid`,`state`'));

			$nid        =   $this->update_once('resume_expect', $expect,array('id'=>$eid,'uid'=>$uid));
			if($doctype){
				$this->update_once('resume_doc',array('doc'=>$doc),array('uid'=>$uid,'eid'=>$eid));
			}
			$etype		=	2;
			$msg		=	'修改';
		}
		if($nid){

			//判断是否已获取发布简历送积分
			$paynum		=	$this->select_num('company_pay',array('com_id'=>$uid,'pay_remark'=>'发布简历'));

			if($paynum < 1){

				include_once('integral.model.php');
	            $integralM  =  new integral_model($this->db, $this->def);
				$integralM->invtalCheck($uid,$usertype,'integral_add_resume','发布简历');
			}

			//更新与resume里相同的字段
			$resume		=	$this->getResumeInfo(array('uid'=>$uid),array('field'=>'`name`,`edu`,`exp`,`sex`,`birthday`,`idcard_status`,`status`,`r_status`,`photo`,`phototype`'));

			$rdata		=	array(
				'edu'			=>	$resume['edu'],
				'exp'			=>	$resume['exp'],
				'uname'			=>	$resume['name'],
				'sex'			=>	$resume['sex'],
				'birthday'		=>	$resume['birthday'],
				'idcard_status'	=>	$resume['idcard_status'],
				'status'		=>	$resume['status'],
				'state'			=>	$resume['r_status']==2 ? 0 : resumeTimeState($this->config['resume_status']),
				'photo'			=>	$resume['photo'],
				'phototype'		=>	$resume['phototype']
			);
            $expectS    =   $this->getExpect(array('id'=>$eid));
            if(!empty($expectS['edu'])){
                if(resumeTimeState($this->config['user_revise_state'])=='0'){
                    $rdata['state'] =   0;
                }
            }
            $resume_v   =   array('lastupdate'=>time());
      
            if (isset($oldresume)){
                if ($oldresume['state'] == 3){

                    $rdata['state'] =   0;
                }
               
            }

       

            $this->setExpectState($rdata,array('id'=>$eid));

			$this->update_once('resume',$resume_v,array('uid'=>$uid));

            // 处理简历职位和城市多选
			$oldCity    =   isset($oldresume['city_classid']) ? $oldresume['city_classid'] : '';
			$oldJob     =   isset($oldresume['job_classid']) ? $oldresume['job_classid'] : '';
			$this->city_job($eid,$uid,$expect['lastupdate'],$expect['city_classid'],$expect['job_classid'],$oldCity,$oldJob);

			if($doctype){
				$docmsg =   '粘贴';
			}

			if($etype==1 && $usertype && $uid){

				$this->addMemberLog($uid,$usertype,$msg.$docmsg.'简历',2,2);
			}elseif($etype==2 && $usertype && $uid){

				$this->addMemberLog($uid,$usertype,$msg.$docmsg.'简历',2,1);
			}
			return array('errcode'=>9,'msg'=>$msg.'成功');
		}else{

			$this->addErrorLog($uid,2,'粘贴简历'.$msg.'失败');
			return array('errcode'=>9,'msg'=>$msg.'失败');
		}
	}

    /**
     * 修改求职意向信息
     * @param $whereData    | $id  int:1   or array('in','1,2,3')
     * @param array $data   |   port 刷新端口 sxlog: 1-需要添加刷新日志；
     * @return boolean | $return
     */
    public function upInfo($whereData, $data = array('eData' => null, 'utype' => null, 'port' => 1, 'sxlog' => null))
    {

        if (!empty($whereData)) {

            //修改求职意向
            if (!empty($data['eData'])) {

                $return['msg']  =   '简历信息';

                $eData          =   $data['eData'];

                //处理简历多职位、多城市
                if ($eData['job_classid'] || $eData['city_classid'] && !empty($whereData['id'])) {

                    $oldJob     =   $oldCity    =   '';

                    $oldExpect  =   $this->select_once('resume_expect', array('id' => $whereData['id']), 'uid,job_classid,city_classid,state');

                    if ($data['utype'] != 'admin' && $whereData['uid'] != $oldExpect['uid']) {

                        $return['msg']      =   '简历信息修改失败！';
                        $return['errcode']  =   8;
                        return $return;
                    }

                    // 处理简历职位和城市多选
                    $oldJob     =   isset($oldExpect['job_classid']) ? $oldExpect['job_classid'] : '';
                    $oldCity    =   isset($oldExpect['city_classid']) ? $oldExpect['city_classid'] : '';
                    $this->city_job($whereData['id'], $oldExpect['uid'], $eData['lastupdate'], $eData['city_classid'], $eData['job_classid'], $oldCity, $oldJob);
                }

                
                if ($data['utype'] == 'user') {

                    if (!isset($oldExpect)){

                        $oldExpect          =   $this->select_once('resume_expect', array('id' => $whereData['id']), 'state');
                        if ($oldExpect['state'] == 3){

                            $eData['state']     =   0;
                            
                        }
                    }else if ($oldExpect['state'] == 3){

                        $eData['state']     =   0;
                     
                    }
                }

                if (isset($eData['state'])) {

                    $return['id']   =   $this->setExpectState($eData, $whereData);
                } else {

                    $return['id']   =   $this->update_once('resume_expect', $eData, $whereData);

                    if (isset($eData['lastupdate']) && $data['sxlog'] == 1){

                        require_once 'log.model.php';
                        $logM       =   new log_model($this->db, $this->def);

                        $logSxData      = array(
                            'uid'       =>  $whereData['uid'],
                            'resume_id' =>  $whereData['id'],
                            'r_time'    =>  $eData['lastupdate'],
                            'port'      =>  $data['port'],
                            'ip'        =>  fun_ip_get()
                        );

                        $logM->addResumeSxLog($logSxData);
                    }
                }

                $resume     =   $this->select_once('resume', array('uid' => $whereData['uid']), '`name`');
                $resumeinfo =   $this->select_once('resume_expect', array('id' => $whereData['id']), '`name`,`integrity`');

                if ($return['id']) {

                    // 求职意向信息更新，调整完善度
                    $ur = $this->select_once('user_resume', array('eid'=>$whereData['id'], 'expect'=>0));
                    if (!empty($ur)){
                        $this->update_once('user_resume', array('expect'=>1), array('id'=>$ur['id']));
                        if ($resumeinfo['integrity'] <= 90){
                            $this->update_once('resume_expect', array('integrity'=>array('+',10)), array('id'=>$whereData['id']));
                        }
                    }
                    
                    if ($data['utype'] == 'admin') {

                        require_once 'log.model.php';
                        $logM   =   new log_model($this->db, $this->def);
                        $logM->addAdminLog('修改简历信息(ID:' . $whereData['id'] . ')');
                    }
                    $return['msg']      =   '简历信息修改成功！';
                    $return['errcode']  =   9;
                } else {

                    $return['msg']      =   '简历信息修改失败！';
                    $return['errcode']  =   8;
                }
            }

            return $return;
        }
    }

    /**
     * 查询求职意向
     * @param $whereData
     * @param array $data
     * @return array
     */
	public function getExpect($whereData,$data = array())
	{

	    $field   =  isset($data['field']) ? $data['field'] : '*';

	    $expect  =  $this -> select_once('resume_expect',$whereData, $field);

	    if ($expect && !empty($data['needCache'])){
            if (!empty($expect['birthday'])) {
                if (strlen($expect['birthday'] > 7)) { // 原来数据生日格式为年月日，如果是旧格式，改为年月
					$expect['birthday'] = date('Y-m',strtotime($expect['birthday']));
                }
            }

	        //处理薪资
	        $expect['salary']  =  salaryUnit($expect['minsalary'], $expect['maxsalary']);

	        if ($data['cache']){

	            $cache  =  $data['cache'];

	        }else{

	            $cache  =  $this -> getClass(array('user','city','job','hy'));
	        }
	        //处理职位类别id
	        if ($expect['job_classid'] ){

	            $job_classid = @explode(',',$expect['job_classid']);

	            if(is_array($job_classid)){

	                foreach($job_classid as $v){

	                    if($cache['job_name'][$v]){

	                        $job_classname[$v]  =  $cache['job_name'][$v];

	                        $jobclassid[]     =  $v;
	                    }
	                }
	                $expect['jobArr']         =  $jobclassid;
	                $expect['jobnameArr']     =  $job_classname;
	                $expect['job_classid']    =  is_array($jobclassid)?pylode(',',$jobclassid):'';
	                $expect['job_classname']  =	 @implode(',',$job_classname);
	            }
	        }
	        //处理城市类别id
	        if ($expect['city_classid']){

	            $city_classid = @explode(',',$expect['city_classid']);

	            if(is_array($city_classid)){

	                foreach($city_classid as $v){

	                    if($cache['city_name'][$v]){

	                        $city_classname[$v]  =  $cache['city_name'][$v];

	                        $cityclassid[]     =  $v;
	                    }
	                }
	                $expect['cityArr']         =  $cityclassid;
	                $expect['citynameArr']     =  $city_classname;
	                $expect['city_classid']    =  is_array($cityclassid)?pylode(',',$cityclassid):'';
	                $expect['city_classname']  =  @implode(' ',$city_classname);
	            }
	        }
			$expect['lastupdate_n']	  =   date('Y-m-d',$expect['lastupdate']);
			if($expect['state']=='1'){
				$expect['r_status_n'] =   '已审核';
			}elseif($expect['state']=='0'){
				$expect['r_status_n'] =   '正在审核中';
			}elseif($expect['state']=='3'){
				$expect['r_status_n'] =   '未通过审核';
			}
			$expect['exp_n']          =   $cache['userclass_name'][$expect['exp']];
			$expect['edu_n']          =   $cache['userclass_name'][$expect['edu']];
			$expect['hy_n']           =   isset($cache['industry_name'][$expect['hy']]) ? $cache['industry_name'][$expect['hy']] : '不限';
	        $expect['report_n']       =   $cache['userclass_name'][$expect['report']];
	        $expect['type_n']         =   $cache['userclass_name'][$expect['type']];
	        $expect['jobstatus_n']    =   $cache['userclass_name'][$expect['jobstatus']];
            $expect['ctime_n'] = date('Y-m-d',$expect['ctime']);
	        if ($expect['topdate'] > time()){
	            $day                  =    ($expect['topdate'] - strtotime(date('Y-m-d')))/86400;
	            $expect['top_day']    =   (int)$day;
	            $expect['topdate_n'] = date('Y-m-d',$expect['topdate']);
            }
            if($data['member']){
                $ur		=	$this->getUserResumeInfo(array('eid'=>$expect['id'],'uid'=>$whereData['uid']));
                if ($ur['expect'] == 0){
                    $expect['wstitle'] = '求职意向';

                }elseif ($ur['work'] == 0){
                    $expect['wstitle'] ='工作经历';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#work_upbox";
                }elseif ($ur['edu'] == 0){
                    $expect['wstitle'] ='教育经历';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#edu_upbox";
                }elseif ($ur['project'] == 0){
                    $expect['wstitle'] ='项目经历';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#project_upbox";
                }elseif ($ur['training'] == 0){
                    $expect['wstitle'] ='培训经历';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#training_upbox";
                }elseif ($ur['skill'] == 0){
                    $expect['wstitle'] ='职业技能';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#skill_upbox";
                }elseif($ur['other'] == 0){
                    $expect['wstitle'] ='其他信息';
                    $expect['wspcurl']= 'index.php?c=expect&e=' .$expect['id'] ."#other_upbox";
                }
            }
	    }
	    return $expect;
	}

    //查询单个教育经历
	public function getResumeEdu($where = array(), $field = '*', $userclass_name = null){

	    $edu   =   $this -> select_once('resume_edu', $where, $field);

      if($edu){
        $edu['sdate_n']  =   date('Y-m',$edu['sdate']);

        $edu['edate_n']  =   $edu['edate'] == 0 ? '至今' : date('Y-m',$edu['edate']);

        //如没有传入缓存，重新调取
        if ($userclass_name == null){

	        $cache     =   $this -> getClass('user');

	        $userclass_name  =  $cache['userclass_name'];
        }
        $edu['education_n']  =  $userclass_name[$edu['education']];
      }

	    return $edu;
	}
	/**
	 * 查询多个教育经历   $userclass_name 个人缓存，调用方法里已经有缓存的，可以先对学历进行处理
	 */
	public function getResumeEdus($where = array(), $field = '*', $userclass_name = null){

	    $edus  =   $this->select_all('resume_edu', $where, $field);

	    foreach ($edus as $k=>$v){
			$edus[$k]['content'] =   str_replace('\r\n', '<br/>', strip_tags($v['content'],'\r\n'));

	        $edus[$k]['sdate_n'] =   date('Y-m',$v['sdate']);

	        $edus[$k]['edate_n'] =   $v['edate'] == 0 ? '至今' : date('Y-m',$v['edate']);

			$edus[$k]['date_n']  =   $edus[$k]['sdate_n'].'-'.$edus[$k]['edate_n'];

	        if ($userclass_name){

	            $edus[$k]['education_n']    =   $userclass_name[$v['education']];
	        }

	    }

	    return $edus;
	}
	//添加/修改 教育经历
	public function addResumeEdu($addData,$data = array()){

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_edu',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_edu',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('edu',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询单条其他描述
	public function getResumeOther($where = array(), $field = '*'){

	    return $this -> select_once('resume_other', $where, $field);
	}


	//查询多条其他描述
	public function getResumeOthers($where = array(), $field = '*'){

		$others	=	$this -> select_all('resume_other', $where, $field);
		foreach ($others as $k=>$v){
			$others[$k]['content'] =   str_replace('\r\n', '<br/>', strip_tags($v['content'],'\r\n'));
		}
		return $others;
	}
	//添加/修改 其他描述
	public function addResumeOther($addData,$data = array()){

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_other',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_other',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('other',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询单条项目经历
	public function getResumeProject($where = array(), $field = '*'){

	    $project  =   $this->select_once('resume_project', $where, $field);

      if($project){

        $project['sdate_n'] =   date('Y-m',$project['sdate']);

         $project['edate_n'] =   $project['edate'] == 0 ? '至今' : date('Y-m',$project['edate']);
      }


	    return $project;
	}
	//查询多条项目经历
	public function getResumeProjects($where = array(), $field = '*'){

	    $projects  =   $this->select_all('resume_project', $where, $field);

	    foreach ($projects as $k=>$v){
			$projects[$k]['content'] =   str_replace('\r\n', '<br/>', strip_tags($v['content'],'\r\n'));

	        $projects[$k]['sdate_n'] =   date('Y-m',$v['sdate']);

	        $projects[$k]['edate_n'] =   $v['edate'] == 0 ? '至今' : date('Y-m',$v['edate']);

			$projects[$k]['date_n']  =   $projects[$k]['sdate_n'].'-'.$projects[$k]['edate_n'];
	    }

	    return $projects;
	}
	//添加/修改 项目经历
	public function addResumeProject($addData,$data = array()){

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_project',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_project',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('project',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询单条职业技能
	public function getResumeSkill($where = array(), $field = '*', $userclass_name = null){

	    $skill = $this->select_once('resume_skill', $where, $field);

		if(!empty($skill)){
			$cache				=	$this -> getClass(array('user'));
			$userclass_name   	= 	$cache['userclass_name'];
			$skill['pic'] 		=	checkpic($skill['pic']);
			$skill['ing_n'] 	=	$userclass_name[$skill['ing']];
		}

	    return $skill;
	}
	//查询多条职业技能
	public function getResumeSkills($where = array(), $field = '*', $userclass_name = null){
	    $cache						=	$this -> getClass(array('user'));
        $userclass_name   = $cache['userclass_name'];
        $skills = $this->select_all('resume_skill', $where, $field);
	    foreach ($skills as $k=>$v){
			$skills[$k]['ing_n']	=	$userclass_name[$v['ing']];
			$skills[$k]['pic'] 		=	checkpic($v['pic']);
			$skills[$k]['picurl']	=	$skills[$k]['pic'];
	    }
	    return $skills;
	}
	//添加/修改 职业技能
	public function addResumeSkill($addData,$data = array()){

	    if($addData['file']['tmp_name']){
	    	$upArr    =  array(
				'file'  =>  $addData['file'],
				'dir'   =>  'user'
			);
	    	include_once('upload.model.php');

	        $uploadM  =  new upload_model($this->db, $this->def);

			$picr      =  $uploadM->newUpload($upArr);

			if (!empty($picr['msg'])){

				$return['msg']		=	$picr['msg'];
				$return['errcode']	=	'8';
				return $return;

			}elseif (!empty($picr['picurl'])){

				$pictures 	=  	$picr['picurl'];
			}
	    }

	    unset($addData['file']);

	    if(isset($pictures)){

	    	$addData['pic']=$pictures;

	    }

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_skill',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_skill',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('skill',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询单条培训经历
	public function getResumeTrain($where = array(), $field = '*'){

	    $training  =   $this->select_once('resume_training', $where, $field);

	    $training['sdate_n'] =   date('Y-m',$training['sdate']);

	    $training['edate_n'] =   $training['edate'] == 0 ? '至今' : date('Y-m',$training['edate']);

	    return $training;
	}
	//查询多条培训经历
	public function getResumeTrains($where = array(), $field = '*'){

	    $trainings  =   $this->select_all('resume_training', $where, $field);

	    foreach ($trainings as $k=>$v){
			$trainings[$k]['content'] =   str_replace('\r\n', '<br/>', strip_tags($v['content'],'\r\n'));

	        $trainings[$k]['sdate_n'] =   date('Y-m', $v['sdate']);

	        $trainings[$k]['edate_n'] =   $v['edate'] == 0 ? '至今' : date('Y-m',$v['edate']);

			$trainings[$k]['date_n']  =   $trainings[$k]['sdate_n'].'-'.$trainings[$k]['edate_n'];
            $trainings[$k]['time_n']  =   $trainings[$k]['sdate_n'].'~'.$trainings[$k]['edate_n'];
	    }

	    return $trainings;
	}
	//添加/修改 培训经历
	public function addResumeTrain($addData,$data = array()){

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_training',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_training',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('training',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询单个工作经历
	public function getResumeWork($where = array(), $field = '*'){

	    $work   =   $this->select_once('resume_work', $where, $field);
	    if($work){

        $work['sdate_n'] =   date('Y-m',$work['sdate']);

        $work['edate_n'] =   $work['edate'] == 0 ? '至今' : date('Y-m',$work['edate']);
      }


	    return $work;
	}

    /**
     * 查询多个工作经历
     *
     * @param array $where
     * @param string $field
     * @return array|bool|false|string|void
     */
    public function getResumeWorks($where = array(), $field = '*')
    {

        $works  =   $this->select_all('resume_work', $where, $field);

        foreach ($works as $k => $v) {

            $works[$k]['content']   =   str_replace('\r\n', '<br/>', strip_tags($v['content'], '\r\n'));
            $works[$k]['sdate_n']   =   date('Y-m', $v['sdate']);
            $works[$k]['edate_n']   =   $v['edate'] == 0 ? '至今' : date('Y-m', $v['edate']);
            $works[$k]['date_n']    =   $works[$k]['sdate_n'] . '-' . $works[$k]['edate_n'];
            if ($v['edate'] == 0) {

                $works[$k]['time_n']=   $this->computeTime(time() - $v['sdate']);
            } else {

                $works[$k]['time_n']=   $this->computeTime($v['edate'] - $v['sdate']);
            }
        }
        return $works;
    }

    /**
     * @desc 时间戳差值转换成多少年多少月；
     * @param $timesDiff
     * @return string
     */
    private function computeTime($timesDiff)
    {

        $avgHour    =   ceil($timesDiff / (86400 * 30));

        if ($avgHour > 11) {

            $time   =   $avgHour > 11 ? avgToYm($avgHour) : $avgHour;

            if (bcmod($avgHour, 12) > 0) {

                return $time . '个月';
            } else {

                return $time;
            }
        } else if($avgHour > 0){

            return $avgHour . '个月';
        } else {

            return '1个月';
        }

    }

    //查询单条粘贴简历
	public function getResumeDoc($where = array(), $field = '*'){

	    return $this -> select_once('resume_doc', $where, $field);
	}
	//添加/修改 工作经历
	public function addResumeWork($addData,$data = array()){

	    if ($data['where'] && !empty($data['where'])){

	        $type     =  'up';
	        $success  =  $this->update_once('resume_work',$addData,$data['where']);

	    } else {
	        $type     =  'add';
	        $success  =	 $this->insert_into('resume_work',$addData);
	    }
	    $fbData   =  array(
	        'id'   =>  $data['where']['id'],
	        'eid'  =>  $addData['eid'],
	        'uid'  =>  $addData['uid']
	    );
	    // 简历附表处理完成的返回值处理
	    $return   =  $this->getFbReturn('work',$fbData,array('type'=>$type,'success'=>$success,'utype'=>$data['utype']));

	    return $return;
	}
	//查询多个简历证书，未见使用
	// public function getResumeCerts($where = array(), $field = '*'){

	// 	return $this->select_all('resume_cert', $where, $field);
	// }
	//查询单个简历附表
	public function getFb($table, $id ,$uid=''){

	    if ($table == 'resume_work'){

	        $info  =   $this -> getResumeWork(array('id'=>$id));

	    }elseif ($table == 'resume_edu'){

	        $cache  =  $this -> getClass('user');

	        $info   =  $this -> getResumeEdu(array('id'=>$id),'*',$cache['userclass_name']);

	    }elseif ($table == 'resume_training'){

	        $info  =   $this -> getResumeTrain(array('id'=>$id));

	    }elseif ($table == 'resume_skill'){

			$cache  =  $this -> getClass('user');

	        $info  =   $this -> getResumeSkill(array('id'=>$id),'*',$cache['userclass_name']);

	    }elseif ($table == 'resume_project'){

	        $info  =   $this -> getResumeProject(array('id'=>$id));

	    }elseif ($table == 'resume_other'){

	        $info  =   $this -> getResumeOther(array('id'=>$id));

	    }elseif ($table == 'resume_doc'){

	        $info  =   $this -> getResumeDoc(array('eid'=>$id));
	    }elseif ($table == 'resume_show'){

	        $info  =   $this -> getResumeShowInfo(array('id'=>$id));
	    }
		if($uid && $info['uid']!=$uid){
			$info = array();
		}
	    return $info;
	}
	public function getFbList($type, $where)
	{

	    $table  =  'resume_'.$type;

	    if ($table == 'resume_work'){

	        $list   =  $this -> getResumeWorks($where);

	    }elseif ($table == 'resume_edu'){

	        $cache  =  $this -> getClass('user');

	        $list   =  $this -> getResumeEdus($where,'*',$cache['userclass_name']);

	    }elseif ($table == 'resume_training'){

	        $list   =   $this -> getResumeTrains($where);

	    }elseif ($table == 'resume_skill'){

			$cache  =   $this -> getClass('user');

	        $list   =   $this -> getResumeSkills($where,'*',$cache['userclass_name']);

	    }elseif ($table == 'resume_project'){

	        $list   =   $this -> getResumeProjects($where);

	    }elseif ($table == 'resume_other'){

	        $list   =   $this -> getResumeOthers($where);

	    }elseif ($table == 'resume_show'){

	        $list   =  $this-> getResumeShowList($where);
	    }

	    return $list;
	}
	/**
	 * 删除简历附表
	 * @param string $table     'work'
	 * @param array $whereData  array('id'=>1,'eid'=>1)
	 * @return string
	 */
	public function delFb($table,$whereData = array(),$data = array()){

	    if(!in_array($table,array('expect','edu','other','project','show','skill','training','work'))){

	        $return['msg']      =  '请选择要删除的内容';
	        $return['errcode']  =  '8';

	        return $return;
	    }

	    $tableName  =  'resume_'.$table;

	    //处理查询条件，如某项传值有问题，即返回错误
	    foreach ($whereData as $v){

	        if (!$v){
	            return false;
	        }
	    }

        if($whereData['id']){
            $whereData['id'] = array('in', is_array($whereData['id']) ? pylode(',', $whereData['id']) : $whereData['id']);
        }

	    $success  =  $this -> delete_all($tableName, $whereData,'');


	    if(!empty($data['utype'])){

            $fbData   =  array(
              'id'   =>  $whereData['id'],
              'eid'  =>  $whereData['eid'],
              'uid'  =>  $whereData['uid']
            );
            //简历附表处理完成的返回值处理
            $return   =  $this -> getFbReturn($table,$fbData,array('type'=>'del','success'=>$success,'utype'=>$data['utype']));

            return $return;

        }else{

           return $success;
        }
	}
	/**
	 * 简历附表处理完成的返回值处理
	 * @param string $fbName          'work'
	 * @param array $fbData           array('id'=>1,'eid'=>'1')
	 * @param array $data             array('id'=>1,'type'=>'add','success'=>$nid)
	 * @return string
	 */
	private function getFbReturn($fbName,$fbData,$data = array('utype'=>null)){
	    //获取统一附表名称
//	    $msg  =  $this -> getFbName($fbName).'(ID:'.$fbData['id'].')';
        $msg  =  $this -> getFbName($fbName);
	    //根据操作类型，分别返回
	    if ($data['type'] == 'add'){

	        $msg  .=  '添加';
	        $type  =  1;
	    }elseif ($data['type'] ==  'up'){

	        $msg  .=  '修改';
	        $type  =  2;
	    }elseif ($data['type'] == 'del'){

	        $msg  .=  '删除';
	        $type  =  3;
	    }
        $msg .= "(eid:{$fbData['eid']},id:" . ($fbData['id'] ? is_array($fbData['id']) ? pylode(',', $fbData['id'][1]) : $fbData['id'] : $data['success']) . ")";
	    //操作是否成功判断
	    if ($data['success']){

	        $msg  .=  '成功';
	        $return['errcode']  =  '9';

	        if ($fbName == 'work'){
	            //处理求职意向表中的平均工作时长
	            $this -> workTime($fbData['eid']);
	        }
	        //操作成功的处理简历完整度
	        if ($data['type'] == 'add'){

	            $this -> upUserResume(array($fbName=>array('+',1)), array('eid'=>$fbData['eid'],'uid'=>$fbData['uid']));
	        }elseif ($data['type'] == 'del'){

	            $this -> upUserResume(array($fbName=>array('-',1)), array('eid'=>$fbData['eid'],'uid'=>$fbData['uid']));
	        }
	    }else {

	        $msg  .=  '失败';
	        $return['errcode']  =  '8';
	    }
	    $return['id']   =  $data['success'];
	    $return['msg']  =  $msg;
	    //后台操作的，添加管理员日志
	    if ($data['utype'] == 'admin'){

	        include_once('log.model.php');

	        $logM  =  new log_model($this->db, $this->def);

	        $logM -> addAdminLog($msg);
	    }
	    if ($data['utype'] == 'user'){

	        $this->addMemberLog($fbData['uid'], 1, $msg, 2, $type);
	    }
	    return $return;
	}
	//简历附表名称处理
	private function getFbName($fb){
	    $tname  =  array(
	        'work'      =>  '工作经历',
	        'edu'       =>  '教育经历',
	        'skill'     =>  '职业技能',
	        'project'   =>  '项目经历',
	        'training'  =>  '培训经历',
	        'other'     =>  '其他描述'
	    );

	    return $tname[$fb];
	}

    /**
     * 获取user_resume      详情
     * $whereData       查询条件
      * $data            自定义处理数组
    * 完整度的查询返回
     */
    public function getUserResumeInfo($whereData, $data = array()) {
        $data['field']		=	empty($data['field']) ? '*' : $data['field'];
        $resumeInfo			=   $this -> select_once('user_resume', $whereData, $data['field']);
        return $resumeInfo;
	}
	/**
	 * 修改简历完整度
	 * 参数格式
	 * $data      = array('work'=>array('+',1));
	 * $whereData = array('eid'=>'1')
	 */
	function upUserResume($data,$whereData){

	    if (isset($whereData['uid']) && isset($whereData['eid'])){
	        $skill  =  $this-> select_num('resume_skill',array('eid'=>$whereData['eid']));
	        $work   =  $this-> select_num('resume_work',array('eid'=>$whereData['eid']));
	        $pro    =  $this-> select_num('resume_project',array('eid'=>$whereData['eid']));
	        $edu    =  $this-> select_num('resume_edu',array('eid'=>$whereData['eid']));
	        $train  =  $this-> select_num('resume_training',array('eid'=>$whereData['eid']));
	        $cert   =  $this-> select_num('resume_cert',array('eid'=>$whereData['eid']));
	        $other  =  $this-> select_num('resume_other',array('eid'=>$whereData['eid']));
	        $integrity = 55;
	        $user_resume_sql['uid']  =	$whereData['uid'];
	        $user_resume_sql['eid']  =	$whereData['eid'];
	        $user_resume_sql['expect']  =  1;

	        if($work > 0){
	            $integrity  =  $integrity + 10;
	        }
	        $user_resume_sql['work']  =  $work;
	        if($edu > 0){
	            $integrity  =  $integrity + 10;
	        }
	        $user_resume_sql['edu']  =  $edu;
	        if($skill > 0){
	            $integrity  =  $integrity + 10;
	        }
	        $user_resume_sql['skill']  =  $skill;
	        if($pro > 0){
	            $integrity  =  $integrity + 8;
	        }
	        $user_resume_sql['project']  =  $pro;
	        if($train > 0){
	            $integrity  =  $integrity + 7;
	        }
	        $user_resume_sql['training']  =	$train;

	        $userResume  =  $this->select_once('user_resume',$whereData);

	        if (!empty($userResume)){
	            $this->update_once('user_resume',$user_resume_sql,$whereData);
	        }else{
	            $this->insert_into('user_resume',$user_resume_sql);
	        }
	        $this->update_once('resume_expect',array('integrity'=>$integrity),array('id'=>$whereData['eid']));
	    }
	}
	/**
	 * @desc 简历审核，个人不是已审核状态，弹出同步操作状态审核
	 * @param int $id
	 * @param array $data|state statusbody
	 */
	public function status($id, $data = array()){

	    if (!$id){

	        $return     =   array(
	            'errcode' => 8,
	            'msg'     => '参数错误！'
	        );
	        return $return;
	    }else{

	        $resume     =   $this->select_once('resume_expect', array('id' => $id), '`id`,`uid`,`name`,`state`');

            require_once 'userinfo.model.php';
            $userinfoM = new userinfo_model($this->db, $this->def);
            if (!empty($data['lock_status'])) {
                if ($data['lock_status'] == 1) { // 解锁的才能走
                    $result = $userinfoM->lock(array('uid' => $resume['uid']), array('post'=>array(
                        'status'     =>  $data['lock_status'],
                        'lock_info'  =>  ''
                    )));

                    if ($result['errcode'] != 9) {
                        return $result;
                    }
                } else {
                    $return     =   array(
                        'errcode' => 8,
                        'msg'     => '参数错误！'
                    );
                    return $return;
                }
            }

            $resumeinfo     =   $this->select_once('resume', array('uid' =>$resume['uid']), '`telphone`');

	        $upData     =   array(
	            'state'     =>  intval($data['state']),
	            'statusbody'=>  trim($data['statusbody'])
	        );


	        $uid        =   $data['uid'];

            $result     =   $this->setExpectState($upData,array('id' => $id, 'uid' => $uid));

	        if ($result) {

	            if ($data['state'] == '1') {
	                $state_n = '已通过';
	                $body    = '';
	                $msg     = '您的简历<a href="resumetpl,'.$id.'">《'.$resume['name'].'》</a>审核通过';


	                // 因简历未审核不能投递，记录的申请职位id，审核通过后，将简历按职位id，进行投递
					$reusmesqjob  =  $this -> select_once('resume_expect',array('id'=>$id),'`id`,`uid`,`sq_jobid`,`integrity`');
					if (!empty($reusmesqjob) && $reusmesqjob['sq_jobid'] > 0){
                        if ($this->config['user_sqintegrity'] <= $reusmesqjob['integrity']) {
                            $reusmesqjobinfos = $this->select_once('company_job', array('id' => $reusmesqjob['sq_jobid']), '`id`,`uid`,`name`,`com_name`,`status`');
                            if (!empty($reusmesqjobinfos) && $reusmesqjobinfos['status'] == 0) {
                                include_once('job.model.php');
                                $jobM = new job_model($this->db, $this->def);
                                $jobfield = '`com_name`,`name`,`uid`,`is_link`,`is_message`,`is_email`,`hy`,`job1`,`job1_son`,`job_post`,`provinceid`,`cityid`,`three_cityid`,`minsalary`,`maxsalary`';
                                $comjob = $jobM->getInfo(array('id' => $reusmesqjob['sq_jobid']), array('field' => $jobfield));
                                $sqdata = array(
                                    'job_id' => $reusmesqjob['sq_jobid'],
                                    'com_name' => $reusmesqjobinfos['com_name'],
                                    'job_name' => $reusmesqjobinfos['name'],
                                    'com_id' => $reusmesqjobinfos['uid'],
                                    'uid' => $reusmesqjob['uid'],
                                    'eid' => $reusmesqjob['id'],
                                    'datetime' => time(),
                                    'resume_state' => $data['state'],
                                );
                                $jobM->addSqJob($sqdata, array('comjob' => $comjob));
                            }
                        }
                        $this -> update_once('resume_expect',array('sq_jobid'=>''), array('id' => $id, 'uid' => $uid));
                    }
			    }else{
			        $state_n = '未通过';
			        $body    = '。原因：'.$data['statusbody'];
			        $msg     = '您的简历<a href="resumetpl,'.$id.'">《'.$resume['name'].'》</a>审核未通过；原因：'.$data['statusbody'];
			    }
                if($resumeinfo['telphone'] && $resume['state']!=$upData['state']){

                    require_once 'notice.model.php';
                    $noticeM    =   new notice_model($this->db, $this->def);
                    $smsdata       =   array(
                        'uid'       =>  $resume['uid'],
                        'usertype'  =>  1,
                        'moblie'    =>  $resumeinfo['telphone'],
                        'type'      =>  'resumestate',
                        'resumename'=>  $resume['name'],
                        'state_n'   =>  $state_n,
                        'port'      =>  '1'
                    );
                    $noticeM->sendSMSType($smsdata);
                }


	            //发送系统通知
	            require_once 'sysmsg.model.php';
	            $sysmsgM    =   new sysmsg_model($this->db, $this->def);
	            $sysmsgM -> addInfo(array('uid' => $uid,'usertype'=>1,'content'=>$msg));

	            $return = array(
	                'errcode' => 9,
	                'msg'     => '简历审核'.$state_n.'设置成功！(ID:'.$id.$body.')'
	            );
	        }else{
	            $return = array(
	                'errcode' => 8,
	                'msg'     => '简历审核设置失败！(ID:'.$id.')'
	            );
	        }
	        return $return;
	    }
	}

    /**
     * 后台简历审核
     * @param string $id 格式：单个，如1 ; 批量，如1,2,3
     * @param array $data
     * @return mixed
     */
	public function statusResume($id,$data = array()){

        $id  =  is_array($id) ? $id : @explode(',',$id);

	    foreach($id as $v){

	        if($v){

	            $ids[]  =  $v;
	        }
	    }

	    if (!empty($ids)){

	        $idstr     =  pylode(',', $ids);

	        $post      =  $data['post'];

            //查询出需审核的简历的名称，为发送系统通知做准备
            $expects  =  $this -> getList(array('id'=>array('in',$idstr),'r_status'=>1),array('field'=>'id,uid,name,state','withResumeField'=>'`uid`,`telphone`'));

	        $result    =  $this -> upInfo(array('id'=>array('in',$idstr),'r_status'=>1),array('eData'=>$post));

	        if ($result){
				if($post['state']==1){
					$reusmesqjob  =  $this -> select_all('resume_expect',array('id'=>array('in',$idstr)),'`id`,`uid`,`sq_jobid`,`integrity`');
					foreach($reusmesqjob as $val){
						$job_ids[]= $val['sq_jobid'];
					}
                    $reusmesqjobinfos  =  $this -> select_all('company_job',array('id'=>array('in',pylode(',', $job_ids))),'`id`,`uid`,`name`,`com_name`,`is_link`,`is_message`,`is_email`,`hy`,`job1`,`job1_son`,`job_post`,`provinceid`,`cityid`,`three_cityid`,`minsalary`,`maxsalary`,`status`');

                    include_once('job.model.php');
                    $jobM  =  new job_model($this->db, $this->def);

                    foreach($reusmesqjob as $k=>$v){
                        if ($this->config['user_sqintegrity'] < $v['integrity'] || $this->config['resume_kstd_req'] == 2 || $v['state']=='0') { // 简历完整度大于系统完整度 或者 快速投递是否遵从简历必填项为关闭 或者 简历状态未审核
                            foreach ($reusmesqjobinfos as $val) {
                                if ($v['sq_jobid'] == $val['id'] && $val['status'] == 0) {

                                    $ysqnum = $jobM->getSqJobNum(array('job_id' => $val['id'], 'com_id' => $val['uid'], 'uid' => $v['uid'], 'eid' => $v['id'], 'isdel' => 9));
                                    if ($ysqnum < 1) {

                                        $sqValue = array('job_id' => $val['id'], 'com_name' => $val['com_name'], 'job_name' => $val['name'], 'com_id' => $val['uid'], 'uid' => $v['uid'], 'eid' => $v['id'], 'datetime' => time(), 'resume_state' => $post['state']);
                                        $jobM->addSqJob($sqValue, array('comjob' => $val));
                                    }
                                }
                            }
                        }
					}
                    $this -> update_once('resume_expect',array('sq_jobid'=>''),array('id'=>array('in',$idstr)));
				}
				if ($post['state'] == 1 || $post['state'] == 3){
					$msg      =  array();
					$uids     =  array();
					/* 处理审核信息 */
					if ($post['state'] == 3){

					    $state_n =  '未通过';
					    $body    =  !empty($post['statusbody']) ? '。 原因：'.$post['statusbody'] : '';

					}elseif($post['state'] == 1){

					    $state_n =  '已通过';
					}

                    require_once 'notice.model.php';
                    $noticeM    =   new notice_model($this->db, $this->def);

					if($expects && is_array($expects)){
						foreach ($expects['list'] as $k=>$v){

							$uids[]  =  $v['uid'];
						    /* 处理审核信息 */
							if ($post['state'] == 3){

								$msg[$v['uid']][]  =  '您的简历《'.$v['name'].'》审核未通过'.$body;

							}elseif($post['state'] == 1){

							    $msg[$v['uid']][]  =  '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>已审核通过';
							}
                            if($v['telphone'] && $v['state']!=$post['state']){
                                $smsdata       =   array(
                                    'uid'       =>  $v['uid'],
                                    'usertype'  =>  1,
                                    'moblie'    =>  $v['telphone'],
                                    'type'      =>  'resumestate',
                                    'resumename'=>  $v['name'],
                                    'state_n'   =>  $state_n,
                                    'port'      =>  '1'
                                );
                                $noticeM->sendSMSType($smsdata);
                            }
						}
						//发送系统通知
						include_once('sysmsg.model.php');
						$sysmsgM  =  new sysmsg_model($this->db, $this->def);
						$sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));
					}
				}

				$resumewhere['id']      =     array('in',$idstr);
				$resumenum              =     $this->getExpectNum($resumewhere);
				if($resumenum>1){
                  $resumetwhere['id']           =     array('in',$idstr);
                  $resumetwhere['r_status']     =     1;
                  $resumetnum                   =     $this->getExpectNum($resumetwhere);

                  $resumewwhere['id']           =     array('in',$idstr);
                  $resumewwhere['r_status']     =     2;
                  $resumewnum              =     $this->getExpectNum($resumewwhere);
                  if($resumewnum>0){
                     $return['msg']      =  '简历批量审核'.$state_n.'成功'.$resumetnum.'条，失败'.$resumewnum.'条。原因:个人账户被锁定';
                  }else{
                    $return['msg']       =  '简历批量审核'.$state_n.'成功(ID:'.$idstr.$body.')';
                  }

                  $return['errcode']  =  9;
                }else{
					$resumewwhere['id']           =     array('in',$idstr);
					$resumewwhere['r_status']     =     2;
					$resumetnum                   =     $this->getExpectNum($resumewwhere);
					if($resumetnum>0){
						$return['msg']      =  '审核简历'.$state_n.'失败，原因:个人账户被锁定(ID:'.$idstr.')';
						$return['errcode']  =  8;
					}else{
					    $return['msg']      =  '审核简历'.$state_n.'设置成功(ID:'.$idstr.$body.')';
						$return['errcode']  =  9;
					}

                }
	        }else{
	            $return['msg']      =  '审核简历设置失败(ID:'.$idstr.')';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要审核的简历';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}

    /**
     * 刷新简历
     * @param string $id 格式：单个，如1 ; 批量，如1,2,3
     * @param array $data
     * @return mixed
     */
    public function refreshResume($id)
    {

        $id =  is_array($id) ? $id :  @explode(',', $id);

        foreach ($id as $v) {
            if ($v) {

                $ids[]  =   $v;
            }
        }
        if (!empty($ids)) {

            $idstr  =   pylode(',', $ids);

            $post   =   array('lastupdate' => time());

            $result['id']   =   $this->update_once('resume_expect', $post, array('id' => array('in', $idstr)));

            if ($result['id']) {

                $msg    =   array();
                $uids   =   array();

                //查询出需刷新的简历的名称，为发送系统通知做准备
                $expects=   $this->getList(array('id' => array('in', $idstr)), array('field' => 'id,uid,name'));

                $logSxData          =   array();
                foreach ($expects['list'] as $k => $v) {

                    $uids[]         =   $v['uid'];

                    $msg[$v['uid']] =   '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>已刷新';

                    $logSxData[$k]['uid']       =   $v['uid'];
                    $logSxData[$k]['resume_id'] =   $v['id'];
                    $logSxData[$k]['r_time']    =   time();
                    $logSxData[$k]['port']      =   5;
                    $logSxData[$k]['ip']        =   fun_ip_get();
                }
                //简历刷新成功，修改个人基本信息表中的更新时间
                $this->update_once('resume', $post, array('uid' => array('in', pylode(',', $uids))));
                //发送系统通知
                include_once('sysmsg.model.php');
                $sysmsgM    =   new sysmsg_model($this->db, $this->def);
                $sysmsgM->addInfo(array('uid' => $uids, 'usertype' => 1, 'content' => $msg));

                include_once('log.model.php');
                $logM       =   new log_model($this->db, $this->def);
                $logM->addResumeSxLogS($logSxData);

                $return['msg']      =   '刷新简历(ID:' . $idstr . ')设置成功';
                $return['errcode']  =   '9';
            } else {

                $return['msg']      =   '刷新简历(ID:' . $idstr . ')设置失败';
                $return['errcode']  =   '8';
            }
        } else {

            $return['msg']      =   '请选择要刷新的简历';
            $return['errcode']  =   '8';
        }
        return $return;
    }

	/**
	 * 推荐简历
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function recResume($id,$rec){
        $id  = is_array($id) ? $id : @explode(',',$id);

	    foreach($id as $v){

	        if($v){

	            $ids[]  =  $v;
	        }
	    }
	    if (!empty($ids)){

	        $idstr   =  pylode(',', $ids);

	        $post    =  array('rec_resume'=>$rec);

	        $result['id']  =  $this -> update_once('resume_expect', $post, array('id'=>array('in',$idstr)));

	        if ($result['id']){

	            $msg      =  array();
	            $uids     =  array();
	            //查询出需推荐的简历的名称，为发送系统通知做准备
	            $expects  =  $this -> getList(array('id'=>array('in',$idstr)),array('field'=>'id,uid,name'));

	            foreach ($expects['list'] as $k=>$v){

	                $uids[]  =  $v['uid'];

	                if ($rec == 1){

	                    $msg[$v['uid']]  =  '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员设为推荐简历';

	                }elseif ($rec == 0){

	                    $msg[$v['uid']]  =  '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员设为非推荐简历';
	                }
	            }
	            //发送系统通知
	            include_once('sysmsg.model.php');

	            $sysmsgM  =  new sysmsg_model($this->db, $this->def);

	            $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));

                if ($rec == 1) {
                    $return['msg'] = '推荐简历(ID:' . $idstr . ')设置成功';
                    $return['errcode'] = '9';
                }elseif ($rec == 0){
                    $return['msg'] = '取消推荐简历(ID:' . $idstr . ')设置成功';
                    $return['errcode'] = '9';
                }
	        }else{
	            $return['msg']      =  '推荐简历(ID:'.$idstr.')设置失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要推荐的简历';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 置顶简历
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function topResume($id,$post){

        $id  =  is_array($id) ? $id : @explode(',',$id);

	    foreach($id as $v){

	        if($v){

	            $ids[]  =  $v;
	        }
	    }
	    if (!empty($ids)){

	        $idstr   =  pylode(',', $ids);

	        $result['id']  =  $this -> update_once('resume_expect', $post, array('id'=>array('in',$idstr)));

	        if ($result['id']){

	            $msg      =  array();
	            $uids     =  array();
	            //查询出需置顶的简历的名称，为发送系统通知做准备
	            $expects  =  $this -> getList(array('id'=>array('in',$idstr)),array('field'=>'id,uid,name'));

	            foreach ($expects['list'] as $k=>$v){

	                $uids[]  =  $v['uid'];

	                if ($post['top'] == 1){

	                    $msg[$v['uid']]  =  '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>已置顶';

	                }elseif ($post['top'] == 0){

	                    $msg[$v['uid']]  =  '您的简历<a href="resumetpl,'.$v['id'].'">《'.$v['name'].'》</a>被管理员取消置顶';
	                }
	            }
	            //发送系统通知
	            include_once('sysmsg.model.php');

	            $sysmsgM  =  new sysmsg_model($this->db, $this->def);

	            $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));

                if ($post['top'] == 1) {
                    $return['msg'] = '置顶简历(ID:' . $idstr . ')设置成功';
                    $return['errcode'] = '9';
                }elseif ($post['top'] == 0){
                    $return['msg'] = '取消置顶简历(ID:' . $idstr . ')设置成功';
                    $return['errcode'] = '9';
                }
	        }else{
	            $return['msg']      =  '置顶简历(ID:'.$idstr.')设置失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要置顶的简历';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
    /**
     * 后台简历状态修改
     */
    public function cstatus($uid,$post){
        if (!empty($uid)){
            $return1 = $this->update_once('resume', $post, array('uid' => $uid));
            $return2 = $this->update_once('resume_expect', $post, array('uid' => $uid));

            $return['msg']      =   '简历状态(UID:'.$uid.')';
            $return['msg']      =   $return1 && $return2 ? $return['msg'].'设置成功！' : $return['msg'].'设置失败！';
            //操作状态 9：成功 8:失败 配合原有提示函数
            $return['errcode']  =   $return1 && $return2 ? '9' :'8';

            return $return;
        }
    }

    /**
     * @desc 备注简历
     * @param array $data （id,uid,auid,label,content）
     * @return mixed
     */
    public function label($data = array())
    {
        if (!empty($data['id'])){
            $result = $this->update_once('resume_expect', array('label' => $data['label'], 'content' => $data['content']), array('id' => $data['id']));
            
            $return['msg']  =	'简历备注(ID:'.$data['id'].')';
        }
        $return['msg']		=	$result ? $return['msg'].'设置成功！' : $return['msg'].'设置失败！';
        //操作状态 9：成功 8:失败 配合原有提示函数
        $return['errcode']	=	$result ? '9' :'8';
        
        return $return;
    }

	/**
	 * 删除简历
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data	utype=admin; delAccount=1, 同步删除账号
	 */
	public function delResume($id,$data=array()){

	    $limit                =  'limit 1';
	    $return['layertype']  =	 0;

	    if (!empty($id)){

	        if(is_array($id)){

	            $id                   =  pylode(',', $id);
	            $return['layertype']  =  1;
	            $limit                =  '';
	        }

			if($data['utype']!='admin'){
				//如果不是后台操作 必须传入UID

				$elist  =  $this -> getList(array('id'=>array('in',$id)),array('field'=>'`id`,`uid`'));
				foreach($elist['list'] as $v){

					if($data['uid'] != $v['uid']){
						$return['msg']      =  '非法操作！';
						$return['errcode']  =  '8';
						return $return;
					}

					$resumenum=  $this -> getExpectNum(array('uid'=>array('in',$v['uid'])));
					if($resumenum==1){
						$return['msg']      =  '请至少保留一份简历！';
						$return['errcode']  =  '8';
						return $return;
					}
				}
			}else if($data['utype'] == 'admin' && $data['delAccount'] == '1'){	// 同步删除账号，提取账号UID

				$elist  =  $this -> getList(array('id'=>array('in',$id)),array('field'=>'`id`,`uid`'));

				$euids	=	array();
				foreach($elist['list'] as $ek => $ev){

					$euids[$ev['uid']]	=	$ev['uid'];
				}

				require_once ('userinfo.model.php');
				$userinfoM	=	new	userinfo_model($this->db, $this->def);
				return  $userinfoM -> delMember($euids);
			}

            //查询删掉的是默认简历的数据
			$expects  =  $this -> getList(array('id'=>array('in',$id),'defaults'=>1),array('field'=>'`id`,`uid`'));

			$expectA  =  $this -> select_all('resume_expect',array('id'=>array('in',$id)),'`uid`');
			if (!empty($expectA)) {
			    foreach ($expectA as $ev) {
			        $expectAuids[$ev['uid']] =   $ev['uid'];
			    }
			}


			$nid  =  $this -> delete_all('resume_expect',array('id'=>array('in',$id)),$limit);

    		if ($nid){
                //删除简历时修改默认简历
                $this -> upDefaults($expects['list']);

                $this -> delete_all('down_resume',array('eid'=>array('in',$id)),'');

                $this -> delete_all('look_resume',array('resume_id'=>array('in',$id)),'');

                $this -> delete_all('resume_city_job_class',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_cityclass',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_doc',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_edu',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_jobclass',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_other',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_refresh_log',array('resume_id'=>array('in',$id)),'');

                $this -> delete_all('resume_remark',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_project',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_show',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_skill',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_training',array('eid'=>array('in',$id)),'');

                $this -> delete_all('resume_work',array('eid'=>array('in',$id)),'');

                $this -> delete_all('report',array('eid'=>array('in',$id), 'usertype' => 2, 'type' => 0),'');

                $this -> delete_all('talent_pool',array('eid'=>array('in',$id)),'');

                $this -> delete_all('user_entrust',array('eid'=>array('in',$id)),'');

                $this -> delete_all('user_entrust_record',array('eid'=>array('in',$id)),'');

                $this -> delete_all('user_resume',array('eid'=>array('in',$id)),'');

                $this -> delete_all('userid_job',array('eid'=>array('in',$id)),'');

                if (!empty($expectAuids)){

                    $this -> delete_all('look_job',array('uid' => array('in', pylode(',',$expectAuids))),'');
                }

                $return['id']       =  $nid;
                $return['msg']      =  '简历(ID:'.$id.')删除成功';
                $return['errcode']  =  '9';

            }else{
                $return['msg']      =  '简历(ID:'.$id.')删除失败';
                $return['errcode']  =  '8';

            }
	    }else{
	        $return['msg']      =  '请选择要删除的简历';
	        $return['errcode']  =  '8';

	    }

	    return $return;
	}
	//删除简历时修改默认简历,并处理与uid相关数据
	private function upDefaults($list = array()){

	    $uids  =  array();
	    foreach ($list as $k=>$v){

	        $uids[]  =  $v['uid'];
	    }
	    if (!empty($uids)){
	        //先去除重复的uid
	        $uids  =  array_unique($uids);

	        $newdef   =  array();//要修改的默认简历数据
	        $newids   =  array();//要修改的默认简历id
	        $newuids  =  array();//还有简历的用户uid
	        $newrnum  =  array();//要修改的简历数量数据
	        //根据删除简历的uid来查询删除后剩下的相关简历
	        $newExpects  =  $this -> getList(array('uid'=>array('in',pylode(',', $uids)),'groupby'=>'uid'),array('field'=>'id,uid,count(id) as rnum'));
	        //被删除简历的用户，def_job默认为0
	        foreach ($uids as $uk=>$uv){
	            $newdef[$uv]   =  0;
	            $newrnum[$uv]  =  0;
	        }
	        //删除后还有简历的，def_job设置查到的简历id
	        foreach ($newExpects['list'] as $ek=>$ev){

	            $newids[]             =  $ev['id'];
	            $newuids[]            =  $ev['uid'];
	            $newdef[$ev['uid']]   =  $ev['id'];
	            $newrnum[$ev['uid']]  =  $ev['rnum'];
	        }
	        //修改基本信息中的默认简历数据
	        $this -> update_once('resume', array('def_job'=>array('CASE','uid',$newdef)), array('uid'=>array('in',pylode(',', $uids))));
	        //修改简历中的默认简历数据
	        $this -> update_once('resume_expect', array('defaults'=>1), array('id'=>array('in',pylode(',', $newids))));
	        //修改现有简历数量
	        $this -> update_once('member_statis',array('resume_num'=>array('CASE','uid',$newrnum)),array('uid'=>array('in',pylode(',', $newuids))));
	        //数组去重，取得已经没有简历的用户uid
	        $noResume = array_diff($uids, $newuids);
	        if (!empty($noResume)){
	            //删除已没有简历用户的关注记录
	            $this -> delete_all('atn',array('uid'=>array('in',pylode(',', $noResume))),'');
	            //删除已没有简历用户的职位浏览记录
	            $this -> delete_all('look_job',array('uid'=>array('in',pylode(',', $noResume))),'');
	            //删除已没有简历用户的邀请面试记录
	            $this -> delete_all('userid_msg',array('uid'=>array('in',pylode(',', $noResume))),'');
	        }
	    }
	}
	/**
	 * 后台个人认证审核
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function statusCert($uid,$data = array()){


	    $uid  =  @explode(',',$uid);

	    if (count($uid) > 1){

	        $return['layertype']  =  1;
	    }else {

	        $return['layertype']  =  0;
	    }

	    foreach($uid as $v){

	        if(!empty($v)){

	            $uids[]  =  $v;
	        }
	    }
	    if (!empty($uids)){

	        $uidstr  =  pylode(',', $uids);

	        $post    =  $data['post'];

	        $result  =  $this -> update_once('resume', $post, array('uid'=>array('in',$uidstr)));

	        if ($result){

	            $status  =  $post['idcard_status'];
	            $this -> update_once('resume_expect', array('idcard_status'=>$status), array('uid'=>array('in',$uidstr)));
	            if ($status == 1 || $status == 2){
	                //通过审核，并后台设置的上传身份验证所加积分大于0，才需要处理加积分
	                if ($status == 1 && $this->config['integral_identity'] > 0){

	                    $compays  =  $this -> select_all('company_pay',array('com_id'=>array('in',$uidstr),'pay_remark'=>'上传身份验证'),'com_id');

	                    if (!empty($compays)){
	                        //获取已经获得过验证积分的uid
	                        foreach ($compays as $v){

	                            $payed[]  =  $v['com_id'];
	                        }
	                        //取差集，得到还未获得认证积分的uid
	                        $needs  =  array_diff($uids, $payed);
	                    }else{

	                        $needs  =  $uids;
	                    }
	                    //发放身份认证积分
	                    if(!empty($needs)){
	                        include_once('integral.model.php');

	                        $integralM  =  new integral_model($this->db, $this->def);

	                        foreach ($needs as $v){

	                            $integralM -> invtalCheck($v,1,'integral_identity','上传身份验证');
	                        }
	                    }
	                }

                    foreach ($uids as $k=>$v){

                      /* 处理审核信息 */
                      if ($post['idcard_status'] == 2){

                        $statusInfo  =  '您的身份证审核未通过 ';

                        if($post['statusbody']){

                          $statusInfo  .=  ', 原因：'.$post['statusbody'];

                        }

                        $msg  =  $statusInfo;

                      }elseif($post['idcard_status'] == 1){

                        $msg  =  '您的身份证已审核通过';

                      }
	                }

	                //发送系统通知
	                include_once('sysmsg.model.php');

	                $sysmsgM  =  new sysmsg_model($this->db, $this->def);

	                $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));
	                //发邮件通知
	                if ($this->config['sy_email_usercert'] == '1'){

	                    $tplData  =  array(
	                        'certinfo'  =>  $msg,
	                        'type'      =>  'usercert'
	                    );
	                    include_once('notice.model.php');

	                    $noticeM   =  new notice_model($this->db, $this->def);

	                    $emailTpl  =  $noticeM -> getTpl($tplData,'email');

	                    $resumes   =  $this -> select_all('member',array('uid'=>array('in',$uidstr)),'uid,email');

                      foreach($resumes as $v){

	                        $edata  =  array(
	                            'uid'      =>  $v['uid'],
	                            'cuid'     =>  0,
                              'certinfo' => $msg,
	                            'email'    =>  $v['email'],
	                            'type'     =>  'usercert'
	                        );

	                        $noticeM -> sendEmailType($edata,$emailTpl);
	                    }
	                }
	            }

	            $return['msg']      =  '个人认证审核(ID:'.$uidstr.')设置成功';
	            $return['errcode']  =  '9';
	        }else{
	            $return['msg']      =  '个人认证审核(ID:'.$uidstr.')设置失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要审核的认证';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 删除个人身份验证
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function delResumeCert($uid){

	    $return['layertype']  =	 0;

	    if (!empty($uid)){

	        if(is_array($uid)){

	            $uid                   =  pylode(',', $uid);
	            $return['layertype']  =  1;
	        }
	        $cdata  =  array(
	            'idcard_pic'     =>  '',
	            'idcard_status'  =>  0,
	            'cert_time'      =>  '',
	            'statusbody'     =>  ''
	        );

	        $nid      =  $this -> update_once('resume',$cdata,array('uid'=>array('in',$uid)));
	        if ($nid){
	            $return['msg']      =  '个人身份认证(ID:'.$uid.')删除成功';
	            $return['errcode']  =  '9';
	        }else{
	            $return['msg']      =  '个人身份认证(ID:'.$uid.')删除失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要删除的个人身份认证';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 后台个人头像审核
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function statusPhoto($uid,$data = array()){

	    $uid  =  @explode(',',$uid);

	    foreach($uid as $v){

	        if($v){

	            $uids[]  =  $v;
	        }
	    }
	    if (!empty($uids)){

	        $uidstr  =  pylode(',', $uids);

	        $post    =  $data['post'];

	        if ($post['photo_status'] == 2){
	        	//审核不通过删除图片
		    	$post['photo']	='';
		    }

	        $result  =  $this -> update_once('resume', $post, array('uid'=>array('in',$uidstr)));
	        if ($result){

	            if ($post['photo_status'] == 2){
	                // 审核不通过，相关表头像删除
	                $this -> update_once('resume_expect',array('photo'=>''),array('uid'=>array('in',$uidstr)));
	                $this -> update_once('answer',array('pic'=>''),array('uid'=>array('in',$uidstr)));
	                $this -> update_once('question',array('pic'=>''),array('uid'=>array('in',$uidstr)));

	                $statusInfo  =  '您的头像';

					foreach ($uids as $k=>$v){

						/* 处理审核信息 */
					    if($post['photo_statusbody']){

					        $statusInfo  .=  ' , 因为'.$post['photo_statusbody'].' , ';
					    }

					    $statusInfo  .=  '已被管理员删除';

					    $msg[$v]  =  $statusInfo;
	                }

	                //发送系统通知
	                include_once('sysmsg.model.php');

	                $sysmsgM  =  new sysmsg_model($this->db, $this->def);

	                $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));

	            }elseif ($post['photo_status'] == 0){
	                // 审核通过，修改相关表头像
	                $resume  =  $this -> select_all('resume',array('uid'=>array('in',$uidstr)),'`uid`,`photo`');
	                foreach ($resume as $k=>$v){

	                    $newphoto[$v['uid']]   =  $v['photo'];
	                }
	                //修改基本信息中的默认简历数据
	                $this -> update_once('resume_expect', array('photo'=>array('CASE','uid',$newphoto)), array('uid'=>array('in',$uidstr)));
	                $this -> update_once('answer',array('pic'=>array('CASE','uid',$newphoto)),array('uid'=>array('in',$uidstr)));
	                $this -> update_once('question',array('pic'=>array('CASE','uid',$newphoto)),array('uid'=>array('in',$uidstr)));
	            }
	            $return['msg']      =  '头像审核(ID:'.$uidstr.')设置成功';
	            $return['errcode']  =  '9';
	        }else{
	            $return['msg']      =  '头像审核(ID:'.$uidstr.')设置失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要审核的头像';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 后台个人作品审核
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 * @param array $data
	 */
	public function statusShow($id,$data = array()){

	    $id  =  @explode(',',$id);

	    foreach($id as $v){

	        if($v){

	            $ids[]  =  $v;
	        }
	    }
	    if (!empty($ids)){

	        $idstr  =  pylode(',', $ids);

	        $shows	=	$this	->	getResumeShowList(array('id'=>array('in',$idstr)),array('field'=>'`uid`,`title`'));

	        $post    =  $data['post'];

	        if ($post['status'] == 2){
	        	//审核不通过删除
		    	$result	=	$this	->	delete_all('resume_show', array('id'=>array('in',$idstr)),'');
		    }elseif($post['status'] == 0){
		    	$result	=	$this	->	update_once('resume_show', $post, array('id'=>array('in',$idstr)));
		    }



	        if ($result){

	            if ($post['status'] == 0 || $post['status'] == 2){

	                foreach ($shows as $k=>$v){
						$uids[]				=	$v['uid'];
						/* 处理审核信息 */
						if ($post['status'] == 2){

						    if ($v['title'] != ''){

                                $statusInfo		=	'您的作品案例('.$v['title'].')审核未通过';
                            }else{

                                $statusInfo		=	'您的作品案例审核未通过';
                            }

							if($post['statusbody']){

								$statusInfo     .=  ', 原因：'.$post['statusbody'];
							}

							$msg[$v['uid']][]   =   $statusInfo;

						}elseif($post['status'] == 0){

                            if ($v['title'] != ''){
                                $msg[$v['uid']][]  =  '您的作品案例('.$v['title'].')已审核通过';
                            }else{
                                $msg[$v['uid']][]  =  '您的作品案例已审核通过';
                            }
						}
	                }
					//发送系统通知
	                include_once('sysmsg.model.php');

	                $sysmsgM  =  new sysmsg_model($this->db, $this->def);

	                $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>1, 'content'=>$msg));

	            }

	            $return['msg']      =  '作品案例审核(ID:'.$idstr.')设置成功';
	            $return['errcode']  =  '9';
	        }else{
	            $return['msg']      =  '作品案例审核(ID:'.$idstr.')设置失败';
	            $return['errcode']  =  '8';
	        }
	    }else{
	        $return['msg']      =  '请选择要审核的作品案';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 修改个人头像
	 * @param array $whereData
	 * @param array $data   photo/需上传的图片文件;   thumb/已处理好的缩略图;  utype/操作的用户类型;  base/需上传的base4图片;  preview/pc预览即上传
	 */
	public function upPhoto($whereData = array(),$data=array('photo'=>null,'thumb'=>null,'utype'=>null,'base'=>null,'preview'=>null))
	{

	    if (!empty($whereData['uid'])){

	        $uid  =  $whereData['uid'];
	        // 头像还需上传的
	        if ($data['photo']['tmp_name'] || $data['base']){

	            $upArr   =  array(
	                'file'     =>  $data['photo'],
	                'dir'      =>  'user',
	                'type'     =>  'logo',
	                'base'     =>  $data['base'],
	                'preview'  =>  $data['preview'],
                    'watermark'=>  0
	            );
	            $result  =  $this -> upload($upArr);

	            if (!empty($result['msg'])){

	                $return['msg']      =  $result['msg'];
	                $return['errcode']  =  '8';
	                return $return;

	            }elseif (!empty($result['picurl'])){

	                $photo  =  $result['picurl'];
	            }
	        }
	        // 已处理好的头像缩略图
	        if ($data['thumb']){

	            $photo  =  str_replace('../data','./data',$data['thumb']);

	        }

	        if (!empty($photo)){
	            // 用户操作，且后台设置用户头像需要审核的
	            $rinfo=$this->getResumeInfo(array('uid'=>$uid),array('field'=>'r_status'));
	            if ($data['utype'] == 'user' && $this -> config['user_photo_status'] == 1){

	                $photo_status  =  1;
	            }else{
	                $photo_status  =  $rinfo['r_status']!=1?1:0;
	            }

                $resumeData = array('photo'=>$photo,'defphoto'=>1);

                $resumeData['photo_status'] = $photo_status;

	            $return['id']  =  $this -> update_once('resume',$resumeData,array('uid'=>$uid));
	        }

	        if (isset($return['id'])) {
	            // 用户操作的，判断处理头像上传积分
	            if ($data['utype'] == 'user'){

	                require_once ('integral.model.php');

	                $IntegralM 	= 	new integral_model($this -> db, $this -> def);
	                $IntegralM	->	invtalCheck($uid,1,'integral_avatar','上传头像');

	                $this -> addMemberLog($uid, 1, '图片信息：上传头像', 16, 1);

	                if ($this -> config['user_photo_status'] == 1){
						// 需审核时，简历表，以前的头像要清除
	                    $this -> update_once('resume_expect',array('photo'=>'', 'defphoto' => 1),array('uid'=>$uid));
	                    $return['errcode']  =  '9';
						$return['msg']      =  '上传成功，管理员审核后对其他用户开放显示';

	                }else{
                        $this->update_once('resume_expect', array('photo' => $photo, 'defphoto' => 1), array('uid' => $uid));
	                    $return['errcode']  =  '9';
	                    $return['msg']      =  '上传成功';

	                }
	                // pc会员中心预览即上传，处理预览图
	                if ($data['preview']){

	                    $return['picurl']  =  checkpic($photo);
	                }
	            }else{
	                $return['msg']      =  '个人头像(ID:'.$uid.')修改成功';
	                $return['errcode']  =  '9';

	            }
	        }else{

	            $return['msg']      =  '个人头像(ID:'.$uid.')修改失败';
	            $return['errcode']  =  '8';

	        }
	    }else{

	        $return['msg']      =  '请选择需要修改的用户';
	        $return['errcode']  =  '8';

	    }
	    return $return;
	}
	/**
	 * 修改个人二维码
	 * @param array $whereData
	 * @param array $data   photo/需上传的图片文件; base/需上传的base4图片;  preview/pc预览即上传
	 */
	public function upEwm($whereData = array(),$data=array('photo'=>null,'base'=>null,'preview'=>null))
	{
	    if (!empty($whereData['uid'])){

	        $uid  =  $whereData['uid'];

	        if ($data['photo'] || $data['base']){

	            $upArr   =  array(
	                'file'     =>  $data['photo'],
	                'dir'      =>  'user',
	                'type'     =>  'ewm',
	                'base'     =>  $data['base'],
	                'preview'  =>  $data['preview']
	            );
	            $result  =  $this -> upload($upArr);

	            if (!empty($result['msg'])){

	                $return['msg']      =  $result['msg'];
	                $return['errcode']  =  '8';

	                return $return;

	            }elseif (!empty($result['picurl'])){

	                $photo  =  $result['picurl'];
	            }
	        }


	        if (!empty($photo)){
                $resumeData = array('wxewm'=>$photo);

	            $return['id']  =  $this -> update_once('resume',$resumeData,array('uid'=>$uid));

                if(isset($resumeData['r_status']) && $resumeData['r_status'] === 0){
                    $this->setExpectState(array('r_status'=>0,'state'=>0),array('uid'=>$uid));
                }
	        }

	        if (isset($return['id'])) {

                $this -> addMemberLog($uid, 1, '图片信息：上传二维码', 16, 1);

                // 处理预览图
                if ($data['preview']){

                    $return['picurl']  =  checkpic($photo);
                }

	            $return['msg']      =  '个人二维码修改成功';
                $return['errcode']  =  '9';
	        }else{

	            $return['msg']      =  '个人二维码修改失败';
	            $return['errcode']  =  '8';
	        }
	    }else{

	        $return['msg']      =  '请选择需要修改的用户';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 删除个人头像
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 */
	public function delPhoto($uid){

	    $return['layertype']  =	 0;

	    if (!empty($uid)){

	        if(is_array($uid)){

	            $uid                  =  pylode(',', $uid);
	            $return['layertype']  =  1;
	        }
	        $rdata  =  array(
	            'photo'             =>  '',
	            'photo_status'      =>  '',
	            'photo_statusbody'  =>  ''
	        );

	        $return['id']  =  $this -> update_once('resume', $rdata, array('uid'=>array('in',$uid)));

	        if ($return['id']){
	            //删除相关头像
	            $this -> update_once('resume_expect',array('photo'=>'','phototype'=>''),array('uid'=>array('in',$uid)));

	            $this -> update_once('answer',array('pic'=>''),array('uid'=>array('in',$uid)));

	            $this -> update_once('question',array('pic'=>''),array('uid'=>array('in',$uid)));

	            $return['msg']      =  '个人头像(ID:'.$uid.')删除成功';
	            $return['errcode']  =  '9';
	        }else{

	            $return['msg']      =  '个人头像(ID:'.$uid.')删除失败';
	            $return['errcode']  =  '8';
	        }
	    }else{

	        $return['msg']      =  '请选择要删除的个人头像';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}

  	/**
	 * 个人作品列表（d单条）
	 * @param string $whereData
	 * @param array $data
	 */
	public function getResumeShowInfo($whereData = array(), $data =array()){

		$data['field']  =  empty($data['field']) ? '*' : $data['field'];

		$resumeshow	    =  $this -> select_once('resume_show', $whereData, $data['field']);

		if (!empty($resumeshow)){

		    if ($resumeshow['picurl']){

		        $resumeshow['picurl_n']  =  checkpic($resumeshow['picurl']);
		    }
		}

		return $resumeshow;

	}

    /**
     * 个人作品列表 查询数量
     * @param $whereData
     * @return int
     */
    public function getResumeShowNum($whereData = array())
    {
        return intval($this->select_num('resume_show', $whereData));
    }

    /**
     * 后台个人作品列表
     * @param array $whereData
     * @param array $data
     * @return array|bool|false|string|void
     */
    public function getResumeShowList($whereData = array(), $data = array('field' => null, 'utype' => null))
    {

        $field  =   $data['field'] ? $data['field'] : '*';

        $List   =   $this->select_all('resume_show', $whereData, $field);

        if ($data['utype'] == 'admin') {
            $uids       =   array();
            $resumeIds  =   array();
            foreach ($List as $v) {
                $uids[] = $v['uid'];
                $resumeIds[]    =   $v['eid'];
            }
            $resume     =   $this->getSimpleList(array('id' => array('in', pylode(',', $resumeIds))), array('field' => '`id`,`name`,`uname`'));
            $arr = $this->select_all('member', array('uid' => array('in', pylode(',', $uids))), '`uid`,`username`');
            foreach ($arr as $value) {
                $member[$value['uid']] = $value;
            }
        }

        foreach ($List as $k => $v) {
            if (!empty($resume)) {
                foreach ($resume as $val) {
                    if ($v['eid'] == $val['id']) {

                        $List[$k]['name']   =   $val['uname'];
                        $List[$k]['expect'] =   $val['name'];
                    }
                }
            }
            if (!empty($member)) {
                $List[$k]['username'] = $member[$v['uid']]['username'];
            }

            $List[$k]['title']      =   mb_substr($v['title'], 0, 15);

            if (strpos($v['picurl'], 'http') === false) {

                $List[$k]['picurl'] =   checkpic($v['picurl']);
            } else {

                $List[$k]['picurl'] =   '';
            }
        }
        return $List;
    }

    /**
     * 添加简历作品 resume_show
     * @param array $data
     * @return bool
     */
	public function addResumeShow($data = array())
	{
		$post	=	$data['post'];
	    if(isset($data['file']) && $data['file']['tmp_name']!=''){
	        // pc端上传
	        $upArr    =  array(
	            'file'  =>  $data['file'],
	            'dir'   =>  'show'
	        );

	        require_once ('upload.model.php');
	        $uploadM  =  new upload_model($this->db, $this->def);
	        $pic      =  $uploadM->newUpload($upArr);

	        if (!empty($pic['msg'])){

	            $return['msg']	    =  $pic['msg'];
	            $return['errcode']	=  8;
	            return $return;

	        }elseif (!empty($pic['picurl'])){

	            $post['picurl']  =  $pic['picurl'];
	        }
	    }


	    if($data['id']){
	        $updateWhere = array('id'=>$data['id']);
	        if (isset($data['uid']) && $data['uid']) {
                $updateWhere['uid'] = $data['uid'];
            }
            $nid  =  $this -> update_once('resume_show',$post,$updateWhere);
			if ($nid) {
				$return['msg']      =  '个人作品(ID:'.$data['id'].')修改成功';
				$return['errcode']  =  '9';
			}else{
				$return['msg']      =  '个人作品(ID:'.$data['id'].')修改失败';
				$return['errcode']  =  '8';
			}
			return $return;
		}else{
			$member			=	$this->select_once('member',array('uid'=>$data['uid']),'`did`');
			$post['did']	=	$member['did'];

			$post['uid']	=	$data['uid'];
			$post['ctime']	=	time();
			$nid   =	 $this->insert_into('resume_show',$post);
			if ($nid){
                $this -> addMemberLog($data['uid'], $data['usertype'], '图片信息：添加作品展示',16,1);
			}
			return $nid;
		}
	}
	/**
	 * 删除个人作品
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 */
	public function delShow($id,$data=array()){

	    $return['layertype']  =	 0;
	    $limit                =  'limit 1';

	    if (!empty($id)){
	        if(is_array($id)){

				$ids    				=	$id;

				$return['layertype']	=	1;
				$limit                	=  '';
			}else{

				$ids   	 				=   @explode(',', $id);
				$return['layertype']	=	0;
			}
			$id             			=   pylode(',', $ids);

	        if($data['utype'] == 'admin'){
				$delWhere = array('id'=>array('in',$id));
			}else{

				$delWhere = array('uid'=>$data['uid'],'id'=>array('in',$id));
			}

	        $return['id']  =  $this -> delete_all('resume_show',$delWhere,$limit);

	        if ($return['id']){
	            $this -> addMemberLog($data['uid'],$data['usertype'],'图片信息：删除作品案例',16,3);
	            $return['msg']      =  '个人作品(ID:'.$id.')删除成功';
	            $return['errcode']  =  '9';
	        }else{

	            $return['msg']      =  '个人作品(ID:'.$id.')删除失败';
	            $return['errcode']  =  '8';
	        }
	    }else{

	        $return['msg']      =  '请选择要删除的个人作品';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	/**
	 * 处理单个图片上传
	 * @param file/需上传文件; dir/上传目录; type/上传图片类型; base/需上传base64; preview/pc预览即上传
	 */
	private function upload($data = array('file'=>null,'dir'=>null,'type'=>null,'base'=>null,'preview'=>null)){

	    include_once('upload.model.php');

	    $UploadM  =  new upload_model($this->db, $this->def);

	    $upArr  =  array(
	        'file'     =>  $data['file'],
	        'dir'      =>  $data['dir'],
	        'type'     =>  $data['type'],
	        'base'     =>  $data['base'],
	        'preview'  =>  $data['preview']
	    );

	    if (isset($data['watermark'])){

	        $upArr['watermark'] =   $data['watermark'];
        }else{

            $upArr['watermark'] =   $this->config['is_watermark'];
        }
	    $return  =  $UploadM -> newUpload($upArr);

	    return $return;
	}
	/**
	 * 处理搜索用简历多城市、多职位记录
	 */
	private function city_job($eid, $uid, $lastupdate, $nowCity, $nowJob, $oldCity = '', $oldJob = '')
	{

        $resumeCityReturn = $this->addCityclass($eid, $uid, $nowCity, $oldCity);
        extract($resumeCityReturn);
        $resumeJobReturn = $this->addJobclass($eid, $uid, $nowJob, $oldJob);
        extract($resumeJobReturn);

        if (!empty($citynochange) || !empty($jobnochange)){
	        if (!empty($oldCity) || !empty($oldJob)){
	            // 经处理的城市、职位不为空，并且已存在的城市或职位也不为空，说明是修改简历，需要将已存在的记录删除
	            $this -> delete_all('resume_city_job_class',array('eid'=>$eid,'uid'=>$uid),'');
	        }
	        $ac1 = $ac2 = $ac3 = $aj1 = $aj2 = $aj3 = $rcj = array();

	        foreach ($resume_city as $k=>$v){
	            foreach ($resume_job as $key=>$val){
	                // 分别统计每个字段出现次数，防止重复
	                $c1key = $v['provinceid'];
	                $c2key = $v['cityid'];
	                $c3key = $v['three_cityid'];
	                $j1key = $val['job1'];
	                $j2key = $val['job1_son'];
	                $j3key = $val['job_post'];

	                $ac1[$c1key] = isset($ac1[$c1key]) ? $ac1[$c1key] + 1 : 1;
	                $ac2[$c2key] = isset($ac2[$c2key]) ? $ac2[$c2key] + 1 : 1;
	                $ac3[$c3key] = isset($ac3[$c3key]) ? $ac3[$c3key] + 1 : 1;
	                $aj1[$j1key] = isset($aj1[$j1key]) ? $aj1[$j1key] + 1 : 1;
	                $aj2[$j2key] = isset($aj2[$j2key]) ? $aj2[$j2key] + 1 : 1;
	                $aj3[$j3key] = isset($aj3[$j3key]) ? $aj3[$j3key] + 1 : 1;

	                $rcj[] = array(
	                    'provinceid'       => $c1key,
	                    'provinceid_num'   => $ac1[$c1key],
	                    'cityid'           => $c2key,
	                    'cityid_num'       => $ac2[$c2key],
	                    'three_cityid'     => $c3key,
	                    'three_cityid_num' => $ac3[$c3key],
	                    'job1'             => $j1key,
	                    'job1_num'         => $aj1[$j1key],
	                    'job1_son'         => $j2key,
	                    'job1_son_num'     => $aj2[$j2key],
	                    'job_post'         => !empty($j3key) ? $j3key : 0,
	                    'job_post_num'     => $aj3[$j3key],
	                );
	            }
	        }
	        // 处理合并后数量
	        if (!empty($rcj)){
	            // 统计字段组合出现次数
	            $c1j1 = $c1j2 = $c1j3 = $c2j1 = $c2j2 = $c2j3 = $c3j1 = $c3j2 = $c3j3 = array();

	            foreach ($rcj as $k=>$v){
	                $c1j1[$v['provinceid'].'_'.$v['job1']][]       = $v['provinceid'].'_'.$v['job1'];
	                $c1j2[$v['provinceid'].'_'.$v['job1_son']][]   = $v['provinceid'].'_'.$v['job1_son'];
	                $c1j3[$v['provinceid'].'_'.$v['job_post']][]   = $v['provinceid'].'_'.$v['job_post'];
	                $c2j1[$v['cityid'].'_'.$v['job1']][]           = $v['cityid'].'_'.$v['job1'];
	                $c2j2[$v['cityid'].'_'.$v['job1_son']][]       = $v['cityid'].'_'.$v['job1_son'];
	                $c2j3[$v['cityid'].'_'.$v['job_post']][]       = $v['cityid'].'_'.$v['job_post'];
	                $c3j1[$v['three_cityid'].'_'.$v['job1']][]     = $v['three_cityid'].'_'.$v['job1'];
	                $c3j2[$v['three_cityid'].'_'.$v['job1_son']][] = $v['three_cityid'].'_'.$v['job1_son'];
	                $c3j3[$v['three_cityid'].'_'.$v['job_post']][] = $v['three_cityid'].'_'.$v['job_post'];


	                $rcj[$k]['provinceid_job1_num']       = count($c1j1[$v['provinceid'].'_'.$v['job1']]);
	                $rcj[$k]['provinceid_job1_son_num']   = count($c1j2[$v['provinceid'].'_'.$v['job1_son']]);
	                $rcj[$k]['provinceid_job_post_num']   = count($c1j3[$v['provinceid'].'_'.$v['job_post']]);
	                $rcj[$k]['cityid_job1_num']           = count($c2j1[$v['cityid'].'_'.$v['job1']]);
	                $rcj[$k]['cityid_job1_son_num']       = count($c2j2[$v['cityid'].'_'.$v['job1_son']]);
	                $rcj[$k]['cityid_job_post_num']       = count($c2j3[$v['cityid'].'_'.$v['job_post']]);
	                $rcj[$k]['three_cityid_job1_num']     = count($c3j1[$v['three_cityid'].'_'.$v['job1']]);
	                $rcj[$k]['three_cityid_job1_son_num'] = count($c3j2[$v['three_cityid'].'_'.$v['job1_son']]);
	                $rcj[$k]['three_cityid_job_post_num'] = count($c3j3[$v['three_cityid'].'_'.$v['job_post']]);
	                $rcj[$k]['eid']                       = $eid;
	                $rcj[$k]['uid']                       = $uid;
	                $rcj[$k]['lastupdate']                = $lastupdate;
	            }
	            $this -> DB_insert_multi('resume_city_job_class',$rcj);
	        }
	    }
	}
	/**
	 * 获取职位、城市字段出现字数
	 */
	private function getNum($id = '', $idarr = array(), $numarr = array())
	{
	    $num  = in_array($id, $idarr) && isset($numarr[$id]) ? $numarr[$id] + 1 : 1;

	    return $num;
	}
	/**
     * 处理简历多城市的添加、修改
     */
    private function addCityclass($eid, $uid, $nowCity, $oldCity = '')
    {
        $resume_city = $city_sort = array();
        $citynochange  =  0;
        if (!empty($eid) && !empty($uid) && !empty($nowCity)){
            $cityArr       =  @explode(',', $nowCity);
            //修改之前先判断是否需要修改，需要修改，先删除之前保存的
            if ($oldCity != ''){
                $oldArr = @explode(',', $oldCity);
                if(array_diff($oldArr,$cityArr) || array_diff($cityArr,$oldArr)){
                    $citynochange  =  1;
                    $this -> delete_all('resume_cityclass',array('eid'=>$eid),'');
                }
            }else{
                $citynochange  =  1;
            }

            include(PLUS_PATH.'cityparent.cache.php');
            foreach ($cityArr as $v){
                //获取当前城市级别，根据级别来处理，获得其他级别的数据
                $lev = getLev($v,$city_parent);
                if ($lev == 1){
                    $provinceid   =  $v;
                    $cityid       =  0;
                    $threecityid  =  0;
                }elseif ($lev == 2){
                    $cityid       =  $v;
                    $provinceid   =  $city_parent[$v];
                    $threecityid  =  0;
                }elseif ($lev == 3){
                    $threecityid  =  $v;
                    $cityid       =  $city_parent[$threecityid];
                    $provinceid   =  $city_parent[$cityid];
                }
                $resume_city[]=array(
                    'provinceid'    =>  $provinceid,
                    'cityid'        =>  $cityid,
                    'three_cityid'  =>  $threecityid
                );
                $city_sort[] = $provinceid;
            }
            if (!empty($resume_city)){
                $list  =  array();
                foreach ($resume_city as $k=>$v){
                    $list[$k]['eid']           =  $eid;
                    $list[$k]['uid']           =  $uid;
                    $list[$k]['provinceid']    =  $v['provinceid'];
                    $list[$k]['cityid']        =  $v['cityid'];
                    $list[$k]['three_cityid']  =  $v['three_cityid'];
                }
                if($citynochange == 1){
                    $this -> DB_insert_multi('resume_cityclass',$list);
                }
                array_multisort($city_sort, SORT_ASC, SORT_NUMERIC, $resume_city);
            }
        }
        return compact('resume_city', 'citynochange');
    }
    /**
     * 处理简历多职位的添加、修改
     */
    private function addJobclass($eid, $uid, $nowJob, $oldJob = ''){
        $resume_job = $job_sort = array();
        $jobnochange  =  0;
        if (!empty($eid) && !empty($uid) && !empty($nowJob)){
            $jobArr       =  @explode(',', $nowJob);
            //修改之前先判断是否需要修改，需要修改，先删除之前保存的
            if ($oldJob != ''){
                $oldArr  =  @explode(',', $oldJob);
                if(array_diff($oldArr,$jobArr) || array_diff($jobArr,$oldArr)){
                    $jobnochange  =  1;
                    $this -> delete_all('resume_jobclass',array('eid'=>$eid),'');
                }
            }else{
                $jobnochange=1;
            }

            include(PLUS_PATH.'jobparent.cache.php');
            foreach ($jobArr as $v){
                //获取当前职位级别，根据级别来处理，获得其他级别的数据
                $lev  =  getLev($v,$job_parent);
                if ($lev == 1){
                    $job      =  $v;
                    $jobson   =  0;
                    $jobpost  =  0;
                }elseif ($lev == 2){
                    $jobson   =  $v;
                    $job      =  $job_parent[$v];
                    $jobpost  =  0;
                }elseif ($lev == 3){
                    $jobpost  =  $v;
                    $jobson   =  $job_parent[$jobpost];
                    $job      =  $job_parent[$jobson];
                }
                $resume_job[]=array(
                    'job1'      =>  $job,
                    'job1_son'  =>  $jobson,
                    'job_post'  =>  $jobpost
                );
                $job_sort[] = $job;
            }
            if (!empty($resume_job)){
                $list = array();
                foreach ($resume_job as $k=>$v){
                    $list[$k]['eid']       =  $eid;
                    $list[$k]['uid']       =  $uid;
                    $list[$k]['job1']      =  $v['job1'];
                    $list[$k]['job1_son']  =  $v['job1_son'];
                    $list[$k]['job_post']  =  $v['job_post'];
                }
                if($jobnochange==1){
                    $this -> DB_insert_multi('resume_jobclass',$list);
                }
                array_multisort($job_sort, SORT_ASC, SORT_NUMERIC, $resume_job);
            }
        }
        return compact('resume_job', 'jobnochange');
    }
    public function getCityclassList($whereData = array(),$data=array())
    {
        $field = !empty($data['field']) ? $data['field'] : '*';
        $rows  =  $this -> select_all('resume_cityclass',$whereData,$field);

        return $rows;
    }

    public function getCityJobClass($whereData = array(), $field = '*'){
        $rows  =  $this -> select_all('resume_city_job_class', $whereData, $field);

        return $rows;
    }

    public function getJobclassList($whereData = array())
    {

        $rows  =  $this -> select_all('resume_jobclass',$whereData);

        return $rows;
    }
	private function getClass($options)
	{
	    if (!empty($options)){

	        include_once('cache.model.php');

	        $cacheM            =   new cache_model($this->db,$this->def);

	        $cache             =   $cacheM -> GetCache($options);

	        return $cache;
	    }
	}

	private function getDataUserList($List){
	    if(!empty($List)){
            include(CONFIG_PATH.'db.data.php');
        
	        $uids   =   $cids   =   array();

            foreach ($List as $val) {

                $uids[] =   $val['uid'];
               
            }

            $member     =   $this->select_all('member', array('uid' => array('in', pylode(',', $uids))), '`uid`,`username`,`usertype`,`reg_date`,`lock_info`,`login_date`,`source`,`status`,`wxopenid`,`unionid`,`wxid`,`moblie`,`login_ip`,`login_address`,`moblie_address`,`login_hits`');
            $sqList     =   $this->select_all('userid_job', array('uid' => array('in', pylode(',', $uids),'isdel'=>9)), '`uid`');
	        foreach($List as $k=>$v){

                $List[$k]['label_n']	=   str_replace(array("\r\n","\r","\n"), array("","",""), strip_tags($v['label'],'\r\n'));

	            foreach($member as $val){

	                if($v['uid']  ==  $val['uid']){
	                    $List[$k]  =  array_merge($List[$k], $val);
                        $List[$k]['source_n']    = $arr_data['source'][$val['source']];
	                    $List[$k]['unionid']     = $val['unionid'];
                        $List[$k]['wxopenid']    = $val['wxopenid'];
                        $List[$k]['wxid']        = $val['wxid'];
                        $List[$k]['moblie']      = $val['moblie'];
						$List[$k]['login_ip']      = $val['login_ip'];
                        $List[$k]['login_address']      = $val['login_address'];
                        $List[$k]['moblie_address']      = $val['moblie_address'];
                        $List[$k]['lock_info']      = $val['lock_info'];
                        $List[$k]['reg_date_n']   = date('Y-m-d H:i',$val['reg_date']);
                        if ($val['login_date']) {
                            $List[$k]['login_date_n'] = date('Y-m-d H:i', $val['login_date']);
                        }
                        $List[$k]['login_hits'] = $val['login_hits'];
	                }
	            }

	            

                foreach ($sqList as $sk => $sv){
                    if ($v['uid'] == $sv['uid']){
                        $List[$k]['sq_num']++;
                    }
                }
	        }
	    }

	    return $List;
	}
	//后台列表处理简历多城市/用户名、审核状态
	private function getDataList($List,$cache)
	{
        foreach ($List as $v) {

            $uids[]     =   $v['uid'];
            $eids[]     =   $v['id'];
        }
	    $member         =   $this->select_all('member',array('uid'=>array('in',pylode(',', $uids))),'uid,username,moblie,moblie_address,lock_info');
	    $resume_city    =   $this->select_all('resume_cityclass',array('eid'=>array('in',pylode(',', $eids))),'eid,provinceid,cityid,three_cityid');
	    $sqList         =   $this->select_all('userid_job', array('eid' => array('in', pylode(',', $eids))), '`uid`,`eid`');


	    foreach($List as $k=>$v){

	        foreach($member as $val){

	            if($val['uid']==$v['uid']){

	                $List[$k]['username']  =  $val['username'];
	                $List[$k]['moblie']    =  $val['moblie'];
	                $List[$k]['moblie_address']    =  $val['moblie_address'];
                    $List[$k]['lock_info']    =  $val['lock_info'];
	            }
	        }

	        if ($resume_city){

	            $city_classid  =   explode(',',$v['city_classid']);

	            foreach($resume_city as $val){

	                if($v['id'] == $val['eid']){

	                    if($val['provinceid'] == $city_classid[0]){

	                        $List[$k]['city_n']  =  $cache['city_name'][$city_classid[0]];

	                    }else if($val['cityid']==$city_classid[0]){

	                        $List[$k]['city_n']  =  $cache['city_name'][$val['provinceid']].'-'.$cache['city_name'][$city_classid[0]];

	                    }else if($val['three_cityid']==$city_classid[0]){

	                        $List[$k]['city_n']  =  $cache['city_name'][$val['provinceid']].'-'.$cache['city_name'][$val['cityid']].'-'.$cache['city_name'][$city_classid[0]];
	                    }
	                }
	            }
	            $cityall = array();

	            if(is_array($city_classid)){

	                $i	=	0;

	                foreach($city_classid as $cv){

	                    if($cache['city_name'][$cv]){

	                        $i++;

	                        $cityall[]	=	$cache['city_name'][$cv];
	                    }
	                }
	                $List[$k]['citynum']	=	$i;

	            }
	            $List[$k]['cityall']	=	implode('、',$cityall);
	        }
            foreach ($sqList as $sk => $sv){
	            if ($v['id'] == $sv['eid'] && $v['uid'] == $sv['uid']){
	                $List[$k]['sq_num']++;
                }
            }
	    }
	    return $List;
	}

	//计算工作经历工作时长
	private function workTime($eid)
	{
	    $workList  =   $this -> getResumeWorks(array('eid'=>$eid),'sdate,edate');
	    $whour     =   0;
	    $hour      =   array();
	    $time      =   time();

	    foreach($workList as $v){
	        //计算每份工作时长(按月)
	        if ($v['edate']){

	            $workTime  =   ceil(($v['edate']-$v['sdate'])/(30*86400));

	        }else{

	            $workTime  =   ceil(($time - $v['sdate'])/(30*86400));
	        }

	        $hour[]    =   $workTime;
	        $whour    +=   $workTime;
	    }
	    //更新当前简历时长字段
	    $avghour   =   ceil($whour/count($hour));

	    $post  =   array(
	        'whour'    =>  $whour,
	        'avghour'  =>  $avghour
	    );

	    $this -> update_once('resume_expect', $post, array('id'=>$eid));
	}

	/**
	 * @desc 简历推送列表
	 *
	 * @param  array $whereData
	 * @param  array $data
	 */
	public function getResTsList($whereData,$data=array()) {

	    $select = $data['field'] ? $data['field'] : '*';

	    $List  =   $this   ->  select_all('user_entrust_record',$whereData,$select);

	    return $List;

	}

	/**
	 * @desc 收藏人才列表查询
	 */
	function getTalentList($whereData = array(), $data = array()) {

	    $field  =  $data['field'] ? $data['field'] : '*';

	    $List   =  $this -> select_all('talent_pool',$whereData,$field);

	    if (!empty($List)) {

 	        $List  =   $this -> subTalentInfo($List,$data);
	    }

	    return $List;

	}
	private function subTalentInfo($List,$data) {
	    
	    $utype = isset($data['utype']) ? $data['utype'] : '';
	    
	    foreach ($List as $v) {
	        
	        $uids[]    =   $v['uid'];
	        $eids[]    =   $v['eid'];
	        $cuids[]   =   $v['cuid'];
	        
	    }
	    
	    //  查询个人简历名称
	    $reWhere['id']     =   array('in', pylode(',', $eids));
	    $reData['field']   =   '`id`,`name`,`job_classid`,`minsalary`,`maxsalary`,`height_status`,`exp`,`edu`,`birthday`,`sex`,`uname`,`photo`,`status`';
	    
	    $expect            =   $this -> getList($reWhere, $reData);
	    
	    if ($utype == 'admin'){
	        /* 提取企业信息 */
	        $cWhere['uid']     =   array('in', pylode(',', $cuids));
	        $comList           =   $this -> select_all('company', $cWhere,'`uid`,`name`');
	    }
	    if ($utype == 'pc'){
	        
	        $userid_msg		   =   $this -> select_all('userid_msg',array('fid'=>$data['uid'],'uid'=>array('in',pylode(',',$uids)),'isdel'=>9),'`uid`');
	        
	        $userid_job		   =   $this -> select_all('userid_job',array('com_id'=>$data['uid'],'uid'=>array('in',pylode(',',$uids)),'isdel'=>9),'`uid`,`is_browse`');
	    }
	    // PC和小程序需要查询简历备注
	    if ($utype == 'pc' || $utype == 'wxapp'){
            $options           =   array('com');
            $cache			   =   $this -> getClass($options);
	        $remark            =   $this -> select_all("resume_remark", array('eid' => array('in', pylode(',', $eids)),'comid'=>$data['uid']), '`eid`,`remark`,`comid`,`status`');
	    }
	    $downArr = array();
	    if(isset($data['isdown']) && $data['isdown'] == 1){
	        $downList      =   $this -> select_all('down_resume', array('comid' => $data['uid'], 'uid'=>array('in',pylode(',', $uids))), '`uid`,`eid`');
	        foreach ($downList as $val){
	            $downArr[] = $v['uid'];
	        }
	    }
	    
	    foreach ($List as $k => $v){
	        
	        $List[$k]['wapurl']             =   Url('wap', array('c'=>'resume','a'=>'show','id'=>$v['eid']));
	        $List[$k]['remark']				=	stripslashes($v['remark']);
	        $List[$k]['ctime_n']   			=	timeForYear($v['ctime']);
            $List[$k]['ctime_n_n']   		=	date('Y-m-d H:i', $v['ctime']);
	        $List[$k]['datetime_n']         =   $List[$k]['ctime_n'];
	        foreach ($expect['list'] as $rv){
	            
	            if ($v['eid']  ==  $rv['id']) {
	                $List[$k]['rname']		=   $rv['name'];
	                $List[$k]['exp']		=   $rv['exp_n'];
	                $List[$k]['edu']		=   $rv['edu_n'];
	                $List[$k]['age']		=   $rv['age_n'];
	                $List[$k]['salary']		=   $rv['salary'];
	                $List[$k]['photo']		=   $rv['photo_n'];
	                $List[$k]['resume_status'] = $rv['status'];
	                if ($rv['job_classid'] != '') {
	                    $List[$k]['jobname'] = $rv['job_classname'];
	                }
	                if ($utype == 'admin'){
	                    // 后台直接展示姓名
	                    $List[$k]['username']  =  $rv['uname'];
	                }else{
	                    if (in_array($v['uid'], $downArr)){
	                        $List[$k]['username']  =  $rv['uname'];
	                    }else{
	                        $nameArr               =  array('nametype'=>$this->config['user_name'], 'name' => $rv['uname'], 'eid' => $rv['id'], 'sex' => $rv['sex']);
	                        $List[$k]['username']  =  $this -> setUsernameShow($nameArr);
	                    }
	                }
	            }
	        }
	        if (!empty($comList)){
	            foreach ($comList as $cv){
	                
	                if ($v['cuid'] == $cv['uid']) {
	                    $List[$k]['com_name'] = $cv['name'];
                        $List[$k]['com_url'] = Url('company', array('c' => 'show', 'id' => $v['cuid'], 'look' => 'admin'));
	                }
	            }
	        }
	        if (!empty($userid_msg)){
	            foreach($userid_msg as $val){
	                
	                if($v['uid']==$val['uid']){
	                    $List[$k]['userid_msg']		=	1;
	                }
	            }
	        }
	        if (!empty($userid_msg)){
	            foreach($userid_job as $val){
	                
	                if($v['uid']==$val['uid']){
	                    $List[$k]['is_browse']		=	$val['is_browse'];
	                }
	            }
	        }
	        if (!empty($downList)){
	            foreach($downList as $dv){
	                
	                if($v['uid']==$dv['uid'] && $v['eid'] == $dv['eid']){
	                    $List[$k]['down']		=	1;
	                }
	            }
	        }
	        if (!empty($remark)){
	            foreach($remark as $rv){
	                if($v['eid'] == $rv['eid'] && $v['cuid'] == $rv['comid']){
	                    $List[$k]['remark']		=	$rv['remark'];
                        if(isset($rv['status'])){
                            $List[$k]['status_n']        =    $cache['comclass_name'][$rv['status']];
                        }
	                }
	            }
	        }
	    }
	    
	    return $List;
	    
	}

	/**
	 * @desc 删除收藏人才
	 */
	function delTalentPool($id = null , $data = array()){

	    if(!empty($id)){

	        if(is_array($id)){

	            $ids    =	$id;

	            $return['layertype']	=	1;

	        }else{

	            $ids    =   @explode(',', $id);
				$return['layertype']	=	0;

	        }

	        $id		=	pylode(',', $ids);
	        if($data['utype'] !='admin'){

				$delWhere	=	array('cuid'=>$data['uid'],'id' => array('in',$id));
			}else{
				$delWhere	=	array('id' => array('in',$id));
			}

			$return['id']	=	$this -> delete_all('talent_pool',$delWhere,'');

			$this -> addMemberLog($data['uid'],$data['usertype'],'收藏管理：删除收藏简历人才',5,3);

            $return['msg']		=	'简历被收藏记录(ID:'.$id.')';

	        $return['errcode']	=	$return['id'] ? '9' :'8';
	        $return['msg']		=	$return['id'] ? $return['msg'].'删除成功！' : $return['msg'].'删除失败！';
		}elseif($data['where']){

			$where		=	$data['where'];

			if($data['norecycle'] == '1'){	//	数据库清理，不插入回收站

				$nid	=	$this -> delete_all('talent_pool', $where, $data['limit'], '', '1');

			}else{

				$nid	=	$this -> delete_all('talent_pool', $where, '');

			}
			return	$nid;
	    }else{

	        $return['msg']		=	'请选择您要删除的数据！';
	        $return['errcode']	=	8;
	    }

	    return	$return;
	}
	/**
	 * @desc 收藏人才列表数量
	 */
	function getTalentNum($whereData = array()) {

	    return $this -> select_num('talent_pool',$whereData);

	}
	/**
	 * 收藏简历
	 */
	public  function addTalent($data = array()){

	    $return    =   array();

		if($data['cuid']==''){

			$return['msg']      =  '您还未登录企业账号，是否登录？';
			$return['errcode']  =  8;
			$return['state']  	=  3;
			return $return;
		}

		if($data['usertype'] != "2"){

		    $return['msg']      =  '只有企业用户，才可以操作！';
			$return['errcode']  =  8;
			$return['state']  	=  0;
			return $return;
		}

		$row  =   $this -> select_once('talent_pool',array('eid' => $data['eid'], 'cuid' => $data['cuid']));

		if(empty($row)){

			$tdata=array(
				'eid'		=>	$data['eid'],
				'cuid'		=>	$data['cuid'],
				'uid'		=>	(int)$data['uid'],
				'remark'	=>	$data['remark'],
				'ctime'		=>	time(),
			);

			$this -> insert_into('talent_pool',$tdata);

			//加入收藏信息
            $company    =   $this -> select_once('company', array('uid' => $data['cuid']), '`name`');

      		require_once ('sysmsg.model.php');
      		$sysmsgM = new sysmsg_model($this->db, $this->def);
      		$sysmsgM -> addInfo(array('uid' => $data['uid'],'usertype'=>1,  'content' => '企业<a href="comtpl,'.$data['cuid'].'"> '.$company['name'].'</a> 收藏您的简历'));

			$return['msg']      =  '收藏成功！';
			$return['errcode']  =  9;
			$return['state']  	=  1;
		}else{
			// $return['msg']      =  '该简历已加入到人才库！';
			// $return['errcode']  =  8;
			// $return['state']  	=  2;

            $this -> delete_all('talent_pool', array('id' => $row['id']));

            $this -> addMemberLog($data['cuid'],$data['usertype'],'收藏管理：取消收藏简历',5,3);

            $return['msg']      =  '取消收藏！';
            $return['errcode']  =  9;
            $return['state']  	=  1;
		}
	    return $return;
	}

	/**
	 * 设置姓名展示
	 */
	public function setUsernameShow($data = array())
	{
		$resUserName				=	'';
		if(empty($this -> config['user_name']) || $this -> config['user_name'] == 1){

			if($data['nametype'] == 1 && !empty($data['name'])){

				if(CheckMobile($data['name'])){

					$resUserName	=	mb_substr($data['name'], 0, 3)."****".	mb_substr($data['name'], 7);

				}else{

					$resUserName	=	$data['name'];
				}
			}else if($data['nametype'] == 2 && !empty($data['eid'])){

				$resUserName		=	'NO.'. $data['eid'];

			}elseif($data['nametype'] == 3 && !empty($data['name'])){

				if($data['sex'] == 1){

					$resUserName	=	mb_substr($data['name'], 0, 1, 'utf-8').'先生';
				}else{

					$resUserName	=	mb_substr($data['name'], 0, 1, 'utf-8').'女士';
				}
			}
		}elseif($this -> config['user_name'] == 2 && !empty($data['eid'])){

			$resUserName = 'NO.'. $data['eid'];
		}elseif($this -> config['user_name'] == 3 && !empty($data['name'])){

			if($data['sex'] == 1){

				$resUserName 		=	mb_substr($data['name'], 0, 1, 'utf-8').'先生';
			}else{

				$resUserName 		=	mb_substr($data['name'], 0, 1, 'utf-8').'女士';
			}
		}elseif($this -> config['user_name'] == 4 && !empty($data['name'])){

			$resUserName			=	$data['name'];
		}

		if(empty($resUserName)){

			$resUserName 			=	$data['name'];
		}
		return $resUserName;
	}

	/**
	 * 设置简历头像展示
	 */
	public function setResumePhotoShow($data = array())
	{
		$resumePhoto  =	'';
		$maleUrl  	  =  checkpic('',$this -> config['sy_member_icon']);
		$femaleUrl    =  checkpic('',$this -> config['sy_member_iconv']);
		$sexArr		  =	 array(1, 152);
        if($data['defphoto']==2){
            $resumePhoto        =   checkpic($data['photo']);
        }else{
        	if(empty($this -> config['user_pic']) || $this -> config['user_pic'] == 1){
        	    if($data['photo'] && $data['photo_status'] == 0 && $data['phototype'] != 1){
        	        $resumePhoto		=	checkpic($data['photo']);
        	    }else{
        	        if(in_array($data['sex'], $sexArr)){
        	            $resumePhoto	=	$maleUrl;
        	        }else{
        	            $resumePhoto	=	$femaleUrl;
        	        }
        	    }
        	}elseif($this -> config['user_pic'] == 2){
        	    if($data['photo'] && $data['photo_status'] == 0){
        	        $resumePhoto		=	checkpic($data['photo']);
        	    }else{
        	        if(in_array($data['sex'], $sexArr)){
        	            $resumePhoto	=	$maleUrl;
        	        }else{
        	            $resumePhoto	=	$femaleUrl;
        	        }
        	    }
        	}elseif($this -> config['user_pic'] == 3){
        	    if(in_array($data['sex'], $sexArr)){
        	        $resumePhoto		=	$maleUrl;
        	    }else{
        	        $resumePhoto		=	$femaleUrl;
        	    }
        	}
        }
		return $resumePhoto;
	}
	//更新职位点击率
	function addExpectHits($id){
		if($this -> config['sy_job_hits'] > 100 || !$this -> config['sy_job_hits']){
			$hits       =   1;
		}else{
			$hits       =   mt_rand(1, $this->config['sy_job_hits']);
		}
		$this -> update_once('resume_expect', array('hits' => array('+', $hits)), array('id' => $id));
	}

    /**
     * @desc 更新简历相关字段$table  比如：resume_edu,exp,other等
     * @param $table
     * @param $whereData
     * @param $data
     * @param array $odata
     * @return bool
     */
    function upResumeTable($table, $whereData, $data, $odata = array())
    {
        if(!in_array($table,$this->resume_tables)){
            return false;
        }
        if (!empty($whereData)) {

            $nid    =   $this->update_once($table, $data, $whereData);

        } else {
            if ($data['eid'] && $data['uid']) {

                $eptNum =   $this->select_num('resume_expect', array('id' => intval($data['eid']), 'uid' => intval($data['uid'])));

                if ($eptNum > 0) {

                    $nid=   $this->insert_into($table, $data);
                }


            }
        }
        $state      =   resumeTimeState($this->config['user_revise_state']);

        $expect     =   $this->select_once('resume_expect', array('id' => $data['eid']), 'state');
        if ($expect['state'] == 3){
            $state  =   0;
        }

        if ($table != 'resume_show') {
            if ($state == '0') {

                $this->setExpectState(array('state' => 0), array('id' => intval($data['eid'])));
            }
        }
        return $nid;
    }

	/**
	 * @desc 简历外发列表查询
	 */
	function getResumeOutList($whereData = array(), $data = array()) {

	    $field  =  $data['field'] ? $data['field'] : '*';

	    $List   =  $this -> select_all('resumeout',$whereData,$field);

	    return $List;

	}
	/**
	 * 添加简历外发
	 */
	public function addResumeOut($data,$param){

        include_once('recommend.model.php');
        $recomM    =   new recommend_model($this->db, $this->def);

        if(isset($this->config['sy_resumeout_day_num']) && $this->config['sy_resumeout_day_num'] > 0){
            $rnwhere=array('uid'=>$param['uid'],'rec_type'=>3,'addtime'=>array('>',strtotime('today')));
            $num    =   $recomM -> getRecommendNum($rnwhere);
            if($num >= $this->config['sy_resumeout_day_num']){
                return array('msg'=>'每天最多外发'.$this->config['sy_resumeout_day_num'].'次简历！','errcode'=>8);
            }
        }else{
            return array('msg'=>'外发简历功能已关闭！','errcode'=>8);
        }
        //判断上一次外发的时间间隔
        if(isset($this->config['sy_resumeout_interval']) && $this->config['sy_resumeout_interval'] > 0){
            $row                    =   $recomM -> getInfo(array('uid' =>$param['uid'],'rec_type'=>3, 'orderby' => 'addtime'));
            if(!empty($row['addtime']) && (time() - $row['addtime']) < $this->config['sy_resumeout_interval']){
                $needTime = $this->config['sy_resumeout_interval'] - (time() - $row['addtime']);
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
                return array('msg'=>'外发简历间隔不得少于'.$recs.'，请'.$needTime.'后再外发！','errcode'=>8);
            }
        }

	    if (!empty($data)){
			if($data['resume']==''){
				return array('msg'=>'请选择简历','errcode'=>8);
            }
            $email=$data['email'];
            if($email==''){
				return array('msg'=>'请输入邮箱','errcode'=>8);
            }elseif (CheckRegEmail($email)==false){
				return array('msg'=>'邮箱格式错误','errcode'=>8);
            }
            if($data['comname']==''){
				return array('msg'=>'请输入企业名称','errcode'=>8);
            }
            if ($data['jobname']==''){
				return array('msg'=>'请输入职位名称','errcode'=>8);
            }
            if($this->config['sy_email_set']!='1'){
				return array('msg'=>'网站邮件服务器不可用','errcode'=>8);
            }
            $today  = strtotime('today');
            $outnum = $this->select_num('resumeout', array('uid'=>$data['uid'],'datetime'=>array('>', $today)));

            if(!empty($this->config['sy_recommend_day_num']) && $outnum > $this->config['sy_recommend_day_num']){
                return array('msg'=>'简历每天只能推荐'.$this->config['sy_recommend_day_num'].'次','errcode'=>8);
            }elseif(!$this->config['sy_recommend_day_num'] || $this->config['sy_recommend_day_num']<=0){
                return array('msg'=>'推荐功能已关闭','errcode'=>8);
            }elseif(isset($_COOKIE['resumeout']) && $_COOKIE['resumeout'] == $data['resume']){
                return array('msg'=>'请不要频繁发送邮件！同一简历发送间隔为两分钟！','errcode'=>8);
            }

            $Info       =   $this->getInfoByEid(array('eid' => $data['resume']));
            // 简历模糊化
            $resumeCheck  =  $this->config['resume_open_check'] == 1 ? 1 : 2;
            global $phpyun;
            $phpyun -> assign('Info',$Info);
            $phpyun -> assign('resumeCheck',$resumeCheck);

            $contents	=	$phpyun -> fetch(TPL_PATH.'resume/sendresume.htm',time());
			//发送邮件并记录入库

			$emailData=array(
				'email'		=> $email,
				'subject'	=> '我看到贵公司在招收'.$data['jobname'].'，向您自荐一份简历！',
				'content'	=> $contents,
				'uid'		=> '',
				'name'		=> $data['recipient'],
				'cuid'		=> $Info['uid'],
				'cname'		=> $Info['name']
			);
			include_once('notice.model.php');
    	    $noticeM	=	new notice_model($this->db, $this->def);
			$sendid 	=	$noticeM -> sendEmail($emailData);
			if($sendid['status'] != -1){
				$arr		=	array(
					'uid'		=>	$Info['uid'],
					'comname'	=>	$data['comname'],
					'jobname'	=>	$data['jobname'],
					'recipient'	=>	$data['comname'],
					'email'		=>	$email,
					'datetime'	=>	time(),
					'resume'	=>	$data['resumename']
				);
				$this -> insert_into('resumeout',$arr);

				require_once ('cookie.model.php');
				$cookieM  =  new cookie_model($this->db, $this->def);
				$cookieM -> SetCookie('resumeout', $data['resume'], time() + 120);

                $recommend                  =   array(
                    'uid'                   =>  $Info['uid'],
                    'rec_type'              =>  3,
                    'rec_id'                =>  $data['resume'],
                    'email'                 =>  $email,
                    'addtime'               =>  time()
                );
                $result                     =   $recomM -> addRecommendInfo($recommend);

				return array('msg'=>'发送成功','errcode'=>9);
			}else{
				return array('msg'=>'邮件发送错误 原因：' . $sendid['msg'],'errcode'=>8);
			}
		}
	}
	/**
	 * 删除简历外发记录
	 * @param string $id    格式：单个，如1 ; 批量，如1,2,3
	 */
	public function delResumeOut($id,$data=array()){

	    $return['layertype']  =	 0;
	    $limit                =  'limit 1';

	    if (!empty($id)){

	        if(is_array($id)){

	            $id                   =  pylode(',', $id);
	            $return['layertype']  =  1;
	            $limit                =  '';
	        }

	        $return['id']  =  $this -> delete_all('resumeout',array('uid'=>$data['uid'],'id'=>array('in',$id)),$limit);

	        if ($return['id']){
	            $this->addMemberLog($data['uid'],$data['usertype'],'简历信息：删除简历外发记录',2,3);
	            $return['msg']      =  '删除成功';
	            $return['errcode']  =  '9';
	        }else{

	            $return['msg']      =  '删除失败';
	            $return['errcode']  =  '8';
	        }
	    }else{

	        $return['msg']      =  '请选择要删除的记录';
	        $return['errcode']  =  '8';
	    }
	    return $return;
	}
	//简历分布定价，按照时间分别定价
	function setDayprice($eid,$data=array()){

		$rinfo		=	$this -> select_once('resume_expect',array('id'=>$eid),'lastupdate');

		$conlist	=	$this -> select_once('admin_config',array('name'=>'integral_down_resume_dayprice'));

		$marr		=	explode(':',$conlist['config']);

		foreach($marr as $v){
			$narr	=	explode('_',$v);
			$days[]	=	$narr[0];
			$data[$narr[0]]	=	$narr[1];
		}

        $ryear      =   intval((time()-$rinfo['lastupdate'])/86400/365);
        $rday       =   (time() - $rinfo['lastupdate'])/(60*60*24);

		if ($ryear != 0) {

            $rday   =   $ryear * 365 + $rday;
        }

		if(in_array($rday,$days)){

			$rdayprice=$data[$rday];
		}else{

			foreach($days as $val){
				if($rday<$val){
					$rdayprice=$data[$val];
					break;
				}
			}
		}
		if(!$rdayprice){
			if($data['integral']&&$data['integral']=='yes'){

				$newdayprice	=	$this->config['integral_down_resume'] * $this->config['integral_proportion'];// 购买简历下载所需积分
			}else{

				$newdayprice	=	$this->config['integral_down_resume']; // 购买简历下载所需金额
			}
		}else{

			if($data['integral']&&$data['integral']=='yes'){

				$newdayprice		=	$rdayprice*$this->config['integral_proportion'];
			}else{

				$newdayprice		=	$rdayprice;
			}
		}
		return $newdayprice;
	}

	/**
	 * 简历开放状态检查
	 * @param array $data
	 */
	function openResumeCheck($data){

		$type		=	2;
		
			/* 个人查看自己 */
	    if(isset($data['ruid']) && $data['uid']==$data['ruid']){

			$type	=	1;

		}else{
			/* 开放状态 */
			if($this->config['resume_open_check'] == 1){
				$type	=	1;
			}
			/* 企业登录状态 */
			if($this->config['resume_open_check'] == 2){
				if($data['uid'] && $data['usertype']==2 || $data['usertype'] == 3){
					$type	=	1;
				}
			}
			/* 发布职位状态 */
			if($this->config['resume_open_check'] == 3){
				//发布职位状态是不需要职位是否上架条件 王旭 20220608
				if($data['uid'] && $data['usertype']==2){
					$where['r_status']	=	1;
					$where['state']		=	'1';
					$where['uid']		=	$data['uid'];
					$jobNum				=	$this->select_num('company_job',$where);
					if($jobNum>0){
						$type	=	1;
					}
				}

			}
			/* 下载简历状态(包括企业可以免费查看投递简历) */
			if($this->config['resume_open_check'] == 4){

				if($data['uid']){
					$where['comid']		=	$data['uid'];
					$where['eid']		=	$data['eid'];
					$where['usertype']	=	$data['usertype'];
					$downNum			=	$this->select_num('down_resume', $where);
					if ($data['usertype']==2) {
						//查询当前企业会员是否过期
						$statis  =   $this->select_once('company_statis', array('uid' => $data['uid']), '`vip_etime`');
						if(isVip($statis['vip_etime'])) {
							$freedownNum = $this->select_num('freedown_resume', $where);
						}
						if ($downNum > 0 || $freedownNum > 0) {
							$type = 1;
						}
					}else{
						if ($downNum > 0) {
							$type = 1;
						}
					}
				}
			}
		}		
		return $type;
	}
	/**
	 * 用户设置默认简历
	 */
    public function defaults($data = array()){

        $return	=	array('errcode'=>'2','msg'=>'设置失败！');

        if (!empty($data)){

            $row  =  $this -> select_once('resume_expect',array('id'=>$data['id']), '`id`,`uid`,`topdate`,`top`');

            if (!empty($row['id'])){

                $id			  =	  $this->update_once('resume', array('def_job'=>$row['id']), array('uid'=>$data['uid']));
                // 如原默认简历有置顶记录，且已过期，将置顶记录清空
                if($row['topdate'] < time()){

                    $redata   =   array(
                        'defaults'  =>  0,
                        'topdate'   =>  '',
                        'top'       =>  ''
                    );
                }else{

                    $redata   =   array(
                        'defaults'  =>  0
                    );
                }
                $this->update_once('resume_expect', $redata, array('uid' => $row['uid']));

                $this->update_once('resume_expect', array('defaults' => 1), array('id' => $row['id']));

                $this->addMemberLog($row['uid'], 1, '简历管理：设置默认简历（ID：' . $row['id'] . '）', 2, 2);

                $return = $id ? array('errcode' => '1', 'msg' => '设置成功！') : array('errcode' => '2', 'msg' => '设置失败！');

            }
        }

        return $return;
    }

	/**
	 * 简历模糊字段
	 */
	public function getTj($data){

        if (!empty($data)){
			/*教育经历*/
			$eduList				=		$data['edu'];
			if($eduList){
				$edumin				=		0;
				$edumax				=		0;
				foreach($eduList as $v){
					if($v['sdate']>0 && $edumin==0){
						$edumin		=		$v['sdate'];
					}elseif($edumin>$v['sdate']){
						$edumin		=		$v['sdate'];
					}
					if($v['edate']==0 ){
						$edumax		=		0;
					}elseif($edumax<$v['edate']){
						$edumax		=		$v['edate'];
					}

					$education[]	=		$v['education_n'];

					$edutitle[]		=		$v['title'];
				}

				$return['edumin']		=		date('Y.m',$edumin);
				$return['edumax']		=		$edumax  == 0 ?  '至今': date('Y.m',$edumax);
				$return['education']	=		@implode(',',$education);
				$return['edutit']		=		@implode(',',$edutitle);

				$return['edu_time']		=		$return['edumin'].'-'.$return['edumax'];
				$return['edu_content']	=		'已完成'.$return['education'].'段学业';
			}

			/*工作经历*/
			$workList			=	$data['work'];
			if($workList){
				$whour     		=   0;
				$hour     	 	=   array();
				$time      		=   time();
				$workmin		=	0;
				$workmax		=	0;
				$worknum		=	count($workList);

				foreach($workList as $v){
					/* 计算每份工作时长(按月) */
					if ($v['edate']){
						$workTime  		=   	ceil(($v['edate']-$v['sdate'])/(30*86400));
					}else{
						$workTime  		=   	ceil(($time - $v['sdate'])/(30*86400));
					}

					if($v['sdate']>0 && $workmin==0){
						$workmin		=		$v['sdate'];
					}elseif($workmin>$v['sdate']){
						$workmin		=		$v['sdate'];
					}

					if($v['edate']==0 ){
						$workmax		=		0;
					}elseif($workmax<$v['edate']){
						$workmax		=		$v['edate'];
					}

					$wtitle[]			=		$v['title'];

					$hour[]   			=		$workTime;
					$whour    			+=   	$workTime;
				}

				$workavg   =   ceil($whour/count($hour));

				$return['worknum']		=		$worknum  > 0 ?  $worknum:0;

				$return['workavg']		=		$workavg  > 11 ?  avgToYm($workavg) :  $workavg;

				$return['workmin']		=		date('Y.m',$workmin);
				$return['workmax']		=		$workmax  == 0 ?  '至今': date('Y.m',$workmax);
				$return['worktit']		=		@implode(',',$wtitle);

				$return['work_time']	=		$return['workmin'].'-'.$return['workmax'];
				if($return['workavg']>0){
					if($return['worktit']!=''){
						$return['work_content']	=	'参加过'.$return['worknum'].'份工作 , 平均工作时长'.$return['workavg'].'个月，涉及'.$return['worktit'].'等岗位';
					}else{
						$return['work_content']	=	'参加过'.$return['worknum'].'份工作 , 平均工作时长'.$return['workavg'].'个月';
					}
				}
			}

			/*项目经历*/
			$xmList				=	$data['xm'];
			if($xmList){
				$xmmin			=	0;
				$xmmax			=	0;
				$xmnum			=	count($xmList);
				foreach($xmList as $v){
					if($v['sdate']>0 && $xmmin==0){
						$xmmin			=		$v['sdate'];
					}elseif($xmmin>$v['sdate']){
						$xmmin			=		$v['sdate'];
					}

					if($v['edate']==0 ){
						$xmmax			=		0;
					}elseif($xmmax<$v['edate']){
						$xmmax			=		$v['edate'];
					}

					$xmtitle[]			=		$v['title'];
				}

				$return['xmnum']		=		$xmnum  > 0 ?  $xmnum:0;
				$return['xmmin']		=		date('Y.m',$xmmin);
				$return['xmmax']		=		$xmmax  == 0 ?  '至今':date('Y.m',$xmmax);
				$return['xmtit']		=		@implode(',',$xmtitle);

				$return['xm_time']		=		$return['xmmin'].'-'.$return['xmmax'];
				if($return['xmtit']!=''){
					$return['xm_content']	=	'独自完成或参与过'.$return['xmnum'].'个项目，并在其中担任过'.$return['xmtit'].'等职务';
				}else{
					$return['xm_content']	=	'独自完成或参与过'.$return['xmnum'].'个项目';
				}
			}

			/*培训经历*/
			$trainList					=		$data['train'];
			if($trainList){
				$trainmin				=		0;
				$trainmax				=		0;
				$trainnum				=		count($trainList);
				foreach($trainList as $v){
					if($v['sdate']>0 && $trainmin==0){
						$trainmin		=		$v['sdate'];
					}elseif($xmmin>$v['sdate']){
						$trainmin		=		$v['sdate'];
					}

					if($v['edate']==0 ){
						$trainmax		=		0;
					}elseif($xmmax<$v['edate']){
						$trainmax		=		$v['edate'];
					}
				}

				$return['trainnum']		=		$trainnum  > 0 ?  $trainnum:0;
				$return['trainmin']		=		date('Y.m',$trainmin);
				$return['trainmax']		=		$trainmax  == 0 ?  '至今':date('Y.m',$trainmax);

				$return['train_time']	=		$return['trainmin'].'-'.$return['trainmax'];
				$return['train_content']=		'参加过'.$return['trainnum'].'次培训，进行自我充电，能力提升';
			}

			/*职业技能*/
			$skillList					=   	$data['skill'];
			if($skillList){
				$skillpic				=		0;
				$skillnum				=		count($skillList);
				foreach($skillList as $v){
					if($v['pic']){
						$skillpic     +=		1;
					}
				}
				$return['skillnum']		=		$skillnum  > 0 ?  $skillnum:0;
				$return['skillpic']		=		$skillpic  > 0 ?  $skillpic:0;

				$return['skill_content']=		'目前已掌握'.$return['skillnum'].'项技能，其中'.$return['skillpic'].'项拥有技能证书';
			}

			/*作品案例*/
			$showList					=   	$data['show'];
			if($showList){
				$shownum				=		0;
				$shownum				=		count($showList);
				$return['shownum']		=		$shownum  > 0 ?  $shownum:0;

				$return['show_content']=		'已上传'.$return['shownum'].'份作品案例给招聘企业提前预览';
			}
		}

		return  $return;
	}
	 /**
     * 获取temporary_resume      详情
     * $whereData       		查询条件
     * $data            		自定义处理数组 scene 场景值，定制不同场景返回的数据
     */
    public function getTempResumeInfo($whereData, $data = array()) {
		$Info							=	array();
        $data['field']  				=	empty($data['field']) ? '*' : $data['field'];
		$Info  							=   $this -> select_once('temporary_resume', $whereData, $data['field']);
		if(!empty($Info)){
			//定制不同场景展示的内容
			if(isset($data['scene']) && $data['scene'] == 'detail'){
				//获取缓存数据
				include_once('cache.model.php');
				$cacheM						=   new cache_model($this->db, $this->def);
				$CacheList					=	$cacheM -> GetCache(array('user','hy','job','city'));

				//处理性别
				if(isset($CacheList['user_sex'][$Info['sex']])){
					$Info['sex']			=	$CacheList['user_sex'][$Info['sex']];
				}else{
					$Info['sex']			=	'';
				}
				//处理职位类别
				if(!empty($Info['job_classid'])){
					$jobids					=	@explode(',', $Info['job_classid']);
					foreach($jobids as $val){
						$jobname[]			=	$CacheList['job_name'][$val];
					}
					$Info['jobname']		=	@implode('、',$jobname);
				}
				//处理省份城市
				if(!empty($Info['provinceid'])){
					$Info['city']			=	$CacheList['city_name'][$Info['provinceid']];
				}
				if(isset($CacheList['city_name'][$Info['cityid']])){
					$Info['city']			.=	'-'.$CacheList['city_name'][$Info['cityid']];
				}
				if(isset($CacheList['city_name'][$Info['three_cityid']])){
					$Info['city']			.=	'-'.$CacheList['city_name'][$Info['three_cityid']];
				}
				//处理薪资
				if(!empty($Info['minsalary'])){
					if($Info['maxsalary']){
						$Info['rsalary']	=	$Info['minsalary'].'-'.$Info['maxsalary'];
					}else{
						$Info['rsalary']	=	$Info['minsalary'];
					}
				}else{
					$Info['rsalary']		=	'面议';
				}
				//处理学历
				if(isset($CacheList['userclass_name'][$Info['edu']])){
					$Info['job_edu']		=	$CacheList['userclass_name'][$Info['edu']];
				}else{
					$Info['job_edu']		=	'';
				}
				//处理工作经验
				if(isset($CacheList['userclass_name'][$Info['exp']])){
					$Info['job_exp']		=	$CacheList['userclass_name'][$Info['exp']];
				}else{
					$Info['job_exp']		=	'';
				}
				//处理工作性质
				if(isset($CacheList['userclass_name'][$Info['type']])){
					$Info['job_type']		=	$CacheList['userclass_name'][$Info['type']];
				}else{
					$Info['job_type']		=	'';
				}
				//处理到岗时间
				if(isset($CacheList['userclass_name'][$Info['report']])){
					$Info['job_report']		=	$CacheList['userclass_name'][$Info['report']];
				}else{
					$Info['job_report']		=	'';
				}
				//处理从事行业
				if(isset($CacheList['industry_name'][$Info['hy']])){
					$Info['job_hy']			=	$CacheList['industry_name'][$Info['hy']];
				}else{
					$Info['job_hy']			=	'';
				}
			}
		}
        return $Info;
	}
	/**
     * 删除temporary_resume      详情
     * $whereData       查询条件
     */
    public function delTempResumeInfo($whereData) {
        $nid  =   $this -> delete_all('temporary_resume', $whereData, '');
        return $nid;
    }
    /**
     * 添加temporary_resume
     * $data	插入数据
     */
    public function addTempResumeInfo($data=array()) {

    	$nid  =   $this -> insert_into('temporary_resume',$data);
        return $nid;
    }
    // 简历置顶检测
    public function topResumeCheck($data = array('eid'=>null,'uid'=>null)){

        $return   =  array();

        $expect   =  $this->select_once('resume_expect',array('id'=>$data['eid'],'uid'=>$data['uid']),'`doc`,`state`');
        $work     =  $this->select_num('resume_work',array('eid'=>$data['eid'],'uid'=>$data['uid']));
        $edu      =  $this->select_num('resume_edu',array('eid'=>$data['eid'],'uid'=>$data['uid']));
        $project  =  $this->select_num('resume_project',array('eid'=>$data['eid'],'uid'=>$data['uid']));

        if(empty($expect)){
            $return['msg']  =  '请先创建简历！';
        }else if($expect['state']!=1){
            $return['msg']  =  '您的简历尚未审核，无法置顶操作！';
        }else{
            if($expect['doc'] == 0){
                if($this->config['user_work_regiser']==1){
                    if($work < 1){

                        $return['msg']  =  '你的简历没有工作经历，请填写工作经历';
                    }
                }
                if($this->config['user_edu_regiser']==1){
                    if($edu < 1){

                        $return['msg']  =  '你的简历没有教育经历，请填写教育经历';
                    }
                }
                if($this->config['user_project_regiser']==1){
                    if($project < 1){

                        $return['msg']  =  '你的简历没有项目经历，请填写项目经历';
                    }
                }
            }
        }

        return $return;
    }
    /**
     * @desc 简历匹配职位
     * @param array $data
     */
    public function likeJob($data=array('id'=>null,'uid'=>null,'limit'=>16)){
        $job			=	array();
        if($data['id']){
            $id			=	$data['id'];
            $uid		=	$data['uid'];

            $resume		=	$this->getInfoByEid(array('eid'=>$id,'uid'=>$uid));

            $where['r_status']				=	1;
            $where['status']				=	0;
            $where['state']					=	1;
            if($resume['job_classid']!=""){
                $where['PHPYUNBTWSTART_A']='';
                $where['job_post']			=	array('in',$resume['job_classid']);
                $where['job1_son']			=	array('in',$resume['job_classid'],'OR');
                $where['PHPYUNBTWEND_A']='';
            }
            if($resume['city_classid']!=""){
                $where['PHPYUNBTWSTART_B']	=	'';
                $where['provinceid']		=	array('in',$resume['city_classid']);
                $where['cityid']			=	array('in',$resume['city_classid'],'OR');
                $where['three_cityid']		=	array('in',$resume['city_classid'],'OR');
                $where['PHPYUNBTWEND_B']	=	'';
            }
            //6.1增加条件，职位刷新时间是30天内的
            $where['lastupdate']			=	array('>', strtotime('-30 day'));
            $where['orderby']				=	'id,desc';
            $where['limit']					=	is_numeric($data['limit']) && $data['limit']>0 ? $data['limit'] : 16;
            $cdata['field']					=	'id,uid,name,cityid,three_cityid,edu,sex,marriage,report,exp,minsalary,maxsalary,lastupdate,com_name,com_logo';

            require_once ('job.model.php');
            $jobM  =  new job_model($this->db, $this->def);
            $List	=	$jobM -> getList($where,$cdata);
            $job	=	$List['list'];
            if(is_array($resume)){

                foreach($job as $k=>$v){

                    $pre			=	60;
                    $city_classname		=	@explode(',',$resume['city_classid']);
                    if(in_array($v['cityid'],$city_classname) || in_array($v['three_cityid'],$city_classname)){//地区
                        $pre		=	$pre+10;
                    }
                    if($resume['useredu']==$v['job_edu'] || $v['job_edu']=="不限"){//学历
                        $pre		=	$pre+5;
                    }
                    if($resume['user_marriage']==$v['job_marriage'] || $v['job_marriage']=="不限"){//婚姻
                        $pre		=	$pre+5;
                    }
                    if($resume['sex']==$v['job_sex'] || $v['job_sex']=='不限'){
                        $pre		=	$pre+5;
                    }
                    if($resume['report']==$v['job_report'] || $v['job_report']=="不限"){//到岗时间
                        $pre		=	$pre+5;
                    }
                    if($resume['user_exp']==$v['job_exp'] || $v['job_exp']=="不限"){//工作经验
                        $pre		=	$pre+5;
                    }
                    $job[$k]['pre']	=	$pre;
					if(empty($v['job_sex'])){
						$job[$k]['job_sex']	=	'保密';
					}
					$job[$k]['wapjob_url'] = Url('wap',array('c'=>'job','a'=>'comapply','id'=>$v['id']));
                    $job[$k]['wapcom_url'] = Url('wap',array('c'=>'company','a'=>'show','id'=>$v['uid']));
                }
                $sort = array(
                    'direction' => 'SORT_DESC', //排序顺序标志 SORT_DESC 降序；SORT_ASC 升序
                    'field'     => 'pre',       //排序字段
                );
                $arrSort 			= 	array();
                foreach($job AS $uniqid => $row){

                    foreach($row AS $key=>$value){

                        $arrSort[$key][$uniqid]		 =	 $value;
                    }
                }
                if($sort['direction']){

                    array_multisort($arrSort[$sort['field']], constant($sort['direction']), $job);
                }

            }
        }
        return $job;
    }
    /*会员添加简历页面，添加前检查是否满足各项条件，例如简历数,并返回部分基本信息
      needcache:是否需要缓存，$from：默认pc：为pc端，wap：wap端，wxapp:小程序端
    **/
    public function addResumeRequireCheck($data=array('uid'=>null,'needcache'=>null), $from='pc'){
    	$error['err']		=	0;
    	if($data['uid']){

    		$uid	=	$data['uid'];

			$num	=	$this -> getExpectNum(array('uid'=>$uid));

			if($this->config['user_number']!='' && $num >= $this->config['user_number']){

				$error['err']  =	1;
				$error['msg']  =	'您最多可以拥有'.$this->config['user_number'].'份简历';
				$error['url']  =	'index.php?c=resume';
			}

			return $error;

		}

    }
    /*会员添加简历页面，添加前检查是否满足各项条件，例如身份认证和简历数,并返回部分基本信息
      needcache:是否需要缓存，$from：默认pc：pc端，wap：wap端，wxapp:小程序端
    **/
    public function addResumePage($data=array('uid'=>null,'needcache'=>null), $from='pc'){
    	$return		=	array();
    	if($data['uid']){

    		$uid	=	$data['uid'];

    		$resume	=	$this -> getResumeInfo(array('uid'=>$uid));
			if(empty($resume)){
				$member=$this->select_once('member',array('uid'=>$uid),'`moblie`,`email`');
				$resume['telphone']	=	$member['moblie'];
				$resume['email']	=	$member['email'];
			}
			//验证是否满足条件
			$error	=	$this->addResumeRequireCheck($data, $from);
			$return['error']	   =	$error;
			//验证结束
			if($return['error']['err']==0){

				$return['setarr']	=     array();

				switch ($from) {

					case 'pc':
						$setarr  =	array(
				            'resume' =>	$resume,
				            'layerv' =>	5,
				            'js_def' =>	2
				        );
						break;

					case 'wap':
						$setarr  =	array(
							'resume'      =>  $resume,
							'headertitle' => '创建简历'
						);
						break;

					case 'wxapp':

						$resume['resume_create_exp']	=	$this->config['resume_create_exp'];
						$resume['resume_create_edu']	=	$this->config['resume_create_edu'];
						$resume['resume_create_project']=	$this->config['resume_create_project'];
						$resume['expcreate']			=	$this->config['expcreate'];
                        $resume['educreate']			=	$this->config['educreate'];

                        $resume['jobClassCreate']		=	$this->config['sy_resume_job_classid'];

						$setarr  =	array('resume' => $resume);
						break;
				}
				if(!empty($data['needcache'])){
				    //是否需要缓存
				    require_once ('cache.model.php');
				    $cacheM	=	new cache_model($this->db, $this->def);
				    $cache	=	$cacheM->GetCache(array('city','user','hy','job'));
				    $return['cache']	=	$cache;

				    $setarr['user_sex'] = $cache['user_sex'];
				    $setarr['userdata'] = $cache['userdata'];
				    $setarr['userclass_name'] = $cache['userclass_name'];
				    $setarr['industry_index'] = $cache['industry_index'];
				    $setarr['industry_name'] = $cache['industry_name'];
				    if(empty($cache['city_type'])){
				        $setarr['cionly'] = 1;
				    }
				    if(empty($cache['job_type'])){
				        $setarr['jionly'] = 1;
				    }
				}


                $setarr['salaryList'] = $this->salary();

				$return['setarr'] = $setarr;

			}
		}
		return $return;
    }

	function upExpectHeight($data=array()){

		$nid	=	0;

		if(!empty($data)){
			$eid	=	$data['eid'];
			$expect	=	$this->select_once('resume_expect',array('id'=>$eid));
			if($expect['height_status']!=0){
				$nid	=	$this->update_once('resume_expect',array('height_status'=>1),array('id'=>$eid));
			}

		}
		return $nid;
	}

	/**
	 * 浏览简历列表类查询个人姓名使用
	 */
	private function getJlData($List){

	    foreach ($List as $v){

	        $uids[]  =  $v['uid'];
	    }
	    $resume  =  $this->select_all('resume',array('uid'=>array('in',pylode(',', $uids))),'`uid`,`name`,`nametype`,`photo`,`phototype`,`defphoto`,`photo_status`');

	    foreach ($List as $k=>$v){
	        $List[$k]['wapurl']  =  Url('wap',array('c'=>'resume','a'=>'show','id'=>$v['id']));
	        foreach ($resume as $val){

	            if ($v['uid'] == $val['uid']){

	                $List[$k]['photo_n']  =  $this->setResumePhotoShow(array('photo'=>$val['photo'],'defphoto'=>$val['defphoto'],'phototype'=>$val['phototype'],'photo_status'=>$val['photo_status'],'sex'=>$v['sex']));
	                $List[$k]['uname_n']  =  $this->setUsernameShow(array('nametype'=>$val['nametype'],'name'=>$val['name'],'eid'=>$v['id'],'sex'=>$v['sex']));
	            }
	        }
	    }
	    return $List;
	}
	/*
     * 获取简历的原始数据，类别汉字化处理,$data中定义获取哪些表，all=1为全部
     */
    function getResumeRaw($Where=array(),$data=array('all'=>0)){

        $return = array();
        $resume =   $this->select_once('resume',$Where);

        if(!empty($resume)){

            $cache              =  $this -> getClass(array('user','city','job','hy','introduce'));

            $userclass_name     =  $cache['userclass_name'];

            $resume['sex']      =   $resume['sex'] == 1?'男':'女';

            if($resume['edu']){
                $resume['edu']  =  $userclass_name[$resume['edu']];
            }
            if($resume['exp']){
                $resume['exp']  =  $userclass_name[$resume['exp']];
            }
            if($resume['marriage']){
                $resume['marriage']  =  $userclass_name[$resume['marriage']];
            }
			//联系方式不同步，删除操作
            unset($resume['telphone']);
            unset($resume['telhome']);
            unset($resume['qq']);
            unset($resume['email']);
            unset($resume['address']);
            $return['resume']       =   $resume;

            $fbwhere                =   array('uid'=>$resume['uid'],'eid'=>$resume['def_job']);

            if($data['all']=='1' || $data['expect']=='1'){

                $resume_expect          =   $this->select_once('resume_expect',array('uid'=>$resume['uid'],'defaults'=>1));

                if(!empty($resume_expect)){
                    $resume_expect['sex']          =   $resume_expect['sex'] == 1?'男':'女';
                    $resume_expect['exp']          =   $cache['userclass_name'][$resume_expect['exp']];
                    $resume_expect['edu']          =   $cache['userclass_name'][$resume_expect['edu']];
                    $resume_expect['hy']           =   $cache['industry_name'][$resume_expect['hy']] ? $cache['industry_name'][$resume_expect['hy']] : '不限';
                    $resume_expect['report']       =   $cache['userclass_name'][$resume_expect['report']];
                    $resume_expect['type']         =   $cache['userclass_name'][$resume_expect['type']];
                    $resume_expect['jobstatus']    =   $cache['userclass_name'][$resume_expect['jobstatus']];
                    //处理职位类别id
                    if ($resume_expect['job_classid'] ){

                        $job_classid = @explode(',',$resume_expect['job_classid']);

                        if(is_array($job_classid)){

                            foreach($job_classid as $v){

                                if($cache['job_name'][$v]){

                                    $job_classname[]  =  $cache['job_name'][$v];

                                }
                            }
                            $resume_expect['job_classid']  =  @implode(',',$job_classname);
                        }
                    }
                    //处理城市类别id
                    if ($resume_expect['city_classid']){

                        $city_classid = @explode(',',$resume_expect['city_classid']);

                        if(is_array($city_classid)){

                            foreach($city_classid as $v){

                                if($cache['city_name'][$v]){

                                    $city_classname[]  =  $cache['city_name'][$v];

                                }
                            }
                            $resume_expect['city_classid']  =  @implode(',',$city_classname);
                        }
                    }
                }

                $return['resume_expect']=   !empty($resume_expect) ? $resume_expect : array();

            }
            if($data['all']=='1' || $data['work']=='1'){

                $resume_work            =   $this->select_all('resume_work', $fbwhere);
                $return['resume_work']  =   !empty($resume_work) ? $resume_work : array();

            }
            if($data['all']=='1' || $data['edu']=='1'){

                $resume_edu             =   $this->select_all('resume_edu', $fbwhere);
                if(!empty($resume_edu)){
                    foreach($resume_edu as $ek=>$ev){
                        $resume_edu[$ek]['education']    =   $userclass_name[$ev['education']];
                    }

                }
                $return['resume_edu']   =   !empty($resume_edu) ? $resume_edu : array();

            }

        }

        return $return;
    }
	public function getResumeCityClassList($whereData,$data=array()){
		$field		=	$data['field'] ? $data['field'] : '*';
		$List		=	$this -> select_all('resume_cityclass',$whereData,$field);

		return	$List;
	}

    //发布工具搜索
    public function Getresume($where = array(),$data = array())
    {

        $lists  =  $this->select_all('resume_expect',$where,'`id`');


        foreach ($lists as $k => $v) {

            $list = $this->getInfoByEid(array('eid' => $v['id']));

            if($list['age']==0){

                $list['age']='保密';
            }else{
                $list['age']=$list['age'].'岁';
            }

            if($list['user_exp']){

                $list['user_exp']=$list['user_exp'].'经验';
            }

            if($list['useredu']){

                $list['useredu']=$list['useredu'].'学历';
            }

            if($list['tj']['worknum']>0){

                $list['resume_workjj'] = $list['tj']['workmin'].'-'.$list['tj']['workmax'].'参加过'.$list['tj']['worknum'].'份工作，平均工作时长'.$list['tj']['workavg'].'个月，涉及'.$list['tj']['worktit'].'等岗位。';
            }

            if($list['tj']['education']){

                $list['resume_edujj'] = $list['tj']['edumin'].'-'.$list['tj']['edumax'].'已完成'.$list['tj']['education'].'段学业。';
            }


            $lists[$k]['list'] = $list;
        }

        return $lists;
    }

    public function updateExpect($data = array(), $where = array())
    {

        if (!empty($where)) {
            $this->update_once('resume_expect', $data, $where);
        }
    }

    public function setExpectState($data = array(), $where = array())
    {

        if (!empty($where)) {

            $expect_all =   $this->select_all('resume_expect', $where, '`id`,`uid`');

            $eid_arr    =   array();

            foreach ($expect_all as $k => $v) {
                if (!in_array($v['id'], $eid_arr)) {

                    $eid_arr[]  =   $v['id'];
                }
            }

            $nid        =   $this->update_once('resume_expect', $data, $where);

            if (isset($data['state']) && !empty($eid_arr)) {

                $this->update_once('userid_job', array('resume_state' => $data['state']), array('eid' => array('in', pylode(',', $eid_arr))));
            }

            return $nid;
        }
    }

    /*
     * 获取用户信息
     */
    public function getMemberInfo($whereData,$data = array()){
        $field   =  isset($data['field']) ? $data['field'] : '*';
        return $this->select_once('member',$whereData,$field);
    }

    // 薪资生成
    public function salary() {
        $salary = array();
        for ($i = 1000; $i <= 50000;) {
            $salary[] = $i;
            if ($i >= 20000) {
                $i += 5000;
            } else {
                $i += 1000;
            }
        }
        return $salary;
    }
}
?>