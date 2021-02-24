use clap::App;

pub fn add_generic_info(cmd: App<'static, 'static>) -> App<'static, 'static> {
    cmd.version("0.0.1") .author("Efi Weiss <valmarelox@gmail.com>")
}

