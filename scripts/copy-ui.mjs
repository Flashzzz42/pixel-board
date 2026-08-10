// 把根目录 index.html（唯一事实源）复制到 ui/index.html 供 Tauri 打包。
// 路径基于本文件定位仓库根目录，与调用时的工作目录无关。
import { copyFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const ui = join(root, 'ui');
mkdirSync(ui, { recursive: true });
copyFileSync(join(root, 'index.html'), join(ui, 'index.html'));
console.log('copied index.html -> ui/index.html');
