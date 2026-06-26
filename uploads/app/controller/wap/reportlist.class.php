<?php

class reportlist_controller extends common{
    
    function index_action(){
        
        $cacheM     =   $this->MODEL('cache');
        $cache      =   $cacheM -> GetCache(array('user'));
        $this       ->  yunset($cache);
        
        $this -> yunset('headertitle', yun_at('wap_com_00350'));
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
            $this->layer_msg(yun_at('wap_00376'), 8);
        }
    }
}
?>