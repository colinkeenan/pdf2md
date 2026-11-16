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
    // the two ways to write formulas
    readonly property var mathForms: [
        { value: "github", text: "For GitHub or mdBun   ($`…`$, ```math)" },
        { value: "standard", text: "For other Markdown readers   ($…$, $$…$$)" }
    ]
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
    height: Kirigami.Units.gridUnit * 32

    // What can be converted in the folder shown (see the backend's `items`).
    readonly property var items: converter.items === "" ? [] : converter.items.split("\n").map(function (line) {
        var f = line.split("\t")
        return { kind: f[0], name: f[1], path: f[2], out: f[3], book: f[4], when: f[5], bookName: f[6] }
    })
    // The items checked (by path), and for each, if asked for, a folder of
    // its own for the book (a file:// URL) and a way of its own to write math.
    property var checked: ({})
    property var outputs: ({})
    property var maths: ({})
    property bool otherFolders: false
    property bool otherMath: false
    readonly property var chosen: items.filter(function (i) { return root.checked[i.path] === true })
    // how the items of the last run ended, by path
    readonly property var results: {
        var r = {}
        if (converter.results !== "") {
            converter.results.split("\n").forEach(function (line) {
                var f = line.split("\t")
                r[f[0]] = { ok: f[1] === "ok", words: f[2] }
            })
        }
        return r
    }
    // the jobs waiting for the answer to "Replace or Keep Both?"
    property string pendingJobs: ""
    property string pendingThere: ""

    function localPath(url) {
        return decodeURIComponent(url.replace("file://", ""))
    }
    function check(path, on) {
        var c = Object.assign({}, checked)
        if (on) {
            c[path] = true
        } else {
            delete c[path]
        }
        checked = c
    }
    function checkAll(on) {
        var c = {}
        if (on) {
            items.forEach(function (i) { c[i.path] = true })
        }
        checked = c
    }
    function setFor(map, path, value) {
        var m = Object.assign({}, map)
        m[path] = value
        return m
    }
    // where an item's book goes, and how its math is written
    function outputOf(item) {
        return otherFolders && outputs[item.path] ? localPath(outputs[item.path]) : item.out
    }
    function mathOf(item) {
        return otherMath && maths[item.path] ? maths[item.path] : converter.math
    }
    // the checked items, in the list's order, as the backend takes them
    function jobs() {
        return chosen.map(function (i) { return [i.kind, i.path, outputOf(i), mathOf(i)].join("\t") }).join("\n")
    }
    function start() {
        var j = jobs()
        var there = converter.existing(j)
        if (there !== "") {
            pendingJobs = j
            pendingThere = there
            existsDialog.open()
        } else {
            converter.convertall(j, false)
        }
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

    function openBrowser(heading, start, onPicked) {
        // (made here, inside the window, and then handed to the page stack:
        // a page the stack makes from the component starts outside the
        // scene, which Qt warns about; a page made here is ours to remove)
        var page = browserComponent.createObject(pageStack, {
            chooseFolder: true,
            heading: heading,
            startFolder: start
        })
        pageStack.push(page)
        page.picked.connect(function (chosen) {
            onPicked(chosen.toString())
            pageStack.pop()
        })
    }

    // asked when a book folder is already there
    Kirigami.PromptDialog {
        id: existsDialog
        readonly property var there: root.pendingThere === "" ? [] : root.pendingThere.split("\n")
        title: there.length === 1 ? "Book folder already exists" : there.length + " book folders already exist"
        subtitle: there.map(function (d) { return "“" + d + "/”" }).join("\n") + (there.length === 1 ? " is" : "\nare") + " already there.\n\n"
            + "Replace moves the old files to the Trash and writes the new conversion into the same folder (a course's book is rebuilt in place). "
            + "Keep Both writes the new conversion to a folder with “(1)” after its name (or the next free number)."
        standardButtons: Kirigami.Dialog.Cancel
        customFooterActions: [
            Kirigami.Action {
                text: existsDialog.there.length === 1 ? "Replace" : "Replace All"
                icon.name: "user-trash"
                onTriggered: {
                    existsDialog.close()
                    converter.convertall(root.pendingJobs, true)
                }
            },
            Kirigami.Action {
                text: "Keep Both"
                icon.name: "folder-new"
                onTriggered: {
                    existsDialog.close()
                    converter.convertall(root.pendingJobs, false)
                }
            }
        ]
    }

    pageStack.initialPage: Kirigami.Page {
        id: page
        title: "Convert textbooks"

        ColumnLayout {
            id: column
            anchors.fill: parent
            spacing: Kirigami.Units.largeSpacing

            Kirigami.FormLayout {
                id: form
                Layout.fillWidth: true
                // always labels beside the fields
                wideMode: true

                // the user's own Markdown reader, to show converted books:
                // one for all of them
                QQC2.TextField {
                    Kirigami.FormData.label: "Reader:"
                    Layout.fillWidth: true
                    implicitWidth: Kirigami.Units.gridUnit * 22
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
                    model: root.mathForms
                    currentIndex: converter.math === "standard" ? 1 : 0
                    onActivated: converter.usemath(currentValue)
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
                    QQC2.ToolTip.text: "How formulas are written in the books.\nmdBun reads both. GitHub needs the first;\nmost other programs need the second."
                }

                // the folder whose PDFs and courses are listed
                RowLayout {
                    Kirigami.FormData.label: "Folder:"
                    Layout.fillWidth: true
                    QQC2.Label {
                        Layout.fillWidth: true
                        text: root.localPath(converter.folder)
                        elide: Text.ElideMiddle
                    }
                    QQC2.Button {
                        icon.name: "folder-open"
                        text: "Choose…"
                        enabled: !converter.busy
                        onClicked: root.openBrowser("Choose the folder of the books", converter.folder, function (url) {
                            converter.remember(url)
                        })
                    }
                }
            }

            // everything in the folder that can be converted: check what to convert
            RowLayout {
                Layout.fillWidth: true
                visible: root.items.length > 0
                QQC2.CheckBox {
                    id: allBox
                    text: "Can be converted"
                    font.bold: true
                    enabled: !converter.busy
                    tristate: true
                    // (a click checks all, or none when all are checked)
                    nextCheckState: function () {
                        return root.chosen.length === root.items.length ? Qt.Unchecked : Qt.Checked
                    }
                    onClicked: root.checkAll(checkState === Qt.Checked)
                    Binding {
                        target: allBox
                        property: "checkState"
                        value: root.chosen.length === 0 ? Qt.Unchecked : root.chosen.length === root.items.length ? Qt.Checked : Qt.PartiallyChecked
                    }
                }
                Item {
                    Layout.fillWidth: true
                }
                QQC2.ToolButton {
                    icon.name: "view-refresh"
                    text: "Look again"
                    display: QQC2.AbstractButton.IconOnly
                    enabled: !converter.busy
                    onClicked: converter.refresh()
                    QQC2.ToolTip.visible: hovered
                    QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
                    QQC2.ToolTip.text: "Look again for what the folder holds"
                }
            }
            QQC2.ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: Kirigami.Units.gridUnit * 6
                Layout.preferredHeight: Math.min(itemList.contentHeight + 2, Kirigami.Units.gridUnit * 22)
                visible: root.items.length > 0
                ListView {
                    id: itemList
                    clip: true
                    model: root.items
                    delegate: ColumnLayout {
                        id: row
                        required property var modelData
                        readonly property bool on: root.checked[modelData.path] === true
                        readonly property var result: root.results[modelData.path]
                        readonly property bool running: converter.busy && converter.current === modelData.path
                        width: ListView.view.width
                        spacing: 0

                        RowLayout {
                            Layout.fillWidth: true
                            Layout.margins: Kirigami.Units.smallSpacing
                            spacing: Kirigami.Units.largeSpacing
                            QQC2.CheckBox {
                                id: box
                                enabled: !converter.busy
                                onClicked: root.check(row.modelData.path, checked)
                                Binding {
                                    target: box
                                    property: "checked"
                                    value: row.on
                                }
                            }
                            Kirigami.Icon {
                                source: row.modelData.kind === "course" ? "folder-documents" : "application-pdf"
                                implicitWidth: Kirigami.Units.iconSizes.smallMedium
                                implicitHeight: Kirigami.Units.iconSizes.smallMedium
                            }
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 0
                                QQC2.Label {
                                    Layout.fillWidth: true
                                    text: row.modelData.name
                                    elide: Text.ElideMiddle
                                    MouseArea {
                                        anchors.fill: parent
                                        enabled: !converter.busy
                                        onClicked: root.check(row.modelData.path, !row.on)
                                    }
                                }
                                QQC2.Label {
                                    Layout.fillWidth: true
                                    // how it ended in this run; else what is known of it
                                    text: row.running ? "Converting…"
                                        : row.result !== undefined ? (row.result.ok ? "Converted just now. " : "Failed: ") + row.result.words
                                        : (row.modelData.kind === "course" ? "A course: saved pages and PDFs. " : "")
                                          + (row.modelData.book !== "" ? "Converted " + row.modelData.when : "Not converted yet")
                                    // (the theme's own colours for good, undecided and bad
                                    // news, on the theme's background: none is set here)
                                    color: row.running ? Kirigami.Theme.textColor
                                        : row.result !== undefined ? (row.result.ok ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.negativeTextColor)
                                        : row.modelData.book !== "" ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.neutralTextColor
                                    elide: Text.ElideMiddle
                                    font: Kirigami.Theme.smallFont
                                }
                            }
                            QQC2.BusyIndicator {
                                visible: row.running
                                running: visible
                                implicitWidth: Kirigami.Units.iconSizes.smallMedium
                                implicitHeight: Kirigami.Units.iconSizes.smallMedium
                            }
                            QQC2.Label {
                                visible: !row.running && row.result !== undefined
                                text: row.result !== undefined && row.result.ok ? "✓" : "✗"
                                color: row.result !== undefined && row.result.ok ? Kirigami.Theme.positiveTextColor : Kirigami.Theme.negativeTextColor
                            }
                            // a converted book, a click away
                            QQC2.ToolButton {
                                visible: row.modelData.book !== ""
                                icon.name: "document-open"
                                text: "Open"
                                onClicked: converter.show(row.modelData.book)
                            }
                        }

                        // a folder of its own for this book
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.leftMargin: Kirigami.Units.gridUnit * 3
                            Layout.rightMargin: Kirigami.Units.smallSpacing
                            Layout.bottomMargin: Kirigami.Units.smallSpacing
                            visible: row.on && root.otherFolders
                            spacing: Kirigami.Units.largeSpacing
                            QQC2.Label {
                                text: "Goes to:"
                            }
                            QQC2.Label {
                                Layout.fillWidth: true
                                text: root.outputOf(row.modelData) + "/" + row.modelData.bookName + "/"
                                elide: Text.ElideMiddle
                            }
                            QQC2.Button {
                                icon.name: "folder-open"
                                text: "Choose…"
                                enabled: !converter.busy
                                onClicked: {
                                    var path = row.modelData.path
                                    root.openBrowser("Choose the folder for “" + row.modelData.bookName + "”", "file://" + root.outputOf(row.modelData), function (url) {
                                        root.outputs = root.setFor(root.outputs, path, url)
                                    })
                                }
                            }
                        }
                        // its own way to write math
                        RowLayout {
                            Layout.fillWidth: true
                            Layout.leftMargin: Kirigami.Units.gridUnit * 3
                            Layout.rightMargin: Kirigami.Units.smallSpacing
                            Layout.bottomMargin: Kirigami.Units.smallSpacing
                            visible: row.on && root.otherMath
                            spacing: Kirigami.Units.largeSpacing
                            QQC2.Label {
                                text: "Math:"
                            }
                            QQC2.ComboBox {
                                Layout.fillWidth: true
                                enabled: !converter.busy
                                textRole: "text"
                                valueRole: "value"
                                model: root.mathForms
                                currentIndex: root.mathOf(row.modelData) === "standard" ? 1 : 0
                                onActivated: root.maths = root.setFor(root.maths, row.modelData.path, currentValue)
                            }
                        }
                        Kirigami.Separator {
                            Layout.fillWidth: true
                            opacity: 0.5
                        }
                    }
                }
            }
            Kirigami.PlaceholderMessage {
                Layout.fillWidth: true
                Layout.fillHeight: true
                visible: root.items.length === 0
                text: "Nothing to convert in this folder"
                explanation: "A PDF can be converted, and a folder of saved web pages (HTML) with PDFs and no Markdown yet."
            }

            // asked only where a book wants something else than the others
            QQC2.CheckBox {
                text: "Some of the books go to a different folder"
                visible: root.items.length > 0
                enabled: !converter.busy
                checked: root.otherFolders
                onToggled: root.otherFolders = checked
                QQC2.ToolTip.visible: hovered
                QQC2.ToolTip.delay: Kirigami.Units.toolTipDelay
                QQC2.ToolTip.text: "Unchecked: each book goes into a folder of its name beside its PDF\n(or to where it was converted to before)."
            }
            RowLayout {
                Layout.fillWidth: true
                visible: root.items.length > 0
                QQC2.CheckBox {
                    text: "Some of the books get different math"
                    enabled: !converter.busy
                    checked: root.otherMath
                    onToggled: root.otherMath = checked
                }
                Item {
                    Layout.fillWidth: true
                }
                QQC2.Button {
                    id: convertButton
                    icon.name: "run-build"
                    text: root.chosen.length > 1 ? "Convert " + root.chosen.length + " Books" : "Convert"
                    enabled: !converter.busy && root.chosen.length > 0
                    onClicked: root.start()
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
                    : converter.status.startsWith("Check") ? Kirigami.MessageType.Information
                    : Kirigami.MessageType.Positive
            }

        }
    }
}
