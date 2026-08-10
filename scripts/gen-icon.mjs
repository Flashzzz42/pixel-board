// 生成 1024×1024 像素风应用图标 app-icon.png（最小 PNG 编码器，无第三方依赖）
import { deflateSync } from 'node:zlib';
import { writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

function crc32(buf) {
  let c = ~0;
  for (let i = 0; i < buf.length; i++) {
    c ^= buf[i];
    for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1));
  }
  return ~c >>> 0;
}
function chunk(type, data) {
  const len = Buffer.alloc(4); len.writeUInt32BE(data.length);
  const td = Buffer.concat([Buffer.from(type, 'ascii'), data]);
  const crc = Buffer.alloc(4); crc.writeUInt32BE(crc32(td));
  return Buffer.concat([len, td, crc]);
}
function png(width, height, rgba) {
  const sig = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0); ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; ihdr[9] = 6; ihdr[10] = 0; ihdr[11] = 0; ihdr[12] = 0;
  const raw = Buffer.alloc(height * (1 + width * 4));
  for (let y = 0; y < height; y++) {
    raw[y * (1 + width * 4)] = 0; // filter none
    rgba.copy(raw, y * (1 + width * 4) + 1, y * width * 4, (y + 1) * width * 4);
  }
  const idat = deflateSync(raw, { level: 9 });
  return Buffer.concat([sig, chunk('IHDR', ihdr), chunk('IDAT', idat), chunk('IEND', Buffer.alloc(0))]);
}

const S = 1024;
const N = 16;                 // 16×16 珠
const cell = S / N;           // 64
const gap = 3;
const img = Buffer.alloc(S * S * 4);

const BG = [31, 36, 48, 255];        // #1f2430 深底
const beadA = [59, 124, 255, 255];   // #3b7cff 蓝
const beadB = [245, 246, 250, 255];  // #f5f6fa 近白
const accent = [247, 201, 72, 255];  // #f7c948 黄（眼睛点缀）

function set(x, y, c) {
  const i = (y * S + x) * 4;
  img[i] = c[0]; img[i + 1] = c[1]; img[i + 2] = c[2]; img[i + 3] = c[3];
}
for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) set(x, y, BG);

// 画一个 16×16 的棋盘格珠面 + 一行黄色“眼睛”
for (let gy = 0; gy < N; gy++) {
  for (let gx = 0; gx < N; gx++) {
    const x0 = Math.round(gx * cell + gap);
    const y0 = Math.round(gy * cell + gap);
    const x1 = Math.round((gx + 1) * cell - gap);
    const y1 = Math.round((gy + 1) * cell - gap);
    const isEye = (gx === 4 && gy === 5) || (gx === 11 && gy === 5);
    const c = isEye ? accent : ((gx + gy) % 2 === 0 ? beadA : beadB);
    for (let y = y0; y < y1; y++) for (let x = x0; x < x1; x++) set(x, y, c);
  }
}

writeFileSync(join(root, 'src-tauri', 'app-icon.png'), png(S, S, img));
console.log('wrote src-tauri/app-icon.png');
