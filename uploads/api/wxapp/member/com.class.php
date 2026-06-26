<?php

class com_controller extends wxapp_controller{
    
    function company_statis($uid)
    {
        $statisM  =  $this -> MODEL('statis');
        $statis   =  $statisM -> vipOver($uid, 2);
        
        $statis['pricename']  =  $this->config['integral_pricename'];
        
        return $statis;
    }
    
    function day_check($uid, $type)
    {
        $comM    =  $this -> MODEL('company');
        $result  =  $comM -> comVipDayActionCheck($type, $uid);
        return $result;
    }
}
?>