<?php
/* *
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */
class reportlist_controller extends common{
    /**
     * 简历详情
     * 举报简历
     * 2019-06-24
     */
    function index_action(){
        
        $cacheM     =   $this->MODEL('cache');
        $cache      =   $cacheM -> GetCache(array('user'));
        $this       ->  yunset($cache);
        
        $this -> yunset('headertitle', '举报');
        $this -> seo('report');
        $this -> yuntpl(array('wap/reportlist'));
    }
    function saveReport_action(){
        
        if ($this->uid){
            $data1['c_uid']      =   $_POST['uid'];
            $data1['inputtime']  =   time();
            $data1['p_uid']      =   $this->uid;
            $data1['did']        =   $this->userdid;
            $data1['usertype']   =   $this->usertype;
            $data1['eid']        =   $_POST['eid'];
            $data1['r_name']     =   $_POST['r_name'];
            $data1['username']   =   $this->username;
            $data1['reason']     =   @implode(',', $_POST['reason']);
            
            $reportM		     =	 $this -> MODEL('report');
            $result              =   $reportM->ReportResume($data1);
            
            $this->layer_msg($result['msg'], $result['errcode']);
        }else{
            $this->layer_msg('请先登录', 8);
        }
    }
}
?>