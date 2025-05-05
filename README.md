# SkyCTL

SkyCTL is a powerful and extensible application designed for astronomy enthusiasts. It integrates with Stellarium Web Engine and provides tools for managing telescopes, observing celestial objects, and analyzing astronomical data. SkyCTL offers a modern and user-friendly interface for both amateur and professional astronomers.

## Features

- **Telescope Management**: Control and monitor telescopes with ASIAIR (other platforms will be added).
- **Stellarium Integration**: View and interact with celestial objects using Stellarium Web Engine.
- **Image Viewer**: Analyze astronomical images with advanced tools like histograms and overlays.
- **Observing Tools**: Access observing panels, target search, and selected object information.
- **Weather Integration**: Check weather conditions for optimal observation.

## Project Structure

The project is organized as follows:

- **`src/`**: Contains the main Vue components, utilities, and assets.
  - `assets/`: Static assets like images and helper scripts.
  - `stellarium-components/`: Components specific to Stellarium integration.
  - `stores.ts`: Pinia store for managing application state.
  - `main.ts`: Entry point for the Vue application.
- **`public/`**: Public assets served directly, such as images and fonts.
- **`scripts/`**: Utility scripts, including `fetch-engine-assets.js` for fetching the latest Stellarium Web Engine files.
- **`src-tauri/`**: Tauri-specific files for building the desktop application.

## Installation

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/skyctl-space/skyctl.git
   cd skyctl
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

4. Start the development server:
   ```bash
   pnpm tauri dev
   ```

## Build

To build the application for production:

```bash
pnpm build
```

This will create a production-ready build in the `dist/` directory.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes and push the branch.
4. Open a pull request.

## License

SkyCTL is licensed under the [AGPL License](LICENSE).

## Acknowledgments

- [Stellarium Web Engine](https://github.com/Stellarium/stellarium-web-engine)
- [Vuetify](https://vuetifyjs.com/)
- [Pinia](https://pinia.vuejs.org/)
- [Tauri](https://tauri.app/)

## Contact

For questions or support, please open an issue on the [GitHub repository](https://github.com/skyctl-space/skyctl).