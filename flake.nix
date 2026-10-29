{
  description = "Rust + Android (SDK, NDK, emulator) dev environment with an egui hello world";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
        };

        # ---- Android SDK -------------------------------------------------
        platformVersion = "34";
        buildToolsVersion = "34.0.0";
        ndkVersion = "27.2.12479018";
        abi = "x86_64"; # matches host so the emulator can use KVM

        # Build SDK (dev shell): platform, build-tools, NDK, adb.
        # Kept lean: no emulator / system images (those are several GB).
        androidComposition = pkgs.androidenv.composeAndroidPackages {
          platformVersions = [ platformVersion ];
          buildToolsVersions = [ buildToolsVersion ];
          includeNDK = true;
          ndkVersions = [ ndkVersion ];
          includeEmulator = false;
          includeSystemImages = false;
          includeSources = false;
        };
        androidSdk = androidComposition.androidsdk;
        sdkRoot = "${androidSdk}/libexec/android-sdk";

        # Emulator + x86_64 system image, as a separate package so it's only
        # fetched when you actually run it: `nix run .#emulator`
        emulator = pkgs.androidenv.emulateApp {
          name = "rust-android-emulator";
          inherit platformVersion;
          abiVersion = abi;
          systemImageType = "google_apis";
          # Persist the AVD across runs (default is a fresh tmp dir every time).
          androidUserHome = "$HOME/.android";
          androidAvdHome = "$HOME/.android/avd";
          deviceName = "rust_x86_64";
          # Host GPU so wgpu gets a real Vulkan/GLES device. Use
          # NIX_ANDROID_EMULATOR_FLAGS="-gpu swiftshader_indirect" if this fails.
          configOptions = {
            "hw.gpu.enabled" = "yes";
            "hw.gpu.mode" = "auto";
            "hw.keyboard" = "yes";
            "hw.ramSize" = "2048";
            # Phone-sized screen (default AVD is a tiny 320x640 @160dpi).
            "hw.lcd.width" = "1080";
            "hw.lcd.height" = "2400";
            "hw.lcd.density" = "420";
          };
          # 'app' left unset: just boots the AVD. Install with `cargo apk run`.
        };

        # ---- Rust --------------------------------------------------------
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [
            "aarch64-linux-android"
            "armv7-linux-androideabi"
            "x86_64-linux-android"
            "i686-linux-android"
            "wasm32-unknown-unknown"
          ];
        };

        # Runtime libs needed for running the desktop build of the egui app
        desktopLibs = with pkgs; [
          libGL
          vulkan-loader
          libxkbcommon
          wayland
          libx11
          libxcursor
          libxi
          libxrandr
        ];
      in
      {
        packages.emulator = emulator;
        packages.androidSdk = androidSdk;

        apps.emulator = {
          type = "app";
          program = "${emulator}/bin/run-test-emulator";
          meta.description = "Boot an x86_64 Android ${platformVersion} emulator (KVM)";
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            androidSdk
            pkgs.cargo-apk
            pkgs.just
            pkgs.trunk
            pkgs.jdk17
            pkgs.pkg-config
          ];

          TIR_DEVSHELL = "1"; # lets the justfile know it's already in this shell
          ANDROID_HOME = sdkRoot;
          ANDROID_NDK_ROOT = "${sdkRoot}/ndk/${ndkVersion}";
          ANDROID_NDK_HOME = "${sdkRoot}/ndk/${ndkVersion}";
          JAVA_HOME = pkgs.jdk17.home;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath desktopLibs;

          # Quiet when invoked non-interactively (e.g. from the justfile).
          shellHook = ''
            if [ -z "$TIR_QUIET" ]; then
              echo "Rust:    $(rustc --version)"
              echo "SDK:     $ANDROID_HOME"
              echo "NDK:     $ANDROID_NDK_ROOT"
              echo ""
              echo "  just            # list all tasks (build, test, apk, emulator, phone, ci)"
            fi
          '';
        };
      });
}
