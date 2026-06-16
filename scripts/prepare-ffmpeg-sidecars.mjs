import { execSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(__dirname, '..');
const binariesDir = path.join(rootDir, 'src-tauri', 'binaries');

function getTargetTriple() {
  if (process.env.CARGO_BUILD_TARGET) {
    return process.env.CARGO_BUILD_TARGET;
  }

  try {
    const rustcOutput = execSync('rustc -vV', { encoding: 'utf8' });
    const match = rustcOutput.match(/^host: (.+)$/m);
    if (match?.[1]) {
      return match[1];
    }
  } catch {
    // Fall back to platform heuristics below.
  }

  if (process.platform === 'darwin') {
    return process.arch === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
  }

  if (process.platform === 'win32') {
    return 'x86_64-pc-windows-msvc';
  }

  return 'x86_64-unknown-linux-gnu';
}

function sidecarName(baseName, targetTriple) {
  if (process.platform === 'win32') {
    return `${baseName}-${targetTriple}.exe`;
  }

  return `${baseName}-${targetTriple}`;
}

function findBinary(envVar, commandName) {
  const fromEnv = process.env[envVar];
  if (fromEnv && fs.existsSync(fromEnv)) {
    return fs.realpathSync(fromEnv);
  }

  const pathCandidates =
    process.platform === 'darwin'
      ? ['/opt/homebrew/bin', '/usr/local/bin', '/usr/bin']
      : process.platform === 'win32'
        ? []
        : ['/usr/bin', '/usr/local/bin'];

  for (const directory of pathCandidates) {
    const candidate = path.join(directory, commandName);
    if (fs.existsSync(candidate)) {
      return fs.realpathSync(candidate);
    }
  }

  try {
    const whichCommand = process.platform === 'win32' ? 'where' : 'which';
    const located = execSync(`${whichCommand} ${commandName}`, {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore']
    })
      .trim()
      .split(/\r?\n/)[0];

    if (located && fs.existsSync(located)) {
      return fs.realpathSync(located);
    }
  } catch {
    // Continue to the final error message.
  }

  return null;
}

function copyBinary(sourcePath, destinationPath) {
  fs.mkdirSync(path.dirname(destinationPath), { recursive: true });
  fs.copyFileSync(sourcePath, destinationPath);
  fs.chmodSync(destinationPath, 0o755);
}

function prepareSidecar(baseName, envVar, targetTriple, force) {
  const destinationPath = path.join(binariesDir, sidecarName(baseName, targetTriple));

  if (!force && fs.existsSync(destinationPath)) {
    console.log(`Using existing ${path.basename(destinationPath)}`);
    return;
  }

  const sourcePath = findBinary(envVar, baseName);
  if (!sourcePath) {
    throw new Error(
      [
        `${baseName} was not found.`,
        `Install FFmpeg before building (macOS: brew install ffmpeg),`,
        `or set ${envVar} to the ${baseName} binary path.`
      ].join(' ')
    );
  }

  copyBinary(sourcePath, destinationPath);
  console.log(`Prepared ${path.basename(destinationPath)} from ${sourcePath}`);
}

const force = process.argv.includes('--force');
const targetTriple = getTargetTriple();

fs.mkdirSync(binariesDir, { recursive: true });
prepareSidecar('ffmpeg', 'WEBM_SNIP_FFMPEG', targetTriple, force);
prepareSidecar('ffprobe', 'WEBM_SNIP_FFPROBE', targetTriple, force);

console.log(`FFmpeg sidecars are ready for ${targetTriple}.`);
