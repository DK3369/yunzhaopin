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
/************
 * 计划任务：每日自动关闭充值超时的订单
 * 仅作参考
 */
global $db_config,$db,$config;
$closeOrder = $config['sy_closeOrder'];
if($closeOrder > 0){
	$stime = time()-86400*$closeOrder;
	$delOrder =$db->select_all('company_order',"`order_time`<'".$stime."' AND `order_state`='1'","id");
	if(!empty($delOrder)){
		$upOrder =array();
		foreach ($delOrder as $key => $value){
			$upOrder[] = $value['id'];
		}

		$updateOrder = $db->update_all('company_order',"`order_state`=4","`id` IN (".implode(",",$upOrder).")");
	}

}
?>