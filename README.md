# 🖌️ Van Gogh Image Viewer

Qt6-based image viewer and manipulation tool. In dire need of modernization. And cool features.

## Setting up

Check out this project using [Git](https://github.com/git-guides/install-git). To build this project, you'll need some tools and dependencies on your system. First off, this project contains configurations for Visual Studio Code, [which you can download here](https://code.visualstudio.com/download). Of course, you can use your favourite editor, but support may vary.

Next up is installing the build tools. Perhapse not so surprisingly, you'll need Rust, [which you can install using the instructions here](https://rustup.rs/). Furthermore, we'll need Clang and CMake.

If you're on **Fedora**, run the following commands:

```shell
sudo dnf install clang cmake
```

On **Ubuntu** or **Debian**, run:

```shell
sudo apt install clang cmake
```

With that out of the way, we need to install some dependencies namely Qt and libpng, which this project relies upon.

On **Fedora**:

```shell
sudo dnf install qt6-qttools-devel libpng-devel
```

On **Ubuntu** or **Debian**:

```shell
sudo apt install libpng-dev qt6-tools-dev qt6-tools-dev-tools qt6-l10n-tools libgl1-mesa-dev qt6-wayland qt6-wayland-dev qt6-wayland-dev-tools
```

All right, now go ahead and open the project folder at the location you checked it out to in VS Code. After a few seconds, VS Code will ask you to install recommended extensions. Click 'yes'. This will install the extensions specified in `.vscode/extensions.json`. This installation may take a while.

## Building and running
Once the extensions are installed, VSCode will ask you to configure the project. Click 'yes' again here. Next, you'll be prompted to select a kit. Make sure to select Clang here. Now, you can press F7 to build the project, and F5 to start debugging. The debug configuration is set up to show the `cuddlyferris.png` image in your freshly built copy of the Van Gogh Image Viewer!

If you want to build run Van Gogh from the command line instead, you can run the following commands:

```shell
cd build
cmake ..
cmake --build .
cd ..
build/Van_Gogh <your_image_here>
```
