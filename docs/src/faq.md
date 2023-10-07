# Frequently Asked Questions

## 1. Isn't there already an `android` tool that ships with the android SDK?

Yes, there is an `android` bash script that comes with the Android SDK. However, it has been deprecated, and its functionalities have been absorbed into Android Studio, making it the only supported option for Android development. This decision has frustrated developers who prefer to use a CLI to manage their projects.

## 2. But wait.. Don't I have to install Android Studio to get the SDK?

Not necessarily. If you go to [Android Studio download page](https://developer.android.com/studio) and scroll down enough you'll see a section named 'Command line tools only'. Just grab that and there's no need for Android Studio.

## 3. Is a simple wrapper even enough?

Well yes and no, I eventually intend to build an interface on top of the existing commands in the near future to make actions more controllable and user-friendly as a whole. For example: imagine a prompt which lets you choose which device to run your project on, and even offer launching an emulator right there. there's so many possibilities.
