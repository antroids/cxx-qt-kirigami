use cxx_qt_build::CxxQtBuilder;

fn main() {

    println!("cargo:rustc-link-search=/usr/lib/x86_64-linux-gnu/libKF6Kirigami2.so");

    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        // Generate C++ from the `#[cxx_qt::bridge]` module
        .file("src/cxxqt_q_template_object.rs")
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .qrc("qml/qml.qrc")
        .cc_builder(|cc| {
            cc.include("cpp");
        })
        .build();
}
