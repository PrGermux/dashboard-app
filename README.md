# Dashboard App

A simple Rust-based GUI application for Windows that serves as a dashboard to launch various tools and applications. The interface features clickable icons with corresponding labels, making it easy to access different functionalities directly from a single window.

## Key Features

- **Simple GUI Interface**: The app provides a clean and straightforward interface for launching multiple applications from a single dashboard.
- **Custom Icons**: Each tool/application has a custom icon, with labels positioned below the icons for easy identification.
- **Prevents Window Resizing**: The window size is fixed, preventing any resizing or maximization, ensuring consistent UI experience.
- **Custom Application Icon**: The app includes a custom icon displayed in the title bar and in the file explorer.
- **Hover and Click Effects**: Cursor changes to a hand icon on hover, and to a loading icon when an application is launched.

## Usage

This application is an essential tool for production managers, data analysts, and engineers who require quick access to various specialized tools and reports. The dashboard provides a centralized interface to launch multiple applications related to yield reporting, error tracking, quality control, and other critical processes. By offering a simple, icon-based navigation system, the app streamlines workflow, reduces the time spent searching for tools, and enhances overall productivity. Whether you’re analyzing production data, planning workflows, or generating reports, this tool ensures that all necessary applications are just a click away, improving efficiency and focus on key tasks.

## Rust Branch and Complexity

- **Rust Version**: This project is built using stable Rust. Make sure you have the latest stable version of Rust installed.
- **Branch**: The default branch is `main`, which contains the latest stable code.
- **Complexity**: The codebase is simple and suitable for beginners to intermediates in Rust. It leverages the `eframe` crate for GUI, `egui` for UI elements, and standard Rust libraries for process management.

## Code Structure

- **`src/main.rs`**: The main entry point of the application, containing the core logic, GUI setup, and event handling.
- **`res/`**: Directory containing the icon file.
- **`Cargo.toml`**: The project configuration file, specifying dependencies and metadata.

### Core Components

- **DashboardApp Struct**: Holds the textures and manages the UI updates.
- **load_image Function**: Handles loading and resizing of PNG images for use as icons.
- **run_exe Function**: Launches the external applications when their respective buttons are clicked.

## Future Enhancements

- **Dynamic Button Addition**: Implement a feature where users can dynamically add new buttons and applications to the dashboard without modifying the code.
- **Configuration File**: Use a configuration file (e.g., `JSON`, `TOML`) to define the applications and icons, allowing easy updates and modifications.
- **Cross-Platform Support**: Extend the app's compatibility to other operating systems like macOS and Linux.
- **Themes and Customization**: Introduce customizable themes and layouts, allowing users to personalize the dashboard appearance.
- **Error Handling and Logging**: Enhance error handling for launching applications and implement logging for better debugging.

## Screenshots

<p align="center">
  <img src="https://github.com/user-attachments/assets/6a1100dd-eb18-4e86-a237-67ca7f538cf7"/>
</p>
