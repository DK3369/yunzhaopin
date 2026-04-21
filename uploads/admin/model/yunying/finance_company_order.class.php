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

class finance_company_order_controller extends adminCommon
{

	function searchType_action()
    {
		include (APP_PATH."/config/db.data.php");
		$data = array();
        $data['pay'] = $arr_data['pay'];
        $data['ordertype'] = $arr_data['ordertype'];

        $ratingM    =   $this->MODEL('rating');
        $rating     =   $ratingM->getList(array('category' => '1', 'orderby' => 'sort'), array('field' => '`id`,`name`'));
        if (!empty($rating)) {

            $ratingarr  =   array();
            foreach ($rating as $v) {
                $ratingarr[]    =  [
                    'value' => $v['id'],
                    'label' => $v['name'],
                ];
            }
        }
        $data['ratingarr'] = $ratingarr;
        $this->render_json('0','',$data);
	}

	function index_action()
    {

		$where							=   array();
	    $keywordStr						=   trim($_POST['keyword']);
		if($_POST['comid']){
			
			$where['uid']				=	$_POST['comid'];
			$where['usertype']			=	'2';
        }
		if($_POST['typezf']!=""){
			$where['order_type']		=	$_POST['typezf'];
        }
		if($_POST['typedd']!=""){

			$where['type']				=	$_POST['typedd'];
			if (isset($_POST['typedd']) && $_POST['typedd'] == 1){
			    if (isset($_POST['rating']) && !empty($_POST['rating'])){
			        $where['rating']    =   $_POST['rating'];
                }
            }
		}

        if (!empty($keywordStr) && $_POST['typeca']=='1') {
            $where['order_id']		=	array('like', $keywordStr);
        }elseif(!empty($keywordStr) && $_POST['typeca']=='2'){

            $UserinfoM				=	$this -> MODEL('userinfo');
            $orderinfo				=	$UserinfoM -> getList(array('username'=>array('like',$keywordStr)),array('field'=>'uid'));

            if (is_array($orderinfo)){
                foreach ($orderinfo as $val){
                    $orderuids[]	=	$val['uid'];
                }
                $where['uid']		=	array('in', pylode(",",$orderuids));
            }
        }elseif(!empty($keywordStr) && $_POST['typeca']=='3'){

            $UserinfoM				=	$this -> MODEL('userinfo');
            $mWhere                 =   array(
                '1' => array('name' =>  array('like', $_POST['keyword'])),
                '2' => array('name' =>  array('like', $_POST['keyword'])),
                '3' => array('realname' =>  array('like', $_POST['keyword'])),
                '4' => array('name' =>  array('like', $_POST['keyword']))
            );
            $m_uids                 =	$UserinfoM -> getUidsByWhere($mWhere);

            $where['uid']		    =	array('in', pylode(",",$m_uids));
        }

        if ($_POST['times'] != "" && is_array($_POST['times']) && count($_POST['times']) == 2) {

			$where['PHPYUNBTWSTART_B']      =   'AND';
            $time = $_POST['times'];
            $start = str_replace(array('T','Z'),' ',$time[0]);
            $end  = str_replace(array('T','Z'),' ',$time[1]);
			if($start!=""){
				$where['order_time'][]		=	array('>=',strtotime($start));
			}
			if($end!=""){
				$where['order_time'][]		=	array('<',strtotime($end." 23:59:59"));
			}
			$where['PHPYUNBTWEND_B']	    =	'';
		}
		if(isset($_POST['order_state']) && $_POST['order_state']!=""){

			$where['order_state']		=	$_POST['order_state'];
		}

        $page  = $_POST['page'];
        $pageSize = !empty($_POST['pageSize']) ? intval($_POST['pageSize']) : intval($this->config['sy_listnum']);
        $pageM	=	$this  -> MODEL('page');
        $pages	=	$pageM -> adminPageList('company_order',$where,$page,array('limit' => $pageSize));
        if(!$pages['total']) {
            $this->render_json(0,'暂无数据',['data'=>[],'total'=>0,'pageSizes'=>$pages['page_sizes']]);
        }
        //统计
        $MsgNum =   $this->MODEL('msgNum');
        $arr =  $MsgNum->orderSum($where);
        $orderSum = json_decode($arr,true);

        if($_POST['order']){
            $where['orderby']		=	$_POST['t'].','.$_POST['order'];
        }else{
            $where['orderby']		=	array('id,desc');
        }
        $where['limit']				=	$pages['limit'];
        $OrderM							=	$this -> MODEL('companyorder');
        $rows    					=   $OrderM -> getList($where,array('utype'=>'admin','field'=>'id,uid,order_id,order_price,type,rating,order_state,order_type,order_time,once_id,crm_uid,usertype'));
        foreach ($rows as &$v){
            $v['order_time'] = $v['order_time']?date('Y-m-d H:i:s',$v['order_time']):'';
        }
        unset($where['limit']);
        $_SESSION['orderXls'] = $where;
        $this->render_json(0,'',['data'=>$rows,'total'=>(int)$pages['total'],'pageSizes'=>$pages['page_sizes']]);

    }

    function edit_action()
    {

        $OrderM =   $this->MODEL('companyorder');
        $row    =   $OrderM->getInfo(array('id' => $_POST['id']));
        if (!$row){
            $this->render_json(1,'参数错误,请重试');
        }
        $htpics = $OrderM->getOrderHtPicList(array('order_id' => $row['id']));
        $row['order_time_ymd'] = $row['order_time']?date('Y-m-d H:i:s',$row['order_time']):'';
        $this->render_json(0,'',['detail'=>$row,'htpics'=>$htpics,'integral_pricename'=>$this->config['integral_pricename']]);
    }
    public function htpics_action(){

        if (!$_POST['id']){
            $this->render_json(1,'参数错误,请重试');
        }
        $OrderM =   $this->MODEL('companyorder');
        $row    =   $OrderM->getInfo(array('id' => $_POST['id']));
        if (!$row){
            $this->render_json(1,'参数错误,请重试');
        }
        $htpics = $OrderM->getOrderHtPicList(array('order_id' => $row['id']));
        $this->render_json(0,'',['htpics'=>$htpics]);
    }

    function save_action()
    {

        $id     =   intval($_POST['id']);
        if(!$id){
            $this->render_json(1,'参数错误');
        }
        $OrderM =   $this->MODEL('companyorder');

        $mData  =   array(
            'order_price'   =>  $_POST['order_price'],
            'order_remark'  =>  $_POST['order_remark']
        );
        $return =   $OrderM->upInfo($id, $mData);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
    }

    function setpay_action()
    {
        $id     =   intval($_POST['id']);
        if(!$id){
            $this->render_json(1,'参数错误');
        }
        $OrderM =   $this->MODEL('companyorder');

        $return =   $OrderM->setPay($id);
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
    }

    function xls_action()
    {
        $where  =   $_SESSION['orderXls'] ? $_SESSION['orderXls'] : array('orderby' => 'id');
        $OrderM =   $this->MODEL('companyorder');

        if ($_POST['uid']) {
            $where['id']    =   array('in', pylode(',', $_POST['uid']));
        }
        $time = $_POST['time'];
        if ($time) {
            $where['PHPYUNBTWSTART_B']  =   'AND';
            if ($time[0] != "") {
                $start = str_replace(array('T','Z'),' ',$time[0]);
                $where['order_time'][]    =   array('>=', strtotime($start));
            }
            if ($time[1] != "") {
                $end = str_replace(array('T','Z'),' ',$time[1]);
                $where['order_time'][]    =   array('<', strtotime($end." 23:59:59"));
            }
            $where['PHPYUNBTWEND_B']    =   '';
        }
        $rows   =   $OrderM->getList($where, array('utype' => 'admin', 'field' => 'id,uid,order_id,order_price,type,usertype,order_state,order_type,order_time,once_id,crm_uid'));
        if (!$rows){
            $this->render_json(1,'没有可以导出的订单信息！');
        }
        $result = $this->excel_export($rows);
        if (!$result['status']){
            $this->render_json(1,'参数错误,请重试');
        }
        $this->MODEL('log')->addAdminLog("导出支付订单信息");
        $this->admin_json(0,'',$result);
    }

    public function excel_export($rows){
        if (!$rows) {
            return ['status'=>0];
        }
        include_once LIB_PATH . 'libs/PHPExcel.php';
        $objPHPExcel = new PHPExcel();

        $objPHPExcel->setActiveSheetIndex(0);

        $col = 'A';
        $thead = ['编号', '用户名称', '公司名称（姓名）', '充值单号', '支付类型', '订单类型', '订单金额', '时间', '状态','业务员'];
        // 循环字段
        foreach ($thead as $tval) {
            $width = 20;
            $objPHPExcel->getActiveSheet()->getColumnDimension($col)->setWidth($width); // 设置列宽

            $objPHPExcel->getActiveSheet()->setCellValue($col . '1', $tval); // 设置表头
            $col++;
        }

        include (APP_PATH."/config/db.data.php");
        $ordertype = $arr_data['ordertype'];
        $col = 'A';
        $paystate = array ('支付失败','等待付款','支付成功','等待确认','交易关闭');

        foreach ($rows as $key => $val) {
            $No = $key + 2;
            if($val['type'] == 2) {
                $typeName =$this->config['integral_pricename']."充值";
            }else {
                $typeName = $ordertype[$val['type']];
            }
            $order_time_ymd  = $val['order_time']?date('Y-m-d H:i:s',$val['order_time']):'-';
//            iconv()
            $objPHPExcel->getActiveSheet()->setCellValue('A' . ($No), $val['id'])
                ->setCellValue('B' . ($No), $val['username'])
                ->setCellValue('C' . ($No), $val['comname'])
                ->setCellValueExplicit('D' . ($No), $val['order_id'],PHPExcel_Cell_DataType::TYPE_STRING)
                ->setCellValue('E' . ($No), $val['order_type_n'])
                ->setCellValue('F' . ($No), $typeName)
                ->setCellValue('G' . ($No), $val['order_price'])
                ->setCellValue('H' . ($No), $order_time_ymd)
                ->setCellValue('I' . ($No), $paystate[$val['order_state']])
                ->setCellValue('J' . ($No), $val['crm_name']);
            $col++;
        }
        $objWriter = PHPExcel_IOFactory::createWriter($objPHPExcel, 'Excel2007');
        ob_start();
        $objWriter->save('php://output');
        $xlsData = ob_get_contents();
        ob_end_clean();
        $data = [
            'file' => base64_encode($xlsData),
            'file_name' => '订单导出' . date('YmdHis') . '.xlsx',
            'status' => 1,
        ];
        return  $data;
    }

    function del_action()
    {
        $delID  =   is_array($_POST['del']) ? $_POST['del'] : $_POST['id'];

        if (!$delID){
            $this->render_json(1,'参数错误');
        }

        $OrderM =   $this->MODEL('companyorder');
        $return =   $OrderM->del($delID, array('utype' => 'admin'));
        if ($return['errcode']==9){
            $this->admin_json(0,$return['msg']);
        }else{
            $this->render_json(1,$return['msg']);
        }
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
        $orderM =	$this->MODEL('companyorder');
        $_POST = $this->post_trim($_POST);
        if (!$_POST['order_id']){
            $this->render_json('1','参数错误，请重试！');
        }

        $nbid = $orderM->addOrderHtPicInfo($_POST);
        if($nbid){
            $this->admin_json('0',"订单合同图片(ID:" . $nbid . ")添加成功！");
        }else{
            $this->admin_json('1',"添加失败！");
        }
    }
    /*
     * 删除订单合同图片
     */
    function htpic_del_action(){
        $ZphM =	$this->MODEL('companyorder');
        $id	= $_POST['delid'];
        if(!$id) {
            $this->render_json(1,'参数错误');
        }
        $delid = $ZphM->delOrderHtPic(array('id'=>$id));
        if ($delid){
            $this->admin_json('0',"订单合同图片(ID:".$_POST['delid'].")删除成功！");
        }else{
            $this->render_json('1','删除失败');
        }
    }
}
?>