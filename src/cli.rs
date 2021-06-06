use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgSettings, ValueHint};

pub fn build_cli() -> App<'static> {
    App::new("flavours")
        .about("A simple way to manage and use base16 standard schemes and templates")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::UnifiedHelpMessage)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::InferSubcommands)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::new("verbose")
            .about("Be more verbose")
            .long("verbose")
            .short('v')
        )
        .arg(
            Arg::new("config")
            .about("Specify a configuration file (Defaults to ~/.config/flavours/config.toml on Linux)")
            .long("config")
            .short('c')
            .value_name("FILE")
            .value_hint(ValueHint::FilePath)
            .takes_value(true)
        )
        .arg(
            Arg::new("directory")
            .about("Specify a data directory (Defaults to ~/.local/share/flavours on Linux)")
            .long("directory")
            .short('d')
            .value_name("DIRECTORY")
            .value_hint(ValueHint::DirPath)
            .takes_value(true)
        )
        .arg(
            Arg::new("completions")
            .setting(ArgSettings::Hidden)
            .about("Generates completion for given shell, outputs to stdout")
            .long("completions")
            .takes_value(true)
            .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"])
        )
        .subcommand(
            App::new("current")
                .about("Prints last applied scheme name")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
        )
        .subcommand(
            App::new("list")
                .about("Prints a list with all matching schemes")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("pattern")
                    .about("Scheme name or glob pattern to match when listing scheme(s). If ommited, defaults to * (all installed schemes).")
                    .setting(ArgSettings::MultipleValues)
                    .value_hint(ValueHint::Other)
                    .multiple(true)
                )
                .arg(
                    Arg::new("lines")
                    .about("Print each scheme on its own line")
                    .long("lines")
                    .short('l')
                )
        )
        .subcommand(
            App::new("info")
                .about("Shows scheme colors for all schemes matching pattern. Optionally uses truecolor")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("pattern")
                    .about("Scheme name or glob pattern to match when showing scheme(s). If ommited, defaults to * (all installed schemes).")
                    .setting(ArgSettings::MultipleValues)
                    .value_hint(ValueHint::Other)
                    .multiple(true)
                )
                .arg(
                    Arg::new("raw")
                    .about("Don't pretty print the colors.")
                    .long("raw")
                    .short('r')
                )
        )
        .subcommand(
            App::new("generate")
                .about("Generates a scheme based on an image")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("mode")
                    .about("Whether to generate a dark or light scheme")
                    .possible_values(&["dark", "light"])
                    .required(true)
                    .value_hint(ValueHint::Other)
                )
                .arg(
                    Arg::new("file")
                    .about("Which image file to use.")
                    .required(true)
                    .value_hint(ValueHint::FilePath)
                )
                .arg(
                    Arg::new("slug")
                    .long("slug")
                    .short('s')
                    .about("Scheme slug (the name you specify when applying schemes) to output to. If ommited, defaults to 'generated'")
                    .value_name("slug")
                    .takes_value(true)
                    .value_hint(ValueHint::Other)
                )
                .arg(
                    Arg::new("name")
                    .long("name")
                    .short('n')
                    .about("Scheme display name (can include spaces and capitalization) to write, defaults to 'Generated'")
                    .value_name("name")
                    .takes_value(true)
                    .value_hint(ValueHint::Other)
                )
                .arg(
                    Arg::new("author")
                    .long("author")
                    .short('a')
                    .about("Scheme author info (name, email, etc) to write, defaults to 'Flavours'")
                    .value_name("author")
                    .takes_value(true)
                    .value_hint(ValueHint::Other)
                )
                .arg(
                    Arg::new("stdout")
                    .about("Outputs scheme to stdout instead of writing it to a file.")
                    .long("stdout")
                )
        )
        .subcommand(
            App::new("apply")
                .about("Applies scheme, according to user configuration")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("pattern")
                    .about("Scheme to be applied, supports glob. If more than one is specified (or if glob pattern matched more than one), chooses one randomly. If ommited, defaults to * (all installed schemes).")
                    .value_hint(ValueHint::Other)
                    .setting(ArgSettings::MultipleValues)
                    .multiple(true)
                )
                .arg(
                    Arg::new("light")
                    .about("Skip running heavier hooks (entries marked 'light=false')")
                    .long("light")
                    .short('l')
                )
        )
        .subcommand(
            App::new("update")
                .about("Downloads schemes, templates, or updates their lists (from repos specified in sources.yml)")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("operation")
                    .about("Update sources lists from repositories or (re)download schemes/templates specified in the lists. Default repositories for lists, and the lists themselves, can be manually changed.")
                    .required(true)
                    .possible_values(&["lists", "schemes", "templates", "all"])
                )
        )
        .subcommand(
            App::new("build")
                .about("Builds a template with given scheme, outputs to stdout")
                .setting(AppSettings::UnifiedHelpMessage)
                .setting(AppSettings::DeriveDisplayOrder)
                .setting(AppSettings::DisableHelpSubcommand)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp)
                .arg(
                    Arg::new("scheme")
                    .about("Path to scheme file.")
                    .required(true)
                    .value_hint(ValueHint::FilePath)
                )
                .arg(
                    Arg::new("template")
                    .about("Path to template file.")
                    .value_hint(ValueHint::FilePath)
                )
        )
}
