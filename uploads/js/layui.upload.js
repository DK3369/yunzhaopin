/**
 * PC file upload.
*/
// Remove a preview image from multi-image uploads.
$(document).on("click", ".del_preview_multi_pic", function () {
    $(this).parents('.preview_multi_pic').remove();
});
layui.use('upload', function(){
	var $ = layui.$
		,upload = layui.upload
		,layer = layui.layer
		,device = layui.device();

	var layupload_type = $("#layupload_type").val();   // File upload mode. 2 means upload automatically after selecting files.
	var laynoupload = $("#laynoupload").val(); 		   // 1 means do not upload after selecting files.
	var uploadmulti = 0;
    uploadmulti = $("#uploadmulti").val();
    if (uploadmulti == 1) {// Multi-file upload.
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
        // Do not upload after selecting files; url is only needed for its style.
        if (laynoupload == 1){
            var layfiletype = $("#layfiletype").val();
            // Uploaded file type.
            if (layfiletype == 2){
                var layaccept = 'file', layexts = 'doc|docx|rar|zip|pdf|xls|xlsx';
            }else{
                var layaccept = 'images', layexts = 'jpg|png|gif|bmp|jpeg';
            }
            upload.render({
                elem: '.noupload'
                ,auto: false
                ,bindAction: '#test9'   // Upload trigger element, currently unused.
                ,accept: layaccept
                ,exts: layexts
                ,choose: function(obj){
                    if(this.imgid){
                        // Preview local files; IE8/9 are not supported.
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
                                $('#'+imgid).attr('src', result); // Image URL in base64.
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
                        if(res.code > 0){                // Upload failed; return the failure reason.
                            return layer.msg(res.msg,{icon: 5, time: 2000});
                        }else{
                            if(res.msg){
                                layer.msg(res.msg,{icon: 6, time: 2000});
                            }
                            if(this.name=='pic'){
                                $('input[name="'+ this.name +'"]').val(res.data.url);
                            }
                            // The image has a wrapper element.
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

        // New no-auto-upload mode: multiple upload controls on one page.
        $(".lay-uploads").each(function(index, item) {
            let file = $(item).data('file');
            let filetype = $(item).data('filetype');
            // Uploaded file type.
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
                        // Preview local files; IE8/9 are not supported.
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
                                $('#'+imgid).attr('src', result); // Image URL in base64.
                            }
                            $('#checka').hide();
                        });
                    }
                }
            });
        })
	}
});
