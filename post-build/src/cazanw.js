#!/usr/bin/env node

const { spawn } = require('child_process');

const fs = require('fs');
const path = require('path');

const platform = process.platform;
const arch = process.arch;

function getWhichTarget() {


if (platform === 'darwin') {
return arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
}

if (platform === 'win32') {
return arch === 'arm64' ? 'aarch64-pc-windows-msvc' : 'x86_64-pc-windows-gnu';
}

if (platform === 'linux') {
return arch === 'arm64' ? 'aarch64-unknown-linux-gnu' : 'x86_64-unknown-linux-gnu';
}

throw new Error(`Unsupported platform or architecture: ${platform} ${arch}`);
}

let exe_ext = process.platform === 'win32' ? '.exe' : '';

let pathToBin = path.join(__dirname, getWhichTarget(), 'cazanw' + exe_ext);

// Run bin with arguments
const child = spawn(pathToBin, process.argv.slice(2), { stdio: 'inherit' });

