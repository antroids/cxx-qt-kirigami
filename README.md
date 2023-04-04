# Rust + CXX-Qt + KDE Kirigami minimal project template

## Requirements:
* Qt6
* KDE Kirigami2 based on Qt6 ([can be built from sources](https://community.kde.org/Get_Involved/development#Building_software_with_kdesrc-build))
* Dependencies of [CXX-Qt](https://github.com/KDAB/cxx-qt)

## Configuration:
* Check the Import Path in **main.rs**
* Make sure that **libKF6Kirigami2.so** is in the search path or specify it in the **build.rs** by **cargo:rustc-link-search**
* Add QT_VERSION_MAJOR=6 to the environment if two major Qt versions are installed

## Debug
* Define **QML_IMPORT_TRACE**=1 to see QML import resolution logs