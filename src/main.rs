extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand, Shell};

fn main() {
    let build_options =
    "--help                       'Print this help and exit'
     --build-file [file]          'Override path to build.zig'
     --cache-dir [path]           'Override path to cache directory'
     --verbose                    'Print commands before executing them'
     --debug-build-verbose        'Print verbose debugging information for the build system itself'
     --prefix [prefix]            'Override default install prefix'";

    let compile_options =
    "--assembly [source]          'add assembly file to build'
     --cache-dir [path]           'override the cache directory'
     --color [auto|off|on]        'enable or disable colored error messages'
     --enable-timing-info         'print timing diagnostics'
     --libc-include-dir [path]    'directory where libc stdlib.h resides'
     --name [name]                'override output name'
     --output [file]              'override destination path'
     --output-h [file]            'override generated header file path'
     --pkg-begin [name] [path]    'make package available to import and push current pkg'
     --pkg-end                    'pop current pkg'
     --release-fast               'build with optimizations on and safety off'
     --release-safe               'build with optimizations on and safety on'
     --static                     'output will be statically linked'
     --strip                      'exclude debug symbols'
     --target-arch [name]         'specify target architecture'
     --target-environ [name]      'specify target environment'
     --target-os [name]           'specify target operating system'
     --verbose                    'turn on compiler debug output'
     --zig-std-dir [path]         'directory where zig standard library resides'
     --dirafter [dir]             'same as -isystem but do it last'
     --isystem [dir]              'add additional search path for other .h files'";

    let linker_options =
    "--ar-path [path]             'set the path to ar'
     --dynamic-linker [path]      'set the path to ld.so'
     --each-lib-rpath             'add rpath for each used dynamic library'
     --libc-lib-dir [path]        'directory where libc crt1.o resides'
     --libc-static-lib-dir [path] 'directory where libc crtbegin.o resides'
     --library [lib]              'link against lib'
     --library-path [dir]         'add a directory to the library search path'
     --linker-script [path]       'use a custom linker script'
     --object [obj]               'add object file to build'
     --L[dir]                     'alias for --library-path'
     --rdynamic                   'add all symbols to the dynamic symbol table'
     --rpath [path]               'add directory to the runtime library search path'
     --mconsole                   '(windows) --subsystem console to the linker'
     --mwindows                   '(windows) --subsystem windows to the linker'
     --municode                   '(windows) link with unicode'
     --framework [name]           '(darwin) link against framework'
     --mios-version-min [ver]     '(darwin) set iOS deployment target'
     --mlinker-version [ver]      '(darwin) override linker version'
     --mmacosx-version-min [ver]  '(darwin) set Mac OS X deployment target'
     --ver-major [ver]            'dynamic library semver major version'
     --ver-minor [ver]            'dynamic library semver minor version'
     --ver-patch [ver]            'dynamic library semver patch version'";

    let test_options =
    "--test-filter [text]         'skip tests that do not match filter'
     --test-name-prefix [text]    'add prefix to all tests'";

    let _ =
    "build                        'build project from build.zig'
     build_exe [source]           'create executable from source or object files'
     build_lib [source]           'create library from source or object files'
     build_obj [source]           'create object from source or assembly'
     parseh [source]              'convert a c header file to zig extern declarations'
     targets                      'list available compilation targets'
     test [source]                'create and run a test build'
     version                      'print version number and exit'
     zen                          'print zen of zig and exit'";

    let mut app =
        App::new("zig")
            .version("0.0.0")
            .setting(AppSettings::ArgRequiredElseHelp)
            .setting(AppSettings::DisableHelpSubcommand)
            .setting(AppSettings::DisableVersion)
            .setting(AppSettings::UnifiedHelpMessage)
            .setting(AppSettings::VersionlessSubcommands)

            .subcommand(SubCommand::with_name("build")
                        .about("build project from build.zig")
                        .args_from_usage(build_options))

            .subcommand(SubCommand::with_name("build_exe")
                        .about("create executable from source or object files")
                        .arg(Arg::with_name("source")
                                .required(true)
                                .index(1))
                        .args_from_usage(compile_options)
                        .args_from_usage(linker_options))

            .subcommand(SubCommand::with_name("build_lib")
                        .about("create library from source or object files")
                        .arg(Arg::with_name("source")
                                .required(true)
                                .index(1))
                        .args_from_usage(compile_options)
                        .args_from_usage(linker_options))

            .subcommand(SubCommand::with_name("build_obj")
                        .about("create object from source or assembly")
                        .arg(Arg::with_name("source")
                                .required(true)
                                .index(1))
                        .args_from_usage(compile_options)
                        .args_from_usage(linker_options))


            .subcommand(SubCommand::with_name("parseh")
                        .arg(Arg::with_name("source")
                                .required(true)
                                .index(1))
                        .about("convert a c header file to zig extern declarations"))

            .subcommand(SubCommand::with_name("targets")
                        .about("list available compilation targets"))

            .subcommand(SubCommand::with_name("test")
                        .about("create and run a test build")
                        .arg(Arg::with_name("source")
                                .required(true)
                                .index(1))
                        .args_from_usage(test_options))

            .subcommand(SubCommand::with_name("version")
                        .about("print version number and exit"))

            .subcommand(SubCommand::with_name("zen")
                        .about("print zen of zig and exit"));


    app.gen_completions("zig", Shell::Bash, "completions");
    app.gen_completions("zig", Shell::Zsh, "completions");
    //app.gen_completions("zig", Shell::Fish, "completions");
    //app.gen_completions("zig", Shell::PowerShell, "completions");

    app.get_matches();
}
