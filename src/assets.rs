use crate::helpers::inline::InlineAssets;

macro_rules! inline {
    (@ $id:ident,$path:literal,$version:literal,$ext:literal,$tag:expr,$mime:literal,$dev_reload:literal) => {
        pub const $id: InlineAssets = const {
            InlineAssets {
                content: include_bytes!(concat!("../dist/",$path,"@",$version,$ext)),
                last_modified: include_str!(concat!("../dist/",$path,"@",$version,$ext,".stamp")),
                tag: $tag,
                path: concat!("dist/",$path,"@",$version,$ext),
                serve_path: concat!("/dist/",$path,"@",$version,$ext),
                mime: $mime,
                dev_reload: $dev_reload,
            }
        };
    };
    ($id:ident,$path:literal,$version:literal,$ext:literal,@,$mime:literal,$dev_reload:literal) => {
        inline!(@ $id,$path,$version,$ext,$version,$mime,$dev_reload);
    };
    ($id:ident,$path:literal,$version:literal,$ext:literal,?,$mime:literal,$dev_reload:literal) => {
        inline!(@ $id,$path,$version,$ext,include_str!(concat!("../dist/",$path,"@",$version,$ext,".tag")),$mime,$dev_reload);
    }
}

inline!(CSS,"output","0.1",".css",?,"text/css",true);
inline!(HX,"hx","2.0.4",".js",@,"application/json",false);
inline!(CAROUSEL,"carousel","8.5.2",".js",@,"application/json",false);

#[cfg(debug_assertions)]
pub fn tw_build() {
    use std::{
        io::Read,
        process::{Command, Stdio},
        thread,
    };

    fn tw_task() -> anyhow::Result<()> {
        let mut tw = Command::new("dist/tailwind")
            .args(["-i","assets/app.css","-o","dist/output@0.1.css","--watch"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut buf = [0u8;1024];
        loop {
            match tw.stderr.as_mut().unwrap().read(&mut buf)? {
                0 => return Ok(()),
                n => if let Ok(line) = std::str::from_utf8(&buf[..n]) {
                    print!("[TW] {line}");
                }
            }
        }
    }

    thread::spawn(|| if let Err(err) = tw_task() {
        eprintln!("tailwind task failed: {err}")
    });
}

