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
            // 添加打开弹窗请求，方便权限控制
            $this->render_json(0);
        }
        $ZphM =	$this->MODEL('zph');
        $info = $this->post_trim($_POST);
        if (!$info['name']) {
            $this->render_json(1, '区域名称不能为空');
        }
        if($_FILES){
            // pc端上传
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
            $msg = "更新";
        }else{
            $nid = $ZphM->addZphSpaceInfo($data);
            $msg = "添加";
        }
        if ($nid) {
            $this->admin_json(0, $msg . "成功！");
        } else {
            $this->render_json(1, $msg . "失败！");
        }
    }

    // 获取二级分类
    function ajaxspace_action(){
        $ZphM =	$this->MODEL('zph');
        $id = intval($_POST['id']);
        if($id != ""){
            $jobs =	$ZphM->getZphSpaceList(array('keyid' => $id));
            $this->render_json(0, '', $jobs);
        }
    }

    function up_action(){
        // 查询子类别
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
        if(isset($_POST['sort'])){//修改招聘会场地排序
            $sValue['sort'] = $_POST['sort'];
            $sWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($sWhere,$sValue);
            $this->MODEL('log')->addAdminLog("修改招聘会场地(ID:".$_POST['id'].")的排序");
        }
        if(isset($_POST['name'])){//修改招聘会场地名称
            $nValue['name'] = $_POST['name'];
            $nWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($nWhere,$nValue);
            $this->MODEL('log')->addAdminLog("修改招聘会场地(ID:".$_POST['id'].")名称");
        }
        if($_POST['price']!=""){//修改招聘会场地名称
            $pValue['price'] = $_POST['price'];
            $pWhere['id'] =	$_POST['id'];
            $ZphM->upZphSpaceInfos($pWhere,$pValue);
            $this->MODEL('log')->addAdminLog("修改招聘会场地(ID:".$_POST['id'].")名称");
        }
        $this->render_json(0, '操作成功');
    }
}