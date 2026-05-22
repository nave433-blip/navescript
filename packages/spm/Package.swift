// swift-tools-version:5.5
import PackageDescription

let package = Package(
    name: "NvsRuntime",
    products: [.library(name: "NvsRuntime", targets: ["NvsRuntime"])],
    targets: [.target(name: "NvsRuntime", dependencies: [])]
)
