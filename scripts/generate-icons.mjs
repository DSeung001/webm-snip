import { existsSync } from 'node:fs';
import { execFileSync } from 'node:child_process';

const source = 'src-tauri/icons/app-icon-source.png';
const target = 'src-tauri/icons/icon.png';

if (!existsSync(source)) {
  throw new Error(`Missing icon source: ${source}`);
}

const script = `
from PIL import Image
source = ${JSON.stringify(source)}
target = ${JSON.stringify(target)}
image = Image.open(source).convert("RGBA")
image = image.resize((512, 512), Image.Resampling.LANCZOS)
image.save(target)
`;

execFileSync('python3', ['-c', script], { stdio: 'inherit' });
