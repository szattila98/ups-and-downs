# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [0.1.2](https://github.com/szattila98/ups-and-downs/compare/0.1.1..0.1.2) - 2024-03-31
### Package updates
- [backend-0.1.2](src-tauri) bumped to [backend-0.1.2](https://github.com/szattila98/ups-and-downs/compare/backend-0.1.1..backend-0.1.2)
### Global changes
#### Bug Fixes
- random emojis are fixed in dimensions - ([ef49417](https://github.com/szattila98/ups-and-downs/commit/ef49417d3e9e793053a56bdcf80d94e938419d67)) - [@szattila98](https://github.com/szattila98)
- record view radio button has focus - ([528ca87](https://github.com/szattila98/ups-and-downs/commit/528ca871068e157f65e0a1347ac555a4e2ff563a)) - [@szattila98](https://github.com/szattila98)
- content with length 1 or 2000 can be submitted without crashing - ([e588fdf](https://github.com/szattila98/ups-and-downs/commit/e588fdfe03f672f78e68d2e9e038c6b912e577cd)) - [@szattila98](https://github.com/szattila98)

- - -

## [0.1.1](https://github.com/szattila98/ups-and-downs/compare/0.1.0..0.1.1) - 2024-03-30
### Package updates
- [backend-0.1.1](src-tauri) bumped to [backend-0.1.1](https://github.com/szattila98/ups-and-downs/compare/backend-0.1.0..backend-0.1.1)
### Global changes
#### Bug Fixes
- views do not invoke commands anymore, only main app - ([8cc4a2a](https://github.com/szattila98/ups-and-downs/commit/8cc4a2af7f67f1b34b6a094eb2b8290e905e96b8)) - [@szattila98](https://github.com/szattila98)
- added padding to textarea - ([4b86318](https://github.com/szattila98/ups-and-downs/commit/4b863185a7d5d01a93e1d23e41f401334d6f2c2c)) - [@szattila98](https://github.com/szattila98)
- selected counter appears next to delete button, submitting 1 length content no longer panics - ([7fb27bb](https://github.com/szattila98/ups-and-downs/commit/7fb27bb0ad6919f15206510ab2ad325454e27e99)) - [@szattila98](https://github.com/szattila98)
- enter only submits, no new line is added - ([e966c4c](https://github.com/szattila98/ups-and-downs/commit/e966c4cdc214d6e3fd0a7e77fa12ca09d92bd304)) - [@szattila98](https://github.com/szattila98)
- image flicker no longe occurs, transitions ease navigation - ([ef20927](https://github.com/szattila98/ups-and-downs/commit/ef2092735f76eef61a298a8d7a6732b96db765c3)) - [@szattila98](https://github.com/szattila98)
- window state is saved on exit button - ([0506289](https://github.com/szattila98/ups-and-downs/commit/0506289fd7e49bb7c63b13a9bd9f015077faa107)) - [@szattila98](https://github.com/szattila98)
- user input is trimmed - ([1d55cc1](https://github.com/szattila98/ups-and-downs/commit/1d55cc1c44b537392481af00a40bdf7c7679cf02)) - [@szattila98](https://github.com/szattila98)
#### Continuous Integration
- lints and tests run in parralel for backend and frontend - ([bf599aa](https://github.com/szattila98/ups-and-downs/commit/bf599aa6de22215b78fa36a2fbeb2bb7b4c2a452)) - [@szattila98](https://github.com/szattila98)

- - -

## [0.1.0](https://github.com/szattila98/ups-and-downs/compare/751e68c4af7fc24cdd63dec33dacdc909095ecdc..0.1.0) - 2024-03-27
### Package updates
- [backend](src-tauri) bumped to [backend-0.1.0](https://github.com/szattila98/ups-and-downs/compare/751e68c4af7fc24cdd63dec33dacdc909095ecdc..backend-0.1.0)
### Global changes
#### Bug Fixes
- release publishes hopefully - ([606a4b2](https://github.com/szattila98/ups-and-downs/commit/606a4b25efdd58fbaad1a177485b618d6fa558f8)) - [@szattila98](https://github.com/szattila98)
- non functional features are invisible - ([d582c02](https://github.com/szattila98/ups-and-downs/commit/d582c02ad0461818e55bb65a3a436c51336185d6)) - [@szattila98](https://github.com/szattila98)
- wait on check action is given a correct check name - ([35deab5](https://github.com/szattila98/ups-and-downs/commit/35deab54de3074b273996ceabd3f484ac7b33de1)) - [@szattila98](https://github.com/szattila98)
#### Continuous Integration
- **(backend)** release can be made manually - ([05ff5ed](https://github.com/szattila98/ups-and-downs/commit/05ff5ed0e36779760bed1b5b5fc14387a8a6b643)) - [@szattila98](https://github.com/szattila98)
- **(backend)** deploy github job uploads bundles to release - ([d301fe1](https://github.com/szattila98/ups-and-downs/commit/d301fe172a5aad7cb91debd232b3ad8ecd03b78d)) - [@szattila98](https://github.com/szattila98)
- lint fixes and older ubuntu runner - ([2d8bf80](https://github.com/szattila98/ups-and-downs/commit/2d8bf808277013781e14b40bfc4bdefbc0b19940)) - [@szattila98](https://github.com/szattila98)
- checks, lints and test run in ci - ([d4f45da](https://github.com/szattila98/ups-and-downs/commit/d4f45daaf7245d8658fd5d65239a32813a180d1f)) - [@szattila98](https://github.com/szattila98)
- basic CI present, lints missing yet - ([38f2f1d](https://github.com/szattila98/ups-and-downs/commit/38f2f1d89c4feca868aa055eac9d3fff8c267218)) - [@szattila98](https://github.com/szattila98)
- git is configure correctly, toml files excluded with taplo - ([cd5a98a](https://github.com/szattila98/ups-and-downs/commit/cd5a98a6043e128a2d6ca5fdca9c266b9b0fb0ac)) - [@szattila98](https://github.com/szattila98)
#### Features
- **(backend)** sqlite database is created and migrations run on startup - ([4cbb17b](https://github.com/szattila98/ups-and-downs/commit/4cbb17b9569744032804359c8a7d40ce34b1a61f)) - [@szattila98](https://github.com/szattila98)
- release workflow waits on every other checks to run - ([f61569a](https://github.com/szattila98/ups-and-downs/commit/f61569a9131e7cf41d30254b9cdfea5cddda8583)) - [@szattila98](https://github.com/szattila98)
- wait on check correctly waits on workflow - ([677ee4f](https://github.com/szattila98/ups-and-downs/commit/677ee4f652533a72f92c5793967c29f9f358412f)) - [@szattila98](https://github.com/szattila98)
- window state is saved on exit and restore on relaunch - ([0a39f3b](https://github.com/szattila98/ups-and-downs/commit/0a39f3b099dcbf31a06814a9620d033883e2a0fb)) - [@szattila98](https://github.com/szattila98)
- many kinds of emojis show up now - ([00080bb](https://github.com/szattila98/ups-and-downs/commit/00080bbb8d02789837c7e3192b77f973a7a84008)) - [@szattila98](https://github.com/szattila98)
- individual highlights can be edited and deleted - ([dd9514b](https://github.com/szattila98/ups-and-downs/commit/dd9514ba2a72f0407133177534c1247f58cfd83f)) - [@szattila98](https://github.com/szattila98)
- footer appears under menu and jump to top button on every scrollable view if scrolled far enough - ([d0a70f0](https://github.com/szattila98/ups-and-downs/commit/d0a70f0d069df7950fa2061ce1e6b0ab682c40bf)) - [@szattila98](https://github.com/szattila98)
- tooltip shows up on hovering over relative date of highlight - ([108f14f](https://github.com/szattila98/ups-and-downs/commit/108f14fba93451aa37cf7b73733b6e81a0e04609)) - [@szattila98](https://github.com/szattila98)
- loader shown on init, icon set is new - ([d0487e7](https://github.com/szattila98/ups-and-downs/commit/d0487e75329d5bb59d6c26090963f07e809ab832)) - [@szattila98](https://github.com/szattila98)
- list and record views are less spaghetti - ([42bd0b8](https://github.com/szattila98/ups-and-downs/commit/42bd0b8d8725064cd22a27424ae266f9e6aab4e3)) - [@szattila98](https://github.com/szattila98)
- record highlight is styled, every view has a layout - ([669b282](https://github.com/szattila98/ups-and-downs/commit/669b282ffce03398ed248e40ba1e700fa62e2d94)) - [@szattila98](https://github.com/szattila98)
- listing is styled, with random but contextual emojis and colors - ([5d47feb](https://github.com/szattila98/ups-and-downs/commit/5d47feb67360b7dddfadd7ae68a51dca6393579c)) - [@szattila98](https://github.com/szattila98)
- new highlight can be recorded once a day - ([4d67159](https://github.com/szattila98/ups-and-downs/commit/4d67159c8e11e41e42cc4c325ab15758e1d65c1f)) - [@szattila98](https://github.com/szattila98)
- highlights can be saved to the database - ([bf51f8e](https://github.com/szattila98/ups-and-downs/commit/bf51f8ee6720cfae6f54327a2bcd1c22c04299c8)) - [@szattila98](https://github.com/szattila98)
- commands are now typesafe thanks to specta - ([9115154](https://github.com/szattila98/ups-and-downs/commit/9115154e35b638f9d1cc543f2abb9574540d7261)) - [@szattila98](https://github.com/szattila98)
- tauri project is initialized - ([a62001d](https://github.com/szattila98/ups-and-downs/commit/a62001dad95bc1fbca67be1a443b777053c662e9)) - [@szattila98](https://github.com/szattila98)
#### Style
- lintig enabled and test framework present - ([637ebd9](https://github.com/szattila98/ups-and-downs/commit/637ebd921d33a177b56505f48edfe521acf7f6ed)) - [@szattila98](https://github.com/szattila98)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).