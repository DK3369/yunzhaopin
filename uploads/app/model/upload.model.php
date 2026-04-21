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

/**
 *  注意设置网站服务器的php.ini以下两项为合适的值：（一般通过现在的手机手机拍照，照片大小都好几兆，可能大于2M）
 *    upload_max_filesize = 8m ; 允许上传文件大小的最大值。默认为2M
 *    post_max_size = 8m ; 通过表单POST给PHP的所能接收的最大值，包括表单里的所有值。默认为8M
 */

class upload_model extends model{

	private $watermark = 1; //watermark :0无视后台的水印开关不添加水印，只用于后台上传水印图片时，以及pc上传头像裁剪保存之前

    /**
     *
     * @param array $data | file/上传文件 ;  dir/上传路径;  type/上传用途(logo表示头像);  base/base4图片;  preview/pc预览即上传;
                            thumb缩率图参数数组，包含width，height，newNamePre，addpreview ;
                            watermark :本类私有属性watermark设置
     * @return array|mixed|string
     */
    function newUpload($data = array('file' => null, 'dir' => null, 'type' => null, 'base' => null, 'preview' => null, 'thumb' => array(), 'watermark' => 1))
    {

        $return     =   array();
        $pictype    =   isset($data['type']) ? $data['type'] : '';
        if (isset($data['watermark'])) {

            $this->watermark    =   $data['watermark'];
        }
        if (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 1) {
            if ($data['dir'] == 'logo') {

                $dir=   'data/logo';
            } else {

                $dir=   'data/upload/' . $data['dir'];
            }
            $upload =   $this->ossUpload();

            if ($data['file']) {

                $thumb  =   array();
                // 分辨率限制条件
                $fbl    =   1920;
                $imginfo=   getimagesize($data['file']['tmp_name']);

                if ($imginfo[0] > $fbl || $imginfo[1] > $fbl) {
                    $thumb['crop']  =   $fbl;
                }
                if ($pictype == 'logo') {   // logo图片太大，需要进行压缩
                    if ($imginfo[0] > 300 || $imginfo[1] > 300) {

                        $thumb['crop']  =   300;
                    }
                }
                if ($this->watermark == 1 && $this->config['is_watermark'] == 1){

                    $thumb['watermark'] =   array(

                        'waterpos'  =>  $this->config['wmark_position'],
                        'waterimg'  =>  $this->config['sy_watermark']
                    );

                }
                $return =   $upload->uploadImg($data['file'], $dir, $thumb);
            } elseif ($data['base']) {

                $return =   $upload->imageBase($data['base'], $dir);
            }
        } else {

            if ($data['dir'] == 'logo') {   // 后台上传网站logo等

                $dir    =   APP_PATH . 'data/logo/';
            } else {

                $dir    =   APP_PATH . 'data/upload/' . $data['dir'] . '/';
            }

            $upload     =   $this->Upload_pic($dir);

            if (!empty($data['base'])) {

                $pic    =   $upload->imageBase($data['base']);
            } else {

                $pic    =   $upload->picture($data['file'], $pictype);
            }

            // 匹配错误信息
            $picmsg     =   $this->picmsg($pic);

            if (!empty($picmsg['msg'])) {

                $return['msg']  =   '上传失败：' . $picmsg['msg'];
            } else {
                if ($pictype == 'thumb') {

                    $return['picurl']   =   $upload->makeThumb($pic, $data['thumb']['width'], $data['thumb']['height'], $data['thumb']['newNamePre'], $data['thumb']['addpreview']);
                } elseif ($pictype == 'news') {

                    //后台新闻，需要原图加缩率图
                    $return['picurl']   =   $pic;
                    $return['thumburl'] =   $upload->news_makeThumb($pic, 200, 133, '_S_');
                } else {

                    $return['picurl']   =   $pic;
                }
                if ($data['dir'] == 'logo') {

                    // 后台上传网站logo等
                    $return['picurl']   =   str_replace(APP_PATH . 'data', 'data', $return['picurl']);
                } else {

                    $return['picurl']   =   str_replace(APP_PATH . 'data', './data', $return['picurl']);

                    if (!empty($return['thumburl'])) {

                        $return['thumburl'] =   str_replace(APP_PATH . 'data', './data', $return['thumburl']);
                    }
                }
            }
        }
        return $return;
    }

    function voiceUpload($data = array('file' => null, 'dir' => null, 'type' => null))
    {

        $return =   array();

        if (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 1) {

//            $dir    =   'data/upload/' . $data['dir'];

            $upload =   $this->ossUpload();

            if ($data['file']) {

                $return =   $upload->uploadVoice($data['file']);
            }
        } else {

//            $varr       =   pathinfo($data['file']['name']);
            $newName    =   time().'C_'.rand(1000, 9999);
            $upfiledir  =   APP_PATH.'data/upload/voice' . '/' . date('Ymd') . '/';
            if (!file_exists($upfiledir)) {
                @mkdir($upfiledir, 0777, true);
            }

            $vdir       =   $upfiledir.$newName.'.mp3';

            if (move_uploaded_file($data['file']['tmp_name'], $vdir)) {

                $voiceDir           =   str_replace(APP_PATH . 'data', 'data', $vdir);
                $return['voiceurl'] =   $voiceDir;
            } else {

                $return['error']    =   2;
                $return['msg']      =   '语音保存出错，请重试';
            }
        }
        return $return;
    }

    function ossUpload()
    {

        include_once(LIB_PATH.'oss/ossupload.class.php');
        $paras  =   $this->paras();
        $upload =   new ossUpload($paras);
        return $upload;
    }

    /**
     * @desc    上传方法
     * @param   string $dir
     * @param   string $destination_folder
     * @return  Upload
     */
    function Upload_pic($dir = '', $destination_folder = '')
    {

        include_once(LIB_PATH.'upload.class.php');

        $paras  =   $this->paras(array('dir' => $dir, 'destination_folder' => $destination_folder));

        //  判断后台水印上传设置
        if ($this->watermark == '1' && $this->config['is_watermark'] == 1) {

            $paras['addwatermark']  =   true;
            $paras['position']      =   $this->config['wmark_position'] ? $this->config['wmark_position'] : 1;  //水印位置
            $paras['waterimg']      =   APP_PATH.$this->config['sy_watermark']; //水印图片
        } else {

            $paras['addwatermark']  =   false;
        }

        $upload =   new Upload($paras);
        return $upload;
    }

    private function paras($data = array())
    {

        if (isset($data['dir'])) {

            $paras['upfiledir'] =   $data['dir'];//上传路径
        }

        //根据后台配置 读取上传限定大小
        if ($this->config['pic_maxsize']) {

            $paras['maxsize']   =   (int)$this->config['pic_maxsize'] * 1024;
        } else {

            $paras['maxsize']   =   5 * 1024;
        }

        //传入设定 图片类型
        if ($this->config['pic_type']) {

            $this->config['pic_type']   =   str_replace('.', '', $this->config['pic_type']);

            $paras['pic_type']  =   explode(',', str_replace(' ', '', $this->config['pic_type']));
        } else {

            $paras['pic_type']  =   array('jpg', 'png', 'jpeg', 'bmp', 'gif');
        }

        //禁止后台设定可执行程序后缀
        foreach ($paras['pic_type'] as $pickey => $picvalue) {

            $pic_type   =   strtolower(str_replace('.', '', trim($picvalue)));
            if (in_array($pic_type, array('php', 'asp', 'aspx', 'jsp', 'exe', 'do'))) {

                unset($paras['pic_type'][$pickey]);
            }
        }
        //判断是否需要进行图片验证
        $paras['is_picself']    =   $this->config['is_picself'];

        //判断是否需要强制压缩
        if ($this->config['is_picthumb'] == 1) {

            $paras['addpreview'] = true;//是否生成缩略图
        } else {

            $paras['addpreview'] = false;//是否生成缩略图
        }

        if (isset($data['destination_folder']) && $data['destination_folder'] != '') {

            $paras['destination_folder']    =   $data['destination_folder'];
        }

        return $paras;
    }

    function picmsg($status)
    {

        $error  =   array('1' => '文件太大', '2' => '文件类型不符', '3' => '同名文件已经存在', '4' => '移动文件出错,请检查upload目录权限', '6' => '非法文件，无法上传');

        if (isset($error[$status])) {

            $data['status'] =   $status;
            $data['msg']    =   $error[$status];
            return $data;
        } else {

            return true;
        }
    }

    /**
     * pc预览直接上传、保存图片
     * @param array $data
     * @return array
     */
    function layUpload($data = array())
    {


        $path   =   isset($data['path']) ? $data['path'] : '';
        $upArr  =   array(

            'file'  =>  $data['file'],
            'dir'   =>  $path,
        );

        if (!empty($data['uid']) && !empty($data['usertype']) && !empty($data['imgid']) && $path != 'logo') {
            // 个人用户
            if ($data['usertype'] == 1) {

                require_once('resume.model.php');
                $resumeM    =   new resume_model($this->db, $this->def);

                if ($data['imgid'] == 'logo') { // 上传头像

                    $this->watermark = 0;
                    $result =   $resumeM->upPhoto(array('uid' => $data['uid']), array('photo' => $data['file'], 'utype' => 'user', 'preview' => 1));
                }

                if ($data['imgid'] == 'ewm') {  // 上传二维码

                    $result =   $resumeM->upEwm(array('uid' => $data['uid']), array('photo' => $data['file'], 'utype' => 'user', 'preview' => 1));
                }
            }
            // 企业用户
            if ($data['usertype'] == 2) {

                require_once('company.model.php');
                $companyM   =   new company_model($this->db, $this->def);

                if ($data['imgid'] == 'logo') { // 上传logo

                    $this->watermark=   0;

                    if (isset($data['auid'])) {

                        $result     =   $companyM->upLogo(array('uid' => $data['uid']), array('photo' => $data['file'], 'preview' => 1));
                        //  后台企业列表直接上传LOGO，记录管理员日志
                        session_start();
                        require_once ('log.model.php');
                        $logM	=	new log_model($this->db, $this->def);
                        $logM -> addAdminLog('企业（UID：'.$data['uid'].'）logo，' .$result['msg']);
                        // 提示中带ID，需要过滤
                        $result['msg'] = preg_replace('/\([^\)]+?\)/x', "", str_replace(array("（", "）"), array("(", ")"), $result['msg']));
                    } else {

                        $result     =   $companyM->upLogo(array('uid' => $data['uid']), array('photo' => $data['file'], 'utype' => 'user', 'preview' => 1));
                    }
                } elseif ($data['imgid'] == 'ewm') { // 上传二维码

                    $result         =   $companyM->upEwm(array('uid' => $data['uid']), array('photo' => $data['file'], 'utype' => 'user', 'preview' => 1));
                } elseif ($data['imgid'] == 'cert' || $data['imgid'] == 'owner_cert' || $data['imgid'] == 'wt_cert' || $data['imgid'] == 'other_cert') {// 企业资质相关图片
                    $result = $companyM->upComYyzz(array('uid' => $data['uid']), array(
                        'photo' => $data['file'],
                        'dir' => $data['imgid'],
                        'preview' => 1,
                        'did' => $data['did']
                    ));
                }
            }
            
 
        } elseif (!empty($data['name']) && $data['path'] == 'logo') {
			
			if(!preg_match('/^[a-zA_Z0-9]+$/',$data['name'])){
                
                return false;
            }
            
            $this->watermark = 0;

            // 后台上传网站logo
            $result =   $this->newUpload($upArr);

            $pic    =   $result['picurl'];

            $result['picurl']   =   checkpic($pic);
            $post[$data['name']]=   $pic;


            //小程序封面图/微信关注欢迎图 需要单独上传至微信素材库
            if ($data['name'] == 'sy_xcxpic') {

                require_once('weixin.model.php');
                $wxM    =   new weixin_model($this->db, $this->def);

                $upMedia=   $wxM->upMedia($pic, $upArr);

                if (!$upMedia['media_id']) {

                    $result['errcode']  =   8;
                    $result['msg']      =   '封面素材上传失败！' . $upMedia['errmsg'];
                } else {

                    $post['sy_xcxmedia']        =   $upMedia['media_id'];
                }
            } elseif ($data['name'] == 'sy_wxcom_pic') {

                require_once('weixin.model.php');
                $wxM    =   new weixin_model($this->db, $this->def);

                $upMedia=   $wxM->upMedia($pic, $upArr);

                if (!$upMedia['media_id']) {

                    $result['errcode']  =   8;
                    $result['msg']      = '欢迎图素材上传失败！' . $upMedia['errmsg'];
                } else {

                    $post['sy_wxcom_picmedia']  =   $upMedia['media_id'];
                }
            }

            require_once('config.model.php');
            $configM    =   new config_model($this->db, $this->def);
            $configM->setConfig($post);
        }

        if (!empty($result['status']) || (isset($result['errcode']) && $result['errcode'] == 8)) {

            $return     =   array(

                'code'  =>  1,
                'msg'   =>  $result['msg'],
                'data'  =>  array()
            );
        } else {

            $return     =   array(

                'code'  =>  0,
                'msg'   =>  $result['msg'],
                'data'  =>  array('url' => $result['picurl'])
            );
        }
        return $return;
    }

    /**
     * pc头像剪裁
     * @param $dir
     * @return mixed
     */
    public function thumb($dir)
    {
        if (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 1) {
            if ($dir) {

                $data['dir'] = 'data/upload/' . $dir;
            }

            $upload =   $this->ossUpload();

            if (isset($_POST['x']) && isset($_POST['y'])) {

                $pictures   =   $upload->cutPic($_POST['img1'], $data['dir'], $_POST['x'], $_POST['y'], $_POST['width'], $_POST['height'], $_POST['scale']);
                $thumb      =   $pictures['picurl'];
            } else {

                $thumb      =   '../data/upload/' . $dir . '/' . date('Ymd') . '/' . end(explode('/', $_POST['img1']));
            }
        } else {

            include LIB_PATH . 'sizer.class.php';
            $sizer  =   new Sizer('../data/upload/' . $dir . '/' . date('Ymd') . '/', $this->config['sy_ossurl']);
            $sizep  =   $sizer->sizeIt();
            $thumb  =   $sizep[1];
        }
        return $thumb;
    }

    /**
     * @desc    thumb缩率图参数数组，包含width，height，newNamePre，addpreview
     * @param   null[] $data | file/上传文件 ;  dir/上传路径;
     * @return  mixed
     */
	public function uploadDoc($data = array('file'=>null,'dir'=>null))
	{

        $dir    =   'data/upload/' . $data['dir'];

        if (isset($this->config['sy_oss']) && $this->config['sy_oss'] == 1) {

            $upload =   $this->ossUpload();
            $return =   $upload->uploadDoc($data['file'], $dir);

            if ($data['dir'] == 'comdoc' && isset($return['docurl'])) {

                // 上传委托书范本的，人才网服务器要传一份，不然小程序因安全域名问题，无法下载
                $arr    =   parse_url($this->config['sy_weburl'] . $return['docurl']);
                $parr   =   pathinfo($arr['path']);

                $this->saveLocalFile($dir, $data, $parr['filename']);
            }
        } else {

            $return =   $this->saveLocalFile($dir, $data);
        }
        return $return;
	}

    /**
     * @desc    保存文件到服务器
     * @param   $dir
     * @param   $data
     * @param   string $filename
     * @return  mixed
     */
    function saveLocalFile($dir, $data, $filename = '')
    {
        if (!is_dir(APP_PATH . $dir . '/' . date('Ymd'))) {

            mkdir(APP_PATH . $dir . '/' . date('Ymd'), 0777, true);
        }

        $nametype   =   @explode('.', trim($data['file']['name']));

        $filetype   =   strtolower(end($nametype));

        if ($this->config['file_type']) {

            $this->config['file_type']   =   str_replace('.', '', $this->config['file_type']);

            $file_type  =   explode(',', str_replace(' ', '', $this->config['file_type']));

            //禁止后台设定可执行程序后缀
            foreach ($file_type as $filekey => $filevalue) {

                $new_file_type  =   strtolower(str_replace('.', '', trim($filevalue)));

                if (in_array($new_file_type, array('php', 'asp', 'aspx', 'jsp', 'exe', 'do'))) {

                    unset($file_type[$filekey]);
                }
            }
        } else {

            $file_type  =   array('jpg', 'png', 'jpeg', 'bmp', 'gif');
        }


        if (!in_array($filetype, $file_type)) {//检查文件类型

            $return['msg']  =   '禁止上传' . $filetype . '类型文件！';
        } else {
            if (empty($filename)) {

                $filename   =   time() .'C_'. rand(1000, 9999);
            }
            $upload     =   $dir . '/' . date('Ymd') . '/' . $filename . '.' . $filetype;

            $pathname   =   APP_PATH . $upload;

            $result     =   move_uploaded_file($data['file']['tmp_name'], $pathname);

            if ($result) {

                $return['docurl']   =   '/' . $upload;
            } else {

                $return['msg']      =   '文件上传失败，请检查DATA目录权限！';
            }
        }
        return $return;
    }

    /**
     * wangEditor
     * @param array $data
     * @return array
     */
    function wangUpload($data = array())
    {

        $path   =   isset($data['path']) ? $data['path'] : '';
        $upArr  =   array(
            'file' => $data['file'],
            'dir'  => $path,
        );
        // 后台上传网站logo
        $result = $this->newUpload($upArr);

        if (!empty($result['status']) || (isset($result['errcode']) && $result['errcode'] == 8)) {

            $return     =   array(

                'errno'   => 1,
                'message' => $result['msg']
            );
        } else {
            $return     =   array(

                'errno' => 0,
                'data'  => array('url' => checkpic($result['picurl']))
            );
        }
        return $return;
    }

}
?>