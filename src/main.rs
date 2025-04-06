// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate rocket;
// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate serde;
// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate serde_json;
#[allow(unused_imports)]
use rocket::{get, launch, routes, catchers, uri, Build, Rocket, http::{ext, hyper, uncased, uri}};
#[allow(unused_imports)]
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
#[warn(unused_imports)]
pub mod about;
static_response_handler! {
    "/favicon.ico"                  => favicon              => "favicon",
    "/favicon-16.png"               => favicon_png_16s      => "favicon-png-16",
    "/favicon-16x16.png"            => favicon_png_16       => "favicon-png-16",
    "/favicon-32.png"               => favicon_png_32s      => "favicon-png-32",
    "/favicon-32x32.png"            => favicon_png_32       => "favicon-png-32",
    "/android-chrome-192x192.png"   => favicon_png_192      => "android-chrome-192",
    "/android-chrome-512x512.png"   => favicon_png_512      => "android-chrome-512",
    "/apple-touch-icon.png"         => favicon_png_apple    => "apple-touch-icon",
}

#[cfg(not(test))]
#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon"               => "rsrc/io.favicon/emoji/Zzz/favicon.ico",
            "favicon-png-16"        => "rsrc/io.favicon/emoji/Zzz/favicon-16x16.png",
            "favicon-png-32"        => "rsrc/io.favicon/emoji/Zzz/favicon-32x32.png",
            "android-chrome-192"    => "rsrc/io.favicon/emoji/Zzz/android-chrome-192x192.png",
            "android-chrome-512"    => "rsrc/io.favicon/emoji/Zzz/android-chrome-512x512.png",
            "apple-touch-icon"      => "rsrc/io.favicon/emoji/Zzz/apple-touch-icon.png",
        ))
        .mount("/", rocket::routes![favicon, favicon_png_16s, favicon_png_16,
                            favicon_png_32s, favicon_png_32,
                            favicon_png_192, favicon_png_512,
                            favicon_png_apple, ])
        .mount("/", rocket::routes![about::get_about])
}
