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
class hotjob_controller extends adminCommon
{
    function set_search(){
        $ratingM = $this->MODEL('rating');
        $rating = $ratingM->getList(array( 'category' => '1', 'orderby' => 'sort,asc' ), array( 'field' => '`id`,`name`' ));
        if (! empty($rating)) {
            foreach ($rating as $k => $v) {
                $ratingarr[$v['id']] = $v['name'];
            }
        }
        $edtime = array(
            '1' => '7天内',
            '2' => '一个月内',
            '3' => '半年内',
            '4' => '一年内',
            '5' => '已到期'
        );
        $search_list['rating'] = array(
            "name" => '会员等级',
            "value" => $ratingarr
        );
        $search_list['time'] = array(
            "name" => '到期时间',
            "value" => $edtime
        );
        return array('search_list' => $search_list);
    }

    function index_action(){
        $ComM = $this->MODEL('company');
        $typeStr = intval($_POST['ctype']);
        $keywordStr = trim($_POST['keyword']);
        if (!empty($keywordStr)){
            if($typeStr == 1){
                $where['username'] = array('like', $keywordStr);
            }else if($typeStr == 2){
                $where['beizhu'] = array('like', $keywordStr);
            }
        }
        if(isset($_POST['rating']) && $_POST['rating']){
            // gengzs  2023/10/20 改版  将之前获取rating_id 从 企业表中去查询 获取uid
            $companyM		=   $this->MODEL('company');
            $companyW['rating']		=	$_POST['rating'];
            $companyW['groupby']	=	'uid';
            $all		=	$companyM ->getChCompanyList($companyW,array('field'=>'`uid`'));
            $uidArr = [];
            if ($all){
                foreach ($all as $value){
                    $uidArr[] = $value['uid'];
                }
                $where['uid'] = array('in', pylode(',', $uidArr));
            }
        }
        if (isset($_POST['rating_name']) && $_POST['rating_name']){
            $where['rating'] = $_POST['rating_name'];
        }
        if($_POST['time']){
            if ($_POST['time'] == '1') {
                $num = "+7 day"; // 7天
            } elseif ($_POST['time'] == '2') {
                $num = "+1 month"; // 一个月
            } elseif ($_POST['time'] == '3') {
                $num = "+6 month"; // 半年
            } elseif ($_POST['time'] == '4') {
                $num = "+1 year"; // 一年
            }
            if($_POST['time']=='5'){
                $where['time_end'] = array('<', time());
            }else{
                $where['PHPYUNBTWSTART'] = '';
                $where['time_end'][] = array('>', time());
                $where['time_end'][] = array('<', strtotime($num));
                $where['PHPYUNBTWEND'] = '';
            }
        }
        //提取分页
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('hotjob', $where, $page, array('limit' => $pageSize));
        //分页数大于0的情况下 执行列表查询
        if($pages['total'] > 0){
            //limit order 只有在列表查询时才需要
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby'] =	array('time_start,desc');
            }
            $where['limit']	= $pages['limit'];
            $rows = $ComM->getHotJobList($where, array('utype'=>'admin'));
        }

        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);

    }
    /**
     *users:王旭
     *Data:2023/10/17
     *Time:10:25
     * 搜索数据信息
     */
    public function  getSearchData_action(){
        $sres = $this->set_search();
        $search_list = $sres['search_list'];
        $this->render_json(0,'',$search_list);
    }
    /*
     * 根据企业名称查询企业
     */
    function getComList_action()
    {
        $comM = $this->MODEL('company');
        $name = trim($_POST['name']);
        $rows = $comM->getChCompanyList(array('name' => array('like', $name)));
        $rt = array();
        foreach ($rows as $v) {
            $rt[] = array('value' => $v['uid'], 'label' => $v['name']);
        }
        $this->render_json(0, '', $rt);
    }
    /*
     * 获取企业名企信息
     */
    function gethotjob_action(){
        $ComM = $this->MODEL('company');
        $uid = trim($_POST['uid']);
        $company = $ComM->getInfo($uid, array('logo' => 1, 'utype' => 'admin', 'field' => '`name` as `username`,`uid`,`logo`,`rec`'));
        if ($company['rec'] == 1) {// 当前已是名企
            $hotjob = $ComM->getHotJob(array('uid' => $uid));
        } else {// 非名企
            $statisM = $this->MODEL('statis');
            $statis = $statisM->getInfo($uid, array('usertype' => '2', 'field' => '`rating` as `rating_id`, `rating_name` as `rating`'));
            $company['hot_pic_n'] = checkpic($company['logo']);
            unset($company['logo']);
            $hotjob = array_merge($statis, $company);
            $hotjob['time_start'] = time();
            $hotjob['time_start_n'] = date('Y-m-d', time());
        }
        $this->render_json(0, '', $hotjob);
    }

    /**
     * @desc 后台  企业  名企设置/修改弹出
     */
    function hotjobinfo_action(){
        $ComM = $this->MODEL('company');
        $id = intval($_POST['id']);
        $uid = intval($_POST['uid']);
        if(!empty($id)){
            $hotjob = $ComM->getHotJob(array('uid' => $id));
        }else if(!empty($uid)){
            $statisM = $this->MODEL('statis');
            $statis = $statisM->getInfo($uid, array('usertype'=>'2','field'=>'`rating` as `rating_id`,`rating_name` as `rating`'));
            $company = $ComM->getInfo($uid,array('logo'=>1,'utype'=>'admin','field'=>'`name` as `username`, `uid`,`logo`'));
            $company['hot_pic_n'] = $company['logo'];
            unset($company['logo']);
            $hotjob = array_merge($statis, $company);
            $hotjob['time_start'] = time();
        }
        $hotjob['time_start_n'] = date('Y-m-d', $hotjob['time_start']);
        $hotjob['time_end_n'] = $hotjob['time_end'] ? date('Y-m-d', $hotjob['time_end']) : '';
        $this->render_json(0, '', $hotjob);
    }

    /**
     * @desc 取消（删除）名企招聘
     */
    function del_action(){
        $uid = $_POST['del'];
        $ComM = $this->MODEL('company');
        $return = $ComM->delHotJob($uid);
        if ($return) {
            $this->admin_json(0, '名企(ID:'.pylode(',', $uid).')删除成功！');
        } else {
            $this->render_json(1, '删除失败！');
        }
    }

    /**
     * @desc 设置/修改名企
     */
    function save_action(){
        $info = $_POST;
        // 新上传图片文件处理
        foreach ($_FILES['mqlogo'] as $nk => $nv) {
            foreach ($nv as $nkk => $nvv) {
                $mqlogo_file[$nkk][$nk] = $nvv;
            }
        }
        if($_FILES['mqlogo']['tmp_name']!=""){
            $upArr = array(
                'file' => $mqlogo_file[0],
                'dir' => 'hotpic'
            );
            $uploadM = $this->MODEL('upload');
            $picr = $uploadM->newUpload($upArr);
            if (!empty($picr['msg'])){
                $this->render_json(1, $picr['msg']);
            }elseif (!empty($picr['picurl'])){
                $pic = $picr['picurl'];
            }
        }
        $ComM = $this->MODEL('company');
        $hotJob = $ComM->getHotJob(array('uid' => $info['uid']));
        if(!$hotJob){
            $com = $ComM->getInfo(intval($info['uid']),array('field'=>'`logo`,`did`','logo'=>1));
            $hotJob['hot_pic'] = $com['logo_n'];
            $hotJob['did'] = $com['did'];
        }
        $value = array(
            'uid'             =>  intval($info['uid']),
            'username'        =>  trim($info['username']),
            'rating'          =>  trim($info['rating']),
            'hot_pic'         =>  trim($pic)?trim($pic):$hotJob['hot_pic'],
            'service_price'   =>  intval($info['service_price']),
            'time_start'      =>  strtotime($info['time_start_n']),
            'time_end'        =>  strtotime($info['time_end_n']),
            'sort'            =>  intval($info['sort']),
            'beizhu'          =>  trim($info['beizhu']),
            'rating_id'       =>  intval($info['rating_id'])
        );
        if(!$info['id']){
            if (empty($hotJob['hot_pic']) && $_FILES['mqlogo']['tmp_name']==''){
                $this->render_json(1, '请上传企业展示LOGO');
            }
            $value['did'] = intval($hotJob['did']);
            $arr = $ComM->addHotJob($value);
        }else{
            $arr = $ComM->upHotJob($info['uid'], $value);
        }
        if ($arr['errcode'] == 9) {
            $this->admin_json(0, $arr['msg']);
        } else {
            $this->render_json(1, $arr['msg']);
        }
    }

    function hotNum_action(){
        $MsgNum = $this->MODEL('msgNum');
        $rt = json_decode($MsgNum->hotNum(), 1);
        $this->render_json(0, '', $rt);
    }
}