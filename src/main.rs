use sysinfo::{System, SystemExt};


fn main(){
    if cfg!(windows) {
        windows_fetch();
    } else if cfg!(unix) {
        linux_fetch();
    }else if cfg!(target_os = "macos") {
        mac_fetch();
    }
    


}

fn linux_fetch() {

    let mut sys = System::new_all();

    sys.refresh_all();


    println!("                     .88888888:.");
    println!("                88888888.88888.");
    println!("              .8888888888888888.");
    println!("             888888888888888888");
    println!("            88' _`88'_  `88888");
    println!("             88 88 88 88  88888");
    println!("             88_88_::_88_:88888");
    println!("             88:::,::,:::::8888");
    println!("             88`:::::::::'`8888               OS: {:?}", sys.name());
    println!("            .88  `::::'    8:88.              Hostname: {:?}", sys.host_name());
    println!("           8888            `8:888.            Kernel: {:?}", sys.kernel_version());
    println!("          .8888'             `888888.         System booted at {} seconds", sys.boot_time());
    println!("        .8888:..  .::.  ...:'8888888:.        System running since {} seconds", sys.uptime());
    println!("       .8888.'     :'     `'::`88:88888       number of processors: {}", sys.processors().len());
    println!("      .8888        '         `.888:8888.      Memory: {} / {}",sys.used_memory(), sys.total_memory()  );
    println!("     888:8         .           888:88888      Swap: {} / {}",sys.used_swap(), sys.total_swap() ); 
    println!("   .888:88        .:           888:88888:");  
    println!("   8888888.       ::           88:888888");
    println!("   `.::.888.      ::          .88888888");
    println!("  .::::::.888.    ::         :::`8888'.:.");
    println!("  ::::::::::.888            .::::::::::::");
    println!("  ::::::::::::.8    '      .:8::::::::::::.");
    println!(" .::::::::::::::.        .:888:::::::::::::");
    println!(" :::::::::::::::88:.__..:88888:::::::::::'");
    println!("  `'.:::::::::::88888888888.88:::::::::'");
    println!("       `':::_:' -- '' -'-' `':_::::'`");
}

fn windows_fetch() {

    let mut sys = System::new_all();

    sys.refresh_all();

    println!("                   .oodMMMMMMMMMMMMM");
    println!("       ..oodMMM  MMMMMMMMMMMMMMMMMMM");
    println!(" oodMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM");
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          OS: {:?}", sys.name());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          Hostname: {:?}", sys.host_name());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          Kernel: {:?}", sys.kernel_version());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          System booted at {} seconds", sys.boot_time());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          System running since {} seconds", sys.uptime());
    println!("                                              number of processors: {}", sys.processors().len());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          Memory: {} / {}",sys.used_memory(), sys.total_memory());
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM          Swap: {} / {}",sys.used_swap(), sys.total_swap() ); 
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM");
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM");
    println!(" MMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM");
    println!(" `^^^^^^MMMMMMM  MMMMMMMMMMMMMMMMMMM");
    println!("       ````^^^^  ^^MMMMMMMMMMMMMMMMM");
    println!("                     ````^^^^^^MMMM");
}

fn mac_fetch() {

    let mut sys = System::new_all();

    sys.refresh_all();
    
    println!("                            .8"); 
    println!("                      .888");
    println!("                    .8888'");
    println!("                   .8888'");
    println!("                   888'");
    println!("                   8'");
    println!("      .88888888888. .88888888888.");
    println!("   .8888888888888888888888888888888.");
    println!(" .8888888888888888888888888888888888.");
    println!(".&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'");
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'         OS: {:?}", sys.name());  
    println!("&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&'          Hostname: {:?}", sys.host_name());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:           Kernel: {:?}", sys.kernel_version());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:           System booted at {} seconds", sys.boot_time());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@:           System running since {} seconds", sys.uptime());
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.          number of processors: {}", sys.processors().len());
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.         Memory: {} / {}",sys.used_memory(), sys.total_memory());
    println!("`%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%.       Swap: {} / {}",sys.used_swap(), sys.total_swap() ); 
    println!(" `00000000000000000000000000000000000'");
    println!(" `000000000000000000000000000000000'");
    println!("   `0000000000000000000000000000000'");
    println!("     `###########################'");
    println!("       `#######################'");
    println!("         `#########''########'");
}
