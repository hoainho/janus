#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const os = require('os');

const PLATFORM = os.platform();
const ARCH = os.arch();

const PLATFORM_MAP = {
  'darwin-arm64': '@nano-step/janus-darwin-arm64',
  'darwin-x64': '@nano-step/janus-darwin-x64',
  'linux-x64': '@nano-step/janus-linux-x64',
  'win32-x64': '@nano-step/janus-win32-x64',
};

const platformKey = `${PLATFORM}-${ARCH}`;
const packageName = PLATFORM_MAP[platformKey];

if (!packageName) {
  console.error(`Unsupported platform: ${platformKey}`);
  console.error(`Supported platforms: ${Object.keys(PLATFORM_MAP).join(', ')}`);
  process.exit(1);
}

const binDir = path.join(__dirname, '..', 'bin');
const binName = PLATFORM === 'win32' ? 'harness-cli.exe' : 'harness-cli';
const binPath = path.join(binDir, binName);

try {
  const packageDir = path.dirname(require.resolve(`${packageName}/package.json`));
  const sourcePath = path.join(packageDir, 'bin', binName);
  
  if (!fs.existsSync(sourcePath)) {
    console.error(`Binary not found at: ${sourcePath}`);
    process.exit(1);
  }
  
  fs.copyFileSync(sourcePath, binPath);
  
  if (PLATFORM !== 'win32') {
    fs.chmodSync(binPath, '755');
  }
  
  console.log(`✓ Installed ${packageName} v${require('../package.json').version}`);
} catch (error) {
  console.error(`Failed to install binary: ${error.message}`);
  console.error(`\nTry installing manually:`);
  console.error(`  npm install -g @nano-step/janus-${platformKey}`);
  process.exit(1);
}
