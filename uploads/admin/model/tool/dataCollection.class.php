<?php

/**
 * Class dataCollection_controller
 *
 * @author shiy
 * @version 7.0
 */

class dataCollection_controller extends adminCommon
{

    function index_action()
    {

        $locoyConfig = array();
        require_once APP_PATH."data/api/locoy/locoy_config.php";

        if (isset($locoyinfo)) {

            $locoyConfig = $locoyinfo;
        }

        $this->render_json(0, '', $locoyConfig);
    }

    public function getCache_action()
    {

        $cacheM =   $this->MODEL('cache');

        $cache  =   $cacheM->GetCache(array('hy', 'job', 'city', 'com', 'user'));

        $industryNameArr    =   $cache['industry_name'];
        foreach ($cache['industry_index'] as $k => $v){
            $industryArr[]  =   array('id' => $v, 'name' => $industryNameArr[$v]);
        }

        $jobNameArr         =   $cache['job_name'];
        foreach ($cache['job_index'] as $k=>$v) {
            $jobOneArr[]    =   array('id' => $v, 'name' => $jobNameArr[$v]);
        }
        foreach ($cache['job_type'] as $k => $jobV){
            foreach ($jobV as $v){
                $jobArr[]   =   array('id' => $v, 'pid' => $k, 'name' => $jobNameArr[$v]);
            }
        }

        $cityNameArr        =   $cache['city_name'];
        foreach ($cache['city_index'] as $k=>$v) {
            $provinceArr[]  =   array('id' => $v, 'name' => $cityNameArr[$v]);
        }
        foreach ($cache['city_type'] as $k => $cityV){
            foreach ($cityV as $v){
                $cityArr[]  =   array('id' => $v, 'pid' => $k, 'name' => $cityNameArr[$v]);
            }
        }

        $comNameArr         =   $cache['comclass_name'];
        $comDataArr         =   $cache['comdata'];

        foreach ($comDataArr['job_edu'] as $k => $v) {
            $jobEduArr[]    =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($comDataArr['job_exp'] as $k => $v) {
            $jobExpArr[]    =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($comDataArr['job_marriage'] as $k => $v) {
            $jobMarriageArr[]   =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($comDataArr['job_report'] as $k => $v) {
            $jobReportArr[] =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($comDataArr['job_pr'] as $k => $v) {
            $jobPrArr[]     =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($comDataArr['job_mun'] as $k => $v) {
            $jobMunArr[]    =   array('id' => $v, 'name' => $comNameArr[$v]);
        }
        foreach ($cache['com_sex'] as $k => $v) {
            $comSexArr[]    =   array('id' => $k.'', 'name' => $v);
        }

        $userNameArr        =   $cache['userclass_name'];
        $userDataArr        =   $cache['userdata'];

        foreach ($userDataArr['user_report'] as $k => $v) {
            $userReportArr[]=   array('id' => $v, 'name' => $userNameArr[$v]);
        }

        $dataArr = array(

            'industryArr'   =>  $industryArr,
            'jobOneArr'     =>  $jobOneArr,
            'jobArr'        =>  $jobArr,
            'provinceArr'   =>  $provinceArr,
            'cityArr'       =>  $cityArr,
            'jobPrArr'      =>  $jobPrArr,
            'jobMunArr'     =>  $jobMunArr,
            'jobEduArr'     =>  $jobEduArr,
            'jobExpArr'     =>  $jobExpArr,
            'jobMarriageArr'=>  $jobMarriageArr,
            'jobReportArr'  =>  $jobReportArr,
            'comSexArr'     =>  $comSexArr,
            'userReportArr' =>  $userReportArr
        );

        $this->render_json(0, '', $dataArr);
    }

    public function getUserCache_action()
    {

        $cacheM = $this->MODEL('cache');

        $cache = $cacheM->GetCache(array('user'));
        $userNameArr = $cache['userclass_name'];
        $userDataArr = $cache['userdata'];

        foreach ($cache['user_sex'] as $k => $v) {
            $userSexArr[] = array('id' => $k.'', 'name' => $v);
        }
        foreach ($userDataArr['user_marriage'] as $k => $v) {
            $userMarriageArr[] = array('id' => $v, 'name' => $userNameArr[$v]);
        }
        foreach ($userDataArr['user_edu'] as $k => $v) {
            $userEduArr[] = array('id' => $v, 'name' => $userNameArr[$v]);
        }
        foreach ($userDataArr['user_word'] as $k => $v) {
            $userExpArr[] = array('id' => $v, 'name' => $userNameArr[$v]);
        }
        foreach ($userDataArr['user_report'] as $k => $v) {
            $userReportArr[] = array('id' => $v, 'name' => $userNameArr[$v]);
        }

        $dataArr = array(

            'userSexArr' => $userSexArr,
            'userMarriageArr' => $userMarriageArr,
            'userEduArr' => $userEduArr,
            'userExpArr' => $userExpArr,
            'userReportArr' => $userReportArr
        );

        $this->render_json(0, '', $dataArr);
    }

    public function getRating_action()
    {

        $ratingList =   $this->MODEL('rating')->getList(array('category'=>1,'orderby'=>'sort,desc'));

        foreach ($ratingList as $k => $v) {

            $ratingArr[]    =   array('id' => $v['id'], 'name' => $v['name']);
        }

        $dataArr    =   array('ratingArr' => $ratingArr);

        $this->render_json(0, '', $dataArr);
    }

    public function getPartCache_action()
    {

        $cacheM = $this->MODEL('cache');

        $cache = $cacheM->GetCache(array('part'));
        $partNameArr = $cache['partclass_name'];
        $partDataArr = $cache['partdata'];

        foreach ($partDataArr['part_type'] as $k => $v) {
            $partTypeArr[] = array('id' => $v, 'name' => $partNameArr[$v]);
        }
        foreach ($partDataArr['part_billing_cycle'] as $k => $v) {
            $billingCycleArr[] = array('id' => $v, 'name' => $partNameArr[$v]);
        }

        $dataArr = array(

            'partTypeArr' => $partTypeArr,
            'billingCycleArr' => $billingCycleArr
        );

        $this->render_json(0, '', $dataArr);
    }

    public function setLocoyConfig_action()
    {

        if (isset($_POST['locoyConfig']) && intval($_POST['locoyConfig']) == 1){

            $_POST  =   $this->post_trim($_POST);

            $config = "<?php \r\n";
            $path = APP_PATH."data/api/locoy/locoy_config.php";
            require_once $path;

            unset($_POST['locoyConfig']);

            if($_POST){

                $parr="";

                foreach($_POST as $key=>$value){
                    $locoyinfo[$key]=$value;
                }

                foreach($locoyinfo as $key=>$value){
                    $parr .= "\"".$key."\"=>\"".$value."\",";
                }
                $parr = rtrim($parr,",");
                $config.="\$locoyinfo=array(".$parr."); \r\n";
            }

            $path = APP_PATH."data/api/locoy/locoy_config.php";
            $fp = @fopen($path,"w");
            fwrite($fp,$config);
            fclose($fp);

            $this->admin_json(0, '信息采集配置成功');
        }
    }
}

?>