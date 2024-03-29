# android-cli

Create, build, and release Android apps faster without Android Studio.

> ⚠  Still️ in early development. Bugs may arise.

## Introduction

Android CLI is a powerful tool that simplifies and streamlines the Native Android development process. It provides a single interface for developers to create, build, and release Android applications without the need for cumbersome IDEs like Android Studio.

## Installation

Refer to the [docs](https://android-cli.syed.world/get-started/installation.html).

## Usage

Android CLI offers a range of features and commands that can be executed from a single interface. Some of the key commands include:

- `android create`: Create a new project
- `android init`: Initialize a project
- `android build`: Build the project
- `android run`: Build and install the app on a device
- `android devices`: List connected devices
- `android shell`: Access the device shell
- `android emulator`: Launch an emulator

And so much more..

## How does it work?

Android CLI is an abstraction layer that simplifies and streamlines the Android development process by providing a user-friendly interface that abstracts away the complexity of Gradle and Android CLI tool commands.

This abstraction layer allows developers to execute complex tasks with ease without having to learn and memorize the underlying command-line options. By abstracting away the details of these commands, Android CLI empowers developers to focus on their code and building great apps.

## FAQ

### Isn't there already an `android` tool that ships with the android SDK?

> Yes, there is an `android` bash script that comes with the Android SDK. However, it has been deprecated, and its functionalities have been absorbed into Android Studio, making it the only supported option for Android development. This decision has frustrated developers who prefer to use a CLI to manage their projects.

### But wait.. Don't I have to install Android Studio to get the SDK?

> Not necessarily. If you go to [Android Studio download page](https://developer.android.com/studio) and scroll down enough you'll see a section named 'Command line tools only'. Just grab that and there's no need for Android Studio.

### Is a simple wrapper even enough?

> Well yes and no, I eventually intend to build an interface on top of the existing commands in the near future to make actions more controllable and user-friendly as a whole. For example: imagine a prompt which lets you choose which device to run your project on, and even offer launching an emulator right there. there's so many possibilities.

## Legal

The tool itself is forever free. Distributed under the [MIT License](LICENSE).

However, the Android SDK is not free. You must accept the [Android SDK License Agreement](https://developer.android.com/studio/terms.html) before using the SDK.

Additionally, the Android CLI is not affiliated with Google or the Android Open Source Project. Android is a trademark of Google Inc.
