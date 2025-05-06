# SkyCTL

![SkyCTL Logo](docs/SkyCtlLogo.png#gh-dark-mode-only)
![SkyCTL Logo](docs/SkyCtlLogoLight.png#gh-light-mode-only)

[![Continuous Integration](https://github.com/skyctl-space/skyctl/actions/workflows/ci.yaml/badge.svg)](https://github.com/skyctl-space/skyctl/actions/workflows/ci.yaml)

SkyCTL is an application designed for astrophography enthusiasts that allows to control multiple astophography setups remotely. It's completely open-source and has the following objectives:

- Multiplatform: Windows, Mac and Linux support. Support for mobile devices is possible but not in scope.
- Multi-target: capable of controlling remote setups over different protocols: Indi, ALPACA, ASIAIR, Seestar, etc. An objective is to easily allow integrations by new platforms.
- Desktop Integration: Enable pipeline of image data directly over with other post-processing tools like Siril and PixInsight.
- Quality: unlike most open source projects around astronomy we want to deliver the code with quality and minimizing regressions, so we implement a from the beggining automated test suites.

It integrates with Stellarium Web Engine and provides tools for managing telescopes, observing celestial objects, and analyzing astronomical data.

## Features

- **Telescope Management**: Control and monitor telescopes with ASIAIR (other platforms will be added).
- **Stellarium Integration**: View and interact with celestial objects using Stellarium Web Engine.
- **Image Viewer**: Analyze astronomical images with advanced tools like histograms and overlays.
- **Observing Tools**: Access observing panels, target search, and selected object information.
- **Weather Integration**: Check weather conditions for optimal observation.

## Contributing

Contributions are welcome! Please visit our [building page](docs/Building.md) to learn how to build and modify the code.

## License

SkyCTL is licensed under the [AGPL License](LICENSE).

## Acknowledgments

- [Stellarium Web Engine](https://github.com/Stellarium/stellarium-web-engine)
- [Vuetify](https://vuetifyjs.com/)
- [Tauri](https://tauri.app/)

## Contact

For questions or support, please open an issue on the [GitHub repository](https://github.com/skyctl-space/skyctl).
