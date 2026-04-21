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
class company_order_controller extends adminCommon
{
    
    function index_action()
    {
        $OrderM = $this->MODEL('companyorder');
        $where = array();
        $keywordStr = trim($_POST['keyword']);
        if($_POST['comid']){
            $where['uid'] =	$_POST['comid'];
            $where['usertype'] = '2';
        }
        if($_POST['typezf']!=""){
            $where['order_type'] = $_POST['typezf'];
        }
        if($_POST['typedd']!=""){
            $where['type'] = $_POST['typedd'];
            if (isset($_POST['typedd']) && $_POST['typedd'] == 1){
                if (isset($_POST['rating']) && !empty($_POST['rating'])){
                    $where['rating'] = $_POST['rating'];
                }
            }
        }
        if($_POST['news_search']){
            if (!empty($keywordStr) && $_POST['typeca']=='1') {
                $where['order_id'] = array('like', $keywordStr);
            }elseif(!empty($keywordStr) && $_POST['typeca']=='2'){
                $UserinfoM = $this->MODEL('userinfo');
                $orderinfo = $UserinfoM->getList(array('username'=>array('like',$keywordStr)),array('field'=>'uid'));
                if (is_array($orderinfo)){
                    foreach ($orderinfo as $val){
                        $orderuids[] = $val['uid'];
                    }
                    $where['uid'] =	array('in', pylode(",",$orderuids));
                }
            }elseif(!empty($keywordStr) && $_POST['typeca']=='3'){
                $UserinfoM = $this->MODEL('userinfo');
                $mWhere = array(
                    '1' => array('name' => array('like', $_POST['keyword'])),
                    '2' => array('name' => array('like', $_POST['keyword'])),
                    '3' => array('realname' => array('like', $_POST['keyword'])),
                    '4' => array('name' => array('like', $_POST['keyword']))
                );
                $m_uids = $UserinfoM->getUidsByWhere($mWhere);
                $where['uid'] =	array('in', pylode(",",$m_uids));
            }
        }
        if(isset($_POST['time']) || $_POST['time_start1'] != "" || $_POST['time_end1'] != ""){
            $where['PHPYUNBTWSTART_B']      =   'AND';
            if($_POST['time']){
                if($_POST['time'] == 1){
                    $where['order_time'][]	=	array('>=',strtotime(date("Y-m-d 00:00:00")));
                }elseif ($_POST['time'] == 31){
                    $month = get_this_month(time());
                    $where['order_time'][] = array('>=',$month['start_month'],'and');
                    $where['order_time'][] = array('<=',$month['end_month'],'and');
                }else{
                    $where['order_time'][] = array('>=',strtotime('-'.intval($_POST['time']).' day'));
                }
            }
            if($_POST['time_start1']!=""){
                $where['order_time'][] = array('>=',strtotime($_POST['time_start1']." 00:00:00"));
            }
            if($_POST['time_end1']!=""){
                $where['order_time'][] = array('<',strtotime($_POST['time_end1']." 23:59:59"));
            }
            $where['PHPYUNBTWEND_B'] = '';
        }
        if(isset($_POST['order_state']) && $_POST['order_state']!=""){
            $where['order_state'] =	$_POST['order_state'];
        }
        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('company_order', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            if($_POST['order']){
                $where['orderby'] =	$_POST['t'].','.$_POST['order'];
            }else{
                $where['orderby'] =	array('id,desc');
            }
            $where['limit'] = $pages['limit'];
            $rows = $OrderM->getList($where,array('utype'=>'admin','field'=>'id,uid,order_id,order_price,type,rating,order_state,order_type,order_time,once_id,crm_uid,usertype,order_remark'));
            unset($where['limit']);
        }
        $rt['list'] = $rows ? $rows : array();
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }

    function edit_action()
    {
        $OrderM = $this->MODEL('companyorder');
        $row = $OrderM->getInfo(array('id' => $_POST['id']));
        $htpics = $OrderM->getOrderHtPicList(array('order_id' => $row['id']));
        $row['order_time_n'] = date('Y-m-d H:i', $row['order_time']);
        $rt['row'] = $row;
        $previewpics = array();
        foreach ($htpics as $v) {
            $previewpics[] = $v['pic_n'];
        }
        $rt['htpics'] = $htpics ? $htpics : array();
        $rt['preview_pics'] = $previewpics;
        $rt['integral_pricename'] = $this->config['integral_pricename'];
        $this->render_json(0, '', $rt);
    }

    function save_action()
    {
        $id = intval($_POST['id']);
        $OrderM = $this->MODEL('companyorder');
        $mData = array(
            'order_price' => $_POST['order_price'],
            'order_remark' => $_POST['order_remark']
        );
        $return = $OrderM->upInfo($id, $mData);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function setpay_action()
    {
        $id = intval($_POST['id']);
        $OrderM = $this->MODEL('companyorder');
        $return = $OrderM->setPay($id);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function del_action()
    {
        $OrderM = $this->MODEL('companyorder');
        $delID = is_array($_POST['del']) ? $_POST['del'] : $_POST['id'];
        $return = $OrderM->del($delID, array('utype' => 'admin'));
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    /*
     * 上传订单合同图片
     */
    function upload_action(){
        $orderM = $this->MODEL('companyorder');
        if($_GET['editid']){
            $editrow = $orderM->getOrderHtPicInfo(array('id'=>$_GET['editid']));
            $this->yunset("pic",substr($editrow['pic'],(strrpos($editrow['pic'],'/')+1)));
            $this->yunset("editrow",$editrow);
            $id	= $editrow['order_id'];
        }
        $row = $orderM->getInfo(array('id' => $_POST['id']));
        $rt['row'] = $row;
        $where['order_id'] = $_POST['id'];

        $page = !empty($_POST['page']) ? intval($_POST['page']) : 1;
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        //提取分页
        $pageM = $this->MODEL('page');
        $pages = $pageM->adminPageList('order_ht_pic', $where, $page, array('limit' => $pageSize));
        if($pages['total'] > 0){
            $where['limit']	= $pages['limit'];
            $rows = $orderM->getOrderHtPicList($where);
        }
        $rt['list'] = $rows ? $rows : array();
        $pics = array();
        foreach ($rows as $v) {
            $pics[] = $v['pic_n'];
        }
        $rt['pics'] = $pics;
        $rt['total'] = intval($pages['total']);
        $rt['perPage'] = $pageSize;
        $rt['pageSizes'] = $pages['page_sizes'];
        $this->render_json(0, '', $rt);
    }
    /*
     * 批量上传图片
     */
    function multiupload_action(){
        if($_FILES['file']['tmp_name']!=''){
            // pc端上传
            $upArr = array(
                'file' => $_FILES['file'],
                'dir' => 'order_ht'
            );
            $uploadM = $this->MODEL('upload');
            $pic = $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])){
                echo json_encode(array('error'=> 1, 'msg' => $pic['msg']));
            }elseif (!empty($pic['picurl'])){
                echo json_encode(array('error'=> 0, 'picurl' => $pic['picurl'], 'picurl_n' => checkpic($pic['picurl'])));
            }
        }
    }

    /*
     * 保存订单合同图片
     */
    function uploadsave_action(){
        $orderM = $this->MODEL('companyorder');
        $_POST = $this->post_trim($_POST);
        if($_POST['add']){
            $nbid = $orderM->addOrderHtPicInfo($_POST);
            if ($nbid) {
                $this->admin_json(0, "订单合同图片(ID:" . $nbid . ")添加成功！");
            } else {
                $this->render_json(1, '添加失败！');
            }
        }
    }
    /*
     * 删除订单合同图片
     */
    function htpic_del_action(){
        $ZphM =	$this->MODEL('companyorder');
        $id	= $_POST['delid'];
        if($id){
            $delid = $ZphM->delOrderHtPic(array('id' => $id));
            if ($delid) {
                $this->admin_json(0, "订单合同图片(ID:" . $id . ")删除成功！");
            } else {
                $this->render_json(1, '删除失败！');
            }
        }
    }
}