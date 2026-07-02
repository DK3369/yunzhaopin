<?php

class special_special_controller extends adminCommon
{
    /**
     * Recruitment specials.
     */
    function index_action()
    {
        $pageM = $this->MODEL('page');
        $specialM = $this->MODEL("special");

        if (trim($_POST['keyword'])) {
            $where['title'] = array('like', trim($_POST['keyword']));
        }
        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('special', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        // Query the list only when records exist.
        if ($pages['total'] > 0) {
            // Sort.
            $orderby = array();
            if ($_POST['t'] && in_array($_POST['order'], array('asc', 'desc'))) {
                $orderby[] = $_POST['t'] . ',' . $_POST['order'];
            } else {
                $orderby[] = 'id,desc';
            }
            $where['orderby'] = $orderby;
            $where['limit'] = $pages['limit'];

            $List = $specialM->getSpecialList($where, array('utype' => 'admin'));
            if (is_array($List['list'])) {
                foreach ($List['list'] as $value) {
                    $list[] = array(
                        'id' => $value['id'],
                        'title' => $value['title'],
                        'title_href' => $value['title_href'],
                        'tpl' => $value['tpl'],
                        'limit' => $value['limit'],
                        'display' => $value['display'],
                        'display_switch' => $value['display_switch'],
                        'sort' => $value['sort'],
                        'comnum' => $value['comnum'],
                        'booking' => $value['booking'],
                    );
                }
            }
        }
        $rt = array();
        $rt['list'] = $list;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pageSizes;
        $this->render_json(0, '', $rt);
    }

    function get_base_data_action()
    {
        $ratingM = $this->MODEL("rating");
        // Company membership levels.
        $qy_rows = $ratingM->getList(array('category' => 1, 'orderby' => 'sort,desc'), array('field' => '`id`,`name`'));
        // Special templates.
        $file = array();
        $publicdir = "../app/template/" . $this->config['style'] . "/special/";
        $filesnames = @scandir($publicdir);
        if (is_array($filesnames)) {
            foreach ($filesnames as $key => $value) {
                if ($value != '.' && $value != '..') {
                    $typearr = explode('.', $value);
                    if (in_array(end($typearr), array('htm'))) {
                        if (!in_array($value, array('index.htm', 'job.htm'))) {
                            $file[] = $value;
                        }
                    }
                }
            }
        }

        $this->render_json(0, '', array('file' => $file, 'qy_rows' => $qy_rows));
    }

    /**
     * Special list edit: get special info.
     */
    function info_action()
    {
        if ($_POST['id']) {
            $specialM = $this->MODEL("special");
            $row = $specialM->getSpecialOne(array('id' => $_POST['id']));
            $row['rating'] = @explode(',', $row['rating']);
            $this->render_json(0, '', $row);
        }
    }


    function  add_action(){
        if (isset($_POST['add'])){
            // Open the add dialog for permission checks.
            if ($_POST['id']) {
                $specialM = $this->MODEL("special");
                $row = $specialM->getSpecialOne(array('id' => $_POST['id']));
                $row['rating'] = @explode(',', $row['rating']);
                $row['etime']=$row['etime']?date('Y-m-d',$row['etime']):'';
                $this->render_json(0, '', $row);
            }else{
                $this->render_json(0);
            }
        }else{
            if (!is_string($_POST['title']) || !strlen($_POST['title'])) {
                $this->render_json(2, yun_at('admin_01439'));
            }
            if (!is_string($_POST['tpl']) || !strlen($_POST['tpl'])) {
                $this->render_json(3, yun_at('admin_01440'));
            }
            $specialM = $this->MODEL("special");
            $id = (int)$_POST['id'];
            $data['title'] = $_POST['title'];
            $data['tpl'] = $_POST['tpl'];
            $data['display'] = (int)$_POST['display'];
            $data['integral'] = (int)$_POST['integral'];
            $data['com_bm'] = (int)$_POST['com_bm'];
            $data['sort'] = (int)$_POST['sort'];
            $data['limit'] = (int)$_POST['limit'];
            $data['etime'] = strtotime($_POST['etime']);
            $data['ctime'] = time();
            $data['intro'] = str_replace(array("&amp;", "background-color:#ffffff", "background-color:#fff", "white-space:nowrap;"), array("&", '', '', ''), $_POST["intro"]);

            if ($_POST['rating'] && is_array($_POST['rating'])) {
                $data['rating'] = implode(',', $_POST['rating']);
            } else {
                $data['rating'] = '';
            }
            if (!empty($_FILES)) {
                if ($_FILES['sl']['tmp_name']) {
                    $upArrSl = array(
                        'file' => $_FILES['sl'],
                        'dir' => 'special',
                    );
                }
                if ($_FILES['bg']['tmp_name']) {
                    $upArrBg = array(
                        'file' => $_FILES['bg'],
                        'dir' => 'special',
                    );
                }
                if ($_FILES['wapsl']['tmp_name']) {
                    $upArrWapsl = array(
                        'file' => $_FILES['wapsl'],
                        'dir' => 'special',
                    );
                }
                if ($_FILES['wapbg']['tmp_name']) {
                    $upArrWapbg = array(
                        'file' => $_FILES['wapbg'],
                        'dir' => 'special',
                    );
                }
                // Thumbnail parameters.
                $uploadM = $this->MODEL('upload');
                if (isset($upArrSl)) {
                    $picSl = $uploadM->newUpload($upArrSl);// Thumbnail.
                }
                if (isset($upArrBg)) {
                    $picBg = $uploadM->newUpload($upArrBg);// Background image.
                }
                if (isset($upArrWapsl)) {
                    $wapSl = $uploadM->newUpload($upArrWapsl);// Mobile thumbnail.
                }
                if (isset($upArrWapbg)) {
                    $wapBg = $uploadM->newUpload($upArrWapbg);// Mobile background image.
                }
                if (isset($picSl) && !empty($picSl['msg'])) {
                    $this->render_json(8, $picSl['msg']);
                } elseif (isset($picBg) && !empty($picBg['msg'])) {
                    $this->render_json(8, $picBg['msg']);
                } elseif (isset($wapSl) && !empty($wapSl['msg'])) {
                    $this->render_json(8, $wapSl['msg']);
                } elseif (isset($wapBg) && !empty($wapBg['msg'])) {
                    $this->render_json(8, $wapBg['msg']);
                } else {
                    if (!empty($picSl['picurl'])) {
                        $data['pic'] = $picSl['picurl'];
                    }
                    if (!empty($picBg['picurl'])) {
                        $data['background'] = $picBg['picurl'];
                    }
                    if (!empty($wapSl['picurl'])) {
                        $data['wappic'] = $wapSl['picurl'];
                    }
                    if (!empty($wapBg['picurl'])) {
                        $data['wapback'] = $wapBg['picurl'];
                    }
                }
            }
            if (!$id) {
                $nid = $specialM->addSpecial($data);
                $successMsg = yun_t('admin_model_00047', array('id' => $nid));
                $errorMsg = yun_t('admin_model_00049');
            } else {
                $nid = $specialM->upSpecial(array('id' => $id), $data);
                $successMsg = yun_t('admin_model_00048', array('id' => $id));
                $errorMsg = yun_t('admin_model_00050', array('id' => $id));
            }
            $nid ? $this->admin_json(0, $successMsg) : $this->render_json(1, $errorMsg);

        }
    }

    /**
     * Submit add/update.
     */
    function save_action()
    {

            }

    /**
     * View participant companies.
     */
    function com_action()
    {
        $pageM = $this->MODEL('page');
        $specialM = $this->MODEL("special");

        $where['sid'] = (int)$_POST['id'];
        $whereJobsnum['sid'] = $where['sid'];
        if (!empty($_POST['keyword'])) {
            $companyM = $this->MODEL('company');
            $comlist = $companyM->getChCompanyList(array('name' => array('like', $_POST['keyword'])), array('field' => 'uid'));
            if (!empty($comlist)) {
                $uids = array();
                foreach ($comlist as $v) {
                    $uids[] = $v['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uids));
            }
        }
        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('special_com', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        /**
         * Active job count for participant companies.
         */
        $jobsNum = 0;
        if ($pages['total'] > 0) {
            // Sort.
            $orderby = array();
            if ($_POST['t'] && in_array($_POST['order'], array('asc', 'desc'))) {
                $orderby[] = $_POST['t'] . ',' . $_POST['order'];
                if ($_POST['t'] != 'id') {
                    $orderby[] = 'id,desc';
                }
            } else {
                $orderby = array('status,asc', 'uid,desc', 'id,desc');
            }
            $where['orderby'] = $orderby;
            $where['limit'] = $pages['limit'];

            $List = $specialM->getSpecialComList($where, array('utype' => 'admin'));
            if (is_array($List['list'])) {
                foreach ($List['list'] as $value) {
                    $list[] = array(
                        'id' => $value['id'],
                        'sid' => $value['sid'],
                        'uid' => $value['uid'],
                        'status' => $value['status'],
                        'name' => $value['name'],
                        'comUrl' => $value['comUrl'],
                        'sort' => $value['sort'],
                        'famous' => $value['famous'],
                    );
                }
            }
            $jobM = $this->MODEL('job');
            // Participant job count.
            $row = $specialM->getSpecialComOne($whereJobsnum, array('field'=>'group_concat(uid) as uidstring'));
            $uidstring = $row['uidstring'];
            $jobwhere['uid'] = array('in', $uidstring);
            $jobwhere['state'] = 1;
            $jobwhere['r_status'] = 1;
            $jobwhere['status'] = 0;
            $jobsNum = intval($jobM->getJobNum($jobwhere));
        }

        $special = $specialM->getSpecialOne(array('id' => (int)$_POST['id']), array('field' => '`title`,`limit`,`tpl`'));

        $applyNum = $specialM->getSpecialComNum(array("sid" => (int)$_POST['id'], 'status' => '1'));


        $this->render_json(0, '', array(
            'list' => $list,
            'total' => intval($pages['total']),
            'perPage' => $pageSize,
            'pageSizes' => $pageSizes,
            'showAdd' => $special['limit'] > $applyNum,// Whether to show Add participant company.
            'applyNum' => $applyNum,// Participant company count.
            'jobsNum' => $jobsNum,// Participant job count.
            'special' => $special,
        ));
    }

    /**
     * Export participant companies.
     */
    function comxls_action(){

        $specialM	=	$this->MODEL("special");

        $CompanyM		=	$this -> MODEL('company');

        $JobM			=	$this -> MODEL('job');
        // Special ID.
        if($_POST['zid']){
            // Company ID.
            if($_POST['cid']){
                $zcwhere = array('id'=>array('in',$_POST['cid']));
            }else{
                $zcwhere = array('sid'=>$_POST['zid']);
            }

            $rows		=	$specialM -> getSpecialComList($zcwhere);

            if(!empty($rows['list'])){

                $cacheM  =  $this->MODEL('cache');
                $cache   =  $cacheM->getCache(array('com'));

                $comclass_name  =  $cache['comclass_name'];

                $jobids = $jobuids = $comids = array();
                foreach ($rows['list'] as $key=>$val){

                    $comids[]   =   $val['uid'];

                    $jobuids[]  =   $val['uid'];

                }

                $comField  =  '`uid`,`name`,`mun`,`content`,`address`,`linktel`,`linkman`,`linkphone`,`welfare`,`money`,`moneytype`';

                $companys  =  $CompanyM -> getChCompanyList(array('uid'=>array('in',@implode(',',$comids))),array('field'=>$comField));

                $jobField  =  '`id`,`uid`,`name`,`zp_num`,`minsalary`,`maxsalary`,`exp`,`edu`,`provinceid`,`cityid`,`three_cityid`';

                $jobWhere['state']          =   1;
                $jobWhere['status']         =   0;// Active jobs.
                $jobWhere['r_status']       =   1;

                $jobWhere['PHPYUNBTWSTART'] =   '';
                $jobWhere['uid']	        =	array('in',pylode(',',$jobuids));
                $jobWhere['id']	            =	array('in',pylode(',',$jobids), 'OR');
                $jobWhere['PHPYUNBTWEND']   =   '';

                $jobsA	   =  $JobM -> getList($jobWhere,array('field'=>$jobField));
                $jobs	   =  $jobsA['list'];


                foreach($companys as $k=>$va){

                    $companys[$k]['content']	=	trim(strip_tags($va['content']));

                    $companys[$k]['mun']		=	$comclass_name[$va['mun']];

                    foreach($jobs as $val){
                        if ($va['uid'] == $val['uid']){
                            $companys[$k]['jobs'][]  =  $val;
                        }
                    }
                }
                $maxJobNum = 1;
                foreach ($companys as $v){
                    $jobnum  =  count($v['jobs']);
                    if ($jobnum > $maxJobNum){
                        $maxJobNum  =  $jobnum;
                    }
                }
                $jobTr = $jobSonTr = '';

                for($i=1;$i<=$maxJobNum;$i++){

                    $jobTr .= '<th colspan="6">' . yun_t('admin_model_00065', array('index' => $i)) . '</th>';

                    $jobSonTr .= '<th>' . yun_t('admin_model_00066') . '</th><th>' . yun_t('admin_model_00067') . '</th><th>' . yun_t('admin_model_00068') . '</th><th>' . yun_t('admin_model_00069') . '</th><th>' . yun_t('admin_model_00070') . '</th><th>' . yun_t('admin_model_00071') . '</th>';
                }

                $this -> yunset('jobTr',$jobTr);

                $this -> yunset('jobSonTr',$jobSonTr);

                $this -> yunset('list',$companys);

                $this -> MODEL('log') -> addAdminLog('admin_01441');

                header('Content-Type: application/vnd.ms-excel');

                header('Content-Disposition: attachment; filename=special.xls');

                $this->yuntpl(array('admin/yunying/special/comxls'));
            }
        }
    }

    /**
     * Batch review or join-save.
     */
    function statuscom_action(){
        $specialM	=	$this->MODEL("special");
        $IntegralM	=	$this->MODEL('integral');

        $pid		=	$_POST['pid'];
        $status		=	(int)$_POST['status'];
        $statusbody	=	trim($_POST['statusbody']);
        // Rejected.
        if($status=='2'){
            $iWhere['id']		=	array('in',$pid);
            $iWhere['status']	=	array('<>','2');
            $idata['field']		=	"`uid`,`integral`";
            $rows				=	$specialM->getSpecialComList($iWhere,$idata);
        }

        $upWhere['id']			=	array('in',$pid);
        $upWhere['status']		=	array('<>','2');
        $upData['status']		=	$status;
        $upData['statusbody']	=	$statusbody;
        $id						=	$specialM->upSpecialCom($upWhere,$upData);

        if($id&&is_array($rows['list'])){
            foreach($rows['list'] as $val){
                if($val['integral']>0){
                    $IntegralM->company_invtal($val['uid'],2,$val['integral'],true,'admin_yunying_00013'.$this->config['integral_pricename'],true,2,'integral');
                }
            }
        }

        $lWhere['id']		=	array('in',$pid);
        $ldata['field']		=	"`sid`,`uid`";
        $list				=	$specialM->getSpecialComList($lWhere,$ldata);

        if($list['list']){
            /* Message prefix. */
            $sysmsgM			=	$this->MODEL('sysmsg');

            $tagName  			=	'admin_01442';

            $v  	    		=	reset($list['list']);
            $sid    			=	$v['sid'];
            $special			= 	$specialM->getSpecialOne(array('id'=>$sid),array('field'=>'`title`'));

            // Send to companies.
            foreach($list['list'] as $v){

                $uids[]  =  $v['uid'];

                /* Build review message. */
                if ($_POST['status'] == 2){

                    $statusInfo  =  yun_t('admin_model_00051', array('title' => $special['title']));

                    if($_POST['statusbody']){

                        $statusInfo  =  yun_t('admin_model_00052', array('title' => $special['title'], 'reason' => $_POST['statusbody']));

                    }

                    $msg[$v['uid']]  =  $statusInfo;

                }elseif($_POST['status'] == 1){

                    $msg[$v['uid']]  =   yun_t('admin_model_00053', array('title' => $special['title']));

                }
            }

            // Send system notification.
            $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>2, 'content'=>$msg));
        }

        if (isset($_POST['single'])){
            if ($id){
                $this->admin_json(0, yun_t('admin_model_00054', array('ids' => $pid)));
            }else{
                $this->render_json(1, yun_at('model_00003'));
            }
        }else{
            $id ? $this->admin_json(0, yun_t('admin_model_00054', array('ids' => $pid))) : $this->render_json(1, 'model_00003');
        }
    }

    /**
     * Recruitment special participant add: get company info.
     */
    function audit_action(){
        $id =  intval($_POST['id']);// special_com.id

        $specialM =	$this->MODEL("special");
        $ComM     = $this -> MODEL('company');
        $userinfoM = $this->MODEL('userinfo');

        $specialCom	  =  $specialM->getSpecialComOne(array('id'=>$id),array('field'=>'id,uid,status,statusbody'));

        $Info   = $ComM->getInfo($specialCom['uid'], array('ywy' => 1));
        $member = $userinfoM->getInfo(array('uid' => $specialCom['uid']),array('field' => 'login_ip,reg_date'));
        // Return only required data.
        $return = array(
            'name' => $Info['name'],
            'rating_name' => $Info['rating_name'],
            'linkman' => $Info['linkman'],
            'linkjob' => $Info['linkjob'],
            'linktel' => $Info['linktel'],
            'infostatus' => $Info['infostatus'],
            'crm_name' => $Info['crm_name'],
            'reg_date_n' => $member['reg_date'] ? date('Y-m-d H:i:s', $member['reg_date']) : '',
            'login_date_n' => $Info['login_date'] ? date('Y-m-d H:i:s', $Info['login_date']) : '',
            'login_ip' => $member['login_ip'],
            'welfare_n' => $Info['welfare_n'],
            'hy_n' => $Info['hy_n'],
            'pr_n' => $Info['pr_n'],
            'mun_n' => $Info['mun_n'],
            'money' => $Info['money'],
            'moneytype_n' => $Info['moneytype_n'],
            'job_city_one' => $Info['job_city_one'],
            'job_city_two' => $Info['job_city_two'],
            'job_city_three' => $Info['job_city_three'],
            'address' => $Info['address'],
            'content' => $Info['content'],
            'special' => $specialCom,
        );
        $this->render_json(0, '', $return);
    }

    /**
     * Recruitment special participant add: get jobs.
     */
    function comjob_action()
    {
        $uid = intval($_POST['uid']);
        if ($uid < 1) {
            $this->render_json(1, yun_at('common_01716'));;
        }

        $pageM = $this->MODEL('page');
        $jobM = $this->MODEL('job');
        $jobwhere['uid'] = $uid;
        $jobwhere['state'] = 1;
        $jobwhere['r_status'] = 1;
        $jobwhere['status'] = 0;

        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_job', $jobwhere, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $total = $pages['total'];
        $list = array();

        if ($total > 0) {
            $jobwhere['orderby'] = array('lastupdate,desc');
            $jobwhere['limit'] = $pageM->pageLimit($_POST);
            $jobsA = $jobM->getList($jobwhere, array('isurl' => 'yes', 'utype' => 'admin'));// Active jobs.
            if (is_array($jobsA['list'])) {
                foreach ($jobsA['list'] as $value){
                    $list[] = array(
                        'id' => $value['id'],
                        'name' => $value['name'],
                        'job_exp' => $value['job_exp'],
                        'job_edu' => $value['job_edu'],
                        'job_salary' => $value['job_salary'],
                        'url' => Url('job', array('c' => 'comapply', 'id' => $value['id'])),
                    );
                }
            }
        }
        $this->render_json(0, '', array('list' => $list, 'total' => (int)$total,'pageSizes'=>$pageSizes));
    }

    function getinfo_action(){
        $specialM		=	$this->MODEL("special");

        $where['id']	=	intval($_POST['id']);

        $data['field']	=	'`statusbody`';

        $row			=	$specialM->getSpecialComOne($where,$data);
        echo $row['statusbody'];die;
    }

    /**
     * Participant company page: cancel/delete.
     */
    function delcom_action()
    {
        $_POST['id'] = intval($_POST['id']);
        if ($_POST['del'] || $_POST['id']) {
            $specialM = $this->MODEL("special");
            if (is_array($_POST['del'])) {
                $del = pylode(',', $_POST['del']);
            } else {
                $del = $_POST['id'];
            }
            $specialM->delSpecialCom(array('id' => array('in', $del)), array('type' => 'all'));

            $this->admin_json(0, yun_t('admin_model_00055', array('ids' => $del)));
        } else {
            $this->render_json(1, yun_at('model_00034'));
        }
    }

    /**
     * Delete.
     */
    function del_action()
    {
        $_POST['id'] = (int)$_POST['id'];
        if ($_POST['del'] || $_POST['id']) {
            if (is_array($_POST['del'])) {
                $type = 'all';
                $del = pylode(',', $_POST['del']);
            } else {
                $type = 'one';
                $del = $_POST['id'];
            }
            $specialM = $this->MODEL("special");
            $specialM->delSpecial(array('id' => array('in', $del)), array('type' => $type));

            $this->admin_json(0, yun_t('admin_model_00056', array('ids' => $del)));
        } else {
            $this->render_json(1, yun_at('model_00034'));
        }
    }

    /**
     * Enable or disable recruitment special.
     */
    function recommend_action()
    {
        if ($_POST['type'] == "rec_display") {
            $specialM = $this->MODEL('special');
            $data['display'] = $_POST['rec'];
            $where['id'] = $_POST['id'];
            $nid = $specialM->upSpecial($where, $data);
            $msg = $_POST['rec'] == 1 ? yun_at('member_com_00287') : yun_at('resume_00030');
            if ($nid) {
                $this->admin_json(0, yun_t('admin_model_00063', array('id' => $_POST['id'], 'action' => $msg)));
            } else {
                $this->render_json(1, yun_t('admin_model_00064', array('id' => $_POST['id'], 'action' => $msg)));
            }
        }
    }

    /**
     * Participant company sort.
     */
    function ajaxsort_action(){
        if ($_POST['id']) {
            $specialM = $this->MODEL('special');
            if (!empty($_POST['sort']) || $_POST['sort'] === '0') {
                $uparr['sort'] = intval($_POST['sort']);
            }
            $specialM->upSpecialCom(array('id' => $_POST['id']), $uparr);
            $this->admin_json(0, yun_t('admin_model_00057', array('id' => $_POST['id'], 'sort' => intval($_POST['sort']))));
        }
    }

    /**
     * Recruitment special sort.
     */
    function setOrder_action()
    {
        $post = $_POST;
        if ($post['id']) {
            $specialM = $this->MODEL('special');
            $where = array('id' => $post['id']);
            $data = array('sort' => $post['sort']);
            $nid = $specialM->upSpecial($where, $data);
            if ($nid) {
                $this->admin_json(0, yun_t('admin_model_00058', array('id' => $post['id'], 'sort' => $post['sort'])));
            } else {
                $this->render_json(1, yun_at('admin_01443'));
            }
        }
    }

    /**
     * Add participant company search filters.
     */
    function set_comaddsearch_action()
    {
        $ratingM = $this->MODEL('rating');
        $rating = $ratingM->getList(array('category' => '1', 'orderby' => 'sort,desc'), array('field' => '`id`,`name`'));
        if (!empty($rating)) {
            $ratingList = array();
            foreach ($rating as $k => $v) {
                $ratingList[] = array('value' => $v['id'], 'label' => $v['name']);
            }
        }

        include(CONFIG_PATH . 'db.data.php');
        $sourceList = array();
        foreach ($arr_data['source'] as $k => $v) {
            $sourceList[] = array('value' => $k, 'label' => $v);
        }

        $timeSection = array(array('value' => '1', 'label' => 'common_01940'), array('value' => '3', 'label' => 'admin_tool_00619'), array('value' => '7', 'label' => 'admin_tool_00622'), array('value' => '15', 'label' => 'admin_yunying_00017'), array('value' => '30', 'label' => 'admin_yunying_00016'), array('value' => '31', 'label' => 'admin_01444'), array('value' => '32', 'label' => 'admin_01445'), array('value' => '33', 'label' => 'admin_01446'), array('value' => '34', 'label' => 'admin_01447'),);
        $status = array(array('value' => '1', 'label' => 'wap_user_00165'), array('value' => '2', 'label' => 'admin_user_00138'), array('value' => '3', 'label' => 'wap_user_00167'), array('value' => '4', 'label' => 'wap_user_00166'), array('value' => '5', 'label' => 'admin_user_00184'),);
        $edtime = array(array('value' => '1', 'label' => 'admin_tool_00622'), array('value' => '2', 'label' => 'common_01659'), array('value' => '3', 'label' => 'common_01897'), array('value' => '4', 'label' => 'common_01875'), array('value' => '5', 'label' => 'wap_com_00319'),);
        $isrec = array(array('value' => '1', 'label' => 'admin_model_00059'), array('value' => '2', 'label' => 'admin_model_00060'), array('value' => '3', 'label' => 'wap_com_00319'),);
        $isgw = array(array('value' => '1', 'label' => 'admin_01303'), array('value' => '2', 'label' => 'admin_user_company_00153'),);

        $result = array(
            'ratingList' => $ratingList,// Membership level.
            'timeList' => $edtime,// Expiration time.
            'statusList' => $status,// Review status.
            'sourceList' => $sourceList,// Data source.
            'recList' => $isrec,// Featured company.
            'gwList' => $isgw,// Company consultant.
            'lotimeList' => $timeSection,// Recent login.
            'adtimeList' => $timeSection,// Recent registration.
        );
        $this->render_json(0, '', $result);
    }

    /**
     * Add participant company button: get companies.
     */
    function addlist_action()
    {
        $pageM = $this->MODEL('page');
        $ComM          =   $this -> MODEL('company');
        $specialM      =   $this->MODEL('special');
        $where         =   array('r_status'=>1);
        $mwhere        =   array();
        if ($_POST['keyword']) {
            $keywordStr =   trim($_POST['keyword']);
            $typeStr    =   intval($_POST['type']);
            if (!empty($keywordStr) && $typeStr == 1) {
                // Company name/short name.
                $where['PHPYUNBTWSTART_C']   = '';
                $where['name']               = array('like',$keywordStr);
                $where['shortname']          = array('like',$keywordStr,'OR');
                $where['PHPYUNBTWEND_C']     = '';
            } elseif (!empty($keywordStr) && $typeStr == 2) {
                // User name.
                $mwhere['username'] =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 3) {
                // Contact.
                $where['linkman']   =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 4) {
                // Contact phone.
                $where['linktel']   =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 5) {
                // User email.
                $where['linkmail']  =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 6) {
                // User ID.
                $where['uid'][]     =   array('=', $keywordStr);
            }
        }
        // Review status.
        if ($_POST['status']) {
            $status =   intval($_POST['status']);
            if ($status == 4) {
                // Pending review.
                $where['r_status']  =   0;
            } else if ($status == 5) {
                // Paused.
                $where['r_status']  =   4;
            } else {
                $where['r_status']  =   $status;
            }
        }
        // Recent registration.
        if ($_POST['adtime']) {
            $adtime = intval($_POST['adtime']);
            if ($adtime == 1) {
                // Today.
                $mwhere['reg_date'] = array('>', strtotime('today'));
            } else if ($adtime < 31) {
                $mwhere['reg_date'] = array('>', strtotime('-' . $adtime . ' day'));
            } else if ($adtime == 31) {// 1 - 3 months.
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-1 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-3 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 32) {// 3 - 6 months.
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-3 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-6 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 33) {// 6 months - 1 year.
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-6 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-12 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 34) {// More than 1 year.
                $mwhere['reg_date'] = array('<', strtotime('-1 year'));
            }
        }
        // Recent login.
        if($_POST['lotime']){
            $lotime    =   intval($_POST['lotime']);
            if($lotime ==  1){
                $mwhere['login_date']  =   array('>',  strtotime('today'));
            }else if ($lotime < 31){
                $mwhere['login_date']  =   array('>',  strtotime('-'.$lotime.' day'));
            }else if ($lotime == 31){
                $mwhere['PHPYUNBTWSTART_C']    =   '';
                $mwhere['login_date'][]  =   array('<',  strtotime('-1 month'));
                $mwhere['login_date'][]  =   array('>=',  strtotime('-3 month'));
                $mwhere['PHPYUNBTWEND_C']      =   '';
            }else if ($lotime == 32){
                $mwhere['PHPYUNBTWSTART_C']    =   '';
                $mwhere['login_date'][]  =   array('<',  strtotime('-3 month'));
                $mwhere['login_date'][]  =   array('>=',  strtotime('-6 month'));
                $mwhere['PHPYUNBTWEND_C']      =   '';
            }else if ($lotime == 33){
                $mwhere['PHPYUNBTWSTART_C']    =   '';
                $mwhere['login_date'][]  =   array('<',  strtotime('-6 month'));
                $mwhere['login_date'][]  =   array('>=',  strtotime('-12 month'));
                $mwhere['PHPYUNBTWEND_C']      =   '';
            }else if ($lotime == 34){
                $mwhere['login_date']  =   array('<',  strtotime('-1 year'));
            }
        }
        // Data source.
        if($_POST['source']){
            $mwhere['source']          =   $_POST['source'];
        }

        $mUids		=	array();
        $UserinfoM	=	$this -> MODEL('userinfo');
        if(!empty($mwhere)){
            $uidList    =   $UserinfoM->getList($mwhere, array('field' => '`uid`'));
            if(!empty($uidList)){
                foreach($uidList as $uv){
                    $mUids[]	=	$uv['uid'];
                }
            }else{
                $mUids			=	array(0);
            }
            $where['uid'][] =	array('in', pylode(',',$mUids));
        }
        // Membership level.
        if($_POST['rating']){
            $where['rating']   =   $_POST['rating'];
        }
        // Expiration time.
        $toDay	    =	strtotime(date('Y-m-d'));
        if($_POST['time']){
            $time   =   intval($_POST['time']);
            if($time == 5){
                // Expired.
                $where['PHPYUNBTWSTART_A']    =   '';
                $where['vipetime'][]         =   array('>', '0','AND');
                $where['vipetime'][]         =   array('<',  $toDay,'AND');
                $where['PHPYUNBTWEND_A']      =   '';
            }else{
                if($time == 1){
                    // Within 7 days.
                    $num   =   '+7 day';
                }elseif($time == 2 ){
                    // Within 1 month.
                    $num   =   '+1 month';
                }elseif($time == 3){
                    // Within 6 months.
                    $num   =   '+6 month';
                }elseif($time == 4){
                    // Within 1 year.
                    $num='+1 year';
                }

                $where['PHPYUNBTWSTART_B']    =   '';
                $where['vipetime'][]         =   array('>', time(),'AND');
                $where['vipetime'][]         =   array('<', strtotime($num),'AND');
                $where['PHPYUNBTWEND_B']      =   '';
            }
        }
        // Featured company.
        if($_POST['rec']){
            $rec    =   intval($_POST['rec']);
            if($rec == 1){
                // Yes.
                $where['rec']       =   '1';
                $where['hottime']   =   array('>', time());
            }elseif ($rec == 2){
                // No.
                $where['rec']       =   '0';
            }elseif ($rec == 3){
                // Expired.
                $where['rec']       =   '1';
                $where['hottime']   =   array('<', time());
            }
        }
        // Company consultant.
        if($_POST['gw']){
            if(intval($_POST['gw']) == 1){
                // Assigned.
                $where['crm_uid']     =   array('<>', '0');
            }else{
                // Unassigned.
                $where['crm_uid']     =   '0';
            }
        }
        // Job status.
        if ($_POST['job']) {
            $job = intval($_POST['job']);
            if (in_array($job, array(1, 2))) {
                $jobM = $this->MODEL('job');
                $jobwhere = array();
                $jobwhere['state'] = 1;// Review state: 0 pending, 1 approved, 2 expired, 3 rejected.
                $jobwhere['r_status'] = 1;// 2 locked, 4 company paused and jobs paused, 1 normal.
                $jobwhere['status'] = 0;// Recruitment state: 1 paused/offline, 0 recruiting/online.
                $jobsUidList = $jobM->getListId($jobwhere, array('field' => 'distinct `uid`'));
                $jobsUidArr = array();
                $jobsUids = '0';
                if (is_array($jobsUidList)) {
                    foreach ($jobsUidList as $k => $v) {
                        $jobsUidArr[] = $v['uid'];
                    }
                    $jobsUids = pylode(',', $jobsUidArr);
                }
                if (intval($_POST['job']) == 1) {
                    // Has jobs.
                    $where['uid'] = array('in', $jobsUids);
                } else {
                    // No jobs.
                    $where['uid'] = array('notin', $jobsUids);
                }
            }
        }
        // Build pagination.
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        // Query the list only when records exist.
        if($pages['total'] > 0){
            // Limit/order is only needed for list queries.
            if($_POST['order']){
                $where['orderby']		=	$_POST['t'].','.$_POST['order'];
            }else if($_POST['time'] == '5'){
                // Expiration time: expired.
                $where['orderby']		=	array('vipetime,desc','uid,desc');
            }else if($_POST['time']){
                // Expiration time.
                $where['orderby']		=	array('vipetime,asc');
            }else if($_POST['lotime']){
                // Recent login.
                $where['orderby']		=	array('login_date,desc');
            }else{
                $where['orderby']		=	'uid,desc';
            }

            $where['limit']				=	$pages['limit'];;

            $ListNew    =   $ComM -> getList($where,array('utype'=>'admin'));

            // Query participants in this online special and mark joined companies in the add list.
            $netcom     =   $specialM->getSpecialComList(array('sid'=>$_POST['sid']),array('field'=>'uid'));

            if (!empty($netcom['list'])){
                foreach ($netcom['list'] as $v){
                    $ncuid[]    =   $v['uid'];
                }
            }

            foreach ($ListNew['list']  as $key => $val){
                $ListNew['list'][$key]['wxBindmsg'] = $this->wxBindState($val);
                $ListNew['list'][$key]['join'] = 0;
                if (!empty($ncuid)){
                    if (in_array($val['uid'], $ncuid)){
                        $ListNew['list'][$key]['join'] = 1;
                    }
                }
                $list[$key] = [
                    'uid' => $val['uid'],
                    'jobnum' => $val['jobnum'],// Job count.
                    'zz_jobnum' => $val['zz_jobnum'],// Active job count.
                    'r_status' => $val['r_status'],
                    'name' => $val['name'],
                    'shortname' => $val['shortname'],
                    'moblie_status' => $val['moblie_status'],
                    'wxid' => $val['wxid'],
                    'wxopenid' => $val['wxopenid'],
                    
                    'wxBindmsg' => $ListNew['list'][$key]['wxBindmsg'],
                    'yyzz_status' => $val['yyzz_status'],
                    'yyzzurl' => $val['yyzzurl'],
                    'owner_cert_url' => $val['owner_cert_url'],
                    'wt_cert_url' => $val['wt_cert_url'],
                    'other_cert_url' => $val['other_cert_url'],
                    'social_credit' => $val['social_credit'],
                    'status' => $val['status'],
                    'comUrl' => $val['comUrl'],
                    'vipetime' => $val['vipetime'],
                    'rating' => $val['rating'],
                    'rating_name' => $val['rating_name'],
                    'oldrating_name' => $val['oldrating_name'],
                    'linktel' => $val['linktel'],
                    'linkphone' => $val['linkphone'],
                    'login_date_n' => $val['login_date'] ? date('Y-m-d H:i', $val['login_date']) : 'admin_user_00139',
                    'crm_uid' => $val['crm_uid'],
                    'crm_uid_n' => $val['crm_uid'] ? $val['crm_name'] : 'admin_user_company_00153',
                    'join' => $ListNew['list'][$key]['join'],
                ];
            }
        }

        $totalNum = intval($ComM->getCompanyNum(array('r_status' => 1)));
        $applyNum = $specialM->getSpecialComNum(array("sid" => (int)$_POST['id'], 'status' => '1'));
        $noNum =  max($totalNum - $applyNum, 0);

        $this->render_json(0, '', array('list' => $list, 'total' => intval($pages['total']),
            'perPage' => $pageSize, 'pageSizes' => $pageSizes,
            'totalNum' => $totalNum, 'applyNum' => $applyNum, 'noNum' => $noNum));
    }

    // Save added participant company.
    function savespecial_action()
    {
        $SpecialM = $this->MODEL('special');

        $id     =   intval($_POST['sid']);
        $uid    =   intval($_POST['uid']);
        $isapply=   $SpecialM->getSpecialComNum(array("uid" => $uid, "sid" => $id));
        if ((int)$isapply > 0) {
            $this->render_json(1, yun_at('admin_yunying_00015'));
        }

        $nid    =   $SpecialM->addSpecialCom(array("sid" => $id, "uid" => $uid, 'sort' => 0, 'status' => '1', 'time' => time()));
        if (isset($nid)) {
            $this->admin_json(0, yun_t('admin_model_00061', array('sid' => $id, 'uid' => $uid)));
        } else {
            $this->render_json(2, yun_at('admin_01448'));
        }
    }

    function mutiAddCom_action()
    {
        $specialM = $this->MODEL('special');
        $data['uid'] = $_POST['uid'];
        $data['sid'] = intval($_POST['sid']);
        $return = $specialM->addSpecialMutiCom($data);
        if ($return['error'] === 0) {
            $this->admin_json(0, yun_t('admin_model_00061', array('sid' => $data['sid'], 'uid' => $data['uid'])));
        } else {
            $this->render_json($return['error'], $return['msg']);
        }
    }

    // Search company list by company name.
    function getcomlist_action(){

        $companyM  =  $this->MODEL('company');

        $comname   =  trim($_POST['comname']);

        $rows	=  $companyM -> getChCompanyList(array('name'=>array('like',$comname)));

        $html 	=  '<option value="">' . yun_t('admin_model_00034') . '</option>';

        foreach ($rows as $v){

            $html .= '<option value="'.$v['uid'].'">'.$v['name'].'</option>';
        }

        echo $html;
    }

    /**
     * View participant company: set or cancel featured company.
     */
    function setFamous_action()
    {
        if ($_POST && $_POST['sid'] && $_POST['uid']) {
            $famous = $_POST['famous'] == 1 ? 0 : 1;

            $specialM = $this->MODEL('special');
            $specialInfo = $specialM->getSpecialOne(array('id' => $_POST['sid'], 'tpl' => 'gl.htm'));
            if (!$specialInfo) {
                $this->render_json(2, yun_at('admin_01449'));
            }

            $nid = $specialM->upSpecialCom(array('sid' => $_POST['sid'], 'uid' => $_POST['uid']), array('famous' => $famous));

            if ($nid) {
                $this->admin_json(0, yun_t('admin_model_00062', array('sid' => $_POST['sid'], 'uid' => $_POST['uid'])));
            } else {
                $this->render_json(1, yun_at('admin_01449'));
            }
        }
    }
}
