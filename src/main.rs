use cxx_qt_lib::{QString, QUrl};
use cxx_qt_lib::QGuiApplication;
use cxx_qt_lib::QQmlApplicationEngine;

mod cxxqt_q_template_object;

fn main() {
    // Create the application and engine
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    // Load the QML path into the engine
    // if let Some(engine) = engine.as_mut() {
    //     engine.add_import_path(&QString::from("qrc:./imports/"));
    // }

    if let Some(engine) = engine.as_mut() {
        engine.add_import_path(&QString::from("/usr/lib/x86_64-linux-gnu/qt6/qml"));
    }

    // Load the QML path into the engine
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:./content/App.qml"));
    }

    // Start the app
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
