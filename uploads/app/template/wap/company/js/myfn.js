// JavaScript Document
$(function(){
    var curpage=1;
    var totalpage,nextpage,lastpage,nexttotalpage;
    totalpage = $(".page").length;
    nexttotalpage = totalpage + 1;

    window.onload = function(){
        $(".loader").css("z-index","1");
        $(".upicon").css("z-index","1000");
        initAudio("audio");
    }

    var audio;
    function initAudio(id){
        audio =  document.getElementById(id);
    }
    document.addEventListener(‘touchmove‘,function(event){
        event.preventDefault(); },false);



    $(".page").swipeUp(function(){
        swichpage();
    })

    $("#audioPlay").on(‘click‘,function(){
        if(audio.paused){
            audio.play();
            this.style.backgroundImage="url({yun:}$wap_style{/yun}/images/music_play.png)";
        }else{
            audio.pause();
            this.style.backgroundImage="url({yun:}$wap_style{/yun}/images/music_pause.png)";
        }
    });

    function swichpage() {






        if (curpage == totalpage) {
            for (var i = 1; i < nexttotalpage; i++) {
                $(".page" + i).removeClass("hide");
                $(".page" + i).removeClass("show");
                $(".page" + i).removeClass("pt-page-moveFromBottomFade");
            }
            $(".page1").addClass("show");
            $(".page1").addClass("pt-page-moveFromBottomFade");
            curpage = 1;
        } else {
            nextpage = curpage + 1;
            lastpage = curpage - 1;
            $(".page" + curpage).removeClass("pt-page-moveFromBottomFade");
            $(".page" + curpage).addClass("pt-page-moveToTopFade");
            $(".page" + curpage).removeClass("show");
            $(".page" + curpage).addClass("hide");
            $(".page" + nextpage).removeClass("hide");
            $(".page" + nextpage).addClass("show");
            $(".page" + nextpage).addClass("pt-page-moveFromBottomFade");
            $(".page" + lastpage).removeClass("pt-page-moveToTopFade");
            curpage = nextpage;
        }
    }
})
//the end
   curpage = nextpage;
        }
    }
  setInterval(function(){
        swichpage();
    },4000);
})
// the end
