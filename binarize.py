from PIL import Image
import os

INPUT = "sign.jpg"

img = Image.open(INPUT).convert("L")

# 1. Fixed threshold (127)
binary_fixed = img.point(lambda p: 255 if p > 128 else 0)
out_fixed = "3ba8021f2f9bdc9ffc81af0f73565c9c_binary_fixed.png"
binary_fixed.save(out_fixed)

# 2. Otsu threshold (auto-calculated optimal threshold)
def otsu_threshold(im):
    hist = im.histogram()
    total = sum(hist)
    sum_b = 0
    w_b = 0
    max_var = 0
    best_t = 127
    sum_total = sum(i * h for i, h in enumerate(hist))
    for t in range(256):
        w_b += hist[t]
        if w_b == 0 or w_b == total:
            continue
        w_f = total - w_b
        sum_b += t * hist[t]
        m_b = sum_b / w_b
        m_f = (sum_total - sum_b) / w_f
        var_between = w_b * w_f * (m_b - m_f) ** 2
        if var_between > max_var:
            max_var = var_between
            best_t = t
    return best_t

otsu_t = otsu_threshold(img)
binary_otsu = img.point(lambda p: 255 if p > otsu_t else 0)
out_otsu = "3ba8021f2f9bdc9ffc81af0f73565c9c_binary_otsu.png"
binary_otsu.save(out_otsu)

# 3. Invert (white text on black → black text on white)
binary_inv = img.point(lambda p: 0 if p > otsu_t else 255)
out_inv = "3ba8021f2f9bdc9ffc81af0f73565c9c_binary_inv.png"
binary_inv.save(out_inv)

print(f"Otsu threshold: {otsu_t}")
print("Output files:")
for name in [out_fixed, out_otsu, out_inv]:
    size_kb = os.path.getsize(name) / 1024
    print(f"  {name}  ({size_kb:.1f} KB)")
