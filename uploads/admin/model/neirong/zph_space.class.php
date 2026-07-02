<?php

class zph_space_controller extends adminCommon
{
    function index_action(){
        $ZphM = $this->MODEL('zph');
        $where = array('keyid' => '0', 'orderby' => 'sort,asc');
        if (trim($_POST['keyword'])) {
            $where['name'] = array('like', trim($_POST['keyword']));
        }
        $position =	$ZphM->getZphSpaceList($where, array('utype' => 'admin'));
        $picarr = array();
        foreach ($position as $v) {
            if ($v['pic_n']) {
                $picarr[] = $v['pic_n'];
            }
        }
        $this->render_json(0, '', array('list' => $position, 'pics' => $picarr));
    }

    function add_action(){

        if (isset($_POST['add'])){
            // Open the add dialog for permission checks.
            $this->render_json(0);
        }
        $ZphM =	$this->MODEL('zph');
        $info = $this->post_trim($_POST);
        if (!$info['name']) {
            $this->render_json(1, yun_at('admin_01357'));
        }
        if($_FILES){
            // PC upload.
            $upArr = array(
                'file' => $_FILES['pic'],
                'dir' => 'zhaopinhui'
            );
            $uploadM = $this->MODEL('upload');
            $pic = $uploadM->newUpload($upArr);
            if (!empty($pic['msg'])){
                $this->render_json(1, $pic['msg']);
            }elseif (!empty($pic['picurl'])){
                $data['pic'] = $pic['picurl'];
            }
        }
        if($info['keyid'] != ''){
            $data['keyid'] = $info['keyid'];
            $data['price'] = $info['price'];
        }
        if (!empty($info['id'])){
            $data['name'] = $info['name'];
        } else {
            $position = str_replace('，', ',', trim($info['name']));
            $data['name'] = explode(',', $position);
        }
        $data['sort'] =	$info['sort'];
        $data['content'] = str_replace("&amp;","&", $info['content']);
        if($info['id']){
            $nid = $ZphM->upZphSpaceInfo(array('id' => $info['id']), $data);
            $msg = 'wap_00225';
        }else{
            $nid = $ZphM->addZphSpaceInfo($data);
            $msg = 'wap_js_00091';
        }
        if ($nid) {
            $this->admin_json(0, $msg . 'wap_js_00104');
        } else {
            $this->render_json(1, $msg . 'wap_js_00103');
        }
    }

    // Get child categories.
    function ajaxspace_action(){
        $ZphM =	$this->MODEL('zph');
        $id = intval($_POST['id']);
        if($id != ""){
            $jobs =	$ZphM->getZphSpaceList(array('keyid' => $id));
            $this->render_json(0, '', $jobs);
        }
    }

    function up_action(){
        // Query child categories.
        $ZphM=$this->MODEL('zph');
        if((int)$_POST['id']){
            $id	= (int)$_POST['id'];
            $onejob	= $ZphM->getZphSpaceInfo(array('id' => $_POST['id']));
            $twojob	= $ZphM->getZphSpaceList(array('keyid' => $_POST['id'], 'orderby' => 'sort,asc'));
            if(is_array($twojob)){
                foreach($twojob as $key => $v){
                    $val[] = $v['id'];
                    $root_arr = @implode(",",$val);
                }
            }
            $jobs =	$ZphM->getZphSpaceList(array('keyid' => $_POST['id'], 'keyid' => array('in', $root_arr, 'or'),'orderby' => array('sort,asc', 'id,desc')));
            $a=0;
            if(is_array($jobs)){
                $threeParentIds = array();
                foreach($jobs as $key => $v){
                    if($v['keyid'] == $id){
                        $twojob[$a]['id'] =	$v['id'];
                        $twojob[$a]['sort']	= $v['sort'];
                        $twojob[$a]['name']	= $v['name'];
                        $a++;
                    }else{
                        $threejob[$v['keyid']][] = $v;
                        $threeParentIds[] = $v['keyid'];
                    }
                }
            }
            foreach ($twojob as $k => $v) {
                if (in_array($v['id'], $threeParentIds)) {
                    $twojob[$k]['children'] = $threejob[$v['id']];
                }
            }
            if ($onejob) {
                $onejob['children'] = $twojob;
                $rt[] = $onejob;
            } else {
                $rt = array();
            }
        }
        $this->render_json(0, '', $rt);
    }

    function del_action(){
        $ZphM =	$this->MODEL('zph');
        $delID = $_POST['del'];
        $return = $ZphM->delZphSpace($delID);
        if ($return['errcode'] == 9) {
            $this->admin_json(0, $return['msg']);
        } else {
            $this->render_json(1, $return['msg']);
        }
    }

    function ajax_action(){
        $ZphM =	$this->MODEL('zph');
        if(isset($_POST['sort'])){// Update job fair venue sort.
            $sValue['sort'] = $_POST['sort'];
            $sWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($sWhere,$sValue);
            $this->MODEL('log')->addAdminLog(yun_t('admin_model_00036', array('id' => $_POST['id'])));
        }
        if(isset($_POST['name'])){// Update job fair venue name.
            $nValue['name'] = $_POST['name'];
            $nWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($nWhere,$nValue);
            $this->MODEL('log')->addAdminLog(yun_t('admin_model_00037', array('id' => $_POST['id'])));
        }
        if($_POST['price']!=""){// Update job fair venue price.
            $pValue['price'] = $_POST['price'];
            $pWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($pWhere,$pValue);
            $this->MODEL('log')->addAdminLog(yun_t('admin_model_00038', array('id' => $_POST['id'])));
        }
        $this->render_json(0, yun_at('wap_user_00264'));
    }
}
