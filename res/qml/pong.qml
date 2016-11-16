import QtQuick 2.6;
import QtQuick.Controls 1.4;
import Pong 1.0;

ApplicationWindow {
  id: window
  visible: true
  width: 600
  height: 400

  Rectangle {
    id: root
    focus: true
    clip: true
    color: "#ccc"
    width: (6/4 * window.height >= window.width ? window.width : 6/4 * window.height) - 16
    height: (4/6 * width) - 16
    anchors.centerIn: parent
    
    Component {
      id: paddleComponent
      Rectangle {
        property real xx: 0
        property real yy: 0
        x: (xx + 1) * root.width/2 - width/2
        y: (1 - yy) * root.height/2 - height/2
        Behavior on x { NumberAnimation { duration: timer.interval } }
        Behavior on y { NumberAnimation { duration: timer.interval } }
        width: root.width * game.paddleRelativeWidth
        height: root.height * game.paddleRelativeHeight
        color: "#222"
      }
    }
    Component {
      id: ballComponent
      Rectangle {
        property real xx: 0
        property real yy: 0
        x: (xx + 1) * root.width/2 - width/2
        y: (1 - yy) * root.height/2 - height/2
        Behavior on x { NumberAnimation { duration: timer.interval } }
        Behavior on y { NumberAnimation { duration: timer.interval } }
        width: root.width * game.ballRelativeWidth
        height: root.height * game.ballRelativeHeight
        color: "#555"
      }
    }

    Keys.onPressed: {
      if(event.key == Qt.Key_Space) game.start();
      else if(event.key == Qt.Key_Up) game.moveRightUp();
      else if(event.key == Qt.Key_Down) game.moveRightDown();
      else if(event.key == Qt.Key_A) game.moveLeftUp();
      else if(event.key == Qt.Key_Z) game.moveLeftDown();
      else if(event.key == Qt.Key_Escape) window.close();
      else if(event.key == Qt.Key_R) game.reset(0);
    }
    Keys.onReleased: {
      if(event.key == Qt.Key_Up) game.moveRightStop();
      else if(event.key == Qt.Key_Down) game.moveRightStop();
      else if(event.key == Qt.Key_A) game.moveLeftStop();
      else if(event.key == Qt.Key_Z) game.moveLeftStop();
    }

    Timer {
      id: timer
      interval: 30
      onTriggered: game.update()
      repeat: true
      running: true
    }

    Text {
      anchors.left: parent.left
      anchors.top: parent.top
      text: game.scoreLeft
      font.pixelSize: root.height/20
      anchors.margins: 8
    }
    Text {
      anchors.right: parent.right
      anchors.top: parent.top
      text: game.scoreRight
      font.pixelSize: root.height/20
      anchors.margins: 8
    }

    PongGame {
      id: game
      property int scoreLeft: 0
      property int scoreRight: 0
      property int t: 0
      property var entities

      function update() {
        t += timer.interval * 1000;
        time(t);
      }
      Component.onCompleted: {
        entities = {};
        game.initialize();
      }
      onCreateLeft: function(id, x, y) {
        entities[id] = paddleComponent.createObject(root, { xx: x, yy: y });
      }
      onCreateRight: function(id, x, y) {
        entities[id] = paddleComponent.createObject(root, { xx: x, yy: y });
      }
      onCreateBall: function(id, x, y) {
        entities[id] = ballComponent.createObject(root, { xx: x, yy: y });
      }
      onDestroyEntity: function(id) {
        entities[id].destroy();
        entities[id] = undefined;
      }
      onMoveEntity: function(id, x, y) {
        var e = entities[id];
        if(e) {
          e.xx = x;
          e.yy = y;
        }
      }
      onGoalLeft: function(score) {
        scoreLeft = score;
      }
      onGoalRight: function(score) {
        scoreRight = score;
      }
      onReseted: function() {
        scoreLeft = 0;
        scoreRight = 0;
      }
      onRoundStart: function() {
      }

    }
  }
}
