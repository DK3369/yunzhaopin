<?php

class fabutool_controller extends adminCommon{
    

    function index_action(){

        $where = array();

        if(trim($_POST['keyword'])){

            $where['title']   =   array('like',trim($_POST['keyword']));

        }
        
        if (isset($_POST['temptype'])) {

            $where['temptype']  =   $_POST['temptype'];
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('wxpub_temps', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            $field  =   '*,CASE WHEN `type`="job" THEN 1';
            $field  .=  ' WHEN `type`="company" THEN 2';
            $field  .=  ' WHEN `type`="resume" THEN 3 END AS `type_px`';

            $where['orderby']   =   array('type_px,asc', 'id,desc');
            $where['limit']     =   $pages['limit'];

            $wxpubtempM =   $this->MODEL('wxpubtemp');
            $list      =   $wxpubtempM->getTempList($where, array('field' => $field));
        }

        $data['list']           =   !empty($list)?$list:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function wxPubTempDel_action(){

        //实例化
        $wxpubtempM =   $this->MODEL('wxpubtemp');

        if($_POST['del']){

            $return     =   $wxpubtempM->delTemp($_POST['del']);

            if(is_array($_POST['del'])){
                
                $delid = pylode(',',$_POST['del']);

            }else{

                $delid = $_POST['del'];
            
            }

            if($return['errcode']==9){
                $error = 0;
                $msg = '(ID:'.$delid.')删除成功！';
            }else{
                $error = 1;
                $msg = '删除失败！';
            }

        }else{

            $error = 1;

            $msg = '请选择要删除的内容！';
        }
        

        $this->render_json($error,$msg);

    }

    function wxPubTemp_action()
    {
        $wxpubtempM =   $this->MODEL('wxpubtemp');

        if ($_POST['id']) {

            $temp       =   $wxpubtempM->getTemp(array('id' => $_POST['id']));

            if ($temp['type'] == 'onejob') {
                $temp['type']   =   'job';
            }

            $selfcolumn =   'pubtoolself_' . $temp['type'] . 'column';
            $typecolumn =   array_merge($wxpubtempM->$selfcolumn, $wxpubtempM->pubtoolself_publiccolumn, $wxpubtempM->pubtoolself_totalcolumn);
            $temptype   =   $temp['temptype'];

            $temp['header'] =   str_replace('{admin_style}', $this->config['sy_weburl'] . "/app/template/admin", $temp['header']);
            $temp['body']   =   str_replace('{admin_style}', $this->config['sy_weburl'] . "/app/template/admin", $temp['body']);
            $temp['footer'] =   str_replace('{admin_style}', $this->config['sy_weburl'] . "/app/template/admin", $temp['footer']);

            $temp['header'] =   preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $temp['header']);
            $temp['body']   =   preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $temp['body']);
            $temp['footer'] =   preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $temp['footer']);

        } else {

            $temptype   =   $_POST['temptype'];
            $typecolumn =   array_merge($wxpubtempM->pubtoolself_jobcolumn, $wxpubtempM->pubtoolself_resumecolumn, $wxpubtempM->pubtoolself_companycolumn, $wxpubtempM->pubtoolself_publiccolumn, $wxpubtempM->pubtoolself_totalcolumn);
        }
        if ($temptype == '1') {
            foreach ($typecolumn as $tk => $tv) {
                if (strpos($tv[1], '{img') !== false || strpos($tv[1], 'H5xcx_') !== false) {

                    unset($typecolumn[$tk]);
                }
            }
        }
        

        $job_map = $resume_map = $company_map = $public_map = $total_map =  array();

        foreach ($wxpubtempM->pubtoolself_jobcolumn_map as $jmk => $jmv) {
            $job_map[] = array(
                'search'=>$jmk,
                'replace'=>$jmv['js']
            );
        }

        foreach ($wxpubtempM->pubtoolself_resumecolumn_map as $rmk => $rmv) {
            $resume_map[] = array(
                'search'=>$rmk,
                'replace'=>$rmv['js']
            );
        }

        foreach ($wxpubtempM->pubtoolself_companycolumn_map as $cmk => $cmv) {
            $company_map[] = array(
                'search'=>$cmk,
                'replace'=>$cmv['js']
            );
        }

        foreach ($wxpubtempM->pubtoolself_publiccolumn_map as $pmk => $pmv) {
            $public_map[] = array(
                'search'=>$pmk,
                'replace'=>$pmv['js']
            );
        }

        foreach ($wxpubtempM->pubtoolself_totalcolumn_map as $tmk => $tmv) {
            $total_map[] = array(
                'search'=>$tmk,
                'replace'=>$tmv['js']
            );
        }

        $data['info']               = !empty($temp)?$temp:array();
        $data['temptype']           = $temptype;
        $data['typecolumn']         = $wxpubtempM->columnForm(array('column'=>$typecolumn));
        $data['totalcolumn']        = $wxpubtempM->columnForm(array('column'=>$wxpubtempM->pubtoolself_totalcolumn));
        $data['job_map']            = $job_map;
        $data['resume_map']         = $resume_map;
        $data['company_map']        = $company_map;
        $data['public_map']         = $public_map;
        $data['total_map']          = $total_map;
        
        $this->render_json(0,'',$data);
    }
    function wxPubTempSave_action()
    {

        $whereData          =   array();

        $_POST['header']    =   $this->downloadWxPic($_POST['header']);
        $_POST['body']      =   $this->downloadWxPic($_POST['body']);
        $_POST['footer']    =   $this->downloadWxPic($_POST['footer']);

        $rep_arr            =   array($this->config['sy_weburl'] . "/app/template/admin", 'http://www.yunjob.com/app/template/admin');
        $_POST['header']    =   str_replace($rep_arr, '{admin_style}', $_POST['header']);
        $_POST['body']      =   str_replace($rep_arr, '{admin_style}', $_POST['body']);
        $_POST['footer']    =   str_replace($rep_arr, '{admin_style}', $_POST['footer']);

        $wxpubtempM         =   $this->MODEL('wxpubtemp');

        $fwhere             =   array();
        $fwhere['title']    =   $_POST['title'];

        if ($_POST['id']) {

            $tempinfo       =   $wxpubtempM->getTemp(array('id' => $_POST['id']));
            $whereData['id']=   $_POST['id'];
            $fwhere['id']   =   array('<>', $_POST['id']);
        }

        $temp   =   $wxpubtempM->getTemp($fwhere);

        $updata =   array(

            'title'     =>  $_POST['title'],
            'header'    =>  preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $_POST['header']),
            'body'      =>  preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $_POST['body']),
            'footer'    =>  preg_replace('#\{yun:\}(.*?)\{\/yun\}#i', '', $_POST['footer']),
            'type'      =>  $tempinfo['type'] == 'onejob' ? 'onejob' : $_POST['type'],
            'temptype'  =>  $_POST['temptype'],
            'time'      =>  time()
        );

        if ($temp) {

            $return['msg']      =   '该模板名称已使用，请重新命名';
            $return['errcode']  =   8;
        } else {

            $return             =   $wxpubtempM->setTemp($updata, $whereData);
        }

        $error = $return['errcode']==9?0:1;

        $this->render_json($error,$return['msg']);
    }

    private function downloadWxPic($str)
    {

        $str    =   stripslashes($str);
        //来源自公众号的图片，且非svg类型的，下载
        if (strpos($str, 'mmbiz.qpic.cn') !== false && strpos($str, 'mmbiz_svg') === false) {

            $preg       =   '/<img.*?src=[\"|\']?(.*?)[\"|\']?\s.*?>/i';
            preg_match_all($preg, $str, $img);

            $imgArr     =   array_unique($img[1]);

            //自定义目录名称
            $dirName    =   APP_PATH.'data/upload/wx/'.date('Ymd');

            include_once(LIB_PATH . 'upload.class.php');
            $upload     =   new Upload();

            if ($this->config['sy_oss'] == 1) {

                include_once(LIB_PATH.'oss/ossupload.class.php');
                $ossUpload  =   new ossUpload();
            }

            foreach ($imgArr as $ik => $iv) {

                if (strpos($iv, 'mmbiz.qpic.cn') !== false) {

                    $iv_arr     =   explode('?', $iv);

                    $CurlReturn =   CurlGet($iv_arr[0]);
                    if ($CurlReturn) {
                        // 重新定义文件名称 图片一律用 jpeg
                        $filename   =   time().rand(1000, 9999).'.jpeg';

                        if (!file_exists($dirName)) {

                            mkdir($dirName, 0777, true);
                        }
                        //对原图进行强制压缩 防止非法图片上传
                        $pic        =   $dirName.'/'.$filename;
                        $ires       =   file_put_contents($pic, $CurlReturn);
                        $picUrl     =   str_replace(APP_PATH.'data', './data', $pic);

                        if ($this->config['sy_oss'] == 1) {

                            $return =   $ossUpload->uploadLocalImg(realpath($pic));
                            $picUrl =   $return['picurl'];
                        }

                        if ($picUrl) {
                            $picUrl =   checkpic($picUrl);
                            $str    =   str_replace($iv, $picUrl, $str);
                        }
                    }
                }
            }
        }
        return $str;
    }


    function pubtool_action()
    {

        $wxpubtempM =   $this->MODEL('wxpubtemp');
        $temps      =   $wxpubtempM->getTempList(array('orderby' => 'id,desc'), array('field' => '`id`,`title`,`type`,`temptype`'));
        foreach ($temps as $key => $value) {
            if ($value['type'] == 'onejob') {
                $temps[$key]['type']    =   'job';
            }
        }
        

        $CacheM     =   $this->MODEL('cache');
        $CacheList  =   $CacheM->GetCache(array('domain'));

        $ratingM    =   $this->MODEL('rating');
        $ratingList =   $ratingM->getList(array('category' => 1, 'orderby' => 'sort,asc'), array('field' => "`id`,`name`"));

        $data['temps'] = $temps;
        $data['rating'] = $ratingList;
        $data['domain'] = (object)$CacheList['Dname'];
        
        $this->render_json(0,'',$data);
    }
    function getWxpubJob_action(){
        $jobM   =   $this->MODEL('job');

        $result    =   array();

        if (!empty($_POST['keyword'])) {
            
            $where['state']     =   1;
            $where['status']    =   0;
            $where['r_status']  =   1;


            $_POST['keyword']    =   trim($_POST['keyword']);

            if(is_numeric($_POST['keyword'])){
                $where['id']    =   $_POST['keyword'];
            }else{

                $where['PHPYUNBTWSTART']    =   '';
                $where['name']              =   array('like', $_POST['keyword']);
                $where['com_name']          =   array('like', $_POST['keyword'],'OR');
                $where['PHPYUNBTWEND']      =   '';

            }

            $jobArr    =   $jobM -> getList($where, array('field' => '`id`,`name`,`com_name`'));
            
            if (!empty($jobArr['list'])) {
                
                $jobList   =   $jobArr['list'];
                foreach ($jobList as $jk => $jv){
                    $result[$jk]['name']    =   $jv['name'];
                    $result[$jk]['value']   =   $jv['id'];
                    $result[$jk]['upname']  =   $jv['com_name'];
                 }
            }
        }
        $this->render_json(0,'',$result);
    }
    
    //微信发布工具搜索企业
    function getComBySearch_action(){
        $result    =   array();

        if (!empty($_POST['keyword'])) {
            
            $where['r_status']     =   1;

            $where['name']  =   array('like', trim($_POST['keyword']));
        
            $where['limit']     =   10;
            
            $comM      =   $this->MODEL('company');
            
            $comArr    =   $comM -> getList($where, array('field' => '`uid`,`name`'));
            
            if (!empty($comArr)) {
                
                $comList   =   $comArr['list'];
                foreach ($comList as $k => $v){
                    $result[$k]['name']    =   $v['name'];
                    $result[$k]['value']   =   $v['uid'];
                }
            }
        }
        $this->render_json(0,'',$result);
    }

    //微信发布工具信息
    function Getpubtool_action()
    {
        $get    =   $_POST;
        //查询类型 0职位列表1简历列表2企业列表
        $type   =   $get['type'];

        switch ($type) {
            case 'job':
                $this->Getjob($get);
                break;
            case 'resume':
                $this->Getresume($get);
                break;
            case 'company':
                $this->Getcompany($get);
                break;
            default:
                echo "暂无信息";
                break;
        }
    }
    // 职位列表
    protected function Getjob($GET)
    {

        $jobM   =   $this->MODEL('job');
        $time   =   time();
        $where  =   array();
        $data   =   array();

        //  会员等级
        if ($GET['rating'] != '') {

            $rating             =   pylode(',', $GET['rating']);
            $where['rating']    =   array('in', $rating);
        }

        // 职位参数：0-置顶、1-紧急、2-推荐
        if ($GET['param'] != '') {

            $param  =   explode(',', $GET['param']);
            if (in_array('0', $param)) {
                $where['xsdate']        =   array('>', $time);
            }
            if (in_array('1', $param)) {
                $where['urgent_time']   =   array('>', $time);
            }
            if (in_array('2', $param)) {
                $where['rec_time']      =   array('>', $time);
            }
        }

        //职位更新时间
        if ($GET['times'] != '0') {

            $times  =   $GET['times'];
            if ($times == '1') {

                $where['lastupdate']    =   array('>', strtotime(date('Y-m-d 00:00:00')));
            } else {

                $where['lastupdate']    =   array('>', strtotime('-' . intval($times) . ' day'));
            }
        }

        //  职位发布时间
        if ($GET['ftimes'] != '0') {

            $times  =   $GET['ftimes'];
            if ($times == '1') {

                $where['sdate'] =   array('>', strtotime(date('Y-m-d 00:00:00')));
            } else {

                $where['sdate'] =   array('>', strtotime('-' . intval($times) . ' day'));
            }
        }

        //  指定职位
        if ($GET['jobcopos'] != '') {

            $where['id']        =   array('in', str_replace('，', ',', $GET['jobcopos']));
        }

        //  工作地点
        if ($GET['provinceid'] != '') {

            $where['provinceid']    =   $GET['provinceid'];
            if ($GET['cityid'] != '') {

                $where['cityid']    =   $GET['cityid'];
                if ($GET['three_cityid'] != '') {

                    $where['three_cityid']  =   $GET['three_cityid'];
                }
            }
        }

        //  职位类别
        if ($GET['job1'] != '') {

            $where['job1']  =   $GET['job1'];
            if ($GET['job1_son'] != '') {

                $where['job1_son']  =   $GET['job1_son'];
                if ($GET['job_post'] != '') {

                    $where['job_post']  =   $GET['job_post'];
                }
            }
        }
        //  薪资待遇
        if ($GET['minsalary'] != '' && $GET['maxsalary'] == '') {

            $where['minsalary'] =   array('>', $GET['minsalary']);
        } elseif ($GET['minsalary'] == '' && $GET['maxsalary'] != '') {

            $where['maxsalary'] =   array('<', $GET['maxsalary']);
        } elseif ($GET['minsalary'] != '' && $GET['maxsalary'] != '') {

            $where['minsalary'] =   array('>', $GET['minsalary']);
            $where['maxsalary'] =   array('<', $GET['maxsalary']);
        }


        //福利待遇
        if ($GET['welfare'] != '') {

            $GET['welfare'] =   trim($GET['welfare']);

            if (strpos($GET['welfare'], '|') !== false) {

                $welfare    =   explode('|', $GET['welfare']);
                if (!empty($welfare)) {

                    $where['PHPYUNBTWSTART_A']    =   '';
                    foreach ($welfare as $key => $value) {
                        $where['welfare'][]     =   array('like', trim($value), 'OR');
                    }
                    $where['PHPYUNBTWEND_A']      =   '';
                }
            } elseif (strpos($GET['welfare'], " ") !== false) {

                $welfare    =   explode(' ', $GET['welfare']);
                if (!empty($welfare)) {

                    $where['PHPYUNBTWSTART_A']    =   '';
                    foreach ($welfare as $key => $value) {
                        $where['welfare'][]     =   array('like', trim($value), 'AND');
                    }
                    $where['PHPYUNBTWEND_A']      =   '';
                }
            } else {

                $where['welfare']               =   array('like', $GET['welfare'], 'AND');
            }
        }

        //关键词
        if ($GET['keyword'] != '') {

            $GET['keyword']         =   trim($GET['keyword']);

            $where['PHPYUNBTWSTART_B']=   '';
            $where['com_name']      =   array('like', $GET['keyword'], 'or');
            $where['name']          =   array('like', $GET['keyword'], 'or');
            $where['PHPYUNBTWEND_B']  =   '';
        }


        //信息数量
        $num = $GET['num'];
        $where['limit'] = $num;

        $where['state'] = '1';
        $where['r_status'] = '1';
        $where['status'] = '0';

        $where['orderby'] = 'lastupdate,desc';
        $lists = $jobM->Getpubtool($where, $data);
        $this->yunset('lists', $lists);
        $this->yunset('ewmtype', $GET['ewmtype']);
        //风格模板


        $res = $this->mk_temp($GET['tpl']);

        if ($res) {
            $this->yuntpl(array($res));
        }


    }

    // 简历列表
    protected function Getresume($GET){
        $resumeM = $this->MODEL('resume');
        $time = time();
        $where = array();
        include_once(PLUS_PATH . 'city.cache.php');
        include_once(PLUS_PATH . 'cityparent.cache.php');
        include_once(PLUS_PATH . 'job.cache.php');
        include_once(PLUS_PATH . 'jobparent.cache.php');
        //简历类型
        if($GET['cvkind']){
            //置顶
            $where['top'] 		  =  1;
            $where['topdate']	  =  array('>',$time);
        }
        $newwhere = array();
        //意向职位
        if($GET['job_class'] != ''){
            if ($job_parent[$GET['job_class']] == '0') {
                $newwhere['job1'] = $GET['job_class'];
            } elseif (in_array($job_parent[$GET['job_class']], $job_index)) {
                $newwhere['job1_son'] = $GET['job_class'];
            } elseif ($job_parent[$GET['job_class']] > 0) {
                $newwhere['job_post'] = $GET['job_class'];
            }

        }

        //意向地区
        if($GET['city_class'] != ''){
            if ($city_parent[$GET['city_class']] == '0') {
                $newwhere['provinceid'] = $GET['city_class'];
            } elseif (in_array($city_parent[$GET['city_class']], $city_index)) {
                $newwhere['cityid'] = $GET['city_class'];
            } elseif ($city_parent[$GET['city_class']] > 0) {
                $newwhere['three_cityid'] = $GET['city_class'];
            }
        }
        $eids = array();
        if (!empty($newwhere)){
            $joblist = $resumeM->getCityJobClass($newwhere,'eid');
            if (!empty($joblist)) {
                foreach ($joblist as $k => $v) {
                    $eids[] = $v['eid'];
                }
            }else{
                $eids = array(0);
            }
            $where['id'] = array('in', pylode(',',$eids));
        }
        //学历
        if($GET['bd'] != ''){
            $where['edu'] = array('in',$GET['bd']);
        }
        //简历发布时间
        if($GET['rtimes'] != '0'){
            $times = $GET['rtimes'];
            if($times  ==  '1'){
                $where['ctime'] = array('>',strtotime(date('Y-m-d 00:00:00')));
            }else{
                $where['ctime'] = array('>',strtotime('-'.intval($times).' day'));
            }
        }

        //简历刷新时间
        if($GET['rltimes'] != '0'){
            $times = $GET['rltimes'];
            if($times  ==  '1'){
                $where['lastupdate'] = array('>',strtotime(date('Y-m-d 00:00:00')));
            }else{
                $where['lastupdate'] = array('>',strtotime('-'.intval($times).' day'));
            }
        }
        //经验
        if($GET['exp'] != ''){
            $where['exp'] = array('in',$GET['exp']);
        }

        //简历完整度
        if($GET['whole']!= ''){
            $where['integrity'] = array('>=',$GET['whole']);
        }

        //信息数量
        $num = $GET['num'];
        $where['limit'] = $num;


        $where['defaults'] = 1;
        $where['state']    = 1;
        $where['status']   = 1;
        $where['r_status'] = 1;
        $where['orderby'] = "lastupdate,desc";
        $lists = $resumeM->Getresume($where,$data);

        $this->yunset('lists',$lists);
        $this->yunset('ewmtype',$GET['ewmtype']);
        //风格模板

        $res = $this->mk_temp($GET['tpl']);
        if($res){
            $this->yuntpl(array($res));
        }

    }

    // 企业列表
    protected function Getcompany($GET){
        global $db_config;
        $companyM = $this->MODEL('company');
        $data  = array();
        $time  = time();

        $comwhere['r_status'] = 1;
        $comwhere['name'] = array('<>','');
        if($GET['rating'] != ''){

            $rating = pylode(',',$GET['rating']);
            $comwhere['rating'] = array('in',$rating);
        }

        $data['jWhere']['r_status'] =   1;
        $data['jWhere']['status']   =   0;
        $data['jWhere']['state']    =   1;
        $data['jWhere']['orderby']  =   "lastupdate,desc";
        
        if($GET['rule']){
            $data['jWhere']['limit'] = $GET['rule'];
        }

        //站点
        if($GET['did'] != ''){
            $comwhere['did'] = $GET['did'];
        }

        //套餐开通时间
        if($GET['vtimes'] != '0'){
            $times = $GET['vtimes'];
            if($times  ==  '1'){

               $comwhere['vipstime'] = array('>',strtotime(date('Y-m-d 00:00:00')));
            }else{

               $comwhere['vipstime'] = array('>',strtotime('-'.intval($times).' day'));
            }
        }

        //企业类型
        if($GET['bekind'] != ''){
            $comwhere['rec'] = 1;
            $comwhere['hotstart'] = array('<=',$time);
            $comwhere['hottime'] = array('>',$time);
        }

        //地点
        if($GET['provinceid'] != ''){
            $comwhere['provinceid'] = $GET['provinceid'];
            if($GET['cityid'] != ''){
                $comwhere['cityid'] = $GET['cityid'];
                if($GET['three_cityid'] != ''){
                    $comwhere['three_cityid'] = $GET['three_cityid'];
                }
            }
        }

        //行业
        if($GET['hy'] != ''){
            $comwhere['hy'] = $GET['hy'];
        }
        $comwhere['orderby'] = 'lastupdate,DESC';
        //指定企业职位 企业ID
        if($GET['copos'] != ''){
            $comuids = str_replace('，',',',$GET['copos']);
            $comwhere['uid'] = array('in',$comuids);
            $comwhere['orderbyfield'] = array('uid',$comuids);
        }

        //关键词
        if($GET['keyword'] != ''){
            $comwhere['name'] = array('like',trim($GET['keyword']));
        }

        //信息数量
        $num = $GET['num'];

        $comwhere['limit'] = $num;
        $data['comfield'] = '`uid`,`name`,`address`,`linkman`,`linktel`,`welfare`,`content`,`lastupdate`,`welfare`';
        
        $lists = $companyM->Getcompany($comwhere,$data);

        $this->yunset('lists',$lists);
        
        $this->yunset('ewmtype',$GET['ewmtype']);
        
        $res = $this->mk_temp($GET['tpl']);
        if($res){
            $this->yuntpl(array($res));
        }
    }

    protected function mk_temp($id, $data = array())
    {

        $tempath    =   '';

        if (empty($id)) {
            return false;
        }
        $wxpubtempM =   $this->MODEL('wxpubtemp');
        $temp       =   $wxpubtempM->getTemp(array('id' => $id));

        if (empty($temp)) {
            return false;
        }

        if ($temp['type'] == 'onejob') {
            $temp['type']   =   'job';
        }

        $temtype    =   'pubtoolself_' . $temp['type'] . 'column_map';
        $tempmap    =   $wxpubtempM->$temtype;

        if (!empty($tempmap)) {

            $search_arr = $replace_arr = array();

            $publiccolumn_map   =   $wxpubtempM->pubtoolself_publiccolumn_map;
            $totalcolumn_map    =   $wxpubtempM->pubtoolself_totalcolumn_map;

            $xcxurltype_v = $xcxurlid_v = $toc_v = $toa_v = $toid_v = $minipath = '';


            if ($temp['type'] == 'resume') {

                $toc_v          =   'resume';
                $toa_v          =   'show';
                $toid_v         =   '$v.list.id';
                $minipath       =   '/pages/resume/show?id={yun:}$v.list.id{/yun}';
                $xcxurltype_v   =   'resume';
                $xcxurlid_v     =   '$v.list.id';
            } else if ($temp['type'] == 'job') {

                $toc_v          =   'job';
                $toa_v          =   'comapply';
                $toid_v         =   '$v.id';
                $minipath       =   '/pages/job/show?id={yun:}$v.id{/yun}';
                $xcxurltype_v   =   'job';
                $xcxurlid_v     =   '$v.id';
            } else if ($temp['type'] == 'company') {

                $toc_v          =   'company';
                $toa_v          =   'show';
                $toid_v         =   '$v.uid';
                $minipath       =   '/pages/company/show?id={yun:}$v.uid{/yun}';

                $xcxurltype_v   =   'company';
                $xcxurlid_v     =   '$v.uid';
            }

            $param_s    =   array('toc_v', 'toa_v', 'toid_v', 'minipath', 'xcxurltype_v', 'xcxurlid_v');
            $param_v    =   array($toc_v, $toa_v, $toid_v, $minipath, $xcxurltype_v, $xcxurlid_v);

            foreach ($publiccolumn_map as $pk => $pv) {
                $publiccolumn_map[$pk]['php']   =   str_replace($param_s, $param_v, $pv['php']);
            }

            $tempmap    =   array_merge($tempmap, $publiccolumn_map, $totalcolumn_map);

            //正文中是否含有需要生成图片标签的参数，有的话生成标签和样式并加入待替换数组
            $result     =   array();
            preg_match_all('#\{img\|(.*?)\}#i', $temp['body'], $result);

            $style_v    =   '""';
            
            foreach ($result[1] as $key => $value) {

                $img_arr    =   explode("|", $value);

                if (count($img_arr) > 1) {

                    $tempmap_key    =   '{' . $img_arr[0] . '}';
                    $style_v        =   str_replace('样式=', '', $img_arr[1]);
                    $style_v        =   str_replace(array('&amp;', '&quot;', '&lt;', '&gt;'), array('&', '"', '<', '>'), $style_v);
                    //将{img|xxxx|样式=""}整体加入$tempmap
                    $tempmap['{img|'.$value.'}']    =   array(
                        'php'   =>  str_replace('style_v', $style_v, $tempmap[$tempmap_key]['php'])
                    );
                }
            }

            //正文中是否含有需要生成图片标签的参数，有的话生成标签和样式并替换end

            //是否含有约束字符串的相关标签{str|xxx|length}
            $str_result     =   array();
            preg_match_all('#\{str\|(.*?)\}#i', $temp['body'], $str_result);

            foreach ($str_result[1] as $sr_k => $sr_v) {

                $str_arr    =   explode("|", $sr_v);

                if (count($str_arr) > 1) {

                    $tempmap_key    =   '{' . $str_arr[0] . '}';
                    
                    $fun_arr    =   explode(":",$str_arr[1]);
                    //约束字符串长度
                    if($fun_arr[0]=='length'){

                        $lengthTemp = '';

                        $length_v = intval($fun_arr[1])>0?'|mb_substr:0:'.$fun_arr[1].':"utf-8"':'';

                        $lengthTemp = str_replace('{/yun}', $length_v.'{/yun}', $tempmap[$tempmap_key]['php']);

                        if($length_v!=''){//设置了文段长度的，将大于设置文段的，截取后加省略号，不然不加省略号

                            $column = str_replace(array('{yun:}','{/yun}'),array('',''), $tempmap[$tempmap_key]['php']);//提取出变量名 例如$v.desc
                            
                            $lengthTemp = '{yun:}if mb_strlen('.$column.')>'.$fun_arr[1].'{/yun}';
                            $lengthTemp .= str_replace('{/yun}', $length_v.'{/yun}', $tempmap[$tempmap_key]['php']).'...';
                            $lengthTemp .= '{yun:}else{/yun}';
                            $lengthTemp .= str_replace('{/yun}', $length_v.'{/yun}', $tempmap[$tempmap_key]['php']);
                            $lengthTemp .= '{yun:}/if{/yun}';
                        }
                        

                        //将{str|xxxx|xxx}整体加入$tempmap
                        $tempmap['{str|'.$sr_v.'}']    =   array(
                            'php'   =>  $lengthTemp
                        );
                    }
                    
                }
            }
            
            //是否含有约束字符串的相关标签end

            foreach ($totalcolumn_map as $totalk => $totalv) {

                $search_total_arr[]     =   $totalk;
                $replace_total_arr[]    =   $totalv['php'];
            }

            foreach ($tempmap as $key => $value) {

                $search_arr[]       =   $key;
                if (strpos($value['php'], 'style_v') !== false) {

                    $replace_arr[]  =   str_replace('style_v', $style_v, $value['php']);
                } else {

                    $replace_arr[]  =   $value['php'];
                }
            }

            $search_total_arr[] =   '&amp;';
            $replace_total_arr[]=   '&';
            $search_arr[]       =   '&amp;';
            $replace_arr[]      =   '&';

            $enter      =   '';
            if ($temp['temptype'] == '1') {
                $enter  =   "\r\n";
            }

            $html   =   '';
            $html   .=  str_replace($search_total_arr, $replace_total_arr, $temp['header']);
            $html   .=  '{yun:}foreach item=v from=$lists{/yun}';
            $html   .=  str_replace($search_arr, $replace_arr, $temp['body']);
            $html   .=  $enter;
            $html   .=  '{yun:}/foreach{/yun}';
            $html   .=  str_replace($search_total_arr, $replace_total_arr, $temp['footer']);

            if ($temp['temptype'] == '1' && !$data['notoBr']) {
                $html   =   str_replace("\n", "</br>", $html);
            }

            if (!file_exists(DATA_PATH.'wxpubtpl')) {

                @mkdir(DATA_PATH.'wxpubtpl', 0777, true);
            }

            file_put_contents(DATA_PATH.'wxpubtpl/'.$temp['id'].'.htm', $html);
            $tempath    =   DATA_PATH.'wxpubtpl/'.$temp['id'];
        }
        return $tempath;
    }

    function twTask_action(){

        $wxpubtempM =   $this->MODEL('wxpubtemp');

        $where = array('type'=>1);

        if ($_POST['keyword']) {

            $keyword    =   trim($_POST['keyword']);
            $jobM       =   $this->MODEL('job');

            $jobid_arr      =   array();

            if (is_numeric($keyword)) {

                $jobid_arr  =   array($keyword);
            } else {

                $jobwhere   =   array();

                $jobwhere['name']       =   array('like', $keyword);
                $jobwhere['com_name']   =   array('like', $keyword, 'or');

                if ($_POST['welfarekeyword']) {

                    $welfarekeyword     =   explode(' ', trim($_POST['welfarekeyword']));
                    $jobwhere['PHPYUNBTWSTART_A']   =   '';
                    foreach ($welfarekeyword as $k => $v) {
                        $jobwhere['welfare'][]      =   array('findin', $v);
                    }
                    $jobwhere['PHPYUNBTWEND_A']     =   '';
                }
                $jobList    =   $jobM->getListId($jobwhere, array('field' => '`id`'));
                foreach ($jobList as $jk => $jv) {

                    $jobid_arr[]    =   $jv['id'];
                }
            }

            $where['PHPYUNBTWSTART_B']  =   '';
            $where['jobid']             =   array('in', pylode(',', $jobid_arr));
            $where['content']           =   array('like', $keyword, 'or');
            $where['PHPYUNBTWEND_B']    =   '';

        } else {

            $jobM       =   $this->MODEL('job');
            $jobid_arr  =   array();

            if ($_POST['welfarekeyword']) {

                $welfarekeyword =   explode(' ', trim($_POST['welfarekeyword']));

                $jobwhere['PHPYUNBTWSTART_A']   =   '';
                foreach ($welfarekeyword as $k => $v) {
                    $jobwhere['welfare'][]      =   array('findin', $v);
                }
                $jobwhere['PHPYUNBTWEND_A']     =   '';

                $jobList    =   $jobM->getListId($jobwhere, array('field' => '`id`'));

                foreach ($jobList as $jk => $jv) {

                    $jobid_arr[]    =   $jv['id'];
                }
                $where['jobid']     =   array('in', pylode(',', $jobid_arr));
            }
        }
        
        if ($_POST['auid']) {
            $where['auid']  =   $_POST['auid'];
        }
        if ($_POST['status']) {

            if ($_POST['status'] == '1') {

                $where['status'] = $_POST['status'];
            } elseif ($_POST['status'] == '2') {

                $where['status'] = '0';
            }

        }

        if ($_POST['urgent']) {
            $where['urgent'] = $_POST['urgent'];
        }

        if ($_POST['wcmoments']) {
            $where['wcmoments'] = $_POST['wcmoments'];
        }

        if ($_POST['gzh']) {
            $where['gzh'] = $_POST['gzh'];
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('wxpub_twtask', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            if ($_POST['order']) {

                $where['orderby']   =   $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby']   =   'ctime';
            }
            
            $where['limit']     =   $pages['limit'];

            
            $twtasks    =   $wxpubtempM->getTwTaskList($where);
        }

        $data['list']           =   !empty($twtasks)?$twtasks:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function twTask_base_data_action()
    {
        $wxpubtempM = $this->MODEL('wxpubtemp');
        $adminM = $this->MODEL('admin');
        $adminList = $adminM->getList(array('orderby' => 'uid'), array('field' => '`uid`,`username`,`name`'));

        $tempWhere['temptype'] = 1;
        $tempWhere['PHPYUNBTWSTART_A'] = '';
        $tempWhere['type'][] = array('=', 'job');
        $tempWhere['type'][] = array('=', 'onejob', 'or');
        $tempWhere['PHPYUNBTWEND_A'] = '';
        $tempWhere['orderby'] = 'id,asc';

        $temps = $wxpubtempM->getTempList($tempWhere, '`id`,`title`');

        $temp2Where = array(
            'temptype' => 0,
            'type' => 'job',
            'orderby' => 'id,asc'
        );
        $temps2 = $wxpubtempM->getTempList($temp2Where, '`id`,`title`');


        $data['temps'] = !empty($temps) ? $temps : array();
        $data['temps2'] = !empty($temps2) ? $temps2 : array();
        $data['adminList'] = !empty($adminList) ? $adminList : array();
        $this->render_json(0, '', $data);
    }

    function delTwTask_action(){

        if ($_POST['del']) {

            $wxpubtempM =   $this->MODEL('wxpubtemp');
            $return     =   $wxpubtempM->delTwtask($_POST['del']);

            $error      =   $return['errcode']==9?0:1;
            $msg        =   $return['msg'];
        } else {
            $error      =   1;
            $msg        =   '请选择要删除的内容！';
        }

        $this->render_json($error,$msg);
    }
    function taskFinish_action()
    {

        $wxpubtempM =   $this->MODEL('wxpubtemp');

        if (is_array($_POST['id'])) {

            $where  =   array('id' => array('in', pylode(',', $_POST['id'])));
        } else if ($_POST['id']) {

            $where  =   array('id' => $_POST['id']);
        }

        if (!empty($where)) {

            $return =   $wxpubtempM->upTwtask($where, array('status' => 1, 'etime' => time()));

            if ($return) {
                $error = 0;
                $msg = '操作成功';
            } else {
                $error = 1;
                $msg = '操作失败请重试';
            }
        }else{
            $error = 1;
            $msg = '参数错误请重试';
        }

        $this->render_json($error,$msg);
    }
    function getTW_action()
    {

        $jobids     =   $_POST['jobids'];
        $tpl        =   $_POST['tpl'];

        $jobid_arr  =   !empty($jobids) ? explode(',', $jobids) : array();
        $jobid_arr  =   array_unique($jobid_arr);

        if (!empty($jobid_arr) && !empty($tpl)) {

            $jobM   =   $this->MODEL('job');
            $where  =   array();

            $where['id']    =   array('in', pylode(',', $jobids));

            $lists          =   $jobM->Getpubtool($where);

            $jobs   =   $joblists   =   array();

            foreach ($lists as $lk => $lv) {
                $jobs[$lv['id']]    =   $lv;
            }
            foreach ($jobid_arr as $jk => $jv) {
                $joblists[]         =   $jobs[$jv];
            }
            $this->yunset('lists', $joblists);

            $wxpubtempM =   $this->MODEL('wxpubtemp');
            $temp       =   $wxpubtempM->getTemp(array('id' => $tpl), array('field' => '`id`,`temptype`'));
            $notoBr     =   $temp['temptype'] == 1 ? true : false;

            //风格模板
            $res        =   $this->mk_temp($tpl, array('notoBr' => $notoBr));
            if ($res) {

                $this->yuntpl(array($res));
            }
        }
    }
    function comtwTask_action()
    {
        
        $wxpubtempM =   $this->MODEL('wxpubtemp');

        $where['type']      =   2;
        if ($_POST['keyword']) {

            $keyword    =   trim($_POST['keyword']);
            $comM       =   $this->MODEL('company');

            $comid_arr      =   array();

            if (is_numeric($keyword)) {

                $comid_arr  =   array($keyword);
            } else {

                $comwhere   =   array();

                $comwhere['name']   =   array('like', $keyword);

                $comList            =   $comM->getChCompanyList($comwhere, array('field' => '`uid`'));
                foreach ($comList as $ck => $cv) {

                    $comid_arr[]    =   $cv['uid'];
                }
            }

            $where['PHPYUNBTWSTART_B']  =   '';
            $where['cuid']              =   array('in', pylode(',', $comid_arr));
            $where['content']           =   array('like', $keyword, 'or');
            $where['PHPYUNBTWEND_B']    =   '';

        }

        if ($_POST['auid']) {
            $where['auid']  =   $_POST['auid'];
        }

        if ($_POST['status']) {

            if ($_POST['status'] == '1') {

                $where['status'] = $_POST['status'];
            } elseif ($_POST['status'] == '2') {

                $where['status'] = '0';
            }

        }

        if ($_POST['urgent']) {
            $where['urgent'] = $_POST['urgent'];
        }

        if ($_POST['wcmoments']) {
            $where['wcmoments'] = $_POST['wcmoments'];
        }

        if ($_POST['gzh']) {
            $where['gzh'] = $_POST['gzh'];
        }

        $pageM          =   $this->MODEL('page');
        $pages          =   $pageM->adminPageList('wxpub_twtask', $where,$_POST['page'],array('limit'=>$_POST['limit']));

        if ($pages['total'] > 0) {

            if ($_POST['order']) {

                $where['orderby']   =   $_POST['t'] . ',' . $_POST['order'];
            } else {
                $where['orderby']   =   'ctime';
            }
            
            $where['limit']     =   $pages['limit'];

            
            $twtasks    =   $wxpubtempM->getTwTaskList($where);
        }
        
        $data['list']           =   !empty($twtasks)?$twtasks:array();
        $data['total']          =   $pages['total'];
        $data['page_sizes']     =   $pages['page_sizes'];
        $data['page_size']     =   $pages['page_size'];
        $this->render_json(0, '', $data);
    }

    function comtwTask_base_data_action()
    {
        $wxpubtempM = $this->MODEL('wxpubtemp');
        $adminM = $this->MODEL('admin');
        $adminList = $adminM->getList(array('orderby' => 'uid'), array('field' => '`uid`,`username`,`name`'));
        $this->yunset('adminList', $adminList);

        $tempWhere['temptype'] = 1;
        $tempWhere['type'] = 'company';
        $tempWhere['orderby'] = 'id,asc';

        $temps = $wxpubtempM->getTempList($tempWhere, '`id`,`title`');

        $temp2Where = array(
            'temptype' => 0,
            'type' => 'company',
            'orderby' => 'id,asc'
        );
        $temps2 = $wxpubtempM->getTempList($temp2Where, '`id`,`title`');

        $data['temps'] = !empty($temps) ? $temps : array();
        $data['temps2'] = !empty($temps2) ? $temps2 : array();
        $data['adminList'] = !empty($adminList) ? $adminList : array();
        $this->render_json(0, '', $data);
    }
    function getComTW_action()
    {

        $get    =   $_POST;
        //查询类型 0职位列表1简历列表2企业列表
        $type   =   $get['type'];
        
        global $db_config;
        $companyM = $this->MODEL('company');
        $data  = array();
        $time  = time();

        $comwhere = array(
            'r_status' => 1,
            'name'=>array('<>','')
        );

        $data['jWhere']['r_status'] =   1;
        $data['jWhere']['status']   =   0;
        $data['jWhere']['state']    =   1;
        $data['jWhere']['orderby']  =   "lastupdate,desc";
        

        //指定企业职位 企业ID
        $comuids = str_replace('，',',',$_POST['cuids']);
       
        $comwhere['uid']= array('in',$comuids);
        
        $comwhere['orderbyfield'] = array('uid',$comuids);
        
        $lists = $companyM->Getcompany($comwhere,$data);

        
        $this->yunset('lists',$lists);

        $wxpubtempM =   $this->MODEL('wxpubtemp');
        $temp       =   $wxpubtempM->getTemp(array('id' => $_POST['tpl']), array('field' => '`id`,`temptype`'));
        $notoBr     =   $temp['temptype'] == 1 ? true : false;

        //风格模板
        $res        =   $this->mk_temp($_POST['tpl'], array('notoBr' => $notoBr));
        if($res){
            $this->yuntpl(array($res));
        }

    }
}
?>