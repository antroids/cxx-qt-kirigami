use cxx_qt_lib::QGuiApplication;
use cxx_qt_lib::QQmlApplicationEngine;
use cxx_qt_lib::{QString, QUrl};
use std::env;

mod cxxqt_q_localized_context;
mod cxxqt_q_template_object;

fn main() {
    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    let mut kde_qml_dir = String::from("/usr/lib/x86_64-linux-gnu/qt6/qml/");

    if let Ok(qml_dir) = env::var("KDE_QMLDIR") {
        kde_qml_dir = qml_dir;
    } else {
        println!(
            "cargo:warning=KDE_QMLDIR is not defined, used default value: {}",
            kde_qml_dir
        );
    }

    if let Some(engine) = engine.as_mut() {
        engine.add_import_path(&QString::from(kde_qml_dir.as_str()));
    }

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:./content/App.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
