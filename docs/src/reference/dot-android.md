# Dot Android

It refers to the `.android` file in your project root directory.

It is used to store certain configuration settings for the Android CLI. It is created when you either initialize a new project or run `android link`.

```ron
// DO NOT MODIFY; Generated by Android CLI for internal usage.
(
    project_name: "testing",
    package_id: "com.example.testing",
    gen_at_version: "0.1.0",
    main_activity_name: "MainActivity", 
)
```

## File format

The file is a RON file (Rusty Object Notation). It is kinda like JSON, but with a heavy rusty influence. You can read more about it [here](https://github.com/ron-rs/ron).

## Fields

### Project name

The name of the project. This is the name you gave when you initialized the project. Usually, it is the name of the directory that contains your project.

### Package ID

This is the package identifier or package name of your app. The convention comes from Java apps. It is used to uniquely identify your app on the device and on the Play Store. It is usually in the form of `com.example.appname`.

### Gen at version

This is the version of the Android CLI that generated this file. It is used to determine if the file is compatible with the current version of the CLI. This is used to prevent breaking changes from breaking your project.

### Main activity name

This is the name of the main activity of your app. This is the activity that is launched when your app is opened. It is usually left untouched as `MainActivity`.

## Usage

It is not recommended to modify this file manually. It is generated and updated by the CLI. In the future, the CLI will be able to perform migrations on your project and update this file for you automatically.