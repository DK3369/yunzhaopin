<?php

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