const fs = require('fs-extra');
const path = require('path');

const srcDir = path.resolve(__dirname, '../dependencies/skydata');
const destDir = path.resolve(__dirname, '../public/skydata');

async function smartCopy(src, dest) {
  const entries = await fs.readdir(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = path.join(src, entry.name);
    const destPath = path.join(dest, entry.name);

    // Skip .git directories
    if (entry.name === '.git') continue;

    if (entry.isDirectory()) {
      await smartCopy(srcPath, destPath);
    } else if (entry.isFile()) {
      const exists = await fs.pathExists(destPath);
      let shouldCopy = true;

      if (exists) {
        const [srcStat, destStat] = await Promise.all([
          fs.stat(srcPath),
          fs.stat(destPath),
        ]);
        shouldCopy = srcStat.size !== destStat.size;
      }

      if (shouldCopy) {
        await fs.ensureDir(path.dirname(destPath));
        await fs.copyFile(srcPath, destPath);
      }
    }
  }
}

(async () => {
  try {
    await smartCopy(srcDir, destDir);
    console.log('Asset copy complete.');
  } catch (err) {
    console.error('Error copying assets:', err);
    process.exit(1);
  }
})();
