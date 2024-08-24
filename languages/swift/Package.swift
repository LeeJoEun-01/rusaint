// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Rusaint",
    platforms: [
        .iOS(.v16),
    ],
    products:[
        .library(
            name: "Rusaint",
            target: ["Rusaint", "RusaintFFI"]
        ),
    ],
    dependencies: [],
    targets: [
        .target(
        name: "Rusaint",
        dependencies: ["RusaintFFI"],
        swiftSettings: [.unsafeFlags(["-suppress-warnings"])]),
        .testTarget(
            name: "RusaintTests",
            dependencies: ["Rusaint"]),
            .binaryTarget(name: "RusaintFFI", path: "RusaintFFI.xcframework")
    ]
)