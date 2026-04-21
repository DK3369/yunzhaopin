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

class hrlog_model extends model
{
    private $night = array();
    private $zuiwan = 0;
    private $lastwork = '';
    /**
     * 给单个企业生成HR报告
     */
    function sethrlog($uid, $data = array()){
        
        $thisyear = date('Y',time());
        $day1 = strtotime($thisyear. '-01-01');
        
        $row = $this->gethrlog(array('uid'=>$uid));

        $update = false;
        if($data['adminup']){
            $update = true;
        }else if($data['update'] && $row['uptime']<$day1){
            $update = true;
        }
        
        if (!empty($row) && !$update){
            return $row;
        }
        
        $lastyear = date('Y', strtotime('-1 year'));
        $start = strtotime($lastyear. '-01-01');
        $end   = strtotime($lastyear. '-12-31');
        
        $this->everyday();
        // 发布职位数量
        $jobW = array('uid' => $uid);
        $jobW['PHPYUNBTWSTART'] = '';
        $jobW['lastupdate'][] = array('>', $start);
        $jobW['lastupdate'][] = array('<', $end);
        $jobW['PHPYUNBTWEND'] = '';
        
        $jobNum = $this->select_num('company_job',$jobW);
        
        // 职位浏览人数
        $lookjobW = array('com_id' => $uid);
        $lookjobW['PHPYUNBTWSTART'] = '';
        $lookjobW['datetime'][] = array('>', $start);
        $lookjobW['datetime'][] = array('<', $end);
        $lookjobW['PHPYUNBTWEND'] = '';
        
        $lookjobNum = $this->select_num('look_job', $lookjobW);
        
        // 查看简历
        $lookresumeW = array('com_id'=>$uid);
        $lookresumeW['PHPYUNBTWSTART'] = '';
        $lookresumeW['datetime'][] = array('>', $start);
        $lookresumeW['datetime'][] = array('<', $end);
        $lookresumeW['PHPYUNBTWEND'] = '';
        
        $lookresume = $this->select_all('look_resume', $lookresumeW, 'datetime');
        $lookresumeNum = count($lookresume);
        
        $lastLook = 0;
        $nightLook = array();
        if (!empty($lookresume)){
            foreach ($lookresume as $lrv){
                // 处理时间
                foreach ($this->night as $val){
                    if ($lrv['datetime'] > $val['stime'] && $lrv['datetime'] < $val['etime']){
                        $nightLook[] = $lrv['datetime'];
                    }
                }
            }
            if (!empty($nightLook)){
                $lastLook = $this->getNightWork($nightLook);
            }
        }
        
        // 收到简历（最受欢迎岗位）
        $sqW = array('com_id'=>$uid);
        $sqW['PHPYUNBTWSTART'] = '';
        $sqW['datetime'][] = array('>', $start);
        $sqW['datetime'][] = array('<', $end);
        $sqW['PHPYUNBTWEND'] = '';
        $sqW['isdel'] = 9;
        $sqW['groupby'] = 'job_id';
        
        $sqlist = $this->select_all('userid_job',$sqW, 'count(*) AS `sqnum`,`job_id`');
        
        $mostjob = 0;
        $sqjobNum = 0;
        if (!empty($sqlist)){
            foreach ($sqlist as $v){
                if ($v['sqnum'] > $mostjob){
                    $mostjob = $v['sqnum'];
                }
                $sqjobNum += $v['sqnum'];
            }
        }
        // 邀请人才
        $yqW = array('fid'=>$uid);
        $yqW['PHPYUNBTWSTART'] = '';
        $yqW['datetime'][] = array('>', $start);
        $yqW['datetime'][] = array('<', $end);
        $yqW['PHPYUNBTWEND'] = '';
        
        $yq = $this->select_all('userid_msg', $yqW, 'datetime');
        $yqNum = count($yq);
        
        $lastYq = 0;
        $nightYq = array();
        if (!empty($yq)){
            foreach ($yq as $yv){
                // 处理时间
                foreach ($this->night as $val){
                    if ($yv['datetime'] > $val['stime'] && $yv['datetime'] < $val['etime']){
                        $nightYq[] = $yv['datetime'];
                    }
                }
            }
            if (!empty($nightYq)){
                $lastYq = $this->getNightWork($nightYq);
            }
        }
        // 下载简历数
        $downW = array('comid'=>$uid);
        $downW['PHPYUNBTWSTART'] = '';
        $downW['downtime'][] = array('>', $start);
        $downW['downtime'][] = array('<', $end);
        $downW['PHPYUNBTWEND'] = '';
        
        $down = $this->select_all('down_resume', $downW, 'downtime');
        $downNum = count($down);
        
        $lastDown = 0;
        $nightDown = array();
        if (!empty($down)){
            foreach ($down as $dv){
                // 处理时间
                foreach ($this->night as $val){
                    if ($dv['downtime'] > $val['stime'] && $dv['downtime'] < $val['etime']){
                        $nightDown[] = $dv['downtime'];
                    }
                }
            }
            if (!empty($nightDown)){
                $lastDown = $this->getNightWork($nightDown);
            }
        }
        
        // 登录天数 XX网陪伴您232天
        $loginW = array('uid' => $uid);
        $loginW['PHPYUNBTWSTART'] = '';
        $loginW['ctime'][] = array('>', $start);
        $loginW['ctime'][] = array('<', $end);
        $loginW['PHPYUNBTWEND'] = '';
        $loginNum = $this->select_num('login_log', $loginW);
        
        // 登录次数（最晚一次登录）
        $logW = array('uid' => $uid, 'content'=>array('like', '登录成功'));
        $logW['PHPYUNBTWSTART'] = '';
        $logW['ctime'][] = array('>', $start);
        $logW['ctime'][] = array('<', $end);
        $logW['PHPYUNBTWEND'] = '';
        $log = $this->select_all('member_log', $logW, 'ctime');
        
        $lastLog = 0;
        $nightLog = array();
        if (!empty($log)){
            foreach ($log as $lv){
                // 处理时间
                foreach ($this->night as $val){
                    if ($lv['ctime'] > $val['stime'] && $lv['ctime'] < $val['etime']){
                        $nightLog[] = $lv['ctime'];
                    }
                }
            }
            if (!empty($nightLog)){
                $lastLog = $this->getNightWork($nightLog);
            }
        }
        // 深夜工作天数
        $nightArr = array();
        $nightWork = array_merge($nightLog, $nightLook, $nightYq);
        foreach ($nightWork as $nv){
            $nightArr[] = date('Y-m-d', $nv);
        }
        $nightWorkDay = count(array_unique($nightArr));
        // 添加HR工作报告
        $addData = array(
            'uid'        => $uid,
            'job'        => $jobNum,  // 发布职位数量
            'lookjob'    => $lookjobNum, // 职位浏览人数
            'lookresume' => $lookresumeNum, // 查看简历数
            'sqjob'      => $sqjobNum, // 收到简历数
            'yq'         => $yqNum + $downNum, // 邀请面试数  + 下载简历数
            'login'      => $loginNum, // 登录天数
            'nightwork'  => $nightWorkDay, // 深夜工作天数
            'lastwork'   => $this->lastwork, // 最晚一次工作时间
            'uptime'      => time()
        );
        if(!empty($row)){

            if($update){
                $this->uphrlog(array('id'=>$row['id']),$addData);
            }

        }else{
            $addData['ctime'] = time();
            $this->addHrlog($addData);
        }
        // 返回报告中企业信息数据
        if (isset($data['back'])){
            $com = $this->select_once('company', array('uid'=>$uid), 'name,linkman');
            $addData['com_name'] = $com['name'];
            $addData['linkman']  = $com['linkman'];
        }
        
        return $addData;
    }
    // 获取夜间工作记录
    function getNightWork($worklog){
        $zuiwan = 0;    // 最晚时间统计数
        $lastwork = '';  // 最晚工作时间
        foreach ($worklog as $v){
            $time4 = strtotime(date('Y-m-d', $v).' 04:00');
            $time22 = strtotime(date('Y-m-d', $v).' 22:00');
            //  发送时间，减去当天（前一天）22点的，得到的数字越大，说明越晚
            if ($v < $time4){
                // 当前凌晨4点之前的
                // 前一天22点
                $yes22 = strtotime(date('Y-m-d', strtotime('-1 day', $v)). ' 22:00');
                $res = $v - $yes22;
            }elseif ($v > $time22){
                // 当天22点之后的
                $res = $v - $time22;
            }
            // 单项最晚时间
            if (isset($res) && $res > $zuiwan){
                $zuiwan = $res;
                $lastwork = $v;
            }
            // 全局最晚工作时间
            if (isset($res) && $res > $this->zuiwan){
                $this->zuiwan = $res;
                $this->lastwork = $v;
            }
        }
        return $lastwork;
    }
    // 每天22点到凌晨4点
    function everyday(){
        
        $lastyear = date('Y', strtotime('-1 year'));
        // 去年1月1号
        $start = $lastyear. '-01-01';
        $starttime = strtotime($start);
        // 去年12月31号
        $end = $lastyear. '-12-31';
        $endtime = strtotime($end);
        
        $this->nextday($starttime, $endtime);
    }
    function nextday($time, $endtime){
        
        $nextday = '';
        
        if ($time < $endtime){
            $nextday = strtotime('+1 day',$time);
            
            $stime = date('Y-m-d', $time).' 22:00';
            $etime = date('Y-m-d',$nextday).' 04:00';
            
            $arr = array(
                'stime' => strtotime($stime),
                'etime' => strtotime($etime)
            );
            array_push($this->night, $arr);
            
            $this->nextday($nextday, $endtime);
        }
    }
    /**
     * @desc 添加HR年度工作报告
     */
    public function addHrlog($data = array())
    {
        
        $this->insert_into('hr_log', $data);
    }
    /**
     * @desc 更新hr_log
     */
    function uphrlog($whereData, $data = array()){
        
        return $this->update_once('hr_log', $data, $whereData);
    }
    /**
     * @desc 获取hr_log 列表
     */
    public function gethrlogList($whereData, $data = array())
    {
        
        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $List           =   $this->select_all('hr_log', $whereData, $data['field']);
        
        if (!empty($List)){
            foreach ($List as $v){
                $uids[] = $v['uid'];
            }
            $com = $this->select_all('company', array('uid'=>array('in', pylode(',', $uids))), 'uid,name');
            foreach ($List as $k=>$v){
                foreach ($com as $val){
                    if ($v['uid'] == $val['uid']){
                        $List[$k]['com_name'] = $val['name'];
                    }
                }
                !empty($v['lastwork']) && $List[$k]['lastwork_n'] = date('Y-m-d H:i:s', $v['lastwork']);
            }
        }
        
        return $List;
    }
    /**
     * @desc 获取hr_log 单条
     */
    public function gethrlog($whereData, $data = array())
    {
        
        $data['field']  =   empty($data['field']) ? '*' : $data['field'];
        $row            =   $this->select_once('hr_log', $whereData, $data['field']);
        
        if (!empty($row)){
            $com = $this->select_once('company', array('uid'=>$row['uid']), 'name,linkman');
            $row['com_name'] = $com['name'];
            $row['linkman']  = $com['linkman'];
        }
        
        return $row;
    }
    private function getComInfo($where, $data = array())
    {

        require_once('company.model.php');
        $ComM   =   new company_model($this->db, $this->def);
        return $ComM->getInfo($where, $data);
    }
    function getLastYearReportHb($data=array()){

        if (!empty($data)) {

            $comId      =   $data['comid'];

            $com        =   $this->getComInfo($comId, array('field' => 'uid, name, shortname, logo,linkman', 'logo' => 1));
            if (!empty($com['shortname'])){

                $com['name']    =   $com['shortname'];
            }
            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                $com['logo_n']  =   str_replace($this->config['sy_weburl'] . '/data/', DATA_PATH, $com['logo']);
            }else{
                $com['logo_n']  =   checkpic($com['logo']);
            }

            $hrlog = $this->sethrlog($comId,array('update'=>true));

            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

                $imgFile = str_replace($this->config['sy_weburl'] . '/data/', DATA_PATH, $this->config['sy_yearreport_pic']);
            }else{

                $imgFile = checkpic($this->config['sy_yearreport_pic']);//背景图
            }
            $loginday_text = $hrlog['login'];//登陆天数
            $jobnum_text = $hrlog['job'];//发布职位数
            $hitnum_text = $hrlog['lookjob'];//被浏览次数
            $resumenum_text = $hrlog['sqjob'];//收到简历数
            //$resumelooknum_text = 365;//查看简历数
            $invitenum_text = $hrlog['yq'];//邀请人才数
            $nightnum_text = $hrlog['nightwork'];//深夜作业次数

            if($hrlog['lastwork']){

                $day = date('Y-m-d',$hrlog['lastwork']);
                $time = strtotime($day.' 22:00:00');
                
                $daystr = date('m月d日',$hrlog['lastwork']);

                $daystr1 = '深夜';

                if($time>$hrlog['lastwork']){
                    $daystr1 = ' 凌晨';
                }
                
                $daystr2 = ' '.date('H:i',$hrlog['lastwork']);

                $night_text = $daystr.$daystr1.$daystr2;//'12月05日 凌晨 01:08';//最晚深夜作业描述
            }
            

            $comname_text = $com['name'];//企业名称
            $hr_text = $com['linkman'];//企业联系人
            $com_logo = $com['logo_n'];//企业logo
            


            $textArr = $imageArr = array();

            //底部企业信息部分
            $hr_text_arr = array(
                'left'      =>  250,
                'top'       =>  1740,
                'fontSize'  =>  40,
                'fontColor' =>  '0,0,0',
                'width'     =>  540
            );

            $textArr[] = $this->diyWidStr($hr_text_arr,$hr_text);

            $comname_text_arr = array(
                'left'      =>  250,
                'top'       =>  1810,
                'fontSize'  =>  30,
                'fontColor' =>  '0,0,0',
                'width'     =>  540
            );
            $textArr[] = $this->diyWidStr($comname_text_arr,$comname_text);

            $imageArr[] = array(
                'url'       =>  Url('wap', array('c' => 'company', 'a' => 'show', 'id' => $comId)),        //二维码资源
                'qrcode'    =>  array('c'=>'company','id'=>$comId,'type'=>'company'),
                'stream'    =>  0,
                'left'      =>  800,
                'top'       =>  1650,
                'right'     =>  0,
                'bottom'    =>  0,
                'width'     =>  200,
                'height'    =>  200,
                'opacity'   =>  100
            );
            $imageArr[] = array(
                'url'       =>  $com_logo,  //  企业LOGO
                'stream'    =>  0,
                'left'      =>  80,
                'top'       =>  1680,
                'right'     =>  0,
                'bottom'    =>  0,
                'width'     =>  150,
                'height'    =>  150,
                'opacity'   =>  100
            );
            //底部企业信息部分end


            $fontColor1 = '255,255,255';//填充字符的字体颜色
            $fontSize1 = 30;//填充字符的字体大小 磅值 不是像素
            $bold1 = 0;//字体加粗维度

            $fontColor2 = '255,102,0';//填充字符的字体颜色
            $fontSize2 = 40;//填充字符的字体大小 磅值 不是像素
            $bold2 = 1;//字体加粗维度

            $top = 400;

            $textArr[] = array(
                'text'      =>  '这一年',
                'left'      =>  80,
                'top'       =>  $top,
                'fontSize'  =>  $fontSize1,
                'fontColor' =>  $fontColor1,
                'bold'      =>  $bold1
            );
            if($jobnum_text){
                $top += 80;
                //发布职位数
                $job_txtdata[] = array(
                    'text'      =>  '您发布了',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $job_txtdata[] = array(
                    'text'      =>  $jobnum_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize2,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $job_txtdata[] = array(
                    'text'      =>  '个岗位',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $job_txt_arr = $this->multiTxtOneline(array('txtdata'=>$job_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$job_txt_arr);
            }    
            //发布职位数end
            if($hitnum_text){
                $top += 80;
                //被浏览数
                $hit_txtdata[] = array(
                    'text'      =>  '吸引',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $hit_txtdata[] = array(
                    'text'      =>  $hitnum_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize2,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $hit_txtdata[] = array(
                    'text'      =>  '位人才浏览',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $hit_txt_arr = $this->multiTxtOneline(array('txtdata'=>$hit_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$hit_txt_arr);
                //被浏览数end
            }

            if($resumenum_text){
                $top += 80;
                //收到简历数
                $resumenum_txtdata[] = array(
                    'text'      =>  '收到',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $resumenum_txtdata[] = array(
                    'text'      =>  $resumenum_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize2,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $resumenum_txtdata[] = array(
                    'text'      =>  '份求职简历',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $resumenum_txt_arr = $this->multiTxtOneline(array('txtdata'=>$resumenum_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$resumenum_txt_arr);
            }
            
            //收到简历数end
            if($invitenum_text){
                $top += 80;
                //邀请人才数
                $invitenum_txtdata[] = array(
                    'text'      =>  '邀约',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $invitenum_txtdata[] = array(
                    'text'      =>  $invitenum_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize2,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $invitenum_txtdata[] = array(
                    'text'      =>  '位人才面试',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  $fontSize1,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $invitenum_txt_arr = $this->multiTxtOneline(array('txtdata'=>$invitenum_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$invitenum_txt_arr);
            }
            

            if($nightnum_text){
                $top += 160;
                //深夜作业次数
                $nightnum_txtdata[] = array(
                    'text'      =>  '有',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $nightnum_txtdata[] = array(
                    'text'      =>  $nightnum_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $nightnum_txtdata[] = array(
                    'text'      =>  '天 深夜还在持续工作',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $nightnum_txt_arr = $this->multiTxtOneline(array('txtdata'=>$nightnum_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$nightnum_txt_arr);
                //深夜作业次数end
            }
            
            if($night_text){
                $top += 80;
                $textArr[] = array(
                    'text'      =>  '最晚一次在',
                    'left'      =>  80,
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $top += 80;
                //最晚描述
                $textArr[] = array(
                    'text'      =>  $night_text,
                    'left'      =>  80,
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  0
                );
            }
            
            if($loginday_text){
                $top += 80;
                //登陆天数
                $loginnum_txtdata[] = array(
                    'text'      =>  '致敬我们一起努力过的',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $loginnum_txtdata[] = array(
                    'text'      =>  $loginday_text,
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor2,
                    'bold'      =>  $bold2
                );
                $loginnum_txtdata[] = array(
                    'text'      =>  '天',
                    'left'      =>  '',
                    'top'       =>  $top,
                    'fontSize'  =>  25,
                    'fontColor' =>  $fontColor1,
                    'bold'      =>  $bold1
                );
                $loginnum_txt_arr = $this->multiTxtOneline(array('txtdata'=>$loginnum_txtdata,'offstep'=>80));
                $textArr = array_merge($textArr,$loginnum_txt_arr);
                //登录天数end
            }
            
            
           
            $imgConfig  =   array(
                'background'    =>  $imgFile,
                'image'         =>  $imageArr,
                'text'          =>  $textArr
            );

            echo $this->createPoster($imgConfig);
        }
    }
     //只取自定义长度的字符串
    private function diyWidStr($pos, $str)
    {
        $return  = array();

        $_str_h = $pos["top"];
        $fontsize = $pos["fontSize"];
        $fontcolor = $pos["fontcolor"]?$pos["fontcolor"]:'0,0,0';
        $width = $pos["width"];
        $margin_lift = $pos["left"];
        $temp_string = "";
        $font_file = TPL_PATH . 'wap/hb_job/OPPOSans-M.ttf';
        
        for ($i = 0; $i < mb_strlen($str); $i++) {

            $box = imagettfbbox($fontsize, 0, $font_file, $temp_string);

            $_string_length = $box[2] - $box[0];
            $temptext = mb_substr($str, $i, 1,'utf-8');

            $temp = imagettfbbox($fontsize, 0, $font_file, $temptext);

            if ($_string_length + $temp[2] - $temp[0] < $width) {//长度不够，字数不够，需要

                //继续拼接字符串。

                $temp_string .= mb_substr($str, $i, 1,'utf-8');

                if ($i == mb_strlen($str) - 1) {//是不是最后半行。不满一行的情况

                    $return = array(
                        'text'      =>  $temp_string,
                        'left'      =>  $margin_lift,
                        'top'       =>  $_str_h,
                        'fontSize'  =>  $fontsize,         //字号
                        'fontColor' =>  $fontcolor   //字体颜色
                    );
                    break;
                }

            } else {//长度够了 返回。

                $i--;

                $tmp_str_len = mb_strlen($temp_string);
                $s = mb_substr($temp_string, $tmp_str_len-1, 1,'utf-8');//取零时字符串最后一位字符

                $return = array(
                    'text'      =>  $temp_string,
                    'left'      =>  $margin_lift,
                    'top'       =>  $_str_h,
                    'fontSize'  =>  $fontsize,         //字号
                    'fontColor' =>  $fontcolor   //字体颜色
                );
                break;
            }
            
        }

        return $return;
    }
    //将多个不同样式的文字合并在一行
    private function multiTxtOneline($data=array()){
        
        $offstep = !empty($data['offstep'])?$data['offstep']:0;//左侧起点的偏移量
        
        $return  =array();

        if(!empty($data['txtdata'])){

            $text_left = $offstep;

            foreach ($data['txtdata'] as $k => $v) {

                $text_fontSize = $v['fontSize'];

                $text = $v['text'];

                $v['left'] = $text_left;
                
                $box = imagettfbbox($text_fontSize,0,TPL_PATH . 'wap/hb_job/OPPOSans-M.ttf',$text);
                $text_left += floor($box[2]-$box[0])+20;
                
                $return[] = $v;
            }
        }
        return $return;
    }
    function createPoster($config = array())
    {

        $imageDefault   =   array(
            'left'      =>  0,
            'top'       =>  0,
            'right'     =>  0,
            'bottom'    =>  0,
            'width'     =>  100,
            'height'    =>  100,
            'opacity'   =>  100
        );
        $textDefault    =   array(
            'text'      =>  '',
            'left'      =>  0,
            'top'       =>  0,
            'fontSize'  =>  32,                 //  字号
            'fontColor' =>  '255,255,255',      //  字体颜色
            'fontPath'  =>  TPL_PATH . 'wap/hb_job/OPPOSans-M.ttf',    //  字体路径
            'angle'     =>  0,
        );

        $backgroundconfig     =   $config['background'];//海报最底层得背景
	
		if (isset($backgroundconfig)) {
				
                $background_n   =   checkpic($backgroundconfig);
				
        }
		if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

			$background    =   str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $background_n);
		} else {

			$background    =   $background_n;
		}

        if (stripos($background, 'http') !== false) {

            //背景方法
            $backgroundInfo =   curlGet($background);
            $background     =   imagecreatefromstring($backgroundInfo);
        } else {

            //背景方法
            $backgroundInfo =   getimagesize($background);
            $backgroundFun  =   'imagecreatefrom'.image_type_to_extension($backgroundInfo[2], false);
            $background     =   $backgroundFun($background);
        }

        $backgroundWidth    =   imagesx($background);    //背景宽度
        $backgroundHeight   =   imagesy($background);   //背景高度

        $imageRes   =   imageCreatetruecolor($backgroundWidth, $backgroundHeight);
        $color      =   imagecolorallocate($imageRes, 255, 255, 255);

        imagefill($imageRes, 0, 0, $color);

        imagecopyresampled($imageRes, $background, 0, 0, 0, 0, imagesx($background), imagesy($background), imagesx($background), imagesy($background));

        //处理了图片
        if (!empty($config['image'])) {
            foreach ($config['image'] as $key => $val) {
                $canvas = false;
                $val    = array_merge($imageDefault, $val);

                if (isset($val['qrcode'])){
                    // 二维码图单独处理
                    if($this->config['sy_yearreport_ewmtype'] == 'xcxQrcode'){
                        $qrcode  = $val['qrcode'];
                        $val['url'] =  Url('ajax', array('c' => 'xcxQrcode','id'=>$qrcode['id'],'type' =>$qrcode['type']));
                    }else if ($this->config['sy_yearreport_ewmtype'] == 'weixin' || $this->config['sy_yearreport_ewmtype'] == 'xcx'){
                        // 场景码
                        $qrcode  = $val['qrcode'];
                        // 只有一个二维码，暂时先在这里申明model
                        require_once('weixin.model.php');
                        $WxM        = new weixin_model($this->db, $this->def);
                        $val['url'] = $WxM->pubWxQrcode($qrcode['c'],$qrcode['id'],$this->config['sy_yearreport_ewmtype']);

                    }else{
                        // 默认二维码，需特殊处理，直接在本model里生成，不调用url
                        $resWidth   =   $val['width'];
                        $resHeight  =   $val['height'];

                        //建立画板
                        $canvas = $this->qrcode($val['url'], $val['width'], $val['height']);
                        $val['url'] =   '';
                    }
                }
                if (!empty($val['url'])){
                    // 判断一下url，默认二维码不需要走这里
                    if ($val['stream']) {

                        $info       =   getimagesizefromstring($val['url']);
                        $function   =   'imagecreatefromstring';
                        $res        =   $function($val['url']);
                    } else if (stripos($val['url'], 'http') !== false) {

                        $urlInfo    =   curlGet($val['url']);
                        if (!empty($urlInfo)) {
                            $info   =   getimagesizefromstring($urlInfo);
                        }
                        $function   =   'imagecreatefromstring';
                        $res        =   $function($urlInfo);
                    } else {

                        $info       =   getimagesize($val['url']);
                        if ($info) {

                            $function   =   'imagecreatefrom'.image_type_to_extension($info[2], false);
                            $res        =   $function($val['url']);
                        }
                    }

                    // 要判断一下，防止图片不存在，导致报错
                    if (!empty($info)) {

                        $resWidth   =   $info[0];
                        $resHeight  =   $info[1];

                        //建立画板 ，缩放图片至指定尺寸
                        $canvas     =   imagecreatetruecolor($val['width'], $val['height']);
                    }
                }
                if (isset($canvas)){
                    if(!empty($val['c_opacity'])){
                        $mycolor = imagecolorallocatealpha($canvas, 255, 255, 255, 127); // 为一幅图像分配颜色和透明度。
                        imagecolortransparent($canvas,$mycolor); //3.设置透明色
                        imagefill($canvas,0,0,$mycolor);//4.填充透明色
                    }else{
                        imagefill($canvas, 0, 0, $color);
                    }

                    //关键函数，参数（目标资源，源，目标资源的开始坐标x,y, 源资源的开始坐标x,y,目标资源的宽高w,h,源资源的宽高w,h）
                    imagecopyresampled($canvas, $res, 0, 0, 0, 0, $val['width'], $val['height'], $resWidth, $resHeight);
                    $val['left']=   $val['left'] < 0 ? $backgroundWidth - abs($val['left']) - $val['width'] : $val['left'];
                    $val['top'] =   $val['top'] < 0 ? $backgroundHeight - abs($val['top']) - $val['height'] : $val['top'];

                    //放置图像
                    imagecopymerge($imageRes, $canvas, $val['left'], $val['top'], $val['right'], $val['bottom'], $val['width'], $val['height'], $val['opacity']);//左，上，右，下，宽度，高度，透明度
                }
            }
        }

        //处理文字
        if (!empty($config['text'])) {
            foreach ($config['text'] as $key => $val) {

                $val    =   array_merge($textDefault, $val);

                list($R, $G, $B)    =   explode(',', $val['fontColor']);
                $fontColor          =   imagecolorallocate($imageRes, $R, $G, $B);
                $val['left']        =   $val['left'] < 0 ? $backgroundWidth - abs($val['left']) : $val['left'];
                $val['top']         =   $val['top'] < 0 ? $backgroundHeight - abs($val['top']) : $val['top'];
                imagettftext($imageRes, $val['fontSize'], $val['angle'], $val['left'], $val['top'], $fontColor, $val['fontPath'], $val['text']);

                $bold = intval($val['bold']);

                if($bold>0){
                    for ($i=1; $i <= $bold; $i++) { 
                        imagettftext($imageRes, $val['fontSize'], $val['angle'], $val['left']+$i, $val['top'], $fontColor, $val['fontPath'], $val['text']);
                    }
                }
                
                
            }
        }

        if (!$config['out']){
            //在浏览器上显示
            header("Content-type: image/png");
            imagejpeg($imageRes);
            imagedestroy($imageRes);
        }else{

            if (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 1) {

                ob_start();
                imagejpeg($imageRes);
                $img    =   ob_get_contents();
                ob_end_clean();
                $base64 =   base64_encode($img);

                require_once('upload.model.php');
                $uploadM    =   new upload_model($this->db, $this->def);

                $upArr = array(
                    'base' => $base64,
                    'dir' => 'company',
                    'type' => 'logo',
                    'watermark' => 0
                );
                $return =   $uploadM->newUpload($upArr);
                return $return['picurl'];
            }else{
                $dir        =   'data/upload/company/'.date("Ymd");
                $dirname    =   APP_PATH.$dir;
                if(!file_exists($dirname)){
                    mkdir($dirname);
                }
                $imgName    =   '/'.time().'.png';
                $filename   =   $dirname.$imgName;
                imagepng($imageRes, $filename);
                return './'.$dir.$imgName;
            }
        }
    }
    // 海报专用二维码生成函数
    private function qrcode($inputContent, $printW = 0, $printhH = 0, $pixelPerPoint = 4, $outerFrame = 4, $back_color = 0xFFFFFF, $fore_color = 0x000000)
    {

        require_once LIB_PATH."phpqrcode.php";
        $enc = QRencode::factory(QR_ECLEVEL_L, 3, 4, $back_color, $fore_color);
        ob_start();
        $frame = $enc->encode($inputContent);
        ob_end_clean();

        $h = count($frame);
        $w = strlen($frame[0]);

        $imgW = $w + 2*$outerFrame;
        $imgH = $h + 2*$outerFrame;

        $base_image =ImageCreate($imgW, $imgH);

        // convert a hexadecimal color code into decimal format (red = 255 0 0, green = 0 255 0, blue = 0 0 255)
        $r1 = round((($fore_color & 0xFF0000) >> 16), 5);
        $g1 = round((($fore_color & 0x00FF00) >> 8), 5);
        $b1 = round(($fore_color & 0x0000FF), 5);

        // convert a hexadecimal color code into decimal format (red = 255 0 0, green = 0 255 0, blue = 0 0 255)
        $r2 = round((($back_color & 0xFF0000) >> 16), 5);
        $g2 = round((($back_color & 0x00FF00) >> 8), 5);
        $b2 = round(($back_color & 0x0000FF), 5);

        $col[0] = ImageColorAllocate($base_image, $r2, $g2, $b2);
        $col[1] = ImageColorAllocate($base_image, $r1, $g1, $b1);

        imagefill($base_image, 0, 0, $col[0]);

        for($y=0; $y<$h; $y++) {
            for($x=0; $x<$w; $x++) {
                if ($frame[$y][$x] == '1') {
                    ImageSetPixel($base_image,$x+$outerFrame,$y+$outerFrame,$col[1]);
                }
            }
        }

        $target_image =ImageCreate($printW, $printW);
        ImageCopyResized($target_image, $base_image, 0, 0, 0, 0, $printW, $printhH, $imgW, $imgH);

        ImageDestroy($base_image);

        return $target_image;
    }
}