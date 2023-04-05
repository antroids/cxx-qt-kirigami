# Rust + CXX-Qt + KDE Kirigami minimal project template

## Requirements:
* Qt6
* KDE Kirigami2 and KI18n based on Qt6 ([can be built from sources](https://community.kde.org/Get_Involved/development#Building_software_with_kdesrc-build))
* Dependencies of [CXX-Qt](https://github.com/KDAB/cxx-qt)

## Configuration:
* **KDE_INCLUDEDIR**: must contain `KF6/KI18n/klocalizedcontext.h` (default `/usr/include/`)
* **KDE_LIBDIR**: libraries search path; `libKF6I18n.so` and `libKF6Kirigami2.so` should be there (default `/usr/lib/x86_64-linux-gnu/`)
* **KDE_QMLDIR**: QML modules directory (default `/usr/lib/x86_64-linux-gnu/qt6/qml/`)
* **LD_LIBRARY_PATH**: may be required to run the QT app, set to the same as **KDE_LIBDIR**

## Debug
* Define **QML_IMPORT_TRACE**=1 to see QML import resolution logs