# Tauri + Leptos

This template should help get you started developing with Tauri and Leptos. 

It uses Tauri 2.0beta and Leptos 0.6.

It has examples of:

* command returning `Result` that can be used with `create_resource`, `Suspense` and `ErrorBoundary`.
* command accepting no arguments
* sending events to the front-end
* logging on both front-end and back-end
* Tailwind CSS integration
* Tray Icon
* Itegration test using tauri-driver

### Setup

Follow the typical Tauri setup process for your platform. Install all dependencies listed in prerequisites.

Install trunk and tailwindcss (`npm install -g tailwindcss`).


### Desktop app


Make sure you have tauri-cli updated to the latestes version.

Run `cargo tauri dev`

### Android app

To build the app for Android on Linux insert following into .bashrc:

```
export JAVA_HOME={java home}
export ANDROID_HOME={android home}
export ANDROID_NDK_HOME={android NDK home}

export TOOLCHAIN=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64
export TARGET=aarch64-linux-android
export API=33

export AR=$TOOLCHAIN/bin/llvm-ar
export CC=$TOOLCHAIN/bin/$TARGET$API-clang
export AS=$CC
export CXX=$TOOLCHAIN/bin/$TARGET$API-clang++
export LD=$TOOLCHAIN/bin/ld
export RANLIB=$TOOLCHAIN/bin/llvm-ranlib
export STRIP=$TOOLCHAIN/bin/llvm-strip

export PATH=$PATH:$ANDROID_HOME/cmdline-tools/latest/bin
export PATH=$PATH:$TOOLCHAIN/bin
```


Run `cargo tauri android init` and `cargo tauri android dev`

### Building apk file

Follow instructions at https://next--tauri.netlify.app/next/guides/distribution/sign-android/
You can use the included local.properties and build.gradle.kts template files.

Copy `local.properties_template` to `/src-tauri/gen/android/local.properties`. 

Copy `app_build.gradle.kts_template` to `/src-tauri/gen/android/app/build.gradle.kts`.
Don't forget to change namespace and applicationId parameters in the grandle build template. 

### Integration tests

Install tauri-driver and WebKitWebDriver (https://tauri.app/v1/guides/testing/webdriver/introduction). Build the app using `cargo tauri build`. Run the test using `cargo test`. 

### Android Auto support

Work in progress
