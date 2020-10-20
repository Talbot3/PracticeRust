use {
    std::fs,
    std::env,
    std::process,
    glib::{MainContext, MainLoop},
    gstreamer_rtsp_server::prelude::*,
    gstreamer_rtsp_server::{RTSPMediaFactory, RTSPServer},
};

fn main() {

    // 读取参数
    let args: Vec<String> =env::args().collect();
    
        println!("请需要需要模拟的rtsp流的数量 $: ");
        println!("{:?}", args);
        let mut count: i32 = match args[1].trim().parse() {
            Ok(num) => num,
            Err(error) => panic!("Problem Invalidate Int: {:?}", error),
        };

        gstreamer::init().unwrap();
        let media_factory = RTSPMediaFactory::new();
        media_factory.set_launch("videotestsrc ! x264enc key-int-max=40 ! rtph264pay name=pay0 pt=96");
        media_factory.set_shared(true);

        let main_context = MainContext::default();
        let server = RTSPServer::new();
        let source_id = server.attach(Some(&main_context));
        let mount_points = server.get_mount_points().unwrap();

        // 数组
        let mut end_points: Vec<String> = Vec::new();
        let mut index = 0;
        while count != 0 {
            let mut end_point = String::from("/");
            let count_str = count.to_string();
            end_point.push_str(&count_str);
            end_points.push(end_point);
            mount_points.add_factory(&end_points[index], &media_factory);
            count -= 1;
            index += 1;
        }

        let main_loop = MainLoop::new(Some(&main_context), false);

        let address = server.get_address().unwrap();
        let port = server.get_bound_port();
        
        // 迭代输出rtsp模拟流
        let path_iter = end_points.iter();
        println!("server running");
        for path in path_iter {
            let location = format!("rtsp://{}:{}{}", address, port, path);
            println!("{}", location);
        }
        
        main_loop.run();
        glib::source_remove(source_id);
}
