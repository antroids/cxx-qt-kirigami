use cxx_qt_build::CxxQtBuilder;
use std::path::PathBuf;
use std::{env, fs};

fn main() {
    let mut kde_include_dir = String::from("/usr/include/");
    let mut kde_lib_dir = String::from("/usr/lib/x86_64-linux-gnu/");

    if let Ok(include_dir) = env::var("KDE_INCLUDEDIR") {
        kde_include_dir = include_dir;
    } else {
        println!(
            "cargo:warning=KDE_INCLUDEDIR is not defined, used default value: {}",
            kde_include_dir
        );
    }
    if let Ok(lib_dir) = env::var("KDE_LIBDIR") {
        kde_lib_dir = lib_dir;
    } else {
        println!(
            "cargo:warning=KDE_LIBDIR is not defined, used default value: {}",
            kde_lib_dir
        );
    }

    let ki18n_include_path = PathBuf::from(kde_include_dir)
        .canonicalize()
        .expect("Cannot get canonical path of KDE_INCLUDEDIR")
        .join("KF6")
        .join("KI18n");
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        // Generate C++ from the `#[cxx_qt::bridge]` module
        .file("src/cxxqt_q_localized_context.rs")
        .file("src/cxxqt_q_template_object.rs")
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .qrc("qml/qml.qrc")
        //.qobject_header("cpp/qlocalizedcontext.h")
        .cc_builder(|cc| {
            cc.include("cpp");
            cc.include(format!("{}", ki18n_include_path.display()));
        })
        .build();

    println!(
        "cargo:rustc-link-search={}",
        fs::canonicalize(kde_lib_dir)
            .expect("Cannot get canonical path of KDE_LIBDIR")
            .display()
    );
    println!("cargo:rustc-link-lib=KF6I18n");
}
