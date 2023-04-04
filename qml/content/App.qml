// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR GPL-3.0

import QtQuick 6.2
import QtQuick.Controls 6.2
import QtQuick.Layouts 6.3

import org.pipewire.pipewire_rust_ui 1.0

Window {
    width: 640
    height: 480

    visible: true
    title: "PipeWireGui"

    QPipeWire {
        id: pipewire
        clients: QMapListModel {
            id: pipewireClients
            roles: ["client_id", "client_version", "application_name", "module_id"]
            roleColumnMap: ({
                client_id: "id",
                client_version: "version",
                application_name: "application.name",
                module_id: "module.id"
            })

            onDataChanged: clientsScreen.model = pipewire.clients
        }
    }

    Pane {
        id: pane1
        anchors.fill: parent

        ScrollView {
            id: sideBar
            width: 200
            visible: true
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom

            TabBar {
                id: tabBar
                height: 200
                position: TabBar.Header
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.rightMargin: 0

                contentItem: ListView {
                    model: tabBar.contentModel
                    currentIndex: mainContentStackView.currentIndex

                    orientation: ListView.Vertical
                    boundsBehavior: Flickable.StopAtBounds
                    flickableDirection: Flickable.VerticalFlick
                    snapMode: ListView.SnapToItem
                }

                Repeater {
                    id: repeater
                    model: ["Home", "Clients", "Devices", "Modules", "Nodes"]

                    TabButton {
                        id: tabButton
                        text: modelData
                        width: sideBar.width
                        checked: mainContentStackView.currentIndex === index
                    }
                }
            }
        }

        Pane {
            id: mainContent
            anchors.left: sideBar.right
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.bottom: parent.bottom

            StackLayout {
                id: mainContentStackView
                anchors.fill: parent
                currentIndex: tabBar.currentIndex

                ScreenContainer {
                    screenTitle: "Home"
                    HomeScreen {}
                }

                ScreenContainer {
                    screenTitle: "Clients"
                    ClientsScreen {
                        id: clientsScreen
                    }
                }

                ScreenContainer {
                    screenTitle: "Devices"
                    HomeScreen {}
                }

                ScreenContainer {
                    screenTitle: "Modules"
                    HomeScreen {}
                }

                ScreenContainer {
                    screenTitle: "Nodes"
                    HomeScreen {}
                }
            }
        }
    }
}

