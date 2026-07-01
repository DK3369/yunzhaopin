<?php

class com_controller extends wxapp_controller{
    /**
     * 剩余套餐数量
     * 会员套餐过期检测，并处理
     */
    function company_statis($uid)
    {
        $statisM  =  $this -> MODEL('statis');
        $statis   =  $statisM -> vipOver($uid, 2);
        
        $statis['pricename']  =  $this->config['integral_pricename'];
        
        return $statis;
    }
    /**
     * 时间会员每日最大操作数量检测
     */
    function day_check($uid, $type)
    {
        $comM    =  $this -> MODEL('company');
        $result  =  $comM -> comVipDayActionCheck($type, $uid);
        return $result;
    }
}
?>