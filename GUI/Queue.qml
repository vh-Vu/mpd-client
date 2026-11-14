import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
Item {
    id: root
    width: 600
    height: 1400
    property var queue: null
    property var selectedSong: new Set()
    property int lastSelected: -1
    property string unSelectedColor: 'transparent'
    property string selectedColor: '#88f0f0f0'
    focus: true
    // const myArray = Array.from(mySet);
    signal songMoveRequest(int targetIndex, var selectedItems)
    signal playSongRequest(int id)
    function selected(index,keyPress){
        var isCtrlPressed = (keyPress & Qt.ControlModifier);
        var isShiftPressed = (keyPress & Qt.ShiftModifier);
        if (lastSelected === -1){
            if (isShiftPressed){
                for(let i = 0; i <= index; i++){
                    // selectedSong[i] = true;
                    // console.log(queue[i].title)
                    selectedSong.add(queue[i].id);
                    // console.log(queue[i].id,queue[i].pos)
                    try{
                        queueView.itemAtIndex(i).color = selectedColor
                    }catch (_) {}
                }
                // itemSelection = index + 1;
            }else{      
                // selectedSong[index] = true;
                selectedSong.add(queue[index].id);
                queueView.itemAtIndex(index).color = selectedColor
                // itemSelection+=1;
            }
            lastSelected = index;
        }else{
            if (isShiftPressed){
                let max = Math.max(lastSelected,index);
                let min = Math.min(lastSelected,index);
                for(let i = min; i <= max; i++){
                    // selectedSong[i] = true;
                    selectedSong.add(queue[i].id);
                     try{
                        queueView.itemAtIndex(i).color = selectedColor
                    }catch (_) {}
                }
                // itemSelection+=max-min+1;
            }else if (isCtrlPressed){
                if (queueView.itemAtIndex(index).color == selectedColor){
                    // selectedSong[index] = false;
                    selectedSong.delete(queue[index].id);
                    queueView.itemAtIndex(index).color = unSelectedColor;
                    lastSelected = index;
                }else{
                    // selectedSong[index] = true;
                    selectedSong.add(queue[index].id);
                    queueView.itemAtIndex(index).color = selectedColor;
                    lastSelected = index;
                    // itemSelection+=1;
                }
            }else{
                root.restore()
                // selectedSong[index] = true;
                selectedSong.add(queue[index].id);
                queueView.itemAtIndex(index).color = selectedColor
                lastSelected = index;
                // itemSelection+=1;
            }
        }
    }

    function restore(){
        root.lastSelected = -1;
        // root.itemSelection = 0;
        // selectedSong = Array(queueView.count).fill(false);
        for (let i = 0; i < queueView.count; i++){
            if (selectedSong.has(queue[i].id)){
                try{
                    queueView.itemAtIndex(i).color = unSelectedColor;
                }catch (_) {}
                selectedSong[i] = false;
            }

        }
        selectedSong.clear();
    }
    

    Item {
        anchors.fill: parent
        anchors.margins: 16
        
        ListView {
            id: queueView
            anchors.fill: parent
            anchors.margins: 16
            // spacing: 2
            clip: true
            width: 100
            model: queue
                property int hoverIndex: -1

            // --- CẤU HÌNH GOM NHÓM THEO ALBUM ---
            section.property: "album"
            section.criteria: ViewSection.FullString

            // ---------------------------------------------------------------------
            // 3. SECTION DELEGATE (Visual Header cho mỗi cụm Album liền kề)
            // ---------------------------------------------------------------------
            section.delegate: Component {
                Rectangle {
                    width: queueView.width
                    height: 100
                    // color: "#222226"
                    color: "transparent"
                    // radius: 6

                    // Lấy thông tin từ phần tử đầu tiên của section hiện tại
                    // property var firstItem: queueModel.get(sectionIndex)
                    function getFilepathForCurrentSection() {
                        var modelData = queueView.model;
                        if (!modelData) return "";
                        var totalCount = modelData.length !== undefined ? modelData.length : 0;
                        
                        for (var i = 0; i < totalCount; i++) {
                            var item = modelData[i];
                            // So sánh thuộc tính album của bài hát với 'section' hiện tại
                            if (item && item.album === section) {
                                var lastIndex = item.file.lastIndexOf('/');
                                var dirPath = item.file.substring(0, lastIndex);
                                return dirPath; // Bốc ngay filepath của bài đầu tiên trong nhóm này!
                            }
                        }
                        return "";
                    }

                    function getFirst() {
                        var modelData = queueView.model;
                        if (!modelData) return null;
                        var totalCount = modelData.length !== undefined ? modelData.length : 0;
                        
                        for (var i = 0; i < totalCount; i++) {
                            var item = modelData[i];
                            if (item && item.album === section) {
                                return item;
                            }
                        }
                        return null;
                    }

                    // Lưu filepath tìm được thành property để dùng thoải mái bên dưới
                    readonly property string targetFilepath: getFilepathForCurrentSection();
                    property var firstItem: getFirst()
                    // var firstItem: 
                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 8
                        spacing: 12

                        // Album Cover Art
                        Rectangle {
                            Layout.preferredWidth: 70
                            Layout.preferredHeight: 70
                            color: "#111"
                            radius: 4
                            clip: true

                            Image {
                                anchors.fill: parent
                                // source: firstItem ? firstItem.coverPath : ""
                                // source: "file://mnt/980/Music/alan/Swear/cover.jpg"
                                source: `file://mnt/980/Music/${targetFilepath}/cover.jpg`
                                
                                fillMode: Image.PreserveAspectCrop
                                asynchronous: true
                                // Placeholder nếu file ảnh không tồn tại
                                // Rectangle {
                                //     anchors.fill: parent
                                //     color: "#333"
                                //     visible: parent.status !== Image.Ready
                                //     Text {
                                //         anchors.centerIn: parent
                                //         text: "COVER"
                                //         color: "#666"
                                //         font.pixelSize: 10
                                //     }
                                // }
                            }
                        }

                        // Album Meta Info
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 2

                            Text {
                                text: section // Tên Album do QML tự inject
                                color: "#ffffff"
                                font.pixelSize: 15
                                font.bold: true
                                elide: Text.ElideRight
                            }

                            RowLayout {
                                spacing: 8
                                Text {
                                    // text: firstItem ? firstItem.artist : ""
                                    text: firstItem.artist.join(", ")
                                    color: "#aaa"
                                    font.pixelSize: 13
                                }
                                Text {
                                    text: "•"
                                    color: "#555"
                                    font.pixelSize: 12
                                    visible: firstItem && firstItem.year > 0
                                }
                                Text {
                                    text: firstItem ? (firstItem.year > 0 ? firstItem.year : "") : ""
                                    color: "#888"
                                    font.pixelSize: 13
                                    visible: firstItem && firstItem.year > 0
                                }
                            }
                        }
                    }
                }
            }

            // ---------------------------------------------------------------------
            // 4. SONG ITEM DELEGATE (Hàng bài hát thuộc Album)
            // ---------------------------------------------------------------------
            delegate: Rectangle {
                property bool isSelected: !!root.selectedSong.has(model.id)
                property bool isHovered: (queueView.hoverIndex === index)
                property bool isDragging: false
                property real startX: 0
                property real startY: 0
                width: queueView.width
                height: 36
                // color: mouseArea.containsMouse ? "#2a2a30" : "#1a1a1e"
                // radius: 4

                color: isSelected? "#aaf0f0f0" : '#transparent'
                // MouseArea {
                //     id: mouseArea
                //     anchors.fill: parent
                //     hoverEnabled: true
                // }

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 12
                    anchors.rightMargin: 12
                    spacing: 12
                    // Text {
                    //     text: 
                    //     color: "#e0e0e0"
                    //     font.pixelSize: 13
                    //     Layout.fillWidth: true
                    //     elide: Text.ElideRight
                    // }
                    Text {
                        text: index + " " +  model.title
                        color: "#e0e0e0"
                        font.pixelSize: 13
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    Text {
                        // text: model.artist.length > 1 ? model.artist.join(', ') : "" +
                        text: model.id
                        color: "#e0e0e0"
                        font.pixelSize: 13
                        Layout.fillWidth: true
                        elide: Text.ElideRight
                    }

                    Text {
                        text: formatDuration(model.time)
                        color: "#444"
                        font.pixelSize: 11
                        font.family: "Monospace"
                    }
                }

               
                    MouseArea {
                        anchors.fill: parent
                        drag.threshold: 10
    
                        drag.target: dragBadge
                        acceptedButtons: Qt.LeftButton
                        // hoverEnabled: true
                        onClicked: function(mouse){                            
                            console.log(modelData.id);
                            root.selected(index,mouse.modifiers)
                        }
                        onDoubleClicked:{
                            // console.log("play");
                            playSongRequest(modelData.id);
                        }
                        onPositionChanged: function(mouse) {
                            // if (pressed && drag.active) {
                            //     dragBadge.visible = true;
                            //     badgeText.text = root.selectedSong.size > 0 ? root.selectedSong.size : "1";

                            //     // --- TỰ ĐỘNG TÍNH TOÁN INDEX ĐANG RÊ QUA ĐỂ ĐỔI MÀU ---
                            //     var ptInListView = mapToItem(queueView, mouse.x, mouse.y);
                            //     var targetIdx = queueView.indexAt(ptInListView.x, ptInListView.y + queueView.contentY);

                            //     // Kẹp biên phòng hờ kéo xuống vùng trống dưới đáy danh sách
                            //     if (targetIdx === -1 && queueView.count > 0) {
                            //         if (ptInListView.y > queueView.contentHeight) {
                            //             targetIdx = queueView.count - 1;
                            //         } else if (ptInListView.y < 0) {
                            //             targetIdx = 0;
                            //         }
                            //     }
                            //     queueView.hoverIndex = targetIdx;
                            // }
                            if (pressed && root.selectedSong.size > 0) {
                                
                                dragBadge.visible = true;
                                badgeText.text = root.selectedSong.size > 0 ? root.selectedSong.size : "1";
                                // console.log(root.selectedSong.size);
                                // 1. Gán tọa độ badge theo chuột
                                var globalPos = mapToItem(root, mouse.x, mouse.y);
                                dragBadge.x = globalPos.x + 12;
                                dragBadge.y = globalPos.y + 12;

                                // queueView.hoverIndex = calculateTargetIndex(mouse);
                                // // 2. Quy đổi tọa độ sang không gian ListView   
                                var ptInListView = mapToItem(queueView, mouse.x, mouse.y);
                                var absoluteY = ptInListView.y + queueView.contentY;

                                // --- 3. KIỂM TRA XEM CÓ KÉO NGANG VĂNG RA NGOÀI BIÊN KHÔNG ---
                                var isOutOfBoundsX = (ptInListView.x < 0 || ptInListView.x > queueView.width);
                                
                                if (isOutOfBoundsX) {
                                    // Nếu kéo ngang ra ngoài biên trái/phải của Queue -> Vô hiệu hóa hover (không chọn dòng nào cả)
                                    queueView.hoverIndex = -1;
                                    return; // Thoát luôn, không chạy thuật toán quét nữa
                                }

                                // 4. THUẬT TOÁN TÌM BÀI GẦN NHẤT (NEAREST NEIGHBOR) CHO TRỤC DỌC
                                var targetIdx = queueView.indexAt(ptInListView.x, absoluteY);
                                // console.log(queueView.indexAt(ptInListView.x, absoluteY));
                                // console.log(ptInListView.x, absoluteY);
                                if (targetIdx === -1 && queueView.count > 0) {
                                    var foundIdx = -1;
                                    var step = 15; 
                                    var maxScan = 200; 

                                    for (var offset = 0; offset <= maxScan; offset += step) {
                                        foundIdx = queueView.indexAt(ptInListView.x, absoluteY + offset);
                                        if (foundIdx !== -1) break;

                                        foundIdx = queueView.indexAt(ptInListView.x, absoluteY - offset);
                                        if (foundIdx !== -1) break;
                                    }

                                    if (foundIdx !== -1) {
                                        targetIdx = foundIdx; 
                                    } else {
                                        // Chỉ kẹp biên khi kéo dọc lên đỉnh hoặc xuống đáy thực sự
                                        if (ptInListView.y > queueView.contentHeight) {
                                            targetIdx = queueView.count - 1; 
                                        } else if (ptInListView.y < 0) {
                                            targetIdx = 0; 
                                        } else {
                                            targetIdx = -1; // Vùng trống lửng lơ thì không ép về 0 nữa
                                        }
                                    }
                                }
                                if (targetIdx !== -1) {
    // Lấy thông tin item tại index tìm được để check tọa độ thực tế của nó trong ListView
                                    var itemObj = queueView.itemAtIndex(targetIdx);
                                    if (itemObj) {
                                        // Map tọa độ y của chuột vào item đó xem nó đang nằm ở phần nào
                                        var ptInItem = mapToItem(itemObj, mouse.x, mouse.y);
                                        var isBottomHalf = ptInItem.y > (itemObj.height / 2);

                                        // Nếu ở nửa dưới, ta có thể đánh dấu hoặc cộng dồn index đích lên 1 đơn vị 
                                        // (để biết là thả vào "sau" dòng này)
                                        var finalDropIndex = isBottomHalf ? targetIdx + 1 : targetIdx;
                                        
                                        // Gán vào hoverIndex (hoặc một biến finalIndex tùy cậu đặt)
                                        targetIdx = finalDropIndex;
                                        // Nếu chuột nằm ở nửa dưới của dòng (y > itemObj.height / 2), 
                                        // ta có thể cộng thêm 1 nếu muốn chỉ định vị trí nằm phía dưới dòng đó (tùy logic model của cậu)
                                        // Hiện tại ta cứ giữ nguyên targetIdx, nhưng có thể đánh dấu biến phụ nếu cần xử lý chèn sau.
                                    }
                                }
                                queueView.hoverIndex = targetIdx;
                            }

                        }

                        onReleased: function(mouse) {
                            dragBadge.visible = false;

                            // --- BÁO VỊ TRÍ KHI THẢ TAY ---
                            if (queueView.hoverIndex !== -1) {
                                // console.log("-> Kéo thành công từ dòng:", index, "đến vị trí index:", queueView.hoverIndex);
                                // let arr = [];
                                // for(let i = 0; i < queue.length;i++){
                                //     if (selectedSong.has(queue[i].id))
                                //     arr.push(queue[i].id)
                                // }
                                let arr = queue.filter(song => selectedSong.has(song.id)).map(song => song.id).reverse();
                                songMoveRequest(queueView.hoverIndex,arr);
                                // TODO: Gọi hàm đổi chỗ model/backend ở đây (ví dụ: root.moveItem(index, queueView.hoverIndex))
                            } else {
                                console.log("-> Hủy thao tác kéo!");
                            }

                            queueView.hoverIndex = -1;
                        }
                }
            }
        }
    }
    Rectangle {
        id: dragBadge
        width: 28; height: 28
        radius: 14
        color: "#fab387"
        visible: false
        z: 9999 // Đảm bảo luôn nổi lên trên cùng

        Text {
            id: badgeText
            anchors.centerIn: parent
            color: "#11111b"
            font.bold: true
            font.pixelSize: 12
        }
    }
    Keys.onEscapePressed: {
        root.restore()
        console.log("esc press")
    }
    Keys.onDeletePressed: (event) => {
        console.log("Delete key was pressed")
        console.log(event)
        // event.accepted = true
    }
    // Keys.onDe
    function formatDuration(seconds) {
        if (!seconds || seconds <= 0) return "--:--";
        var mins = Math.floor(seconds / 60);
        var secs = Math.floor(seconds % 60);
        return mins + ":" + (secs < 10 ? "0" : "") + secs;
    }
}



        // --- PHẦN TRÊN: Cover và Control Buttons ---
        // RowLayout {
        //     Layout.alignment: Qt.AlignHCenter
        //     spacing: 20

        //     Cover {
        //         songData: backend.currentSong
        //         Layout.preferredWidth: 150
        //         Layout.preferredHeight: 150
        //     }
        // }

        // RowLayout {
        //     Layout.alignment: Qt.AlignHCenter
        //     spacing: 20

        //     Button {
        //         text: "Prev"
        //         onClicked: backend.toggle_prev()
        //     } 
        //     Button {
        //         text: backend.is_play ? "Pause" : "Play" 
        //         onClicked: backend.toggle_play()
        //     } 
        //     Button {
        //         text: "Next"
        //         onClicked: backend.toggle_next()
        //     }
        // }

        // MouseArea {
        //     anchors.fill: parent
        //     acceptedButtons: Qt.LeftButton
        //     onClicked: {
        //         root.restore()
        //     }
        // }