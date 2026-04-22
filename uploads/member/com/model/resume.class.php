<?php

class resume_controller extends company{
    function index_action(){
        $CacheM		=	$this -> MODEL('cache');
        
        $keywordlogM=   $this -> MODEL('keywordlog');
        
        $keywordlogData = array(
            'uid'       =>  $this->uid,
            'usertype'  =>  $this->usertype,
            'keyword'   =>  $_GET['keyword'],
            'ctime'     =>  time(),
        );

        $keywordlogM->addlog($keywordlogData,array('utype'=>'user'));
        
        $CacheList	=	$CacheM -> GetCache (array('city','user','job','hy','uptime'));
		$this -> yunset($CacheList);
        if(empty($CacheList['city_type'])){
            $this   ->  yunset('cionly',1);
        }
        if(empty($CacheList['job_type'])){
            $this   ->  yunset('jionly',1);
        }

        $this -> yunset("date",date("Y",0));
        $this -> yunset("time",date("Y",time()));

        $this -> yunset("type",$_GET['type']);
        $this -> public_action();
        $this -> company_satic();
        $this->yunset('comInfo',$this->comInfo);
        $this -> com_tpl('resume');
    }
}