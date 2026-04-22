<?php

class paylogtc_controller extends company{
	function index_action(){
		$this	->	public_action();
		$statis	=	$this	->	company_satic();
		if($statis['rating']){
			$ratingM	=	$this	->	MODEL('rating');
			$rating		=	$ratingM	->	getInfo(array('id'=>$statis['rating']));
		}
		$comM	=	$this	->	MODEL('company');
		$com	=	$comM	->	getInfo($this->uid);
		if($statis['days']){
			
			$this	->	yunset("days",$statis['days']);
			
		}		
		
		$comorderM		=	$this		->	MODEL('companyorder');
		$allprice		=	$comorderM	->	getCompanyPaySumPrice(array('com_id'=>$this->uid,'usertype' => 2,'type'=>'1','order_price'=>array('<','0')));
		
		$statis['zhjf']	=	number_format($statis['integral']);

		if ($statis['rating_type'] == 1) {

            $jobNum     =   $this->obj->select_num('company_job', array('uid' => $this->uid, 'status' => 0));
            $partNum    =   $this->obj->select_num('partjob', array('uid' => $this->uid, 'status' => 0));
            $zzNum      =   $jobNum + $partNum;
            $JobNum     =   $statis['job_num'] - $zzNum;
            $statis['job_num']    =   $JobNum > 0 ? $JobNum : 0;
            $this->yunset('JobNum', $JobNum);
        }

  		$this	->	yunset("integral",number_format(str_replace("-","", $allprice)));
		$this	->	yunset("com",$com);
		$this	->	yunset("statis",$statis);
		$this	->	yunset("rating",$rating);
		$this	->	com_tpl('paylogtc');
	}
}
?>