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
class keywordlog_model extends model{

    function addlog($addData=array(),$data=array()){

        if($data['utype']=='user'){

            $addData['keyword'] = trim($addData['keyword']);

            if($addData['uid'] && $addData['usertype']=='2' && !empty($addData['keyword'])){

                $id  =  $this -> insert_into('keyword_log',$addData);

                return  $id;
            }
        }
    }
}
?>