<?php


class index_controller extends common
{
    //计划任务
    function index_action()
    {
        $id = isset($_GET['id']) ? intval($_GET['id']) : '';
        //读取定时任务缓存文件
        include PLUS_PATH.'cron.cache.php';
		
        //判断是否开启定时任务(定时任务缓存文件不为空),并且其他任务未处于执行状态

        if (!empty($cron) && isset($start) && (!$start || $starttime < (time() - 600))) {


            $CronM = $this->MODEL('cron');
			
            $CronM->excron($cron,$id);

        }
    }

}

?>