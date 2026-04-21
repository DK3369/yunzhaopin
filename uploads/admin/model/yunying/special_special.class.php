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
class special_special_controller extends adminCommon
{
    /**
     * 招聘专题
     */
    function index_action()
    {
        $pageM = $this->MODEL('page');
        $specialM = $this->MODEL("special");

        if (trim($_POST['keyword'])) {
            $where['title'] = array('like', trim($_POST['keyword']));
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('special', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        //分页数大于0的情况下 执行列表查询
        if ($pages['total'] > 0) {
            //排序
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
        //企业等级 会员等级
        $qy_rows = $ratingM->getList(array('category' => 1, 'orderby' => 'sort,desc'), array('field' => '`id`,`name`'));
        //专题模板
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
     * 专题列表，编辑操作，获取专题信息
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
            // 添加打开弹窗请求，方便权限控制
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
                $this->render_json(2, '请填写专题名称！');
            }
            if (!is_string($_POST['tpl']) || !strlen($_POST['tpl'])) {
                $this->render_json(3, '请选择专题模板！');
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
                //缩率图参数
                $uploadM = $this->MODEL('upload');
                if (isset($upArrSl)) {
                    $picSl = $uploadM->newUpload($upArrSl);//缩略图
                }
                if (isset($upArrBg)) {
                    $picBg = $uploadM->newUpload($upArrBg);//背景图
                }
                if (isset($upArrWapsl)) {
                    $wapSl = $uploadM->newUpload($upArrWapsl);//移动端缩略图
                }
                if (isset($upArrWapbg)) {
                    $wapBg = $uploadM->newUpload($upArrWapbg);//移动端背景图
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
                $name = "专题招聘（ID：" . $nid . "）添加";
            } else {
                $nid = $specialM->upSpecial(array('id' => $id), $data);
                $name = "专题招聘（ID：" . $id . "）更新";
            }
            $nid ? $this->admin_json(0, $name . "成功！") : $this->render_json(1, $name . "失败！");

        }
    }

    /**
     * 添加/修改 提交
     */
    function save_action()
    {

            }

    /**
     * 查看参会企业
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
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('special_com', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        /**
         * 参会企业的在招职位数量
         */
        $jobsNum = 0;
        if ($pages['total'] > 0) {
            //排序
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
            //参会职位数量
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
            'showAdd' => $special['limit'] > $applyNum,//是否显示 添加参会企业
            'applyNum' => $applyNum,//参会企业的数量
            'jobsNum' => $jobsNum,//参会职位数量
            'special' => $special,
        ));
    }

    /**
     * 导出参会企业
     */
    function comxls_action(){

        $specialM	=	$this->MODEL("special");

        $CompanyM		=	$this -> MODEL('company');

        $JobM			=	$this -> MODEL('job');
        //专题ID
        if($_POST['zid']){
            //企业ID
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
                $jobWhere['status']         =   0;//招聘中职位
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

                    $jobTr .= '<th colspan="6">岗位'.$i.'</th>';

                    $jobSonTr .= '<th>招聘岗位</th><th>招聘人数</th><th>薪酬</th><th>经验要求</th><th>学历要求</th><th>工作地点</th>';
                }

                $this -> yunset('jobTr',$jobTr);

                $this -> yunset('jobSonTr',$jobSonTr);

                $this -> yunset('list',$companys);

                $this -> MODEL('log') -> addAdminLog('导出报名专题招聘信息');

                header('Content-Type: application/vnd.ms-excel');

                header('Content-Disposition: attachment; filename=special.xls');

                $this->yuntpl(array('admin/yunying/special/comxls'));
            }
        }
    }

    /**
     * 批量审核 | 加入-保存
     */
    function statuscom_action(){
        $specialM	=	$this->MODEL("special");
        $IntegralM	=	$this->MODEL('integral');

        $pid		=	$_POST['pid'];
        $status		=	(int)$_POST['status'];
        $statusbody	=	trim($_POST['statusbody']);
        //未通过
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
                    $IntegralM->company_invtal($val['uid'],2,$val['integral'],true,"专题招聘未通过审核，退还".$this->config['integral_pricename'],true,2,'integral');
                }
            }
        }

        $lWhere['id']		=	array('in',$pid);
        $ldata['field']		=	"`sid`,`uid`";
        $list				=	$specialM->getSpecialComList($lWhere,$ldata);

        if($list['list']){
            /* 消息前缀 */
            $sysmsgM			=	$this->MODEL('sysmsg');

            $tagName  			=	'专题报名';

            $v  	    		=	reset($list['list']);
            $sid    			=	$v['sid'];
            $special			= 	$specialM->getSpecialOne(array('id'=>$sid),array('field'=>'`title`'));

            //发送企业
            foreach($list['list'] as $v){

                $uids[]  =  $v['uid'];

                /* 处理审核信息 */
                if ($_POST['status'] == 2){

                    $statusInfo  =  '您参与的专题'.$special['title'].':报名审核未通过';

                    if($_POST['statusbody']){

                        $statusInfo  .=  ' , 原因：'.$_POST['statusbody'];

                    }

                    $msg[$v['uid']]  =  $statusInfo;

                }elseif($_POST['status'] == 1){

                    $msg[$v['uid']]  =   '您参与的专题'.$special['title'].':报名已审核通过';

                }
            }

            //发送系统通知
            $sysmsgM -> addInfo(array('uid'=>$uids,'usertype'=>2, 'content'=>$msg));
        }

        if (isset($_POST['single'])){
            if ($id){
                $this->admin_json(0, '招聘专题参会企业审核(ID：' . $pid . ')成功！');
            }else{
                $this->render_json(1, '操作失败！');
            }
        }else{
            $id ? $this->admin_json(0, '招聘专题参会企业审核(ID：' . $pid . ')成功！') : $this->render_json(1, '操作失败！');
        }
    }

    /**
     * 招聘专题-参会企业-加入，获取企业信息
     */
    function audit_action(){
        $id =  intval($_POST['id']);//special_com.id

        $specialM =	$this->MODEL("special");
        $ComM     = $this -> MODEL('company');
        $userinfoM = $this->MODEL('userinfo');

        $specialCom	  =  $specialM->getSpecialComOne(array('id'=>$id),array('field'=>'id,uid,status,statusbody'));

        $Info   = $ComM->getInfo($specialCom['uid'], array('ywy' => 1));
        $member = $userinfoM->getInfo(array('uid' => $specialCom['uid']),array('field' => 'login_ip,reg_date'));
        //不返回多余数据
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
     * 招聘专题-参会企业-加入，获取招聘岗位
     */
    function comjob_action()
    {
        $uid = intval($_POST['uid']);
        if ($uid < 1) {
            $this->render_json(1, '参数有误');;
        }

        $pageM = $this->MODEL('page');
        $jobM = $this->MODEL('job');
        $jobwhere['uid'] = $uid;
        $jobwhere['state'] = 1;
        $jobwhere['r_status'] = 1;
        $jobwhere['status'] = 0;

        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company_job', $jobwhere, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];
        $total = $pages['total'];
        $list = array();

        if ($total > 0) {
            $jobwhere['orderby'] = array('lastupdate,desc');
            $jobwhere['limit'] = $pageM->pageLimit($_POST);
            $jobsA = $jobM->getList($jobwhere, array('isurl' => 'yes', 'utype' => 'admin'));//招聘中职位
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
     * 查看参会企业页面，取消/删除
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

            $this->admin_json(0, "公司申请(ID:" . $del . ")删除成功！");
        } else {
            $this->render_json(1, "请选择您要删除的信息！");
        }
    }

    /**
     * 删除
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

            $this->admin_json(0, "专题(ID:" . $del . ")删除成功！");
        } else {
            $this->render_json(1, "请选择您要删除的信息！");
        }
    }

    /**
     * 开启或关闭专题招聘
     */
    function recommend_action()
    {
        if ($_POST['type'] == "rec_display") {
            $specialM = $this->MODEL('special');
            $data['display'] = $_POST['rec'];
            $where['id'] = $_POST['id'];
            $nid = $specialM->upSpecial($where, $data);
            $msg = $_POST['rec'] == 1 ? '开启' : '关闭';
            if ($nid) {
                $this->admin_json(0, "专题招聘(ID:" . $_POST['id'] . ")" . $msg . "成功！");
            } else {
                $this->render_json(1, "专题招聘(ID:" . $_POST['id'] . ")" . $msg . "失败！");
            }
        }
    }

    /**
     * 参会企业 排序
     */
    function ajaxsort_action(){
        if ($_POST['id']) {
            $specialM = $this->MODEL('special');
            if (!empty($_POST['sort']) || $_POST['sort'] === '0') {
                $uparr['sort'] = intval($_POST['sort']);
            }
            $specialM->upSpecialCom(array('id' => $_POST['id']), $uparr);
            $this->admin_json(0, '参会企业(ID:' . $_POST['id'] . ' sort=' . intval($_POST['sort']) . ')排序修改成功');
        }
    }

    /**
     * 招聘专题 排序
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
                $this->admin_json(0, '专题招聘(ID：' . $post['id'] . ', sort=' . $post['sort'] . ')排序更新成功');
            } else {
                $this->render_json(1, '专题招聘排序更新失败');
            }
        }
    }

    /**
     * 添加参会企业，页面的搜索条件
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

        $timeSection = array(array('value' => '1', 'label' => '今天'), array('value' => '3', 'label' => '3天内'), array('value' => '7', 'label' => '7天内'), array('value' => '15', 'label' => '半月内'), array('value' => '30', 'label' => '1个月内'), array('value' => '31', 'label' => '1-3个月'), array('value' => '32', 'label' => '3-6个月'), array('value' => '33', 'label' => '6个月-1年'), array('value' => '34', 'label' => '1年以上'),);
        $status = array(array('value' => '1', 'label' => '已审核'), array('value' => '2', 'label' => '已锁定'), array('value' => '3', 'label' => '未通过'), array('value' => '4', 'label' => '未审核'), array('value' => '5', 'label' => '已暂停'),);
        $edtime = array(array('value' => '1', 'label' => '7天内'), array('value' => '2', 'label' => '一个月内'), array('value' => '3', 'label' => '半年内'), array('value' => '4', 'label' => '一年内'), array('value' => '5', 'label' => '已到期'),);
        $isrec = array(array('value' => '1', 'label' => '是'), array('value' => '2', 'label' => '否'), array('value' => '3', 'label' => '已到期'),);
        $isgw = array(array('value' => '1', 'label' => '已分配'), array('value' => '2', 'label' => '未分配'),);

        $result = array(
            'ratingList' => $ratingList,//会员等级
            'timeList' => $edtime,//到期时间
            'statusList' => $status,//审核状态
            'sourceList' => $sourceList,//数据来源
            'recList' => $isrec,//知名企业
            'gwList' => $isgw,//企业顾问
            'lotimeList' => $timeSection,//最近登录
            'adtimeList' => $timeSection,//最近注册
        );
        $this->render_json(0, '', $result);
    }

    /**
     * 按钮[添加参会企业]，获取参会企业
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
                //企业名称/简称
                $where['PHPYUNBTWSTART_C']   = '';
                $where['name']               = array('like',$keywordStr);
                $where['shortname']          = array('like',$keywordStr,'OR');
                $where['PHPYUNBTWEND_C']     = '';
            } elseif (!empty($keywordStr) && $typeStr == 2) {
                //用户名称
                $mwhere['username'] =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 3) {
                //联系人
                $where['linkman']   =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 4) {
                //联系电话
                $where['linktel']   =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 5) {
                //用户邮箱
                $where['linkmail']  =   array('like', $keywordStr);
            } else if (!empty($keywordStr) && $typeStr == 6) {
                //用户ID
                $where['uid'][]     =   array('=', $keywordStr);
            }
        }
        //审核状态
        if ($_POST['status']) {
            $status =   intval($_POST['status']);
            if ($status == 4) {
                //未审核
                $where['r_status']  =   0;
            } else if ($status == 5) {
                //已暂停
                $where['r_status']  =   4;
            } else {
                $where['r_status']  =   $status;
            }
        }
        //最近注册
        if ($_POST['adtime']) {
            $adtime = intval($_POST['adtime']);
            if ($adtime == 1) {
                //今天
                $mwhere['reg_date'] = array('>', strtotime('today'));
            } else if ($adtime < 31) {
                $mwhere['reg_date'] = array('>', strtotime('-' . $adtime . ' day'));
            } else if ($adtime == 31) {// 1 - 3 个月
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-1 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-3 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 32) {// 3 - 6个月
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-3 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-6 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 33) {// 6个月 - 1年
                $mwhere['PHPYUNBTWSTART_C'] = '';
                $mwhere['reg_date'][] = array('<', strtotime('-6 month'));
                $mwhere['reg_date'][] = array('>=', strtotime('-12 month'));
                $mwhere['PHPYUNBTWEND_C'] = '';
            } else if ($adtime == 34) {// 1年以上
                $mwhere['reg_date'] = array('<', strtotime('-1 year'));
            }
        }
        //最近登录
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
        //数据来源
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
        //会员等级
        if($_POST['rating']){
            $where['rating']   =   $_POST['rating'];
        }
        //到期时间
        $toDay	    =	strtotime(date('Y-m-d'));
        if($_POST['time']){
            $time   =   intval($_POST['time']);
            if($time == 5){
                //已到期
                $where['PHPYUNBTWSTART_A']    =   '';
                $where['vipetime'][]         =   array('>', '0','AND');
                $where['vipetime'][]         =   array('<',  $toDay,'AND');
                $where['PHPYUNBTWEND_A']      =   '';
            }else{
                if($time == 1){
                    //7天内
                    $num   =   '+7 day';
                }elseif($time == 2 ){
                    //一个月内
                    $num   =   '+1 month';
                }elseif($time == 3){
                    //半年内
                    $num   =   '+6 month';
                }elseif($time == 4){
                    //一年内
                    $num='+1 year';
                }

                $where['PHPYUNBTWSTART_B']    =   '';
                $where['vipetime'][]         =   array('>', time(),'AND');
                $where['vipetime'][]         =   array('<', strtotime($num),'AND');
                $where['PHPYUNBTWEND_B']      =   '';
            }
        }
        //知名企业
        if($_POST['rec']){
            $rec    =   intval($_POST['rec']);
            if($rec == 1){
                //是
                $where['rec']       =   '1';
                $where['hottime']   =   array('>', time());
            }elseif ($rec == 2){
                //否
                $where['rec']       =   '0';
            }elseif ($rec == 3){
                //已到期
                $where['rec']       =   '1';
                $where['hottime']   =   array('<', time());
            }
        }
        //企业顾问
        if($_POST['gw']){
            if(intval($_POST['gw']) == 1){
                //已分配
                $where['crm_uid']     =   array('<>', '0');
            }else{
                //未分配
                $where['crm_uid']     =   '0';
            }
        }
        //职位状况
        if ($_POST['job']) {
            $job = intval($_POST['job']);
            if (in_array($job, array(1, 2))) {
                $jobM = $this->MODEL('job');
                $jobwhere = array();
                $jobwhere['state'] = 1;//(审核)状态:0-未审核，1-已审核，2-已过期，3-未通过
                $jobwhere['r_status'] = 1;//2-锁定。企业会员被锁定，会同时锁定职位，4-企业暂停同时暂停职位,1-正常
                $jobwhere['status'] = 0;//(招聘)状态,1-已暂停（下架），0:招聘中（已上架）
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
                    //有职位
                    $where['uid'] = array('in', $jobsUids);
                } else {
                    //无职位
                    $where['uid'] = array('notin', $jobsUids);
                }
            }
        }
        //提取分页
        $page = $pageM->page($_POST);
        $pageSize = $pageM->limit($_POST);
        $pages = $pageM->adminPageList('company', $where, $page, array('limit' => $pageSize));
        $pageSizes = $pages['page_sizes'];

        $list = array();
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby']		=	$_POST['t'].','.$_POST['order'];
            }else if($_POST['time'] == '5'){
                //到期时间 已到期
                $where['orderby']		=	array('vipetime,desc','uid,desc');
            }else if($_POST['time']){
                //到期时间
                $where['orderby']		=	array('vipetime,asc');
            }else if($_POST['lotime']){
                //最近登录
                $where['orderby']		=	array('login_date,desc');
            }else{
                $where['orderby']		=	'uid,desc';
            }

            $where['limit']				=	$pages['limit'];;

            $ListNew    =   $ComM -> getList($where,array('utype'=>'admin'));

            // 查询本场网招参会企业，在待添加企业列表中展示，企业是否已参加
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
                    'jobnum' => $val['jobnum'],//职位数
                    'zz_jobnum' => $val['zz_jobnum'],//在招职位数
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
                    'login_date_n' => $val['login_date'] ? date('Y-m-d H:i', $val['login_date']) : '未登录',
                    'crm_uid' => $val['crm_uid'],
                    'crm_uid_n' => $val['crm_uid'] ? $val['crm_name'] : '未分配',
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

    // 添加参会企业保存
    function savespecial_action()
    {
        $SpecialM = $this->MODEL('special');

        $id     =   intval($_POST['sid']);
        $uid    =   intval($_POST['uid']);
        $isapply=   $SpecialM->getSpecialComNum(array("uid" => $uid, "sid" => $id));
        if ((int)$isapply > 0) {
            $this->render_json(1, '该企业已报名');
        }

        $nid    =   $SpecialM->addSpecialCom(array("sid" => $id, "uid" => $uid, 'sort' => 0, 'status' => '1', 'time' => time()));
        if (isset($nid)) {
            $this->admin_json(0, '专题招聘报名成功(专题ID：' . $id . '，企业ID：' . $uid . ')');
        } else {
            $this->render_json(2, '专题招聘报名失败');
        }
    }

    function mutiAddCom_action()
    {
        $specialM = $this->MODEL('special');
        $data['uid'] = $_POST['uid'];
        $data['sid'] = intval($_POST['sid']);
        $return = $specialM->addSpecialMutiCom($data);
        if ($return['error'] === 0) {
            $this->admin_json(0, '专题招聘报名成功(专题ID：' . $data['sid'] . '，企业ID：' . $data['uid'] . ')');
        } else {
            $this->render_json($return['error'], $return['msg']);
        }
    }

    //根据企业名称搜索企业列表
    function getcomlist_action(){

        $companyM  =  $this->MODEL('company');

        $comname   =  trim($_POST['comname']);

        $rows	=  $companyM -> getChCompanyList(array('name'=>array('like',$comname)));

        $html 	=  '<option value="">请选择</option>';

        foreach ($rows as $v){

            $html .= '<option value="'.$v['uid'].'">'.$v['name'].'</option>';
        }

        echo $html;
    }

    /**
     * 查看参会企业，设为名企 取消名企
     */
    function setFamous_action()
    {
        if ($_POST && $_POST['sid'] && $_POST['uid']) {
            $famous = $_POST['famous'] == 1 ? 0 : 1;

            $specialM = $this->MODEL('special');
            $specialInfo = $specialM->getSpecialOne(array('id' => $_POST['sid'], 'tpl' => 'gl.htm'));
            if (!$specialInfo) {
                $this->render_json(2, '名企设置失败');
            }

            $nid = $specialM->upSpecialCom(array('sid' => $_POST['sid'], 'uid' => $_POST['uid']), array('famous' => $famous));

            if ($nid) {
                $this->admin_json(0, '名企设置成功(sid=' . $_POST['sid'] . ',uid=' . $_POST['uid'] . ')');
            } else {
                $this->render_json(1, '名企设置失败');
            }
        }
    }
}