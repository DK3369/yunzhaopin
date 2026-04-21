<?php

/**
 * $Author ：PHPYUN开发团队
 *
 * 官网: http://www.phpyun.com
 *
 * 版权所有 2009-2021 宿迁鑫潮信息技术有限公司，并保留所有权利。
 *
 * 软件声明：未经授权前提下，不得用于商业运营、二次开发以及任何形式的再次发布。
 */

class whb_model extends model
{

    public function getWhbList($whereData = array(), $data = array())
    {

        $list   =   array();

        if (!empty($whereData)) {

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

			if(!isset($whereData['orderby'])){
                $whereData['orderby']   =   array('num,desc', 'sort,desc');
            }

            $list           =   $this->select_all('admin_jobwhb', $whereData, $data['field']);

			$style  =   array();
            foreach ($list as $key => $value) {

                if ($value['pic']) {
                    $list[$key]['pic_n']    =   checkpic($value['pic']);
                }
                if ($data['only']==1){
                    if (!in_array($value['style'], $style)){
                        $style[]    =   $value['style'];
                    }else{
                        unset($list[$key]);
                    }
                }
            }
        }
        return $list;
    }

    public function getWhb($whereData = array(), $data = array())
    {

        if (!empty($whereData)) {

            $data['field']  =   empty($data['field']) ? '*' : $data['field'];

            $whb            =   $this->select_once('admin_jobwhb', $whereData, $data['field']);

            if (isset($whb['pic'])) {

                $whb['pic_n']   =   checkpic($whb['pic']);
            }
            return $whb;
        }
    }

    public function updateWhb($updata = array(), $whereData = array())
    {

        if (!empty($whereData)) {

            $return = $this->update_once('admin_jobwhb', $updata, $whereData);
        }
    }

    public function delWhb($whereData = array())
    {

        if (!empty($whereData)) {

            $return =   $this->delete_all('admin_jobwhb', $whereData, '');
        }
    }

    public function setWhb($updata = array(), $whereData = array())
    {

        if (trim($updata['name']) == '') {

            return array('msg' => '海报名称不能为空！', 'errcode' => 8);
        }

        if (!empty($whereData)) {

            $return['id']   =   $this->update_once('admin_jobwhb', $updata, $whereData);

            $return['msg']  =   '微海报更新';
        } else {

            $return['id']   =   $this->insert_into('admin_jobwhb', $updata);

            $return['msg']  =   '微海报添加';
        }

        $return['errcode']  =   $return['id'] ? '9' : '8';

        $return['msg']      =   $return['id'] ? $return['msg'] . '成功！' : $return['msg'] . '失败！';

        return $return;
    }

    function getJobHb($data = array())
    {

        if (!empty($data)) {

            $whb    =   $this->getWhb(array('id' => $data['hb']));

            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

                $imgFile    =   str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $whb['pic_n']);
            } else {

                $imgFile    =   $whb['pic_n'];
            }

            $jobId  =   $data['id'];

            $job    =   $this->getJobInfo(array('id' => $jobId, 'status' => 0, 'state' => 1));

            if (!empty($job) && is_array($job)) {

                $jobCity    =   !empty($job['job_city_two']) ? $job['job_city_two'] : $job['job_city_one'];
                $jobExp     =   !empty($job['job_exp']) ? $job['job_exp'] : '经验不限';
                $jobEdu     =   !empty($job['job_edu']) ? $job['job_edu'] : '学历不限';

                if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                    $job['com_logo_n']  =   str_replace($this->config['sy_weburl'] . '/data/', DATA_PATH, $job['com_logo_n']);
                }

                $imgConfig  =   array(
                    'text'  =>  array(
                        array(
                            'text'      =>  mb_substr($job['name'], 0, 16, 'utf-8'),
                            'left'      =>  116,
                            'top'       =>  1499,
                            'fontSize'  =>  '40px',         //字号
                            'fontColor' =>  '0,0,0'         //字体颜色
                        ),
                        array(
                            'text'      =>  $job['job_salary'],
                            'left'      =>  116,
                            'top'       =>  1605,
                            'fontSize'  =>  '40px',         //字号
                            'fontColor' =>  '255,0,0'       //字体颜色
                        ),
                        array(
                            'text'      =>  mb_substr($job['com_name'], 0, 15, 'utf-8'),
                            'left'      =>  240,
                            'top'       =>  1710,
                            'fontSize'  =>  '28px',         //字号
                            'fontColor' =>  '51,51,51'         //字体颜色
                        ),

                        array(
                            'text'      =>  mb_substr($jobCity . ' - ' . $jobExp . ' - ' . $jobEdu . '学历', 0, 35, 'utf-8'),
                            'left'      =>  240,
                            'top'       =>  1760,
                            'fontSize'  =>  '22px',         //字号
                            'fontColor' =>  '102,102,102',  //字体颜色
                        ),
                        array(
                            'text'      =>  '长按识别二维码',
                            'left'      =>  782,
                            'top'       =>  1760,
                            'fontSize'  =>  '22px',         //字号
                            'fontColor' =>  '102,102,102',  //字体颜色
                        )
                    ),
                    'image' =>  array(
                        array(
                            'url'       =>  Url('wap', array('c' => 'job', 'a' => 'comapply', 'id' => $jobId)),        //二维码资源
                            'qrcode'    =>  array('c'=>'job','id'=>$jobId),
                            'stream'    =>  0,
                            'left'      =>  790,
                            'top'       =>  -190,
                            'right'     =>  0,
                            'bottom'    =>  0,
                            'width'     =>  180,
                            'height'    =>  180,
                            'opacity'   =>  100
						),
                        array(
                            'url'       =>  $job['com_logo_n'],  //  企业LOGO
                            'stream'    =>  0,
                            'left'      =>  116,
                            'top'       =>  -150,
                            'right'     =>  0,
                            'bottom'    =>  0,
                            'width'     =>  100,
                            'height'    =>  100,
                            'opacity'   =>  100
                        )
                    ),
                    'background'    =>  $imgFile    //背景图
                );

                if($this->config['sy_haibao_web_type'] == 1){ // 自定义网站名称
                    $imgConfig['text'][] = array(
                        'text'      =>  $this->config['sy_haibao_web_name'],
                        'left'      =>  50,
                        'top'       =>  140,
                        'fontSize'  =>  '60px',         //字号
                        'fontColor' =>  '255,255,255'   //字体颜色
                    );
                } elseif ($this->config['sy_haibao_web_type'] == 2){
                    if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                        $web_logo = APP_PATH.$this->config['sy_haibao_web_logo'];
                    }else{
                        $web_logo = $this->config['sy_ossurl'] . '/' . $this->config['sy_haibao_web_logo'];
                    }
                    $imgsize = @getimagesize($web_logo);
                    if($imgsize){ // 图片宽高获取成功
                        $imgConfig['image'][] = array(
                            'url'       =>  $web_logo,
                            'stream'    =>  0,
                            'left'      =>  50,
                            'top'       =>  70,
                            // 'right'     =>  0,
                            // 'bottom'    =>  0,
                            'width'     =>  $imgsize[0],
                            'height'    =>  $imgsize[1],
                            'opacity'   =>  100,
                            'c_opacity' => 1,
                        );
                    }else{
						$imgConfig['image'][] = array(
							'url'       =>  $web_logo,
							'stream'    =>  0,
							'left'      =>  50,
							'top'       =>  70,
							'width'     =>  960,
							'height'    =>  130,
							'opacity'   =>  100,
							'c_opacity' => 1,
						);
					}
                }

                echo $this->createPoster($imgConfig);
            }
        }
    }

    function getInviteRegHb($data = array())
    {

        if (!empty($data)) {

            $whb    =   $this->getWhb(array('id' => $data['hb']));

            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

                $imgFile    =   str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $whb['pic_n']);
            } else {

                $imgFile    =   $whb['pic_n'];
            }


            $imgConfig  =   array(
                'text'  =>  array(),
                'image' =>  array(
                    array(
                        'url'       =>  Url('wap', array('c' => 'register', 'uid' => $data['uid'])),        //二维码资源
                        'qrcode'    =>  array('c'=>'register','id'=>$data['uid']),
                        'stream'    =>  0,
                        'left'      =>  305,
                        'top'       =>  658,
                        'right'     =>  0,
                        'bottom'    =>  0,
                        'width'     =>  470,
                        'height'    =>  470,
                        'opacity'   =>  100
                    )
                ),
                'background'    =>  $imgFile    //背景图
            );

            if($this->config['sy_haibao_web_type'] == 1){ // 自定义网站名称
                $imgConfig['text'][] = array(
                    'text'      =>  $this->config['sy_haibao_web_name'],
                    'left'      =>  50,
                    'top'       =>  140,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                );
            } elseif ($this->config['sy_haibao_web_type'] == 2){
                if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                    $web_logo = APP_PATH.$this->config['sy_haibao_web_logo'];
                }else{
                    $web_logo = $this->config['sy_ossurl'] . '/' . $this->config['sy_haibao_web_logo'];
                }
                $imgsize = @getimagesize($web_logo);
                if($imgsize){ // 图片宽高获取成功
                    $imgConfig['image'][] = array(
                        'url'       =>  $web_logo,
                        'stream'    =>  0,
                        'left'      =>  50,
                        'top'       =>  70,
                        // 'right'     =>  0,
                        // 'bottom'    =>  0,
                        'width'     =>  $imgsize[0],
                        'height'    =>  $imgsize[1],
                        'opacity'   =>  100,
                        'c_opacity' => 1,
                    );
                }else{
					$imgConfig['image'][] = array(
						'url'       =>  $web_logo,
						'stream'    =>  0,
						'left'      =>  50,
						'top'       =>  70,
						'width'     =>  960,
						'height'    =>  130,
						'opacity'   =>  100,
						'c_opacity' => 1,
					);
				}
            }

            echo $this->createPoster($imgConfig);
        }
    }

    /*
     * 推广人邀请海报
     */
    function getPromoterInviteRegHb($data = array())
    {
        if (!empty($data)) {
            $whb = $this->getWhb(array('id' => $data['hb']));
            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                $imgFile = str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $whb['pic_n']);
            } else {
                $imgFile    =   $whb['pic_n'];
            }
            $imgConfig  =   array(
                'text'  =>  array(),
                'image' =>  array(
                    array(
                        'url'       =>  Url('promoter', array('c' => 'fastResume', 'tgcode' => $data['code'])),//二维码资源
                        'qrcode'    =>  array('c'=>'fastResume','tgcode'=>$data['code']),
                        'stream'    =>  0,
                        'left'      =>  305,
                        'top'       =>  658,
                        'right'     =>  0,
                        'bottom'    =>  0,
                        'width'     =>  470,
                        'height'    =>  470,
                        'opacity'   =>  100
                    )
                ),
                'background'    =>  $imgFile    //背景图
            );
            if($this->config['sy_haibao_web_type'] == 1){ // 自定义网站名称
                $imgConfig['text'][] = array(
                    'text'      =>  $this->config['sy_haibao_web_name'],
                    'left'      =>  50,
                    'top'       =>  140,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                );
            } elseif ($this->config['sy_haibao_web_type'] == 2){
                if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                    $web_logo = APP_PATH.$this->config['sy_haibao_web_logo'];
                }else{
                    $web_logo = $this->config['sy_ossurl'] . '/' . $this->config['sy_haibao_web_logo'];
                }
                $imgsize = @getimagesize($web_logo);
                if($imgsize){ // 图片宽高获取成功
                    $imgConfig['image'][] = array(
                        'url'       =>  $web_logo,
                        'stream'    =>  0,
                        'left'      =>  50,
                        'top'       =>  70,
                        // 'right'     =>  0,
                        // 'bottom'    =>  0,
                        'width'     =>  $imgsize[0],
                        'height'    =>  $imgsize[1],
                        'opacity'   =>  100,
                        'c_opacity' => 1,
                    );
                }else{
					$imgConfig['image'][] = array(
						'url'       =>  $web_logo,
						'stream'    =>  0,
						'left'      =>  50,
						'top'       =>  70,
						'width'     =>  960,
						'height'    =>  130,
						'opacity'   =>  100,
						'c_opacity' => 1,
					);
				}
            }
            echo $this->createPoster($imgConfig);
        }
    }

    function getGongzhaoHb($data=array()){

        if (!empty($data)) {

            $whb    =   $this->getWhb(array('id' => $data['hb']));

            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

                $imgFile    =   str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $whb['pic_n']);
            } else {

                $imgFile    =   $whb['pic_n'];
            }

            $gongzhao   =   $this->select_once('gongzhao',array('id'=>$data['id']),'`title`,`pic`,`description`');
            
            $imgConfig  =   array(
                'text'  =>  array(),
                'image' =>  array(
                    array(
                        'url'       =>  Url('wap', array('c' => 'gongzhao','a'=>'show','id' => $data['id'])),        //二维码资源
                        'qrcode'    =>  array('c'=>'gongzhao','id'=>$data['id']),
                        'stream'    =>  0,
                        'left'      =>  365,
                        'top'       =>  1330,
                        'right'     =>  0,
                        'bottom'    =>  0,
                        'width'     =>  350,
                        'height'    =>  350,
                        'opacity'   =>  100
                    )
                ),
                'background'    =>  $imgFile    //背景图
            );

            $bottom_text = $this->config['sy_webname'];
            if($this->config['sy_haibao_web_name']){
                $bottom_text = $this->config['sy_haibao_web_name'];
            }
            
            if($bottom_text!=''){
                
                $bottom_text_fontSize = 20;

                $box = imagettfbbox($bottom_text_fontSize,0,TPL_PATH . 'wap/hb_job/OPPOSans-M.ttf',$bottom_text);
                
                $bottom_text_left= 540-floor(($box[2]-$box[0])/2);

                $imgConfig['text'][] = array(
                    'text'      =>  $bottom_text,
                    'left'      =>  $bottom_text_left,
                    'top'       =>  1900,
                    'fontSize'  =>  $bottom_text_fontSize,         //字号 磅值不是像素
                    'fontColor' =>  '50,50,50',   //字体颜色
                );

            }

            

            if($this->config['sy_haibao_web_type'] == 1){ // 自定义网站名称
                $imgConfig['text'][] = array(
                    'text'      =>  $this->config['sy_haibao_web_name'],
                    'left'      =>  50,
                    'top'       =>  140,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '0,0,0'   //字体颜色
                );
            } elseif ($this->config['sy_haibao_web_type'] == 2){
                if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                    $web_logo = APP_PATH.$this->config['sy_haibao_web_logo'];
                }else{
                    $web_logo = $this->config['sy_ossurl'] . '/' . $this->config['sy_haibao_web_logo'];
                }
                $imgsize = @getimagesize($web_logo);
                if($imgsize){ // 图片宽高获取成功
                    $imgConfig['image'][] = array(
                        'url'       =>  $web_logo,
                        'stream'    =>  0,
                        'left'      =>  50,
                        'top'       =>  70,
                        'width'     =>  $imgsize[0],
                        'height'    =>  $imgsize[1],
                        'opacity'   =>  100,
                        'c_opacity' => 1,
                    );
                }else{
					$imgConfig['image'][] = array(
						'url'       =>  $web_logo,
						'stream'    =>  0,
						'left'      =>  50,
						'top'       =>  70,
						'width'     =>  960,
						'height'    =>  130,
						'opacity'   =>  100,
						'c_opacity' => 1,
					);
				}
            }

            if(!empty($gongzhao['title'])){

                $tthharr = array(
                    'top'       =>  270,
                    'fontsize'  =>  40,
                    'width'     =>  980,
                    'left'      =>  50,
                    'hang_size' =>  70,
                    'bottom'    =>  410
                );
                $tthhres=$this->txtHuanHang($tthharr,$gongzhao['title']);
                foreach ($tthhres as $ttk => $ttv) {
                    $imgConfig['text'][] = $ttv;
                }
                
            }

            if(!empty($gongzhao['description'])){
                
                $deschharr = array(
                    'top'       =>  1050,
                    'fontsize'  =>  30,
                    'width'     =>  980,
                    'left'      =>  50,
                    'hang_size' =>  52,
                    'bottom'    =>  1270,
                    'fontcolor' =>  '50,50,50',
                );

                $deschhres=$this->txtHuanHang($deschharr,strip_tags($gongzhao['description']));
                foreach ($deschhres as $hhk => $hhv) {
                    $imgConfig['text'][] = $hhv;
                }
                
            }

            if(!empty($gongzhao['pic'])){
                $gz_middle_pic = checkpic($gongzhao['pic']);;
            }else{
                $gz_middle_pic  =   APP_PATH.$this->config['sy_gongzhaologo'];
            }

            $gz_imgsize = @getimagesize($gz_middle_pic);
            $imgConfig['image'][] = array(
                'url'       =>  $gz_middle_pic,
                'stream'    =>  0,
                'left'      =>  50,
                'top'       =>  400,
                'right'     =>  0,
                'bottom'    =>  0,
                'width'     =>  980,
                'height'    =>  538,
                'opacity'   =>  100,
                'c_opacity' =>  1,
            );

            echo $this->createPoster($imgConfig);
        }
    }
    /**
    长文本换行
    $pos 数组 top顶端距离 width:每行宽度，left左边距，hang_size行高
    $str 文本
    */
    private function txtHuanHang($pos, $str)
    {
        $return  = array();

        $_str_h = $pos["top"];
        $fontsize = $pos["fontsize"];
        $fontcolor = $pos["fontcolor"]?$pos["fontcolor"]:'0,0,0';
        $width = $pos["width"];
        $margin_lift = $pos["left"];
        $hang_size = $pos["hang_size"];
        $bottom = $pos["bottom"];
        $temp_string = "";
        $font_file = TPL_PATH . 'wap/hb_job/OPPOSans-M.ttf';
        $tp = 0;

        for ($i = 0; $i < mb_strlen($str); $i++) {

            $box = imagettfbbox($fontsize, 0, $font_file, $temp_string);

            $_string_length = $box[2] - $box[0];
            $temptext = mb_substr($str, $i, 1);

            $temp = imagettfbbox($fontsize, 0, $font_file, $temptext);

            if ($_string_length + $temp[2] - $temp[0] < $width) {//长度不够，字数不够，需要

                //继续拼接字符串。

                $temp_string .= mb_substr($str, $i, 1);

                if ($i == mb_strlen($str) - 1) {//是不是最后半行。不满一行的情况
                    
                    $return[] = array(
                        'text'      =>  $temp_string,
                        'left'      =>  $margin_lift,
                        'top'       =>  $_str_h,
                        'fontSize'  =>  $fontsize,         //字号
                        'fontColor' =>  $fontcolor   //字体颜色
                    );
                    $_str_h += $hang_size;//计算整个文字换行后的高度。
                    $tp++;//行数
                }
            } else {//一行的字数够了，长度够了。

    //            打印输出，对字符串零时字符串置null
                $texts = mb_substr($str, $i, 1);//零时行的开头第一个字。

                $i--;
    

                $tmp_str_len = mb_strlen($temp_string);
                $s = mb_substr($temp_string, $tmp_str_len-1, 1);//取零时字符串最后一位字符

                $return[] = array(
                    'text'      =>  $temp_string,
                    'left'      =>  $margin_lift,
                    'top'       =>  $_str_h,
                    'fontSize'  =>  $fontsize,         //字号
                    'fontColor' =>  $fontcolor   //字体颜色
                );
                //            计算行高，和行数。
                $_str_h += $hang_size;
                $tp++;
    //           写完了改行，置null该行的临时字符串。
                $temp_string = "";
            }
            //下一行如果大于规定顶部距离的话就结束循环
            if($_str_h+$hang_size>$bottom){
                break;
            }
        }

        
        return $return;
    }
    private function getJobInfo($where, $data = array())
    {

        require_once('job.model.php');
        $JobM   =   new job_model($this->db, $this->def);
        return $JobM->getInfo($where, $data);
    }

    private function getJobList($where, $data = array())
    {
        require_once('job.model.php');
        $jobM   =   new job_model($this->db, $this->def);

        $data['salary_unit'] = 1;
        $jobList = $jobM->getList($where, $data);
        $jobList = !empty($jobList['list']) ? $jobList['list'] : array();
        return $jobList;
    }

    function getComHb($data = array())
    {

        if (!empty($data)) {

            $comId      =   $data['uid'];

            $com        =   $this->getComInfo($comId, array('field' => 'uid, name, shortname, logo, mun, hy', 'logo' => 1));
            if (!empty($com['shortname'])){

                $com['name']    =   $com['shortname'];
            }

            if ($data['hb']){

                $whb    =   $this->getWhb(array('id' => $data['hb']));

            }else{

                if ($data['jobids']){

                    $jobIdA =   explode(',', $data['jobids']);
                    $num    =   count($jobIdA);
                }else{

                    $num    =   $this->select_num('company_job', array('state' => 1, 'status' => '0', 'r_status' => 1, 'uid' => $comId));
                }

                $whb_arr = $this->getWhbList(array('style' => $data['style'],'type' => 2,'isopen' => 1,'orderby' => 'sort, desc'));

                $whb_data = array();

                foreach($whb_arr as $wk=>$wv){
                    $numM = abs($wv['num']-$num);
                    $whb_data[$numM][] = $wv;
                }
                ksort($whb_data);

                $fwhb = current($whb_data);

                $whb = !empty($fwhb[0])?$fwhb[0]:array();
                
            }

            if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {

                $imgFile    =   str_replace($this->config['sy_weburl'].'/data/', DATA_PATH, $whb['pic_n']);
            } else {

                $imgFile    =   $whb['pic_n'];
            }

            $num    =   $whb['num'];

            if (isset($data['jobids'])){

                $jobWhere   =   array(
                    'uid'       =>  $comId,
                    'id'        =>  array('in', $data['jobids']),
                    'orderby'   =>  'lastupdate, desc',
                    'limit'     =>  $num
                );
            }else{
                $jobWhere   =   array(
                    'uid'       =>  $comId,
                    'state'     =>  1,
                    'status'    =>  0,
                    'r_status'  =>  1,
                    'orderby'   =>  'lastupdate, desc',
                    'limit'     =>  $num
                );
            }

            $jobs   =   $this->getJobList($jobWhere, array('field' => 'id, name, minsalary, maxsalary'));

            if (!empty($com) && is_array($com)) {

                $comMun =   !empty($com['mun_n']) ? $com['mun_n'] : '';
                $comHy  =   !empty($com['hy_n']) ? $com['hy_n'] : '';

                if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                    $com['logo_n']  =   str_replace($this->config['sy_weburl'] . '/data/', DATA_PATH, $com['logo']);
                }else{
                    $com['logo_n']  =   checkpic($com['logo']);
                }

                $textArr    =   array(
                    array(
                        'text'      =>  mb_substr($com['name'], 0, 12, 'utf-8'),
                        'left'      =>  250,
                        'top'       =>  -350,
                        'fontSize'  =>  '30px',         //字号
                        'fontColor' =>  '0,0,0'         //字体颜色
                    ),
                    array(
                        'text'      =>  $comMun.' | '.$comHy,
                        'left'      =>  250,
                        'top'       =>  -280,
                        'fontSize'  =>  '20px',         //字号
                        'fontColor' =>  '128,128,128'         //字体颜色
                    )
                );

                foreach ($jobs as $jk => $jv) {


                    if($jk <= 5 && $whb['num'] > $jk){

                        if (mb_strlen($jv['name']) > 8){

                            $jv['name']     =   mb_substr($jv['name'].$jv['name'], 0, 8, 'utf-8').'...';
                        }

                        array_push($textArr,
                            array(
                                'text'      =>  $jv['name'],
                                'left'      =>  120,
                                'top'       =>  -570-100*$jk,
                                'fontSize'  =>  '40px',
                                'fontColor' =>  '0,0,0'
                            ),
                            array(
                                'text'      =>  $jv['job_salary'],
                                'left'      =>  -450,
                                'top'       =>  -570-100*$jk,
                                'fontSize'  =>  '30px',
                                'fontColor' =>  '255,0,0'
                            )
                        );
                    }
                }

                $imageArr = array(
                    array(
                        'url'       =>  Url('wap', array('c' => 'company', 'a' => 'show', 'id' => $comId)),        //二维码资源
                        'qrcode'    =>  array('c'=>'company','id'=>$comId),
                        'stream'    =>  0,
                        'left'      =>  -150,
                        'top'       =>  -240,
                        'right'     =>  0,
                        'bottom'    =>  0,
                        'width'     =>  160,
                        'height'    =>  160,
                        'opacity'   =>  100
                    ),
                    array(
                        'url'       =>  $com['logo_n'],  //  企业LOGO
                        'stream'    =>  0,
                        'left'      =>  120,
                        'top'       =>  -270,
                        'right'     =>  0,
                        'bottom'    =>  0,
                        'width'     =>  120,
                        'height'    =>  120,
                        'opacity'   =>  100
                    )
                );

                if($this->config['sy_haibao_web_type'] == 1){ // 自定义网站名称
                    $textArr[] = array(
                        'text'      =>  $this->config['sy_haibao_web_name'],
                        'left'      =>  66,
                        'top'       =>  140,
                        'fontSize'  =>  '60px',         //字号
                        'fontColor' =>  '255,255,255'   //字体颜色
                    );
                } elseif ($this->config['sy_haibao_web_type'] == 2){
                    if (!isset($this->config['sy_oss']) || $this->config['sy_oss'] != 1) {
                        $web_logo = APP_PATH.$this->config['sy_haibao_web_logo'];
                    }else{
                        $web_logo = $this->config['sy_ossurl'] . '/' . $this->config['sy_haibao_web_logo'];
                    }
                    $imgsize = @getimagesize($web_logo);
                    if($imgsize){ // 图片宽高获取成功
                        $imageArr[] = array(
                            'url'       =>  $web_logo,
                            'stream'    =>  0,
                            'left'      =>  66,
                            'top'       =>  80,
                            // 'right'     =>  0,
                            // 'bottom'    =>  0,
                            'width'     =>  $imgsize[0],
                            'height'    =>  $imgsize[1],
                            'opacity'   =>  100,
                            'c_opacity' => 1,
                        );
                    }else{
						$imageArr[] = array(
							'url'       =>  $web_logo,
							'stream'    =>  0,
							'left'      =>  66,
							'top'       =>  80,
							'width'     =>  960,
							'height'    =>  130,
							'opacity'   =>  100,
							'c_opacity' => 1,
						);
					}
                }

                $imgConfig  =   array(
                    'text'  =>  $textArr,
                    'image' =>  $imageArr,
                    'background'    =>  $imgFile    //背景图
                );

                echo $this->createPoster($imgConfig);
            }
        }
    }

    private function getComInfo($where, $data = array())
    {

        require_once('company.model.php');
        $ComM   =   new company_model($this->db, $this->def);
        return $ComM->getInfo($where, $data);
    }

    function mbStrSplit($string, $len = 1)
    {

        $start  =    0;
        $strlen =   mb_strlen($string);

        while ($strlen) {

            $array[]    =   mb_substr($string, $start, $len, "utf8");

            $string     =   mb_substr($string, $len, $strlen, "utf8");

            $strlen     =   mb_strlen($string);
        }

        return $array;
    }

    function getLogoHb($data = array())
    {

        $hb =   $data['hb'];

        $bg =   DATA_PATH.'upload/whb/logo/'.$hb.'.png';

        $str    =   $this->mbStrSplit($data['text']);

        if ($str[3]){
            $textArr    =   array(
                array(
                    'text'      =>  $str[0],
                    'left'      =>  10,
                    'top'       =>  90,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[1],
                    'left'      =>  110,
                    'top'       =>  90,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[2],
                    'left'      =>  10,
                    'top'       =>  170,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[3],
                    'left'      =>  110,
                    'top'       =>  170,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                )
            );
        }else if ($str[2]){
            $textArr    =   array(
                array(
                    'text'      =>  $str[0],
                    'left'      =>  10,
                    'top'       =>  110,
                    'fontSize'  =>  '40px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[1],
                    'left'      =>  70,
                    'top'       =>  110,
                    'fontSize'  =>  '40px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[2],
                    'left'      =>  130,
                    'top'       =>  110,
                    'fontSize'  =>  '40px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                )
            );
        } else {

            $textArr    =   array(
                array(
                    'text'      =>  $str[0],
                    'left'      =>  20,
                    'top'       =>  120,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                ),
                array(
                    'text'      =>  $str[1],
                    'left'      =>  105,
                    'top'       =>  120,
                    'fontSize'  =>  '60px',         //字号
                    'fontColor' =>  '255,255,255'   //字体颜色
                )
            );
        }

        $imgConfig  =   array(
            'dir'           => $data['dir'] ? $data['dir'] : 'company',
            'text'          =>  $textArr,
            'background'    =>  $bg    //背景图
        );

        if (isset($data['out']) && $data['out'] == 1){
            $imgConfig['out']   =   1;
        }

        echo $this->createPoster($imgConfig);

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

        $background     =   $config['background'];//海报最底层得背景

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

                if (isset($val['qrcode']['c']) && $val['qrcode']['c'] == 'fastResume') {// 推广人二维码
                    // 默认二维码，需特殊处理，直接在本model里生成，不调用url
                    $resWidth   =   $val['width'];
                    $resHeight  =   $val['height'];

                    //建立画板
                    $canvas = $this->qrcode($val['url'], $val['width'], $val['height']);
                    $val['url'] =   '';
                } else if (isset($val['qrcode'])){
                    // 二维码图单独处理
                    if ($this->config['sy_ewm_type'] == 'weixin' || $this->config['sy_ewm_type'] == 'xcx'){
                        // 场景码
                        $qrcode  = $val['qrcode'];
                        // 只有一个二维码，暂时先在这里申明model
                        require_once('weixin.model.php');
                        $WxM        = new weixin_model($this->db, $this->def);
                        $val['url'] = $WxM->pubWxQrcode($qrcode['c'],$qrcode['id'],$this->config['sy_ewm_type']);
                        
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
            $dirName = $config['dir'] ? $config['dir'] : 'company';

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
                    'dir' => $dirName,
                    'type' => 'logo',
                    'watermark' => 0
                );
                $return =   $uploadM->newUpload($upArr);
                return $return['picurl'];
            }else{
                $dir        =   'data/upload/'.$dirName.'/'.date("Ymd");
                $dirname    =   APP_PATH.$dir;
                if(!file_exists($dirname)){
                    mkdir($dirname,0777,true);
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

?>