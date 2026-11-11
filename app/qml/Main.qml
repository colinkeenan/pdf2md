import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.pdf2md

Kirigami.ApplicationWindow {
    id: root

    // Ask for a normal window with title bar and borders.
    flags: Qt.Window | Qt.WindowTitleHint | Qt.WindowSystemMenuHint
         | Qt.WindowMinMaxButtonsHint | Qt.WindowCloseButtonHint

    title: "PDF to Markdown"
    // Wide enough for the form with the page's padding around it, however
    // long the chosen paths are (but not wider than the screen): the
    // window grows instead of the buttons running into its right edge.
    readonly property real formWidth: Math.ceil(form.implicitWidth + page.leftPadding + page.rightPadding)
    minimumWidth: Math.max(Kirigami.Units.gridUnit * 24, Math.min(formWidth, Screen.desktopAvailableWidth))
    minimumHeight: Kirigami.Units.gridUnit * 18
    width: Kirigami.Units.gridUnit * 36
    height: Kirigami.Units.gridUnit * 24

    // file:// URLs as strings
    property string inputUrl: ""
    property string outputUrl: ""
    property bool outputChosenByUser: false

    function parentOf(url) {
        return url.substring(0, url.lastIndexOf("/"))
    }
    function localName(url) {
        return decodeURIComponent(url.substring(url.lastIndexOf("/") + 1))
    }
    function localPath(url) {
        return decodeURIComponent(url.replace("file://", ""))
    }
    function stemOf(url) {
        var n = localName(url)
        var dot = n.lastIndexOf(".")
        return dot > 0 ? n.substring(0, dot) : n
    }

    // The Rust object from src/backend.rs
    Converter {
        id: converter
    }

    Component {
        id: browserComponent
        FileBrowser {}
    }

    function openBrowser(chooseFolder, onPicked) {
        var page = pageStack.push(browserComponent, {
            chooseFolder: chooseFolder,
            startFolder: converter.folder
        })
        page.picked.connect(function (chosen) {
            onPicked(chosen.toString())
            pageStack.pop()
        })
    }

    // asked when the book folder is already there
    Kirigami.PromptDialog {
        id: existsDialog
        title: "Book folder already exists"
        subtitle: "“" + root.localPath(root.outputUrl) + "/" + root.stemOf(root.inputUrl) + "/” is already there.\n\n"
            + "Replace moves the old files to the Trash and writes the new conversion into the same folder. "
            + "Keep Both writes the new conversion to “" + root.stemOf(root.inputUrl) + " (1)” (or the next free number)."
        standardButtons: Kirigami.Dialog.Cancel
        customFooterActions: [
            Kirigami.Action {
                text: "Replace"
                icon.name: "user-trash"
                onTriggered: {
                    existsDialog.close()
                    converter.convert(root.inputUrl, root.outputUrl, true)
                }
            },
            Kirigami.Action {
                text: "Keep Both"
                icon.name: "folder-new"
                onTriggered: {
                    existsDialog.close()
                    converter.convert(root.inputUrl, root.outputUrl, false)
                }
            }
        ]
    }

    pageStack.initialPage: Kirigami.Page {
        id: page
        title: "Convert a textbook"

        ColumnLayout {
            anchors.fill: parent
            spacing: Kirigami.Units.largeSpacing

            Kirigami.FormLayout {
                id: form
                Layout.fillWidth: true

                RowLayout {
                    Kirigami.FormData.label: "Textbook:"
                    QQC2.TextField {
                        Layout.fillWidth: true
                        // as wide as the path in it
                        implicitWidth: Math.max(Kirigami.Units.gridUnit * 12, Math.ceil(contentWidth) + leftPadding + rightPadding)
                        readOnly: true
                        placeholderText: "PDF file"
                        text: root.inputUrl === "" ? "" : root.localPath(root.inputUrl)
                    }
                    QQC2.Button {
                        icon.name: "document-open"
                        text: "Choose…"
                        onClicked: root.openBrowser(false, function (url) {
                            root.inputUrl = url
                            var folder = root.parentOf(url)
                            converter.remember(folder)
                            // output defaults to the folder the book is in
                            if (!root.outputChosenByUser) {
                                root.outputUrl = folder
                            }
                        })
                    }
                }

                RowLayout {
                    Kirigami.FormData.label: "Output folder:"
                    QQC2.TextField {
                        Layout.fillWidth: true
                        // as wide as the path in it
                        implicitWidth: Math.max(Kirigami.Units.gridUnit * 12, Math.ceil(contentWidth) + leftPadding + rightPadding)
                        readOnly: true
                        placeholderText: "Defaults to the textbook's folder"
                        text: root.outputUrl === "" ? "" : root.localPath(root.outputUrl)
                    }
                    QQC2.Button {
                        icon.name: "folder-open"
                        text: "Choose…"
                        onClicked: root.openBrowser(true, function (url) {
                            root.outputUrl = url
                            root.outputChosenByUser = true
                            converter.remember(url)
                        })
                    }
                }

                QQC2.Label {
                    Kirigami.FormData.label: "Files go in:"
                    visible: root.inputUrl !== "" && root.outputUrl !== ""
                    text: root.localPath(root.outputUrl) + "/" + root.stemOf(root.inputUrl) + "/"
                    elide: Text.ElideMiddle
                    Layout.fillWidth: true
                }

                // set apart below the form, right-aligned under the Choose… buttons
                Item {
                    Layout.fillWidth: true
                    implicitHeight: convertButton.implicitHeight + Kirigami.Units.gridUnit * 1.5
                    QQC2.Button {
                        id: convertButton
                        anchors.right: parent.right
                        anchors.bottom: parent.bottom
                        icon.name: "run-build"
                        text: "Convert"
                        enabled: !converter.busy && root.inputUrl !== "" && root.outputUrl !== ""
                        onClicked: {
                            if (converter.exists(root.inputUrl, root.outputUrl)) {
                                existsDialog.open()
                            } else {
                                converter.convert(root.inputUrl, root.outputUrl, false)
                            }
                        }
                    }
                }
            }

            QQC2.ProgressBar {
                Layout.fillWidth: true
                indeterminate: true
                visible: converter.busy
            }

            Kirigami.InlineMessage {
                Layout.fillWidth: true
                visible: converter.status !== ""
                text: converter.status
                type: converter.busy ? Kirigami.MessageType.Information
                    : converter.status.startsWith("Failed") ? Kirigami.MessageType.Error
                    : Kirigami.MessageType.Positive
            }

            Item { Layout.fillHeight: true }
        }
    }
}
