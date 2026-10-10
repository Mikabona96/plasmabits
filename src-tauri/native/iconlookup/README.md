
# IconLookup

A small KDE icon lookup utility for Linux.

IconLookup uses KDE Frameworks to resolve icon names through the system's icon theme and return the path to the matching icon file.

It is designed to be used as a helper utility in **Plasma Bits**.

## Features

- Resolves icon names such as `vscode` using KDE's icon lookup mechanism.
- Respects the available KDE icon themes.
- Returns the path to the resolved icon file.
- Supports SVG, PNG, and other formats supported by the installed icon themes.
- Can be invoked from other applications or scripts.

## Requirements

### Build dependencies

- CMake
- C++ compiler with C++17 support
- Qt 6
- KDE Frameworks 6 — Icon Themes

### Runtime dependencies

- Qt 6
- KDE Frameworks 6 — Icon Themes
- Other shared libraries required by the compiled executable

The required runtime libraries are normally available on KDE Plasma systems.

## Build

### Arch Linux

Install the build dependencies:

```bash
sudo pacman -S --needed cmake extra-cmake-modules qt6-base kiconthemes
```

Configure the project:

```bash
cmake -B build -S .
```

Build the executable:

```bash
cmake --build build
```

The compiled executable will be located at:

```text
build/iconlookup
```

## Usage

Pass the icon name as a command-line argument:

```bash
./build/iconlookup vscode
```

If the icon is found, the utility prints the resolved file path to standard output.

Example output:

```text
/usr/share/icons/breeze/apps/48/vscode.svg
```

The actual path depends on the installed icon themes and available icon files.

If the icon cannot be resolved, the utility exits with a non-zero status code.

## Integration

IconLookup can be launched by other applications and scripts. Its standard output contains the resolved icon path, making it suitable for integration with Plasma Bits.

## License

See the repository's license file.
