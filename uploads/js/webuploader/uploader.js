function webUploaderCropT(key, params, fallback) {
    var text;
    if (typeof yunT === 'function') {
        text = yunT(key, params, fallback);
    } else if (typeof yunAt === 'function') {
        text = yunAt(key, params, fallback);
    } else {
        text = fallback !== undefined ? fallback : key;
    }
    if (params && typeof text === 'string') {
        for (var name in params) {
            if (Object.prototype.hasOwnProperty.call(params, name)) {
                text = text.split('{' + name + '}').join(params[name]);
            }
        }
    }
    return text;
}

(function( factory ) {
    if ( !window.jQuery ) {
        alert('jQuery is required.')
    }

    jQuery(function() {
        factory.call( null, jQuery );
    });
})(function( $ ) {
// -----------------------------------------------------
// ------------ START ----------------------------------
// -----------------------------------------------------

// ---------------------------------
// ---------  Uploader -------------
// ---------------------------------
var Uploader = (function() {

    // -------setting-------
    // Large original images can freeze the Croper UI, so shrink before cropping.
    var FRAME_WIDTH = 400;
    var _ = WebUploader;
    var Uploader = _.Uploader;
    var uploaderContainer = $('.uploader-container');
    var uploader, file;
    if ( !Uploader.support() ) {
		layer.alert(webUploaderCropT('webuploader_js_00004', null, 'Web Uploader does not support your browser!'));
    }

    // hook,
    // Crop before the file starts uploading.
    Uploader.register({
        'before-send-file': 'cropImage'
    }, {

        cropImage: function( file ) {

            var data = file._cropData,
                image, deferred;

            file = this.request( 'get-file', file );
            deferred = _.Deferred();

            image = new _.Lib.Image();

            deferred.always(function() {
                image.destroy();
                image = null;
            });
            image.once( 'error', deferred.reject );
            image.once( 'load', function() {
                image.crop( data.x, data.y, data.width, data.height, data.scale );
            });

            image.once( 'complete', function() {
                var blob, size;

                // In image-free mode on mobile UC/QQ browsers.
                // ctx.getImageData can throw when processing large images.
                // INDEX_SIZE_ERR: DOM Exception 1
                try {
                    blob = image.getAsBlob();
                    size = file.size;
                    file.source = blob;
                    file.size = blob.size;

                    file.trigger( 'resize', blob.size, size );

                    deferred.resolve();
                } catch ( e ) {
                    console.log( e );
                    // Continue on error and upload the original image.
                    deferred.resolve();
                }
            });

            file._info && image.info( file._info );
            file._meta && image.meta( file._meta );
            image.loadFromBlob( file.source );
            return deferred.promise();
        }
    });

    return {
        init: function( selectCb ) {
            uploader = new Uploader({
                pick: {
                    id: '#filePicker',
                    multiple: false
                },

                // Configure how thumbnails are generated.
                thumb: {
                    quality: 100,

                    // Do not magnify.
                    allowMagnify: false,

                    // Whether to use crop mode; this can avoid blank content.
                    crop: false
                },

                // Disable chunked upload; it is enabled by default.
                chunked: false,

                // Disable pre-upload compression because cropping is handled manually.
                compress: false,

                // fileSingleSizeLimit: 2 * 1024 * 1024,

                server: serverPath,
                swf:  tplPath+'js/webuploader/Uploader.swf',
                fileNumLimit: 1,
                onError: function() {
                    var args = [].slice.call(arguments, 0);
                    alert(args.join('\n'));
                }
            });


			uploader.on( 'uploadProgress', function( file, percentage ) {
				layer.msg(webUploaderCropT('webuploader_js_00028', null, 'Processing...'),{icon:16,time:100000});return;
			});
			uploader.on( 'uploadSuccess', function( file ,st) {
				if(st=='1'){
					layer.msg(webUploaderCropT('webuploader_js_00026', null, 'Upload successful!'),{icon:6},function(){location.reload();});
				}else{
					layer.msg(webUploaderCropT('webuploader_js_00029', null, 'Upload failed!'),{icon:5},function(){location.reload();});
				}
			});
            uploader.on('fileQueued', function( _file ) {
                file = _file;

                uploader.makeThumb( file, function( error, src ) {

                    if ( error ) {
						layer.msg(webUploaderCropT('webuploader_js_00030', null, 'Cannot preview!'),{icon:5});return;
                    }

                    selectCb( src );

                }, FRAME_WIDTH, 1 );   // Height value 1 is treated as 100% here.
            });
        },

        crop: function( data ) {

            var scale = Croper.getImageSize().width / file._info.width;
            data.scale = scale;

            file._cropData = {
                x: data.x1,
                y: data.y1,
                width: data.width,
                height: data.height,
                scale: data.scale
            };
        },

        upload: function() {
            uploader.upload();
        }
    }
})();

// ---------------------------------
// ---------  Crpper ---------------
// ---------------------------------
var Croper = (function() {
    var container = $('.cropper-wraper');
    var $image = container.find('.img-container img');
    var btn = $('.upload-btn');
    var isBase64Supported, callback;

    $image.cropper({
        aspectRatio: imgwidth / imgheight,
        preview: ".img-preview",
        done: function(data) {
            //console.log(data);
        }
    });

    function srcWrap( src, cb ) {

        // we need to check this at the first time.
        if (typeof isBase64Supported === 'undefined') {
            (function() {
                var data = new Image();
                var support = true;
                data.onload = data.onerror = function() {
                    if( this.width != 1 || this.height != 1 ) {
                        support = false;
                    }
                }
                data.src = src;
                isBase64Supported = support;
            })();
        }

        if ( isBase64Supported ) {
            cb( src );
        } else {
            // otherwise we need server support.
            // convert base64 to a file.
            $.ajax(previewPath, {
                method: 'POST',
                data: src,
                dataType:'json'
            }).done(function( response ) {
                if (response.result) {
                    cb( response.result );
                } else {
					layer.msg(webUploaderCropT('webuploader_js_00015', null, 'Preview error'),{icon:5});return false;
                }
            });
        }
    }

    btn.on('click', function() {
        callback && callback($image.cropper("getData"));
        return false;
    });

    return {
        setSource: function( src ) {

            // Handle browsers without base64 support.
            // Usually appears in IE6-IE8.
            srcWrap( src, function( src ) {
                $image.cropper("setImgSrc", src);
            });

            container.removeClass('webuploader-element-invisible');

            return this;
        },

        getImageSize: function() {
            var img = $image.get(0);
            return {
                width: img.naturalWidth,
                height: img.naturalHeight
            }
        },

        setCallback: function( cb ) {
            callback = cb;
            return this;
        },

        disable: function() {
            $image.cropper("disable");
            return this;
        },

        enable: function() {
            $image.cropper("enable");
            return this;
        }
    }

})();


// ------------------------------
// -----------logic--------------
// ------------------------------
var container = $('.uploader-container');

Uploader.init(function( src ) {

    Croper.setSource( src );

    // Hide the select button.
    container.addClass('webuploader-element-invisible');

    // Start uploading when the user chooses upload.
    Croper.setCallback(function( data ) {
        Uploader.crop(data);
        Uploader.upload();
    });
});



// -----------------------------------------------------
// ------------ END ------------------------------------
// -----------------------------------------------------
});