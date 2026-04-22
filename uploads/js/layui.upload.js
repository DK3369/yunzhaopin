/**
 * pc文件上传
*/
// 多图片上传预览图片删除
$(document).on("click", ".del_preview_multi_pic", function () {
    $(this).parents('.preview_multi_pic').remove();
});
layui.use('upload', function(){
	var $ = layui.$
		,upload = layui.upload
		,layer = layui.layer
		,device = layui.device();

	var layupload_type = $("#layupload_type").val();   //文件上传方式   2、选完文件后自动上传
	var laynoupload = $("#laynoupload").val(); 		   //1、选完不上传
	var uploadmulti = 0;
    uploadmulti = $("#uploadmulti").val();
    if (uploadmulti == 1) {// 多文件上传
        upload.render({
            elem: '.noupload'
            ,multiple: true
            ,url: this.url
            ,accept: layaccept
            ,exts: layexts
            ,done: function(upres){
                var parentid = null;
                if(this.parentid){
                    parentid = this.parentid;
                }
                $('#' + parentid).removeClass('none');
                $('#' + parentid).append(
                    '<div class="preview_multi_pic"><img src="'+ upres.picurl_n +'" class="layui-upload-img"><input name="picurl[]" class="addpicurls" type="hidden" value="'+ upres.picurl +'"><a href="javascript:void(0)" class="del_preview_multi_pic"><img src="images/ylimg_close.png" alt=""></a></div>')
                $('#checka').hide();
            }
        });
	} else {
        //选完不上传，url暂未用到，只是需要其样式
        if (laynoupload == 1){
            var layfiletype = $("#layfiletype").val();
            //上传文件类型
            if (layfiletype == 2){
                var layaccept = 'file', layexts = 'doc|docx|rar|zip|pdf|xls|xlsx';
            }else{
                var layaccept = 'images', layexts = 'jpg|png|gif|bmp|jpeg';
            }
            upload.render({
                elem: '.noupload'
                ,auto: false
                ,bindAction: '#test9'   //触发上传的对象，暂未用到
                ,accept: layaccept
                ,exts: layexts
                ,choose: function(obj){
                    if(this.imgid){
                        //预读本地文件示例，不支持ie8/9
                        var imgid = null,
                            parentid = null;
                        if(this.imgid){
                            imgid = this.imgid;
                        }
                        if(this.parentid){
                            parentid = this.parentid;
                        }
                        obj.preview(function(index, file, result){
                            if (parentid && $('#'+parentid).length>0){
                                $('#'+parentid).removeClass('none');
                                $('#'+imgid).attr('src', result);
                            }else if(imgid && $('#'+imgid).length>0){
                                $('#'+imgid).removeClass('none');
                                $('#'+imgid).attr('src', result); //图片链接（base64）
                            }
                            $('#checka').hide();
                        });
                    }
                }
            });
        }
        if (layupload_type == 2){
            if($(".adminupload").length>0){
                var newData = {};
                var url = '';

                url = weburl+'/index.php?m=ajax&c=layui_upload';

                var uploadInst = upload.render({
                    elem: '.adminupload'
                    ,url: url
                    ,data: newData
                    ,choose: function(obj){
                        if(this.name){
                            newData.name = this.name;
                        }
                        if(this.path){
                            newData.path = this.path;
                        }
                        if(this.imgid){
                            newData.imgid = this.imgid;
                        }
                        if(this.uid){
                            newData.uid = this.uid;
                        }
                        if(this.usertype){
                            newData.usertype = this.usertype;
                        }
                    }
                    ,before: function(obj){
                        layer.load();
                    }
                    ,done: function(res){
                        layer.closeAll('loading');
                        if(res.code > 0){                //上传失败，返回失败原因
                            return layer.msg(res.msg,{icon: 5, time: 2000});
                        }else{
                            if(res.msg){
                                layer.msg(res.msg,{icon: 6, time: 2000});
                            }
                            if(this.name=='pic'){
                                $('input[name="'+ this.name +'"]').val(res.data.url);
                            }
                            //图片外层有其他元素
                            if ($('#'+this.parentid).length>0){
                                $('#'+this.parentid).removeClass('none');
                                $('#'+this.imgid).attr('src', res.data.url);
                            }else if(this.imgid){
                                $('#'+this.imgid).removeClass('none');
                                $('#'+this.imgid).attr('src', res.data.url);
                            }

                            if(document.getElementById('newbind')){
                                $('#newbind').removeClass('none');
                            }
                        }
                    }
                });
            }
        }

        // 新选完不上传 - 同一页面多上传功能
        $(".lay-uploads").each(function(index, item) {
            let file = $(item).data('file');
            let filetype = $(item).data('filetype');
            //上传文件类型
            if (filetype == 2){
                let layaccept = file,
                    layexts = 'doc|docx|rar|zip|pdf|xls|xlsx';
            }else{
                let layaccept = file,
                    layexts = 'jpg|png|gif|bmp|jpeg';
            }
            upload.render({
                elem: '.upload-' + file
                ,auto: false
                ,field: file
                ,accept: layaccept
                ,exts: layexts
                ,choose: function(obj){
                    if(this.imgid){
                        //预读本地文件示例，不支持ie8/9
                        var imgid = null,
                            parentid = null;
                        if(this.imgid){
                            imgid = this.imgid;
                        }
                        if(this.parentid){
                            parentid = this.parentid;
                        }
                        obj.preview(function(index, file, result){
                            if (parentid && $('#'+parentid).length>0){
                                $('#'+parentid).removeClass('none');
                                $('#'+imgid).attr('src', result);
                            }else if(imgid && $('#'+imgid).length>0){
                                $('#'+imgid).removeClass('none');
                                $('#'+imgid).attr('src', result); //图片链接（base64）
                            }
                            $('#checka').hide();
                        });
                    }
                }
            });
        })
	}
});
