// Node.js script to fetch latest release files from stellarium-web-engine
import https from 'https';
import fs from 'fs';
import path from 'path';

const filesToFetch = [
    'src/assets/js/stellarium-web-engine.js',
    'src/assets/js/stellarium-web-engine.wasm',
    // Add more files as needed
];

const repo = 'skyctl-space/stellarium-web-engine';

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    https.get(url, { headers: { 'User-Agent': 'node.js' } }, res => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        // Follow redirect
        const redirectUrl = res.headers.location;
        if (redirectUrl) {
          console.log(`Redirecting to ${redirectUrl}`);
          return downloadFile(redirectUrl, dest).then(resolve).catch(reject);
        } else {
          return reject(new Error(`Redirect location not provided for ${url}`));
        }
      }

      if (res.statusCode !== 200) {
        return reject(new Error(`Failed to get '${url}' (${res.statusCode})`));
      }

      const fileStream = fs.createWriteStream(dest);
      res.pipe(fileStream);
      fileStream.on('finish', () => fileStream.close(resolve));
    }).on('error', reject);
  });
}


for (const file of filesToFetch) {
    const fileName = path.basename(file);
    const url = `https://github.com/${repo}/releases/latest/download/${fileName}`;
    const dest = path.join(path.resolve(), 'src', 'assets', 'js', fileName);

    // Check if the file already exists
    if (fs.existsSync(dest)) {
        console.log(`File already exists, skipping: ${dest}`);
        continue;
    }

    fs.mkdirSync(path.dirname(dest), { recursive: true });
    console.log(`Downloading ${url} -> ${dest}`);
    try {
        await downloadFile(url, dest);
        console.log(`Downloaded: ${fileName}`);
    } catch (e) {
        console.error(`Failed to download ${fileName}:`, e);
        process.exit(1); // Abort the script with a non-zero exit code
    }
}
