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
class zhaopinhui_controller extends adminCommon
{
    function index_action(){
        if(trim($_POST['keyword'])){
            if ($_POST['type'] == '2'){
                $where['address'] =	array('like', trim($_POST['keyword']));
            }else{
                $where['title'] = array('like', trim($_POST['keyword']));
            }
        }
        if($_POST['status'] == '3'){
            $where['starttime'] = array('unixtime', '>', time());
        } elseif ($_POST['status'] == '1'){
            $where['starttime'] = array('unixtime', '<', time());
            $where['endtime'] =	array('unixtime', '>', time());
        }elseif($_POST['status'] == '2'){
            $where['endtime'] =	array('unixtime', '<', time());
        }
        //排序
        if($_POST['order']){
            $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
        }else{
            $where['orderby'] = 'id';
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('zhaopinhui', $where, $page, array('limit' => $pageSize));
        $ZphM = $this->MODEL('zph');
        if($pages['total'] > 0){
            $where['limit'] = $pages['limit'];
            $rows = $ZphM->getList($where, array('utype' => 'admin'));
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }

    /**
     * @return void
     */
    public function  getGroup_action(){

        $ZphM = $this->MODEL('zph');

        $space = $ZphM->getZphSpaceList(array('keyid' => '0'));
        $rt['preview_url'] = $this->config['sy_weburl'] . '/index.php?m=zph&c=show&id=';
        //提取分站内容
        $cacheM = $this->MODEL('cache');
        $domain = $cacheM->GetCache('domain');
        $rt['Dname'] = (object)$domain['Dname'];

        $rt['space'] = $space;
        $this->render_json(0, '', $rt);


    }

    function add_action(){
        //提取分站内容
        if ($_POST['submit']){
            $ZphM =	$this->MODEL('zph');
            $info = $this->post_trim($_POST);
            if (!$info['title']) {
                $this->render_json(1, '招聘会标题不能为空');
            }
            if($_FILES){
                $upData = [];
                foreach ($_FILES as $key => $value) {
                    if ($value['tmp_name']) {
                        $upData[$key]['file'] = $_FILES[$key];
                        $upData[$key]['dir'] = 'zhaopinhui';
                    }
                }
                $arr = [
                    'sl' => 'is_themb',
                    'hf' => 'banner',
                    'wapsl' => 'is_themb_wap',
                    'waphf' => 'banner_wap',
                ];
                if ($upData) {
                    $uploadM = $this->MODEL('upload');
                    foreach ($upData as $key => $value) {
                        $upRes = $uploadM->newUpload($value);
                        if ($upRes && !empty($upRes['msg'])) {
                            $return['msg'] = $upRes['msg'] ? $upRes['msg'] : $upRes['msg'];
                            $this->render_json(1, $return['msg']);
                        } else {
                            $info[$arr[$key]] = $upRes['picurl'];
                        }
                    }
                }
            }
            $info['body'] = str_replace("&amp;", "&", $info['body']);
            $info['media'] = str_replace("&amp;", "&", $info['media']);
            $info['packages'] = str_replace("&amp;", "&", $info['packages']);
            $info['booth'] = str_replace("&amp;", "&", $info['booth']);
            $info['participate'] =	str_replace("&amp;", "&", $info['participate']);
            if(strtotime($info['starttime'])>strtotime($info['endtime'])){
                $this->render_json(1, '开始时间不得大于结束时间');
            }
            $reserved_arr = json_decode(stripslashes($_POST['reserved_arr']), 1);
            $reserved_arr = $this->post_trim($reserved_arr);
            $reserved = array();
            foreach ($reserved_arr as $v) {
                $v[1] && $reserved[] = $v[1];
            }
            $info['reserved'] = pylode(',', $reserved);
            if($info['id']){
                $nbid =	$ZphM->upInfo(array('id' => intval($info['id'])), $info);
                if (isset($nbid)) {
                    $this->admin_json(0, "招聘会(ID:" . $info['id'] . ")修改成功！");
                } else {
                    $this->render_json(1, "招聘会(ID:" . $info['id'] . ")修改失败！");
                }
            }else{
                $nbid =	$ZphM->addInfo($info);
                if (isset($nbid)) {
                    $this->admin_json(0, "招聘会(ID:$nbid)添加成功！");
                } else {
                    $this->render_json(1, "招聘会(ID:$nbid)添加失败！");
                }
            }
        }else{
            $cacheM = $this->MODEL('cache');
            $domain = $cacheM->GetCache('domain');
            $rt['Dname'] = (object)$domain['Dname'];
            $ZphM =	$this->MODEL('zph');
            if(intval($_POST['id'])){
                $info =	$ZphM->getInfo(array('id' => intval($_POST['id'])), array('pic' => 1, 'banner' => 1));
            }
            $space = $ZphM->getZphSpaceList(array('keyid' => '0'));
            $rt['info'] = $info;
            $rt['space'] = $space;
            $this->render_json(0, '', $rt);
        }
    }
    // 招聘会添加、修改保存
    function save_action(){
        if (isset($_POST['add'])){
            // 添加打开弹窗请求，方便权限控制
            $this->render_json(0);
        }


    }

    function del_action(){
        $delID = $_POST['del'];
        $ZphM =	$this->MODEL('zph');
        $return	= $ZphM->delZph($delID);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function upload_action(){
        $ZphM = $this->MODEL('zph');
        if($_POST['editid']){
            $editrow = $ZphM->getZphPicInfo(array('id' => $_POST['editid']));
            $rt['pic'] = substr($editrow['pic'], (strrpos($editrow['pic'],'/')+1));
            $rt['editrow'] = $editrow;
        }
        $row = $ZphM->getInfo(array('id' => $_POST['id']));
        $rt['row'] = $row;
        $where['zid'] =	$_POST['id'];
        $rows = $ZphM->getZphPicList($where);
        $rt['list'] = $rows ? $rows : array();
        $pics = array();
        foreach ($rows as $v) {
            $pics[] = $v['pic_n'];
        }
        $rt['pics'] = $pics;
        $this->render_json(0,'ok', $rt);
    }

    function uploadsave_action(){
        $ZphM =	$this->MODEL('zph');
        $_POST = $this->post_trim($_POST);
        $id = $_POST['id'];
        if ($_FILES['file']['tmp_name']) {
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'zhaopinhui',
            );
            $uploadM = $this->MODEL('upload');
            $pic = $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])) {
                $this->render_json(1,$pic['msg']);
            } elseif (!empty($pic['picurl'])) {
                $_POST['pic'] = $pic['picurl'];
            }
        }
        if($id == ''){
            $row = $ZphM->getInfo(array('id' => intval($_POST['zph_id'])));
            $_POST['did'] = $row['did'];
            $_POST['zid'] = intval($_POST['zph_id']);
            $nbid = $ZphM->addZphPicInfo($_POST);
            if (isset($nbid)) {
                $this->admin_json(0, "招聘会图片(ID:" . $nbid . ")添加成功！");
            } else {
                $this->render_json(1, "添加失败！");
            }
        } else {
            $nbid =	$ZphM->upZphPicInfo(array('id' => $id), $_POST);
            if (isset($nbid)) {
                $this->admin_json(0, "招聘会图片(ID:" . $id . ")修改成功！");
            } else {
                $this->render_json(1, "修改失败！");
            }
        }
    }
    //设为缩略图
    function setthemb_action(){
        $ZphM =	$this->MODEL('zph');
        $ZphM->getSetThembInfo(array('id' => $_POST['id']));
        $this->render_json(0, '设置成功');
    }
    // 删除招聘会图片
    function delpic_action(){
        $ZphM =	$this->MODEL('zph');
        $id = $_POST['del'];
        if($id){
            $delid = $ZphM->delZphPic(array('id' => $id));
            if ($delid) {
                $this->admin_json(0, "招聘会图片(ID:" . $id . ")删除成功！");
            } else {
                $this->render_json(1, '删除失败！');
            }
        }
    }

    //设置高级搜索功能
    function set_searchs(){
        $search_list[] = array("param"=>"status","name"=>'审核状态',"value"=>array("3"=>"未审核","1"=>"已通过","2"=>"未通过"));
        $this->yunset("search_list",$search_list);
    }

    /*
     * 招聘会参会企业
     */
    function com_action(){
        $ZphM =	$this->MODEL('zph');
        if($_POST['id']){
            $where['zid'] =	intval($_POST['id']);
        }
        if($_POST['status']){
            if($_POST['status'] == "3"){
                $where['status'] = 0;
            }elseif($_POST['status'] == "1"){
                $where['status'] = 1;
            }elseif($_POST['status'] == "2"){
                $where['status'] = 2;
            }
        }
        if ($_POST['type']){
            if($_POST['type'] == 1){
                if (trim($_POST['keyword'])){
                    $zph = $ZphM->getList(array('title' => array('like', trim($_POST['keyword']))), array('field' => 'id'));
                    if($zph){
                        foreach($zph as $v){
                            $zid[] = $v['id'];
                        }
                        $where['zid'] =	array('in', pylode(',', $zid));
                    }
                }
            } elseif ($_POST['type'] == 2){
                if (trim($_POST['keyword'])){
                    $companyM = $this->MODEL('company');
                    $company = $companyM->getChCompanyList(array('name' => array('like', trim($_POST['keyword']))), array('field' => 'uid'));
                    if($company){
                        foreach($company as $v){
                            $uid[] = $v['uid'];
                        }
                        $where['uid'] =	array('in', pylode(',', $uid));
                    }
                }
            }
        }
        //排序
        if($_POST['order']){
            if($_POST['t'] == 'id') {
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            }elseif ($_POST['t'] == 'sid'){
                $where['orderby'] = array($_POST['t'] . ',' . $_POST['order'],'cid,' . $_POST['order'],'bid,' . $_POST['order']);
            }elseif($_POST['t'] == 'sort'){
                $where['orderby'] = $_POST['t'] . ',' . $_POST['order'];
            }
        }else{
            $where['orderby'] =	array('status,asc','id,desc');
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('zhaopinhui_com', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            $where['limit']	= $pages['limit'];
            $rows = $ZphM->getZphCompanyList($where);
            if($rows){
                $space = $ZphM->getZphSpaceList();
                $spacearr =	array();
                foreach($space as $val){
                    $spacearr[$val['id']] =	$val['name'];
                }
                foreach ($rows as $k => $v) {
                    $rows[$k]['space_n'] = $spacearr[$v['sid']] . ' - ' . $spacearr[$v['cid']] . ' - ' . $spacearr[$v['bid']];
                }
            }
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0,'ok', $rt);
    }

    function sbody_action(){
        $ZphM =	$this->MODEL('zph');
        $zhaopinhui_com	= $ZphM->getZphComInfo(array('id' => intval($_POST['id'])), array('field' => 'statusbody'));
        echo $zhaopinhui_com['statusbody'];die;
    }

    function status_action(){
        $ZphM =	$this->MODEL('zph');
        $data['status'] = $_POST['status'];
        $data['statusbody']	= trim($_POST['statusbody']);
        $id = @explode(",", $_POST['pid']);
        $nid = $ZphM->upZphCom($id, $data);
        if ($nid){
            $this->admin_json(0, "招聘会(ID:".$_POST['pid'].")设置成功！");
        }else{
            $this->render_json(1, "设置失败！");
        }
    }

    function audit_action(){
        $id = intval($_POST['id']);
        $ZphM =	$this->MODEL('zph');
        $ComM = $this->MODEL('company');
        $zphCom	= $ZphM->getZphComInfo(array('id' => $id),array('field' => 'id,uid,status,statusbody,jobid,zid'));
        if ($_POST['zph_info']) {
            $zph = $ZphM->getInfo(array('id' => $zphCom['zid']), array('field' => 'id,title'));
            $zphCom['title'] = $zph['title'];
        }
        $info = $ComM->getInfo($zphCom['uid'], array('jlist' => 1, 'ywy'=>1));
        $info['jobid_arr'] = $zphCom['jobid'] ? explode(',', $zphCom['jobid']) : array();
        foreach ($info['job_list'] as $k => $v) {
            if (!empty($info['jobid_arr'])) {
                if (in_array($v['id'], $info['jobid_arr'])) {
                    $info['job_list'][$k]['ch_n'] = '已参会';
                } else {
                    $info['job_list'][$k]['ch_n'] = '未参会';
                }
            } else {
                $info['job_list'][$k]['ch_n'] = '已参会';
            }
        }
        $info['zph'] = $zphCom;
        $this->render_json(0, '', $info);
    }
    /*
     * 参会企业导出校验
     */
    function comxlscheck_action(){
        $ZphM =	$this->MODEL('zph');
        if($_POST['zid']){
            if($_POST['cid']){
                $zcwhere = array('id' => array('in', $_POST['cid']));
            }else{
                $zcwhere = array('zid' => $_POST['zid']);
            }
            $rows =	$ZphM->getZphCompanyList($zcwhere);
            if ($rows) {
                $this->render_json(0, '');
            } else {
                $this->render_json(1, '没有可以导出的参会企业信息！');
            }
        }
    }

    private function getA_ZZ(){
        $letter = range('A', 'Z');
        $x = $letter;
        $y = $letter;
        foreach ($x as $xv) {
            foreach ($y as $yv) {
                $letter[] = $xv . $yv;
            }
        }
        return $letter;
    }
    private function getfiled_index($letters, $search){
        $rt = false;
        foreach ($letters as $k => $v) {
            if ($search == $v) {
                $rt = $k;
                break;
            }
        }
        return $rt;
    }

    /*
     * 参会企业导出
     */
    function comxls_action(){
        $letters = $this->getA_ZZ();
        $ZphM =	$this->MODEL('zph');
        $CompanyM =	$this->MODEL('company');
        $JobM =	$this->MODEL('job');
        if($_POST['zid']){
            $space = $ZphM->getZphSpaceList();
            $spacearr =	array();
            foreach($space as $val){
                $spacearr[$val['id']] =	$val['name'];
            }
            if($_POST['cid']){
                $zcwhere = array('id' => array('in', $_POST['cid']));
            }else{
                $zcwhere = array('zid' => $_POST['zid']);
            }
            $rows =	$ZphM->getZphCompanyList($zcwhere);
            if (!empty($rows)){
                $cacheM = $this->MODEL('cache');
                $cache = $cacheM->getCache(array('com'));
                $comclass_name = $cache['comclass_name'];
                foreach ($rows as $val){
                    $comids[] = $val['uid'];
                    $jobids[] = pylode(',', $val['jobid']);
                    $comSpace[$val['uid']] = array('sid' => $val['sid'], 'cid' => $val['cid'], 'bid' => $val['bid']);
                }
                $comField = '`uid`,`name`,`mun`,`content`,`address`,`linktel`,`linkman`,`linkphone`,`welfare`,`money`,`moneytype`';
                $companys = $CompanyM->getChCompanyList(array('uid' => array('in', @implode(',', $comids))),array('field' => $comField));
                $jobField = '`id`,`uid`,`name`,`number`,`minsalary`,`maxsalary`,`exp`,`edu`,`provinceid`,`cityid`,`three_cityid`,`description`,`zp_num`';
                $jobsA = $JobM->getList(array('id' => array('in', @implode(',', $jobids)),'uid' => array('in', @implode(',', $comids)), 'state' => 1, 'status' => 0, 'r_status' => 1), array('field' => $jobField));
                $jobs = $jobsA['list'];
                foreach($companys as $k=>$va){
                    $companys[$k]['content'] = trim(strip_tags($va['content']));
                    $companys[$k]['mun'] = $comclass_name[$va['mun']];
                    $companys[$k]['space'] = $comSpace[$va['uid']];
                    foreach($jobs as $val){
                        if ($va['uid'] == $val['uid']){
                            $companys[$k]['jobs'][] = $val;
                        }
                    }
                }
                $maxJobNum = 1;
                foreach ($companys as $v){
                    $jobnum = is_array($v['jobs']) ? count($v['jobs']) : 0;
                    if ($jobnum > $maxJobNum){
                        $maxJobNum = $jobnum;
                    }
                }
                $jobheader1 = array();
                for($i=1;$i<=$maxJobNum;$i++){
                    $jobheader1[] = '岗位' . $i;
                }
                $comfieldsList = [
                    'space' => '展位号座位',
                    'name' => '公司名称',
                    'address' => '地址',
                    'linkphone' => '联系电话',
                    'linkman' => '联系人',
                    'linktel' => '联系手机',
                    'content' => '公司简介',
                    'welfare' => '福利待遇',
                    'mun' => '公司规模',
                    'money' => '注册资金',
                ];
                $jobfieldsList = [
                    'name' => '招聘岗位',
                    'job_number' => '招聘人数',
                    'job_salary' => '薪酬',
                    'job_exp' => '经验要求',
                    'job_edu' => '学历要求',
                    'address' => '工作地点'
                ];
                include_once LIB_PATH . 'libs/PHPExcel.php';
                $objPHPExcel = new PHPExcel();
                $objPHPExcel->setActiveSheetIndex(0);
                $width = 20;
                $objPHPExcel->getActiveSheet()->getColumnDimension('A')->setWidth($width); // 设置列宽
                $objPHPExcel->getActiveSheet()->setCellValue('A1', '公司信息'); // 设置表头
                $objPHPExcel->getActiveSheet()->mergeCells('A1:J1'); // 表头列合并
                $colindex = $this->getfiled_index($letters, 'A');
                // 循环字段
                foreach ($comfieldsList as $com_v) {
                    $objPHPExcel->getActiveSheet()->getColumnDimension($letters[$colindex])->setWidth($width); // 设置列宽
                    $objPHPExcel->getActiveSheet()->setCellValue($letters[$colindex] . '2', $com_v); // 设置表头
                    $colindex++;
                }
                $colindex = $this->getfiled_index($letters, 'K');
                foreach ($jobheader1 as $hv) {
                    $objPHPExcel->getActiveSheet()->getColumnDimension($letters[$colindex])->setWidth($width); // 设置列宽
                    $objPHPExcel->getActiveSheet()->setCellValue($letters[$colindex] . '1', $hv); // 设置表头
                    // 表头列合并
                    $objPHPExcel->getActiveSheet()->mergeCells($letters[$colindex] .'1:' . $letters[$colindex + 5] . '1');
                    foreach ($jobfieldsList as $job_v) {
                        $objPHPExcel->getActiveSheet()->getColumnDimension($letters[$colindex])->setWidth($width); // 设置列宽
                        $objPHPExcel->getActiveSheet()->setCellValue($letters[$colindex] . '2', $job_v); // 设置表头
                        $colindex++;
                    }
                }
                // 头部样式设置
                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,1, 10 + $maxJobNum * 6, 1)->getFont()->setBold(true);
                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,1, 10 + $maxJobNum * 6, 1)->getAlignment()->setHorizontal(PHPExcel_Style_Alignment::HORIZONTAL_CENTER);
                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,1, 10 + $maxJobNum * 6, 1)->getAlignment()->setVertical(PHPExcel_Style_Alignment::VERTICAL_CENTER);

                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,2, 10 + $maxJobNum * 6, 2)->getFont()->setBold(true);
                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,2, 10 + $maxJobNum * 6, 2)->getAlignment()->setHorizontal(PHPExcel_Style_Alignment::HORIZONTAL_CENTER);
                $objPHPExcel->getActiveSheet()->getStyleByColumnAndRow(0,2, 10 + $maxJobNum * 6, 2)->getAlignment()->setVertical(PHPExcel_Style_Alignment::VERTICAL_CENTER);

                foreach ($companys as $key => $val) {
                    $colindex = $this->getfiled_index($letters, 'A');
                    // 循环企业信息字段
                    foreach ($comfieldsList as $com_k => $com_v) {
                        if ($com_k == 'space') {
                            $text = $spacearr[$val['space']['sid']] . '-' . $spacearr[$val['space']['cid']] . '-' . $spacearr[$val['space']['bid']];
                        } else if ($com_k == 'money') {
                            if ($val['money'] > 0) {
                                $text = $val['money'];
                                if ($val['moneytype'] == 1) {
                                    $text .= '万元';
                                } else if ($val['moneytype'] == 2) {
                                    $text .= '万美元';
                                }
                            } else {
                                $text = '';
                            }
                        } else if ($com_k == 'content') {
                            $text = mb_substr($val['content'], 0, 50);
                        } else {
                            $text = $val[$com_k] ? $val[$com_k] : '';
                        }
                        if (in_array($com_k, array('linkphone', 'linktel'))) { // 数字转字符的字段
                            $objPHPExcel->getActiveSheet()->setCellValueExplicit($letters[$colindex] . ($key + 3), $text, PHPExcel_Cell_DataType::TYPE_STRING); // 转类型
                        } else {
                            $objPHPExcel->getActiveSheet()->setCellValue($letters[$colindex] . ($key + 3), $text); // 动态表格内容
                        }
                        $colindex++;
                    }
                    $colindex = $this->getfiled_index($letters, 'K');
                    foreach ($val['jobs'] as $job) {
                        // 循环职位信息字段
                        foreach ($jobfieldsList as $job_k => $job_v) {
                            if ($job_k == 'address') {
                                $text = $job['job_city_one'] . '-' . $job['job_city_two'];
                                if ($job['job_city_three']) {
                                    $text .=  '-' . $job['job_city_three'];
                                }
                            } else {
                                $text = $job[$job_k] ? $job[$job_k] : '';
                            }
                            if (in_array($job_k, array('job_number'))) { // 数字转字符的字段
                                $objPHPExcel->getActiveSheet()->setCellValueExplicit($letters[$colindex] . ($key + 3), $text, PHPExcel_Cell_DataType::TYPE_STRING); // 转类型
                            } else {
                                $objPHPExcel->getActiveSheet()->setCellValue($letters[$colindex] . ($key + 3), $text); // 动态表格内容
                            }
                            $colindex++;
                        }
                    }
                }
                // 设置边框
                $styleArray = array(
                    'borders' => array(
                        'allborders' => array(
                            'style' => PHPExcel_Style_Border::BORDER_THIN,//细边框
                        ),
                    ),
                );
                $objPHPExcel->getActiveSheet()->getStyle('A1:' . $letters[9 + $maxJobNum * 6] .(count($companys) + 2))->applyFromArray($styleArray);
                $objWriter = PHPExcel_IOFactory::createWriter($objPHPExcel, 'Excel2007');
                ob_start();
                $objWriter->save('php://output');
                $xlsData = ob_get_contents();
                ob_end_clean();
                $data = [
                    'file' => base64_encode($xlsData),
                    'file_name' => '招聘会参会企业信息' . date('YmdHis') . '.xlsx'
                ];
                return $this->admin_json(0, "导出公司信息", $data);
            } else {
                $this->render_json(1, '暂无符合条件的企业数据');
            }
        }
    }
    //删除链接
    function delcom_action(){
        $ZphM =	$this->MODEL('zph');
        $delID = $_POST['del'];
        $list =	$ZphM->getZphCompanyList(array('status' => '0', 'price' => array('>', '0'), 'id' => array('in', $delID)));
        if(is_array($list)){
            foreach($list as $v){
                $IntegralM = $this->MODEL('integral');
                $IntegralM->company_invtal($v['uid'], 2, $v['price'], true, "删除招聘会", true, 2, 'integral');//积分操作记录
            }
        }
        $arr = $ZphM->delZphCom($delID, array('utype' => 'admin'));
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    function checksitedid_action(){
        if($_POST['uid']){
            $uids =	@explode(',', $_POST['uid']);
            $uid = pylode(',', $uids);
            if($uid){
                $siteDomain = $this->MODEL('site');
                $siteDomain->updDid(array('zhaopinhui'), array('id' => array('in', $uid)), array('did' => $_POST['did']));
                $siteDomain->updDid(array('zhaopinhui_com','zhaopinhui_pic'), array('zid' => array('in', $uid)), array('did' => $_POST['did']));
                $this->admin_json(0, '招聘会(ID:'.$_POST['uid'].')分配站点成功！');
            }else{
                $this->render_json(1, '请正确选择需分配数据！');
            }
        }else{
            $this->render_json(1, '参数不全请重试！');
        }
    }

    //获取展位
    function getzhanwei_action(){
        $ZphM = $this->MODEL('zph');
        if($_POST['sid']){
            $com_bid = $ZphM->getZphCompanyList(array('zid' => intval($_POST['zid'])));
            foreach($com_bid as $v){
                $com_bid[] = $v['bid'];
            }
            $linkarr = $ZphM->getInfo(array('id'=>intval($_POST['zid'])));
            $reserved =	@explode(',',$linkarr['reserved']);
            $third = $ZphM->getZphSpaceList(array('id' => array('in', pylode(',', $reserved))));
            $parendids = array();
            foreach ($third as $v) {
                $parendids[$v['id']] = $v['keyid'];
            }
            $reserved_arr = array();
            foreach ($reserved as $v) {
                $reserved_arr[] = array($parendids[$v], $v);
            }
            $list = $ZphM->getZphSpaceList(array('keyid' => intval($_POST['sid'])));

            foreach ($list as $v) {
                $rt[] = array('value' => $v['id'], 'label' => $v['name']);
            }
            if($rt){
                foreach($rt as $k=>$v){
                    $rows =	$ZphM->getZphSpaceList(array('keyid' => $v['value'], 'orderby' => 'sort,asc'));
                    $space = array();
                    foreach($rows as $key => $val){
                        $space[$key]['value'] = $val['id'];
                        $space[$key]['label'] = $val['name'];
                        if (in_array($val['id'], $com_bid)) {
                            $space[$key]['disabled'] = true;
                        }
                    }
                    $rt[$k]['children'] = !empty($space) ? $space : array();
                }
            }
            $this->render_json(0, '', array('reserved_arr' => $reserved_arr, 'space' => $rt));
        }
    }
    //修改展位
    function upzhanwei_action(){
        if ($_POST['zcomid']){
            $value = array(
                'cid' => $_POST['cid'],
                'bid' => $_POST['bid']
            );
            $zphM = $this->MODEL('zph');
            $nid = $zphM->upZphComSort($_POST['zcomid'],$value);
            if ($nid) {
                $this->render_json(0, '修改成功');
            } else {
                $this->render_json(0, '修改失败');
            }
        }else{
            $this->render_json(1, '非法访问');
        }
    }
    //添加参会企业
    function comadd_action(){
        $ZphM =	$this->MODEL('zph');
        $zph = $ZphM->getInfo(array('id' => intval($_POST['id'])));
        $keyid = $zph['sid'];//场地id
        $space = $ZphM->getZphSpaceList(array('keyid' => $keyid, 'orderby' => 'sort,asc'),array( 'id' => $_POST['id'], 'utype' => 'index'));
        $this->render_json(0, '', array('spacelist' => $space));
    }

    //获取招聘会展位
    function getspacelist_action(){
        $ZphM = $this->MODEL('zph');
        $cid = intval($_POST['cid']);
        $zphid = intval($_POST['zphid']);
        $space_bid = $ZphM->getZphSpaceList(array('keyid' => $cid, 'orderby' => 'sort,asc'));
        $bids =	array();
        $spacename = array();
        foreach ($space_bid as $v){
            $bids[]	= $v['id'];
            $spacename[$v['id']] = $v['name'];
        }
        $zphcom = $ZphM->getZphCompanyList(array('zid' => $zphid, 'cid' => $cid));
        $combids = array();
        foreach ($zphcom as $v){
            $combids[] = $v['bid'];
        }
        // 取全部展位和已被报名展位的差集为可选展位，没有展位的进行提示
        $rows = array_diff($bids, $combids);
        if (!empty($rows)){
            $html = '<option value="">请选择</option>';
            foreach ($rows as $v){
                $html .= '<option value="'.$v.'">'.$spacename[$v].'</option>';
            }
            echo $html;
        }else{
            echo 1;
        }
    }

    //根据企业名称搜索企业列表
    function getcomlist_action(){
        $companyM = $this->MODEL('company');
        $comname = trim($_POST['comname']);
        $rows = $companyM->getChCompanyList(array('name' => array('like', $comname)));
        $arr = array();
        foreach ($rows as $v) {
            $arr[] = array('label' => $v['name'], 'value' => $v['uid']);
        }
        $this->render_json(0, '', $arr);
    }

    //后台修改参会企业职位
    function upjob_action(){
        if ($_POST['zphjob']){
            $value = array(
                'jobid'=> $_POST['zphjob']
            );
            $zphM = $this->MODEL('zph');
            $nid = $zphM->upZphComSort($_POST['zcomid'], $value);
            if ($nid) {
                $this->admin_json(0, "招聘会企业职位修改成功！");
            } else {
                $this->render_json(1, "招聘会企业职位修改失败！");
            }
        } else {
            $this->render_json(1, "请选择参会职位！");
        }
    }

    //根据选择的企业展示前台可以显示的职位列表
    function getjoblist_action(){
        $JobM =	$this->MODEL('job');
        $comid = intval($_POST['comid']);
        $type = $_POST['type'];
        $arr = array();
        if($type=='admin'){
            $zphM = $this->MODEL('zph');

            $rows = $JobM->getList(array('uid' => $comid, 'state' => '1', 'r_status' => array('<>', '2'), 'status' => array('<>', '1')));

        }else {
            $rows = $JobM->getList(array('uid' => $comid, 'state' => '1', 'r_status' => array('<>', '2'), 'status' => array('<>', '1')));

        }
        foreach ($rows['list'] as $r) {
            $arr[] = array('value' => $r['id'], 'label' => $r['name']);
        }
        $this->render_json(0, '', $arr);
    }

    //保存参会企业
    function comaddsave_action(){
        $ZphM =	$this->MODEL('zph');
        $zphcom	= $ZphM->getZphComInfo(array('uid' => intval($_POST['comid']),'zid' => intval($_POST['zphid'])));
        if ($zphcom){
            $this->render_json(1, "该企业已参加本次招聘会！");
        }else{
            $_POST['uid'] =	intval($_POST['comid']);
            $_POST['zid'] =	intval($_POST['zphid']);
            $_POST['sid'] =	intval($_POST['zphsid']);
            $_POST['cid'] =	intval($_POST['cid']);
            $_POST['bid'] =	intval($_POST['bid']);
            $nid = $ZphM->addZCom($_POST);
            if ($nid) {
                $this->admin_json(0, "招聘会企业(ID:".$nid.")添加成功！");
            } else {
                $this->render_json(1, "设置失败！");
            }
        }
    }
    //招聘会前台显示情况修改
    function upisopen_action(){
        if(!$_POST['pid']) {
            $this->render_json(1, "参数错误,请重试!");
        }

        $ZphM = $this->MODEL('zph');
        $return = $ZphM->upIsOpen($_POST['pid'], $_POST['is_open']);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }

    }
    // 参会企业排序修改
    function ajaxsort_action(){
        if($_POST['id']){
            $ZphM =	$this->MODEL('zph');
            if (!empty($_POST['sort'])){
                $uparr['sort'] = $_POST['sort'];
            }
            $ZphM->upZphComSort($_POST['id'],$uparr);
            $this->render_json(0, '修改成功');
        }
    }
}