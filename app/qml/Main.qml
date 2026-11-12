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
    // (the same for the steps of a long conversion, under the form)
    readonly property real formWidth: Math.ceil(Math.max(form.implicitWidth,
            converter.detailed ? stepList.implicitWidth + 2 * Kirigami.Units.largeSpacing : 0) + page.leftPadding + page.rightPadding)
    // (The window grows a moment after the form does: growing while the
    // file browser's page is still sliding away leaves that page drawn
    // over the form, with the buttons under it.)
    property real settledWidth: 0
    onFormWidthChanged: widen.restart()
    Timer {
        id: widen
        interval: Kirigami.Units.veryLongDuration * 2
        onTriggered: root.settledWidth = root.formWidth
    }
    minimumWidth: Math.max(Kirigami.Units.gridUnit * 24, Math.min(settledWidth, Screen.desktopAvailableWidth))
    // and high enough for the steps of a long conversion (the page's
    // title bar is about three grid units high)
    readonly property real neededHeight: Math.ceil(column.implicitHeight + page.topPadding + page.bottomPadding + Kirigami.Units.gridUnit * 3)
    minimumHeight: Math.max(Kirigami.Units.gridUnit * 18, Math.min(neededHeight, Screen.desktopAvailableHeight))
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

    // a file browser page that has left the stack (a choice made, or the
    // back arrow) is done with
    Connections {
        target: pageStack
        function onPageRemoved(page) {
            if (page.picked !== undefined) {
                page.destroy(2000)
            }
        }
    }

    function openBrowser(chooseFolder, onPicked) {
        // (made here, inside the window, and then handed to the page stack:
        // a page the stack makes from the component starts outside the
        // scene, which Qt warns about; a page made here is ours to remove)
        var page = browserComponent.createObject(pageStack, {
            chooseFolder: chooseFolder,
            startFolder: converter.folder
        })
        pageStack.push(page)
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
            id: column
            anchors.fill: parent
            spacing: Kirigami.Units.largeSpacing

            Kirigami.FormLayout {
                id: form
                Layout.fillWidth: true
                // always labels beside the fields: the window is made wide
                // enough for that (left to itself, the form switches to
                // labels above the fields just when a long path arrives,
                // and then asks for less room than it needs)
                wideMode: true

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

                // the user's own Markdown reader, to show converted books
                QQC2.TextField {
                    Kirigami.FormData.label: "Reader:"
                    Layout.fillWidth: true
                    placeholderText: "Command that shows a book, e.g. md (empty: the system's choice)"
                    text: converter.reader
                    onEditingFinished: converter.usereader(text)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
                    QQC2.ToolTip.text: "The command gets the book's folder as its last argument.\nWrite {folder} or {file} (the whole book in one file, book.md) to put it elsewhere."
                }

                // how formulas are written: follows the reader when that is a
                // program known to show math only between "$"
                QQC2.ComboBox {
                    Kirigami.FormData.label: "Math:"
                    Layout.fillWidth: true
                    textRole: "text"
                    valueRole: "value"
                    model: [
                        { value: "github", text: "$`…`$ and ```math — for GitHub and mdBun" },
                        { value: "standard", text: "$…$ and $$…$$ — for most other readers" }
                    ]
                    currentIndex: converter.math === "standard" ? 1 : 0
                    onActivated: converter.usemath(currentValue)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
                    QQC2.ToolTip.text: "How formulas are written in the book.\nGitHub garbles formulas between $ signs; most other Markdown programs show the first form as program code.\nmdBun reads both. A book is written anew in the form chosen when you convert it again."
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

            // how far the conversion is, by time, and how long it will take still
            RowLayout {
                Layout.fillWidth: true
                visible: converter.busy
                spacing: Kirigami.Units.largeSpacing
                QQC2.ProgressBar {
                    Layout.fillWidth: true
                    from: 0
                    to: 1
                    value: converter.progress
                }
                QQC2.Label {
                    text: converter.remaining
                }
            }

            // A long conversion (over 30 s in all) shows its
            // steps like a terminal: what is done, what is running and how
            // long it has left, what is to come. It stays until the next one.
            Rectangle {
                Layout.fillWidth: true
                visible: converter.detailed && converter.steps !== ""
                implicitHeight: stepList.implicitHeight + 2 * Kirigami.Units.largeSpacing
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                Kirigami.Theme.inherit: false
                color: Kirigami.Theme.backgroundColor
                border.color: Kirigami.ColorUtils.linearInterpolation(Kirigami.Theme.backgroundColor, Kirigami.Theme.textColor, 0.2)
                radius: Kirigami.Units.cornerRadius

                GridLayout {
                    id: stepList
                    anchors.fill: parent
                    anchors.margins: Kirigami.Units.largeSpacing
                    columns: 4
                    columnSpacing: Kirigami.Units.largeSpacing
                    rowSpacing: Kirigami.Units.smallSpacing

                    Repeater {
                        // four cells for each step: mark, name, bar, words
                        model: converter.steps === "" ? 0 : converter.steps.split("\n").length * 4
                        Item {
                            id: cell
                            required property int index
                            readonly property var step: converter.steps.split("\n")[Math.floor(index / 4)].split("\t")
                            readonly property int column: index % 4
                            readonly property bool waiting: step[0] === "waiting"
                            Layout.fillWidth: column === 2
                            Layout.minimumWidth: column === 2 ? Kirigami.Units.gridUnit * 6 : 0
                            implicitWidth: column === 2 ? Kirigami.Units.gridUnit * 6 : words.implicitWidth
                            implicitHeight: words.implicitHeight
                            QQC2.Label {
                                id: words
                                visible: cell.column !== 2
                                font.family: "monospace"
                                opacity: cell.waiting ? 0.5 : 1
                                color: cell.column === 0 && cell.step[0] === "done" ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.textColor
                                text: cell.column === 0 ? (cell.step[0] === "done" ? "✓" : cell.step[0] === "running" ? "▸" : "·")
                                    : cell.column === 1 ? cell.step[1]
                                    // (the running step's times are on a line of their own)
                                    : cell.step[3].split("; ").join("\n")
                            }
                            QQC2.ProgressBar {
                                visible: cell.column === 2
                                anchors.left: parent.left
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                opacity: cell.waiting ? 0.5 : 1
                                from: 0
                                to: 1
                                value: cell.step[0] === "done" ? 1 : Number(cell.step[2])
                            }
                        }
                    }
                }
            }

            Kirigami.InlineMessage {
                Layout.fillWidth: true
                visible: converter.status !== ""
                text: converter.status
                type: converter.busy ? Kirigami.MessageType.Information
                    : converter.status.startsWith("Failed") ? Kirigami.MessageType.Error
                    : Kirigami.MessageType.Positive
                // the book just converted, a click away
                actions: [
                    Kirigami.Action {
                        text: "Open Book"
                        icon.name: "document-open"
                        visible: !converter.busy && converter.last !== ""
                        onTriggered: converter.show(converter.last)
                    }
                ]
            }

            // the converted books: a click shows one in the reader
            Kirigami.Heading {
                level: 4
                text: "Converted books"
                visible: bookList.count > 0
            }
            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: Math.min(bookList.count, 3) * Kirigami.Units.gridUnit * 2.5
                visible: bookList.count > 0
                ListView {
                    id: bookList
                    clip: true
                    model: converter.books === "" ? [] : converter.books.split("\n")
                    delegate: QQC2.ItemDelegate {
                        id: book
                        required property string modelData
                        readonly property var parts: modelData.split("\t")
                        width: ListView.view.width
                        icon.name: "document-open"
                        text: parts[0]
                        onClicked: converter.show(parts[1])
                        contentItem: RowLayout {
                            spacing: Kirigami.Units.largeSpacing
                            Kirigami.Icon {
                                source: "x-office-document"
                                implicitWidth: Kirigami.Units.iconSizes.smallMedium
                                implicitHeight: Kirigami.Units.iconSizes.smallMedium
                            }
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 0
                                QQC2.Label {
                                    Layout.fillWidth: true
                                    text: book.parts[0]
                                    elide: Text.ElideRight
                                }
                                QQC2.Label {
                                    Layout.fillWidth: true
                                    text: book.parts[1]
                                    elide: Text.ElideMiddle
                                    opacity: 0.6
                                    font: Kirigami.Theme.smallFont
                                }
                            }
                            QQC2.Label {
                                text: book.parts[2]
                                opacity: 0.6
                            }
                        }
                    }
                }
            }

            Item {
                Layout.fillHeight: true
                visible: bookList.count === 0
            }
        }
    }
}
