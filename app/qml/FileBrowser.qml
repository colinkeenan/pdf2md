import QtQuick
import QtQuick.Controls as QQC2
import Qt.labs.folderlistmodel
import org.kde.kirigami as Kirigami

// A tiny file/folder browser where ONE click does the thing:
//   click a folder  -> go into it
//   click a file    -> choose it (emits `picked`)
// In folder mode (chooseFolder: true) only folders are listed and the
// "Use this folder" action chooses the folder you are currently in.
Kirigami.ScrollablePage {
    id: page

    property bool chooseFolder: false
    property url startFolder
    signal picked(url chosen)

    title: chooseFolder ? "Choose output folder" : "Choose a textbook"

    function displayPath(u) {
        return decodeURIComponent(u.toString().replace("file://", ""))
    }

    actions: [
        Kirigami.Action {
            icon.name: "go-up"
            text: "Up"
            onTriggered: folderModel.folder = folderModel.parentFolder
        },
        Kirigami.Action {
            icon.name: "dialog-ok-apply"
            text: "Use this folder"
            visible: page.chooseFolder
            onTriggered: page.picked(folderModel.folder)
        }
    ]

    FolderListModel {
        id: folderModel
        folder: page.startFolder
        showDirs: true
        showFiles: !page.chooseFolder
        showDirsFirst: true
        showDotAndDotDot: false
        showHidden: false
        nameFilters: ["*.pdf", "*.PDF"]
        sortField: FolderListModel.Name
    }

    ListView {
        id: list
        model: folderModel
        clip: true

        header: QQC2.Label {
            width: list.width
            padding: Kirigami.Units.largeSpacing
            text: page.displayPath(folderModel.folder)
            elide: Text.ElideMiddle
            font.bold: true
        }

        delegate: QQC2.ItemDelegate {
            width: ListView.view.width
            icon.name: fileIsDir ? "folder" : "application-pdf"
            text: fileName
            onClicked: {
                if (fileIsDir) {
                    folderModel.folder = fileUrl
                } else {
                    page.picked(fileUrl)
                }
            }
        }

        Kirigami.PlaceholderMessage {
            anchors.centerIn: parent
            width: parent.width - Kirigami.Units.gridUnit * 4
            visible: list.count === 0
            text: page.chooseFolder ? "No subfolders here" : "No PDF files here"
        }
    }
}
