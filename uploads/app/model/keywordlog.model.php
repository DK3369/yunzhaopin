<?php

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