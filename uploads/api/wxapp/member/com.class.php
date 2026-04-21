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