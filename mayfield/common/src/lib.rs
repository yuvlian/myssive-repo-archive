pub fn init_tracing() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support().expect("failed to enable ansi");

    tracing_subscriber::fmt().init();
}

const MAYFIELD_ASCII: &str = r#"
                o-o      o    o 
                |  o     |    | 
o-O-o  oo o  o -O-   o-o |  o-O 
| | | | | |  |  |  | |-' | |  | 
o o o o-o-o--O  o  | o-o o  o-o 
             |                  
          o--o                                      
"#;

pub fn print_mayfield() {
    println!("{}", MAYFIELD_ASCII);
}
