pub struct Release<'a> {
    pub url: &'a str,
}

#[cfg(target_os = "linux")]
pub static RELEASE_INFO: Release = Release {
    url: "https://github.com/shader-slang/slang/releases/download/v0.24.38/slang-0.24.38-linux-x86_64.tar.gz",
};
// #[cfg(target_os = "windows")]
// pub static RELEASE_INFO: Release = Release {
//     url: "https://github.com/shader-slang/slang/releases/download/v0.24.38/slang-0.24.38-win64.zip",
// };

fn main() {}

fn download() {}
