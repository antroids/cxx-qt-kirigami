#[cxx_qt::bridge(cxx_file_stem = "q_localized_context")]
mod ffi {
    unsafe extern "C++" {
        include!("klocalizedcontext.h");
    }

    #[cxx_qt::qobject(
        base = "KLocalizedContext",
        qml_uri = "org.cxx_qt_kirigami.template",
        qml_version = "1.0"
    )]
    #[derive(Default)]
    pub struct QLocalizedContext {}
}
