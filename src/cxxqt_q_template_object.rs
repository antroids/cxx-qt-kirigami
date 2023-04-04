#[cxx_qt::bridge(cxx_file_stem = "q_template_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "org.cxx_qt_kirigami.template", qml_version = "1.0")]
    #[derive(Default)]
    pub struct QTemplateObject {
        #[qproperty]
        string_property: QString
    }

    impl qobject::QTemplateObject {
        #[qinvokable]
        fn update(mut self: Pin<&mut Self>) {
            self.as_mut().set_string_property(QString::from("Hello world!"));
        }
    }
}