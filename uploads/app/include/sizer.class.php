<?php

/**
 *	参数规范：
 *
 *	$_POST['count']			小图个数
 *	$_POST['width'.number]	第number个小图的宽
 *	$_POST['height'.number]	第number个小图的高
 *	$_POST['x'.number]		第number个小图的起始切图点的x坐标
 *	$_POST['scale'.number]	缩放比例
 *	$_POST['img'.number]	源文件名，不带路径，带后缀，如：xxxx.jpg
 */

class Sizer {
	
	//设置图片保存路径
	private $_path;

	//小图计数
	private $_count;
	
	//post过来的数据存这里，二维数组，一维的key是数字
	private $_paramsArr;
	
	private $_oriImg;
	
	private $_resultImgArr;
	
	private $ossurl;
	
	function Sizer($path, $ossurl = ''){
		
		$this->_path=$path;
		$this->_paramsArr=array();
		$this->_count=0;
		$this->ossurl = $ossurl;
	}
	
	//唯一的参数，生成小图后是否删除原图？
	function sizeIt($picdelete='1'){
		
		//到底要生成几个小图
		if(isset($_POST['count'])){
			$this->_count=intval($_POST['count']);
		}
		
		//post过来的参数保存到自己的私有属性里
		for($i=1;$i<=$this->_count;$i++){
		    
		    $p = array('width'=>$_POST['width'],
		        'height'=>$_POST['height'],
		        'x'=>$_POST['x'],
		        'y'=>$_POST['y'],
		        'scale'=> $_POST['scale']); //传的img字符串要包括扩展名
		    
	        if ($this->ossurl != ''){
	            $p['img']  =  str_replace($this->ossurl.'/data', '../data', $_POST['img'.$i]);
	        }else{
	            $p['img']  =  $_POST['img'.$i];
	        }
	        $this->_paramsArr[$i-1]  =  $p;
		}
		//trace($this->_paramsArr);
		
		//依次生成每个小图
		if(!file_exists($this->_path)){
			@mkdir($this->_path,0777,true);
		}
		$idx=0;
		foreach($this->_paramsArr as $key=>$value){
			$idx++;
			$parts=explode('.',$value['img']);
			
			$t_name[$idx] = $this->resizeThumbnailImage(//目前的算法是小图在大图文件名后加'_number',比如xxx.jpg的第二种小图是xxx_2.jpg
										$this->_path.(time().rand(100,999)).'_'.$idx.'.'.end($parts),
										$value['img'],
										$value['width'],
										$value['height'],
										$value['x'],
										$value['y'],
										$value['scale']);
		}

		
		if($picdelete=='1')
		{
			$deleteImg = $this->_path.end(explode('/',$_POST['img1']));
			$pictype=getimagesize($deleteImg);
			if($pictype[2]=='1' || $pictype[2]=='2' || $pictype[2]=='3')
			{
				@unlink($deleteImg);
			}
		}elseif($picdelete=='2' && $idx=='1'){
			$t_name[2] = $_POST['img1'];
		}

		return $t_name;
	}
	
	
	//预览后，实际调整图片大小的函数
	function resizeThumbnailImage(	$thumb_image_name,  //小图文件名，包括路径
									$image, 			//要剪裁的文件名，包括路径
									$width, 			//要切多宽
									$height, 			//要切多长
									$start_width, 		//开始切图的x坐标点
									$start_height, 		//开始切图的y坐标点
									$scale				//缩放比例
									)
	{
		
		list($imagewidth, $imageheight, $imageType) = getimagesize($image);
		$imageType = image_type_to_mime_type($imageType);
		
		$newImageWidth = ceil($width * $scale);
		$newImageHeight = ceil($height * $scale);
		//$newImage = imagecreatetruecolor($newImageWidth,$newImageHeight);
		switch($imageType) {
			case "image/gif":
				$source=imagecreatefromgif($image); 
				break;
			case "image/pjpeg":
			case "image/jpeg":
			case "image/jpg":
				$source=imagecreatefromjpeg($image); 
				break;
			case "image/png":
			case "image/x-png":
				$source=imagecreatefrompng($image); 
				break;
		}
		//imagecopyresampled($newImage,$source,0,0,$start_width,$start_height,$newImageWidth,$newImageHeight,$width,$height);

 		//GD2.0.1
		if (function_exists("imagecreatetruecolor")){
			$newImage = imagecreatetruecolor($newImageWidth, $newImageHeight);
			if(in_array($imageType, array('image/gif', 'image/png', 'image/x-png'))){
				$color = imagecolorallocate($newImage,255,255,255);
				imagecolortransparent($newImage,$color);
				imagefill($newImage,0,0,$color);
			}
			ImageCopyResampled($newImage, $source, 0, 0, $start_width, $start_height, $newImageWidth, $newImageHeight, $width, $height);
		}else{
			$newImage = imagecreate($newImageWidth, $newImageHeight);
			if(in_array($imageType, array('image/gif', 'image/png', 'image/x-png'))){
				$color = imagecolorallocate($newImage,255,255,255);
				imagecolortransparent($newImage,$color);
				imagefill($newImage,0,0,$color);
			}
			ImageCopyResized($newImage, $source, 0, 0, $start_width, $start_height, $newImageWidth, $newImageHeight, $width, $height);
		}

		switch($imageType) {
			case "image/gif":
				imagegif($newImage,$thumb_image_name); 
				break;
			case "image/pjpeg":
			case "image/jpeg":
			case "image/jpg":
				imagejpeg($newImage,$thumb_image_name,90); 
				break;
			case "image/png":
			case "image/x-png":
				imagepng($newImage,$thumb_image_name);  
				break;
		}
		
		chmod($thumb_image_name, 0666);
		return $thumb_image_name;
	}
	//==========================================
	// 函数: waterMarkimg($destination,$image_size,$destination_folder)
	// 功能: 给图片加水印
	// 参数: $destination 图片文件名
	// 参数: $image_size 大小数组(包含二个字符串)
	//destination_folder文件存放路径
	// 返回: 1 成功 成功时返回生成的图片路径
	//==========================================
	function waterMarkimg($destination)
	{	
		global $config;
		$waterimg = APP_PATH.$config['sy_watermark'];
		$image_size = getimagesize($destination);
	    $iinfo=getimagesize($destination,$iinfo);//取得GIF、JPEG、PNG或SWF图形的大小
        $iinfo2=getimagesize($waterimg,$iinfo2);//取得GIF、JPEG、PNG或SWF水印图形的大小
        $nimage=imagecreatetruecolor($image_size[0],$image_size[1]);//建立一个新的图形
        $white=imagecolorallocate($nimage,255,255,255);//分配图形的颜色
        $black=imagecolorallocate($nimage,0,0,0);//分配图形的颜色
        $red=imagecolorallocate($nimage,255,0,0);//分配图形的颜色
        imagefill($nimage,0,0,$white);//将图形着色
        
        switch ($iinfo[2])
        {
            case 1:
            $simage =imagecreatefromgif($destination);//从文件或URL建立一个新的图形
            break;
            case 2:
            $simage =imagecreatefromjpeg($destination);
            break;
            case 3:
            $simage =imagecreatefrompng($destination);
            break;
            case 6:
            $simage =imagecreatefromwbmp($destination);
            break;
            default:$this->errorType=5;//不支持的文件类型
            return;
        }
        imagecopy($nimage,$simage,0,0,0,0,$image_size[0],$image_size[1]);
       // imagefilledrectangle($nimage,1,$image_size[1]-15,80,$image_size[1],$white);//建立一个矩形并且填满颜色
        
           switch ($iinfo2[2]){
            case 1:
            $simage1 =imagecreatefromgif($waterimg);
            break;
            case 2:
            $simage1 =imagecreatefromjpeg($waterimg);
            break;
            case 3:
            $simage1 =imagecreatefrompng($waterimg);
            break;
            case 6:
            $simage1 =imagecreatefromwbmp($waterimg);
            break;
            default:

            $this->errorType=6;//不支持的水印文件类型
            return;
        }
        
		$gifsize=getimagesize($waterimg);//取得GIF、JPEG、PNG或SWF图形的大小
       switch($config['wmark_position']){//水印位置
		  case 1:// 上左
          imagecopy($nimage,$simage1,0,0,0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
		  case 2:// 上中
          imagecopy($nimage,$simage1,($image_size[0]-$gifsize[0])/2,0,0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
		  case 3:// 右上
		  imagecopy($nimage,$simage1,$image_size[0]-$gifsize[0],0,0,0,$gifsize[0],$gifsize[1]);
		  break;
		  case 4:// 中左
          imagecopy($nimage,$simage1,0,($image_size[1]-$gifsize[1])/2,0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
		  case 5:// 居中
		  imagecopy($nimage,$simage1,($image_size[0]-$gifsize[0])/2, ($image_size[1]-$gifsize[1])/2,0,0,$gifsize[0],$gifsize[1]);
		  break;
		  case 6:// 中右
          imagecopy($nimage,$simage1,$image_size[0]-$gifsize[0],($image_size[1]-$gifsize[1])/2,0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
          case 9:// 右下
          imagecopy($nimage,$simage1,$image_size[0]-$gifsize[0], $image_size[1]-$gifsize[1],0,0,$gifsize[0],$gifsize[1]);
          break;
		  case 7:// 左下
          imagecopy($nimage,$simage1,0,$image_size[1]-$gifsize[1],0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
		  case 8:// 底中
          imagecopy($nimage,$simage1,($image_size[0]-$gifsize[0])/2,$image_size[1]-$gifsize[1],0,0,$gifsize[0],$gifsize[1]); // 左下
          break;
        }
       imagedestroy($simage1);// 结束图形
       switch ($iinfo[2]){
            case 1:
            imagejpeg($nimage, $destination);
            break;
            case 2:
            imagejpeg($nimage, $destination);
            break;
            case 3:
            imagepng($nimage, $destination);
            break;
            case 6:
            imagewbmp($nimage, $destination);
            break;
        }
        //覆盖原上传文件
        imagedestroy($nimage);// 结束图形
        imagedestroy($simage);// 结束图形
        return $destination;
	}
}

?>