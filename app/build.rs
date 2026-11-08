use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    // Registers the Rust QObject in src/backend.rs and bundles the QML files
    // into the executable under qrc:/qt/qml/org/pdf2md/...
    CxxQtBuilder::new()
        .qml_module(QmlModule {
            uri: "org.pdf2md",
            rust_files: &["src/backend.rs"],
            qml_files: &["qml/Main.qml", "qml/FileBrowser.qml"],
            ..Default::default()
        })
        .build();
}
