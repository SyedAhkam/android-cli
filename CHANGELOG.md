# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.0 (2023-10-09)

### Chore

 - <csr-id-8115f6dcc8046cadeb05d339c66d576f249bebd8/> run cargo fmt
 - <csr-id-49bb3a696906bbcbc9cdb6eabf673329033406b8/> run cargo fmt
 - <csr-id-61f73e592f74a00ba38eda3fb648e1f036ddc42a/> add /out to .gitignore

### New Features

 - <csr-id-5c10fbec3b2fec9298019a46cdbffea03e713c92/> add CHANGELOG.md
 - <csr-id-b9fa8c2514477af5f6a165a51d4baad28819e617/> document `.android` reference in docs
 - <csr-id-90ed6aa99b896beb39b31908e2dc4fffa1796c35/> add `shell` command
 - <csr-id-31d9f4f024d04a7485b43d45c6dbe657e7111908/> add early development warning in book
 - <csr-id-4aaa768b27f85e4820759c9036400f321ea9c301/> almost everything documented
 - <csr-id-76486ef337ccd67da19e81caa3884487d055e261/> add new answer to faq section
 - <csr-id-97eda8786bf22d9e5ed05a152b0a8af12eb8f2ee/> Add action to deploy to pages
 - <csr-id-2859b23262c9eb42a89cb85935929f5cf9ebc86d/> Initialize mdbook based docs
 - <csr-id-1e8cf99468aa6d43a4a3d40f40c73e8606e59365/> impl link command + project_name in .android
 - <csr-id-fc4652ea8b3d380968977c12a38f8c0fbbe13806/> better prompt handling + default values
 - <csr-id-5c863c95e60f45edfdd7421c8308d5c03e56c299/> implement the devices command
 - <csr-id-4bb6c45444969bd946f5e98295aa315d6e55dc34/> implement the launch command
 - <csr-id-84559042dda6f07b80ed75485c7be1d4daf6f9e8/> install apk first while running + refactor
 - <csr-id-994559ea50d4b91d705d83d74b56ee7c1e745603/> impl the run command
   Allows you to build the project and launch the main activity
 - <csr-id-6f295ce66d89aa8a4ecd5ccc8d14574d82628fcc/> better and descriptive errors with anyhow
 - <csr-id-ec2734d07b897f33e65a91115a9c06fe98ef3d8d/> leave a .android dotfile after project creation
   This could later makes things quite easier. Instead of parsing the whole structure again to look out for certain metadata fields, the tool would just be able to read `.android` file from now.
 - <csr-id-37d5d22ba08e6e5ccf4561285ee2779343a0f71d/> impl the install command
   Allows the user to run `android install` and it will automatically install the apk to the currently connected device.
 - <csr-id-4deac6acace945c9ff956ceab5c145d332c8a36c/> organize core logic into lib.rs
 - <csr-id-0c242a9eafca865be7ba7a9983b5331b7d28155f/> impl build command
 - <csr-id-3cc09cd87004fa7a6a158f05ee46ea8b61534220/> configure sdk versions while creating project
   It's now possible to supply min-sdk-version, compile-sdk-version and target-sdk-version flags while invoking the `create` command.
 - <csr-id-1eebc639deda2b2f1d2f4ac507f6ccbe49933496/> impl the create command

### Bug Fixes

 - <csr-id-616261fac3a337d291852fa5f94bddbe6ee2a424/> a few things in docs
 - <csr-id-ff5d370569bab49cfe324fb8f42f2b2ae468e290/> unused import warning
 - <csr-id-894b9a651699a1168b0f5bb521e929ab95a75c0c/> git repository and edit url setting
 - <csr-id-d41b6645286b01d52fd90e295ca41436aa9c7e8a/> failing doc tests
 - <csr-id-4874057fe79bbf9ff228566ae6904fc2ca639aa0/> windows compatible local.properties file
 - <csr-id-0c341ba3985c4adaa28b8d56b9d6c32fc42ed368/> find gradle is windows compatible now
 - <csr-id-c7c21f6b47701a8b9cfcb75f79ce048d6924db87/> remove debug println
 - <csr-id-b617bf09f861ee8137216f6912169e173e9d54d8/> buggy deploy path in workflow
 - <csr-id-658b1219f4d8fe14b695578ca83e0e219d7fc12d/> ignore dead_code lint in utils.rs

### Other

 - <csr-id-8a65f4641856c8c9965ed5ac4d6d3279fb312d13/> create build workflow

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 43 commits contributed to the release over the course of 180 calendar days.
 - 34 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add CHANGELOG.md ([`5c10fbe`](https://github.com/SyedAhkam/android-cli/commit/5c10fbec3b2fec9298019a46cdbffea03e713c92))
    - A few things in docs ([`616261f`](https://github.com/SyedAhkam/android-cli/commit/616261fac3a337d291852fa5f94bddbe6ee2a424))
    - Document `.android` reference in docs ([`b9fa8c2`](https://github.com/SyedAhkam/android-cli/commit/b9fa8c2514477af5f6a165a51d4baad28819e617))
    - Unused import warning ([`ff5d370`](https://github.com/SyedAhkam/android-cli/commit/ff5d370569bab49cfe324fb8f42f2b2ae468e290))
    - Add `shell` command ([`90ed6aa`](https://github.com/SyedAhkam/android-cli/commit/90ed6aa99b896beb39b31908e2dc4fffa1796c35))
    - Git repository and edit url setting ([`894b9a6`](https://github.com/SyedAhkam/android-cli/commit/894b9a651699a1168b0f5bb521e929ab95a75c0c))
    - Add early development warning in book ([`31d9f4f`](https://github.com/SyedAhkam/android-cli/commit/31d9f4f024d04a7485b43d45c6dbe657e7111908))
    - Failing doc tests ([`d41b664`](https://github.com/SyedAhkam/android-cli/commit/d41b6645286b01d52fd90e295ca41436aa9c7e8a))
    - Almost everything documented ([`4aaa768`](https://github.com/SyedAhkam/android-cli/commit/4aaa768b27f85e4820759c9036400f321ea9c301))
    - Windows compatible local.properties file ([`4874057`](https://github.com/SyedAhkam/android-cli/commit/4874057fe79bbf9ff228566ae6904fc2ca639aa0))
    - Find gradle is windows compatible now ([`0c341ba`](https://github.com/SyedAhkam/android-cli/commit/0c341ba3985c4adaa28b8d56b9d6c32fc42ed368))
    - Remove debug println ([`c7c21f6`](https://github.com/SyedAhkam/android-cli/commit/c7c21f6b47701a8b9cfcb75f79ce048d6924db87))
    - Add new answer to faq section ([`76486ef`](https://github.com/SyedAhkam/android-cli/commit/76486ef337ccd67da19e81caa3884487d055e261))
    - Buggy deploy path in workflow ([`b617bf0`](https://github.com/SyedAhkam/android-cli/commit/b617bf09f861ee8137216f6912169e173e9d54d8))
    - Add action to deploy to pages ([`97eda87`](https://github.com/SyedAhkam/android-cli/commit/97eda8786bf22d9e5ed05a152b0a8af12eb8f2ee))
    - Initialize mdbook based docs ([`2859b23`](https://github.com/SyedAhkam/android-cli/commit/2859b23262c9eb42a89cb85935929f5cf9ebc86d))
    - Run cargo fmt ([`8115f6d`](https://github.com/SyedAhkam/android-cli/commit/8115f6dcc8046cadeb05d339c66d576f249bebd8))
    - Impl link command + project_name in .android ([`1e8cf99`](https://github.com/SyedAhkam/android-cli/commit/1e8cf99468aa6d43a4a3d40f40c73e8606e59365))
    - Better prompt handling + default values ([`fc4652e`](https://github.com/SyedAhkam/android-cli/commit/fc4652ea8b3d380968977c12a38f8c0fbbe13806))
    - Implement the devices command ([`5c863c9`](https://github.com/SyedAhkam/android-cli/commit/5c863c95e60f45edfdd7421c8308d5c03e56c299))
    - Implement the launch command ([`4bb6c45`](https://github.com/SyedAhkam/android-cli/commit/4bb6c45444969bd946f5e98295aa315d6e55dc34))
    - Run cargo fmt ([`49bb3a6`](https://github.com/SyedAhkam/android-cli/commit/49bb3a696906bbcbc9cdb6eabf673329033406b8))
    - Install apk first while running + refactor ([`8455904`](https://github.com/SyedAhkam/android-cli/commit/84559042dda6f07b80ed75485c7be1d4daf6f9e8))
    - Impl the run command ([`994559e`](https://github.com/SyedAhkam/android-cli/commit/994559ea50d4b91d705d83d74b56ee7c1e745603))
    - Better and descriptive errors with anyhow ([`6f295ce`](https://github.com/SyedAhkam/android-cli/commit/6f295ce66d89aa8a4ecd5ccc8d14574d82628fcc))
    - Leave a .android dotfile after project creation ([`ec2734d`](https://github.com/SyedAhkam/android-cli/commit/ec2734d07b897f33e65a91115a9c06fe98ef3d8d))
    - Ignore dead_code lint in utils.rs ([`658b121`](https://github.com/SyedAhkam/android-cli/commit/658b1219f4d8fe14b695578ca83e0e219d7fc12d))
    - Impl the install command ([`37d5d22`](https://github.com/SyedAhkam/android-cli/commit/37d5d22ba08e6e5ccf4561285ee2779343a0f71d))
    - Add /out to .gitignore ([`61f73e5`](https://github.com/SyedAhkam/android-cli/commit/61f73e592f74a00ba38eda3fb648e1f036ddc42a))
    - Organize core logic into lib.rs ([`4deac6a`](https://github.com/SyedAhkam/android-cli/commit/4deac6acace945c9ff956ceab5c145d332c8a36c))
    - Impl build command ([`0c242a9`](https://github.com/SyedAhkam/android-cli/commit/0c242a9eafca865be7ba7a9983b5331b7d28155f))
    - Update README.md ([`da92c6f`](https://github.com/SyedAhkam/android-cli/commit/da92c6f6041512efbecc9375619211108ca51b47))
    - Update README.md ([`95b7f27`](https://github.com/SyedAhkam/android-cli/commit/95b7f272809f5afc17389bcf9b94de97a7e9c53e))
    - Update README.md ([`09a3bf3`](https://github.com/SyedAhkam/android-cli/commit/09a3bf3a116eb7878574d0d8c2e4e28dd059cc58))
    - Configure sdk versions while creating project ([`3cc09cd`](https://github.com/SyedAhkam/android-cli/commit/3cc09cd87004fa7a6a158f05ee46ea8b61534220))
    - Create build workflow ([`8a65f46`](https://github.com/SyedAhkam/android-cli/commit/8a65f4641856c8c9965ed5ac4d6d3279fb312d13))
    - Update README.md ([`e66ca2f`](https://github.com/SyedAhkam/android-cli/commit/e66ca2f14870ab38bdc80f33d22ced24b7aec1c2))
    - Impl the create command ([`1eebc63`](https://github.com/SyedAhkam/android-cli/commit/1eebc639deda2b2f1d2f4ac507f6ccbe49933496))
    - Build subcommands structure ([`72b5387`](https://github.com/SyedAhkam/android-cli/commit/72b5387d3f2a55d07cf7a0cb2f591b92e7443da2))
    - Downgrade to clap 3.x ([`d9bca8a`](https://github.com/SyedAhkam/android-cli/commit/d9bca8a7a9f2037cc10cb594f5529c178c7c2c1e))
    - Write README and Clap application ([`c46d873`](https://github.com/SyedAhkam/android-cli/commit/c46d873f7c1f2efa5d5cd82fc2cd3bcb2ac057b4))
    - Reserve crates.io package ([`e932438`](https://github.com/SyedAhkam/android-cli/commit/e932438ea630eee90f24f3eef009fd61ebd4300e))
    - Initial commit ([`b862197`](https://github.com/SyedAhkam/android-cli/commit/b862197d332f07d7cbdffa7756fde14e435a1b79))
</details>

## v0.1.0

- Reserve crates.io name

