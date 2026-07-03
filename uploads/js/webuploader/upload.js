function webUploaderT(key, params, fallback) {
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

(function( $ ){
    // Initialize when DOM is ready.
    $(function() {
        var $wrap = $('#uploader'),

			$usershowid = $( '#usershowid' ).val(),
			// Admin company context.
			$comid = $( '#comid' ).val(),
			$pytoken = $('#pytoken').val(),

            // Image container.
            $queue = $( '<ul class="filelist"></ul>' )
                .appendTo( $wrap.find( '.queueList' ) ),

            // Status bar, including progress and control buttons.
            $statusBar = $wrap.find( '.statusBar' ),

            // Overall selected file info.
            $info = $statusBar.find( '.info' ),

            // Upload button.
            $upload = $wrap.find( '.uploadBtn' ),

            // Placeholder before files are selected.
            $placeHolder = $wrap.find( '.placeholder' ),

            $progress = $statusBar.find( '.progress' ).hide(),

            // Added file count.
            fileCount = 0,

            // Total size of added files.
            fileSize = 0,

            // Retina optimization; this value is 2 on retina screens.
            ratio = window.devicePixelRatio || 1,

            // Thumbnail size.
            thumbnailWidth = 110 * ratio,
            thumbnailHeight = 110 * ratio,

            // Possible states: pedding, ready, uploading, confirm, done.
            state = 'pedding',

            // Progress info for all files, keyed by file id.
            percentages = {},
            // Check whether the browser supports image base64.
            isSupportBase64 = ( function() {
                var data = new Image();
                var support = true;
                data.onload = data.onerror = function() {
                    if( this.width != 1 || this.height != 1 ) {
                        support = false;
                    }
                }
                data.src = "data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw==";
                return support;
            } )(),

            // Detect whether Flash is installed and get its version.
            flashVersion = ( function() {
                var version;

                try {
                    version = navigator.plugins[ 'Shockwave Flash' ];
                    version = version.description;
                } catch ( ex ) {
                    try {
                        version = new ActiveXObject('ShockwaveFlash.ShockwaveFlash')
                                .GetVariable('$version');
                    } catch ( ex2 ) {
                        version = '0.0';
                    }
                }
                version = version.match( /\d+/g );
                return parseFloat( version[ 0 ] + '.' + version[ 1 ], 10 );
            } )(),

            supportTransition = (function(){
                var s = document.createElement('p').style,
                    r = 'transition' in s ||
                            'WebkitTransition' in s ||
                            'MozTransition' in s ||
                            'msTransition' in s ||
                            'OTransition' in s;
                s = null;
                return r;
            })(),

            // WebUploader instance.
            uploader;

        if ( !WebUploader.Uploader.support('flash') && WebUploader.browser.ie ) {

            // Flash is installed but the version is too low.
            if (flashVersion) {
                (function(container) {
                    window['expressinstallcallback'] = function( state ) {
                        switch(state) {
                            case 'Download.Cancelled':
								layer.msg(webUploaderT('webuploader_js_00001', null, 'Update canceled!'),2,8); break;
                            case 'Download.Failed':
								layer.msg(webUploaderT('webuploader_js_00002', null, 'Installation failed!'),2,8); break;
                            default:
								layer.msg(webUploaderT('webuploader_js_00003', null, 'Installation completed. Please refresh!'),2,8);return;
                        }
                        delete window['expressinstallcallback'];
                    };

                    var swf = tplPath+'js/webuploader/expressInstall.swf';
                    // insert flash object
                    var html = '<object type="application/' +
                            'x-shockwave-flash" data="' +  swf + '" ';

                    if (WebUploader.browser.ie) {
                        html += 'classid="clsid:d27cdb6e-ae6d-11cf-96b8-444553540000" ';
                    }

                    html += 'width="100%" height="100%" style="outline:0">'  +
                        '<param name="movie" value="' + swf + '" />' +
                        '<param name="wmode" value="transparent" />' +
                        '<param name="allowscriptaccess" value="always" />' +
                    '</object>';

                    container.html(html);

                })($wrap);

            // Flash is not installed at all.
            } else {
                $wrap.html('<a href="http://www.adobe.com/go/getflashplayer" target="_blank" border="0"><img alt="get flash player" src="http://www.adobe.com/macromedia/style_guide/images/160x41_Get_Flash_Player.jpg" /></a>');
            }

            return;
        } else if (!WebUploader.Uploader.support()) {
            layer.msg(webUploaderT('webuploader_js_00004', null, 'Web Uploader does not support your browser!'),2,8);return;
        }

        // Instantiate.
        uploader = WebUploader.create({
            pick: {
                id: '#filePicker',
                label: webUploaderT('webuploader_js_00005', null, 'Select Images')
            },
            formData: {
                usershowid: $usershowid,
				comid: $comid,
				pytoken: $pytoken
            },
            dnd: '#dndArea',
            paste: '#uploader',
            swf: tplPath+'js/webuploader/Uploader.swf',
            chunked: false,
            chunkSize: 512 * 1024,
            server: serverPath,
            // runtimeOrder: 'flash',

             accept: {
                 title: 'Images',
                 extensions: 'gif,jpg,jpeg,bmp,png',
                 mimeTypes: 'image/*'
             },

            // Disable global drag-and-drop so dropped images do not open the page.
            disableGlobalDnd: true,
            fileNumLimit: 50,
            fileSizeLimit: 5 * 1024 * 1024,    // 200 M
            fileSingleSizeLimit: 5 * 1024 * 1024    // 50 M
        });

        // Do not accept js or txt files when dragging.
        uploader.on( 'dndAccept', function( items ) {
            var denied = false,
                len = items.length,
                i = 0,
                // Mark disallowed js type.
                unAllowed = 'text/plain;application/javascript ';

            for ( ; i < len; i++ ) {
                // If the file type is in the disallowed list.
                if ( ~unAllowed.indexOf( items[ i ].type ) ) {
                    denied = true;
                    break;
                }
            }

            return !denied;
        });
/*
        uploader.on('dialogOpen', function() {
            console.log('here');
        });
*/
        // uploader.on('filesQueued', function() {
        //     uploader.sort(function( a, b ) {
        //         if ( a.name < b.name )
        //           return -1;
        //         if ( a.name > b.name )
        //           return 1;
        //         return 0;
        //     });
        // });

        // Add the add-file button.
        uploader.addButton({
            id: '#filePicker2',
            label: webUploaderT('webuploader_js_00006', null, 'Add More')
        });

        uploader.on('ready', function() {
            window.uploader = uploader;
        });

        // Create the item view when a file is added.
        function addFile( file ) {
            var $li = $( '<li id="' + file.id + '">' +
                    '<p class="title">' + file.name + '</p>' +
                    '<p class="imgWrap"></p>'+
                    '<p class="progress"><span></span></p>' +
                    '</li>' ),

                $btns = $('<div class="file-panel">' +
                    '<span class="cancel">' + webUploaderT('webuploader_js_00007', null, 'Delete') + '</span>' +
                    '<span class="rotateRight">' + webUploaderT('webuploader_js_00008', null, 'Rotate Right') + '</span>' +
                    '<span class="rotateLeft">' + webUploaderT('webuploader_js_00009', null, 'Rotate Left') + '</span></div>').appendTo( $li ),
                $prgress = $li.find('p.progress span'),
                $wrap = $li.find( 'p.imgWrap' ),
                $info = $('<p class="error"></p>'),

                showError = function( code ) {
                    switch( code ) {
                        case 'exceed_size':
                            text = webUploaderT('webuploader_js_00010', null, 'File size exceeded');
                            break;

                        case 'interrupt':
                            text = webUploaderT('webuploader_js_00011', null, 'Upload paused');
                            break;

                        default:
                            text = webUploaderT('webuploader_js_00012', null, 'Upload failed. Please try again');
                            break;
                    }

                    $info.text( text ).appendTo( $li );
                };

            if ( file.getStatus() === 'invalid' ) {
                showError( file.statusText );
            } else {
                // @todo lazyload
                $wrap.text(webUploaderT('webuploader_js_00013', null, 'Previewing'));
                uploader.makeThumb( file, function( error, src ) {
                    var img;

                    if ( error ) {
                        $wrap.text(webUploaderT('webuploader_js_00014', null, 'Cannot preview'));
                        return;
                    }

                    if( isSupportBase64 ) {
                        img = $('<img src="'+src+'">');
                        $wrap.empty().append( img );
                    } else {
                        $.ajax('../../server/preview.php', {
                            method: 'POST',
                            data: src,
                            dataType:'json'
                        }).done(function( response ) {
                            if (response.result) {
                                img = $('<img src="'+response.result+'">');
                                $wrap.empty().append( img );
                            } else {
                                $wrap.text(webUploaderT('webuploader_js_00015', null, 'Preview error'));
                            }
                        });
                    }
                }, thumbnailWidth, thumbnailHeight );

                percentages[ file.id ] = [ file.size, 0 ];
                file.rotation = 0;
            }

            file.on('statuschange', function( cur, prev ) {
                if ( prev === 'progress' ) {
                    $prgress.hide().width(0);
                } else if ( prev === 'queued' ) {
                    $li.off( 'mouseenter mouseleave' );
                    $btns.remove();
                }

                // Success.
                if ( cur === 'error' || cur === 'invalid' ) {
                    console.log( file.statusText );
                    showError( file.statusText );
                    percentages[ file.id ][ 1 ] = 1;
                } else if ( cur === 'interrupt' ) {
                    showError( 'interrupt' );
                } else if ( cur === 'queued' ) {
                    $info.remove();
                    $prgress.css('display', 'block');
                    percentages[ file.id ][ 1 ] = 0;
                } else if ( cur === 'progress' ) {
                    $info.remove();
                    $prgress.css('display', 'block');
                } else if ( cur === 'complete' ) {
                    $prgress.hide().width(0);
                    $li.append( '<span class="success"></span>' );
                }

                $li.removeClass( 'state-' + prev ).addClass( 'state-' + cur );
            });

            $li.on( 'mouseenter', function() {
                $btns.stop().animate({height: 30});
            });

            $li.on( 'mouseleave', function() {
                $btns.stop().animate({height: 0});
            });

            $btns.on( 'click', 'span', function() {
                var index = $(this).index(),
                    deg;

                switch ( index ) {
                    case 0:
                        uploader.removeFile( file );
                        return;

                    case 1:
                        file.rotation += 90;
                        break;

                    case 2:
                        file.rotation -= 90;
                        break;
                }

                if ( supportTransition ) {
                    deg = 'rotate(' + file.rotation + 'deg)';
                    $wrap.css({
                        '-webkit-transform': deg,
                        '-mos-transform': deg,
                        '-o-transform': deg,
                        'transform': deg
                    });
                } else {
                    $wrap.css( 'filter', 'progid:DXImageTransform.Microsoft.BasicImage(rotation='+ (~~((file.rotation/90)%4 + 4)%4) +')');
                    // use jquery animate to rotation
                    // $({
                    //     rotation: rotation
                    // }).animate({
                    //     rotation: file.rotation
                    // }, {
                    //     easing: 'linear',
                    //     step: function( now ) {
                    //         now = now * Math.PI / 180;

                    //         var cos = Math.cos( now ),
                    //             sin = Math.sin( now );

                    //         $wrap.css( 'filter', "progid:DXImageTransform.Microsoft.Matrix(M11=" + cos + ",M12=" + (-sin) + ",M21=" + sin + ",M22=" + cos + ",SizingMethod='auto expand')");
                    //     }
                    // });
                }


            });

            $li.appendTo( $queue );
        }

        // Destroy the item view.
        function removeFile( file ) {
            var $li = $('#'+file.id);

            delete percentages[ file.id ];
            updateTotalProgress();
            $li.off().find('.file-panel').off().end().remove();
        }

        function updateTotalProgress() {
            var loaded = 0,
                total = 0,
                spans = $progress.children(),
                percent;

            $.each( percentages, function( k, v ) {
                total += v[ 0 ];
                loaded += v[ 0 ] * v[ 1 ];
            } );

            percent = total ? loaded / total : 0;


            spans.eq( 0 ).text( Math.round( percent * 100 ) + '%' );
            spans.eq( 1 ).css( 'width', Math.round( percent * 100 ) + '%' );
            updateStatus();
        }

        function updateStatus() {
            var text = '', stats;

            if ( state === 'ready' ) {
                text = webUploaderT('webuploader_js_00016', {
                    count: fileCount,
                    size: WebUploader.formatSize( fileSize )
                }, 'Selected {count} images, total {size}.');
            } else if ( state === 'confirm' ) {
                stats = uploader.getStats();
                if ( stats.uploadFailNum ) {
                    text = webUploaderT('webuploader_js_00017', {
                        success: stats.successNum,
                        fail: stats.uploadFailNum
                    }, 'Uploaded {success} photos successfully, {fail} photos failed, ') +
                        '<a class="retry" href="#">' + webUploaderT('webuploader_js_00018', null, 'Retry upload') + '</a>' +
                        webUploaderT('webuploader_js_00019', null, ' failed images or ') +
                        '<a class="ignore" href="#">' + webUploaderT('webuploader_js_00020', null, 'Ignore') + '</a>'
                }

            } else {
                stats = uploader.getStats();
                text = webUploaderT('webuploader_js_00021', {
                    count: fileCount,
                    size: WebUploader.formatSize( fileSize ),
                    success: stats.successNum
                }, 'Total {count} images ({size}), uploaded {success}');

                if ( stats.uploadFailNum ) {
                    text += webUploaderT('webuploader_js_00022', {fail: stats.uploadFailNum}, ', failed {fail}');
                }
            }

            $info.html( text );
        }

        function setState( val ) {
            var file, stats;

            if ( val === state ) {
                return;
            }

            $upload.removeClass( 'state-' + state );
            $upload.addClass( 'state-' + val );
            state = val;
			updateStatus();
            switch ( state ) {
                case 'pedding':
                    $placeHolder.removeClass( 'element-invisible' );
                    $queue.hide();
                    $statusBar.addClass( 'element-invisible' );
                    uploader.refresh();
                    break;

                case 'ready':
                    $placeHolder.addClass( 'element-invisible' );
                    $( '#filePicker2' ).removeClass( 'element-invisible');
                    $queue.show();
                    $statusBar.removeClass('element-invisible');
                    uploader.refresh();
                    break;

                case 'uploading':
                    $( '#filePicker2' ).addClass( 'element-invisible' );
                    $progress.show();
                    $upload.text(webUploaderT('webuploader_js_00023', null, 'Pause Upload'));
                    break;

                case 'paused':
                    $progress.show();
                    $upload.text(webUploaderT('webuploader_js_00024', null, 'Continue Upload'));
                    break;

                case 'confirm':
                    $progress.hide();
                    $( '#filePicker2' ).removeClass( 'element-invisible' );
                    $upload.text(webUploaderT('webuploader_js_00025', null, 'Start Upload'));

                    stats = uploader.getStats();
                    if ( stats.successNum && !stats.uploadFailNum ) {
                        setState( 'finish' );
                        return;
                    }
                    break;
                case 'finish':
                    stats = uploader.getStats();
                    if ( stats.successNum ) {
						if(returnUrl){
							layer.msg(webUploaderT('webuploader_js_00026', null, 'Upload successful!'),2,9,function(data){window.location.href=returnUrl;});
						}else{
							layer.msg(webUploaderT('webuploader_js_00026', null, 'Upload successful!'),2,9);
						}
                    } else {
                        // No image uploaded successfully; reset.
                        state = 'done';
                        location.reload();
                    }
                    break;
            }


        }
		uploader.on( 'uploadProgress', function( file, percentage ) {
			layer.load(webUploaderT('webuploader_js_00027', null, 'Processing, please wait...'),0);return;
		});
        uploader.onUploadProgress = function( file, percentage ) {
            var $li = $('#'+file.id),
                $percent = $li.find('.progress span');

            $percent.css( 'width', percentage * 100 + '%' );
            percentages[ file.id ][ 1 ] = percentage;
            updateTotalProgress();
        };

        uploader.onFileQueued = function( file ) {
            fileCount++;
            fileSize += file.size;

            if ( fileCount === 1 ) {
                $placeHolder.addClass( 'element-invisible' );
                $statusBar.show();
            }

            addFile( file );
            setState( 'ready' );
            updateTotalProgress();
        };

        uploader.onFileDequeued = function( file ) {
            fileCount--;
            fileSize -= file.size;

            if ( !fileCount ) {
                setState( 'pedding' );
            }

            removeFile( file );
            updateTotalProgress();

        };

        uploader.on( 'all', function( type ) {
            var stats;
            switch( type ) {
                case 'uploadFinished':
                    setState( 'confirm' );
                    break;

                case 'startUpload':
                    setState( 'uploading' );
                    break;

                case 'stopUpload':
                    setState( 'paused' );
                    break;

            }
        });

        uploader.onError = function( code ) {
            alert( 'Eroor: ' + code );
        };

        $upload.on('click', function() {
            if ( $(this).hasClass( 'disabled' ) ) {
                return false;
            }

            if ( state === 'ready' ) {
                uploader.upload();
            } else if ( state === 'paused' ) {
                uploader.upload();
            } else if ( state === 'uploading' ) {
                uploader.stop();
            }
        });

        $info.on( 'click', '.retry', function() {
            uploader.retry();
        } );

        $info.on( 'click', '.ignore', function() {
            alert( 'todo' );
        } );

        $upload.addClass( 'state-' + state );
        updateTotalProgress();
    });

})( jQuery );
