package com.pixelboard.app;

import android.content.ContentValues;
import android.net.Uri;
import android.os.Build;
import android.os.Environment;
import android.provider.MediaStore;
import android.util.Base64;

import com.getcapacitor.JSObject;
import com.getcapacitor.Plugin;
import com.getcapacitor.PluginCall;
import com.getcapacitor.PluginMethod;
import com.getcapacitor.annotation.CapacitorPlugin;

import java.io.OutputStream;
import java.nio.charset.StandardCharsets;

/**
 * 导出到系统相册 / 下载目录的原生桥。
 * Android 10+（API 29）用 MediaStore，免权限、免对话框：
 *  - saveImage：PNG 写进 Pictures/像素工坊 → 系统相册可见
 *  - saveDraft：JSON 写进 Downloads 根目录
 * 旧系统（API<29）无法用 RELATIVE_PATH，直接返回错误提示，由 JS 展示。
 */
@CapacitorPlugin(name = "ExportBridge")
public class ExportBridgePlugin extends Plugin {

    @PluginMethod
    public void saveImage(PluginCall call) {
        String base64 = call.getString("base64");
        String fileName = call.getString("fileName");
        if (base64 == null || fileName == null) {
            call.resolve(err("参数缺失"));
            return;
        }
        if (Build.VERSION.SDK_INT < 29) {
            call.resolve(err("请在 Android 10 及以上系统导出"));
            return;
        }
        try {
            byte[] data = Base64.decode(base64, Base64.NO_WRAP);
            ContentValues values = new ContentValues();
            values.put(MediaStore.Images.Media.DISPLAY_NAME, fileName);
            values.put(MediaStore.Images.Media.MIME_TYPE, "image/png");
            values.put(MediaStore.Images.Media.RELATIVE_PATH, Environment.DIRECTORY_PICTURES + "/像素工坊");
            values.put(MediaStore.Images.Media.IS_PENDING, 1);
            Uri uri = getContext().getContentResolver().insert(MediaStore.Images.Media.EXTERNAL_CONTENT_URI, values);
            if (uri == null) {
                call.resolve(err("无法创建图片文件"));
                return;
            }
            OutputStream os = getContext().getContentResolver().openOutputStream(uri);
            if (os != null) {
                os.write(data);
                os.flush();
                os.close();
            }
            values.clear();
            values.put(MediaStore.Images.Media.IS_PENDING, 0);
            getContext().getContentResolver().update(uri, values, null, null);
            JSObject ok = new JSObject();
            ok.put("ok", true);
            ok.put("msg", "已保存到相册");
            call.resolve(ok);
        } catch (Exception e) {
            call.resolve(err("保存图片失败：" + e.getMessage()));
        }
    }

    @PluginMethod
    public void saveDraft(PluginCall call) {
        String content = call.getString("content");
        String fileName = call.getString("fileName");
        if (content == null || fileName == null) {
            call.resolve(err("参数缺失"));
            return;
        }
        if (Build.VERSION.SDK_INT < 29) {
            call.resolve(err("请在 Android 10 及以上系统导出"));
            return;
        }
        try {
            byte[] data = content.getBytes(StandardCharsets.UTF_8);
            ContentValues values = new ContentValues();
            values.put(MediaStore.Downloads.DISPLAY_NAME, fileName);
            values.put(MediaStore.Downloads.MIME_TYPE, "application/json");
            values.put(MediaStore.Downloads.RELATIVE_PATH, Environment.DIRECTORY_DOWNLOADS);
            values.put(MediaStore.Downloads.IS_PENDING, 1);
            Uri uri = getContext().getContentResolver().insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, values);
            if (uri == null) {
                call.resolve(err("无法创建草稿文件"));
                return;
            }
            OutputStream os = getContext().getContentResolver().openOutputStream(uri);
            if (os != null) {
                os.write(data);
                os.flush();
                os.close();
            }
            values.clear();
            values.put(MediaStore.Downloads.IS_PENDING, 0);
            getContext().getContentResolver().update(uri, values, null, null);
            JSObject ok = new JSObject();
            ok.put("ok", true);
            ok.put("msg", "已保存到下载目录");
            call.resolve(ok);
        } catch (Exception e) {
            call.resolve(err("保存草稿失败：" + e.getMessage()));
        }
    }

    private JSObject err(String msg) {
        JSObject r = new JSObject();
        r.put("ok", false);
        r.put("msg", msg);
        return r;
    }
}
