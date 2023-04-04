use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        // Link Qt's Network library
        // - Qt Core is always linked
        // - Qt Gui is linked by enabling the qt_gui Cargo feature (default).
        // - Qt Qml is linked by enabling the qt_qml Cargo feature (default).
        // - Qt Qml requires linking Qt Network on macOS
        .qt_module("Network")
        // Generate C++ from the `#[cxx_qt::bridge]` module
        //.file("src/cxxqt_q_global_object.rs")
        //.file("src/cxxqt_q_global_object_list_model.rs")
        .file("src/cxxqt_q_map_list_model.rs")
        .file("src/cxxqt_q_pipe_wire.rs")
        // Generate C++ code from the .qrc file with the rcc tool
        // https://doc.qt.io/qt-6/resources.html
        .qrc("qml/qml.qrc")
        .cc_builder(|cc| {
            cc.include("cpp");
            cc.file("cpp/qglobalobject.cpp");
        })
        // .qobject_header("include/qglobalobject.h")
        .setup_linker()
        .build();
}
