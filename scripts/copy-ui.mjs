// 把前端复制到 ui/index.html 供 Tauri 打包。
// 桌面（Win/mac）用 index.html；iOS 构建（TAURI_ENV_PLATFORM=ios）用移动版 index-apk.html。
// 路径基于本文件定位仓库根目录，与调用时的工作目录无关。
import { copyFileSync, mkdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const ui = join(root, 'ui');
mkdirSync(ui, { recursive: true });
const platform = process.env.TAURI_ENV_PLATFORM || 'desktop';
const src = platform === 'ios' ? 'index-apk.html' : 'index.html';
copyFileSync(join(root, src), join(ui, 'index.html'));
console.log(`copied ${src} -> ui/index.html (platform=${platform})`);
