// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.ContentResolver;
import android.content.Context;
import android.content.Intent;
import android.database.Cursor;
import android.net.Uri;
import android.os.Bundle;
import android.provider.OpenableColumns;
import android.util.Log;

import com.google.androidgamesdk.GameActivity;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * The activity every Mahay app runs in. The app itself is Rust (eframe on android-activity's
 * GameActivity glue); this class only does what needs Java: the document pickers, incoming
 * VIEW/SEND intents, the clipboard and launching sibling apps. Rust calls the
 * public methods through JNI (libs/core/crates/android) and gets answers through the native
 * methods. Answers never block the Rust thread.
 */
public class MahayActivity extends GameActivity {
    private static final String TAG = "Mahay";
    private static final int REQ_OPEN = 0x4d01;
    private static final int REQ_EXPORT = 0x4d02;

    /** Where to copy the next picked document (a folder under files/docs). */
    private String openInto;
    /** Rust's id for the open in flight; 0 means "queue it for poll()". */
    private int openRequest;
    /** Local copy waiting for a CREATE_DOCUMENT answer. */
    private String exporting;
    /** The intent the app was launched with, until Rust takes it. */
    private Intent launchIntent;

    private static native void nativeOnOpened(int request, String path, String name, String uri);
    private static native void nativeOnExported(String path, String uri);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        launchIntent = getIntent();
        super.onCreate(savedInstanceState);
    }

    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        String[] got = copyFromIntent(intent, new File(getFilesDir(), "docs/in-" + System.nanoTime()).getPath());
        if (got != null) {
            nativeOnOpened(0, got[0], got[1], got[2].isEmpty() ? null : got[2]);
        }
    }

    // ---- Called from Rust -------------------------------------------------------------------

    public String cacheDirPath() {
        return getCacheDir().getPath();
    }

    public void openDocument(final int request, final String[] mimes, final String into) {
        runOnUiThread(() -> {
            openInto = into;
            openRequest = request;
            Intent i = new Intent(Intent.ACTION_OPEN_DOCUMENT);
            i.addCategory(Intent.CATEGORY_OPENABLE);
            i.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION | Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                    | Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION);
            if (mimes == null || mimes.length == 0) {
                i.setType("*/*");
            } else if (mimes.length == 1) {
                i.setType(mimes[0]);
            } else {
                i.setType("*/*");
                i.putExtra(Intent.EXTRA_MIME_TYPES, mimes);
            }
            try {
                startActivityForResult(i, REQ_OPEN);
            } catch (Exception e) {
                Log.w(TAG, "no document picker", e);
                nativeOnOpened(request, null, null, null);
            }
        });
    }

    public void exportDocument(final String src, final String name, final String mime) {
        runOnUiThread(() -> {
            exporting = src;
            Intent i = new Intent(Intent.ACTION_CREATE_DOCUMENT);
            i.addCategory(Intent.CATEGORY_OPENABLE);
            i.setType(mime);
            i.putExtra(Intent.EXTRA_TITLE, name);
            try {
                startActivityForResult(i, REQ_EXPORT);
            } catch (Exception e) {
                Log.w(TAG, "no document creator", e);
                exporting = null;
                nativeOnExported(src, null);
            }
        });
    }

    /** Copy a local file over the document at uri. */
    public boolean writeBack(String src, String uri) {
        try (InputStream in = new FileInputStream(src);
             OutputStream out = getContentResolver().openOutputStream(Uri.parse(uri), "wt")) {
            if (out == null) return false;
            copy(in, out);
            return true;
        } catch (Exception e) {
            Log.w(TAG, "write back failed: " + uri, e);
            return false;
        }
    }

    /** The launch document copied into dir, as {path, name, uri-or-empty}, or null. */
    public String[] takeLaunchFile(String dir) {
        Intent intent = launchIntent;
        launchIntent = null;
        return intent == null ? null : copyFromIntent(intent, dir);
    }

    public boolean launchApp(String pkg) {
        Intent i = getPackageManager().getLaunchIntentForPackage(pkg);
        if (i == null) return false;
        i.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        try {
            startActivity(i);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

    public boolean isInstalled(String pkg) {
        return getPackageManager().getLaunchIntentForPackage(pkg) != null;
    }

    public void openUrl(String url) {
        Uri uri = Uri.parse(url);
        String scheme = uri.getScheme();
        if (!"https".equals(scheme) && !"http".equals(scheme) && !"mailto".equals(scheme)) return;
        try {
            startActivity(new Intent(Intent.ACTION_VIEW, uri).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
        } catch (Exception e) {
            Log.w(TAG, "no app for " + url, e);
        }
    }

    /**
     * System bars, display cutout and soft keyboard insets in pixels: {left, top, right, bottom}.
     * Android 15+ draws apps edge to edge, so the app keeps its content out of these itself.
     */
    public int[] systemInsets() {
        android.view.View view = getWindow().getDecorView();
        android.view.WindowInsets wi = view.getRootWindowInsets();
        if (wi == null) return new int[] {0, 0, 0, 0};
        if (android.os.Build.VERSION.SDK_INT >= 30) {
            android.graphics.Insets bars = wi.getInsets(android.view.WindowInsets.Type.systemBars()
                    | android.view.WindowInsets.Type.displayCutout());
            android.graphics.Insets ime = wi.getInsets(android.view.WindowInsets.Type.ime());
            return new int[] {bars.left, bars.top, bars.right, Math.max(bars.bottom, ime.bottom)};
        }
        return new int[] {wi.getSystemWindowInsetLeft(), wi.getSystemWindowInsetTop(),
                wi.getSystemWindowInsetRight(), wi.getSystemWindowInsetBottom()};
    }

    public String clipboardText() {
        final String[] out = {null};
        final Object lock = new Object();
        synchronized (lock) {
            runOnUiThread(() -> {
                synchronized (lock) {
                    ClipboardManager cm = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
                    if (cm != null && cm.hasPrimaryClip() && cm.getPrimaryClip().getItemCount() > 0) {
                        CharSequence t = cm.getPrimaryClip().getItemAt(0).coerceToText(this);
                        out[0] = t == null ? null : t.toString();
                    }
                    lock.notify();
                }
            });
            try {
                lock.wait(1000);
            } catch (InterruptedException ignored) {
            }
        }
        return out[0];
    }

    public void setClipboardText(final String text) {
        runOnUiThread(() -> {
            ClipboardManager cm = (ClipboardManager) getSystemService(Context.CLIPBOARD_SERVICE);
            if (cm != null) cm.setPrimaryClip(ClipData.newPlainText("Mahay", text));
        });
    }

    // ---- Picker answers ---------------------------------------------------------------------

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        Uri uri = (resultCode == RESULT_OK && data != null) ? data.getData() : null;
        if (requestCode == REQ_OPEN) {
            int request = openRequest;
            openRequest = 0;
            String[] got = null;
            if (uri != null && openInto != null) {
                boolean writable = keepPermission(uri, data);
                got = copyIn(uri, openInto, writable);
            }
            if (got != null) {
                nativeOnOpened(request, got[0], got[1], got[2].isEmpty() ? null : got[2]);
            } else if (request != 0) {
                nativeOnOpened(request, null, null, null);
            }
        } else if (requestCode == REQ_EXPORT) {
            String src = exporting;
            exporting = null;
            if (src == null) return;
            if (uri == null) {
                nativeOnExported(src, null);
                return;
            }
            keepPermission(uri, data);
            nativeOnExported(src, writeBack(src, uri.toString()) ? uri.toString() : null);
        }
    }

    // ---- Helpers ----------------------------------------------------------------------------

    /** Keep access across restarts; true when the grant includes writing. */
    private boolean keepPermission(Uri uri, Intent data) {
        int flags = data.getFlags() & (Intent.FLAG_GRANT_READ_URI_PERMISSION | Intent.FLAG_GRANT_WRITE_URI_PERMISSION);
        try {
            getContentResolver().takePersistableUriPermission(uri, flags);
        } catch (Exception ignored) {
            // Not every provider offers persistable grants; this session's grant still works.
        }
        return (flags & Intent.FLAG_GRANT_WRITE_URI_PERMISSION) != 0;
    }

    private String[] copyFromIntent(Intent intent, String dir) {
        String action = intent.getAction();
        Uri uri = null;
        if (Intent.ACTION_VIEW.equals(action) || Intent.ACTION_EDIT.equals(action)) {
            uri = intent.getData();
        } else if (Intent.ACTION_SEND.equals(action)) {
            uri = intent.getParcelableExtra(Intent.EXTRA_STREAM);
        }
        if (uri == null) return null;
        boolean writable = (intent.getFlags() & Intent.FLAG_GRANT_WRITE_URI_PERMISSION) != 0;
        return copyIn(uri, dir, writable);
    }

    /** Copy uri into dir under its display name: {path, name, uri if writable else ""}. */
    private String[] copyIn(Uri uri, String dir, boolean writable) {
        String name = displayName(uri);
        File folder = new File(dir);
        if (!folder.isDirectory() && !folder.mkdirs()) return null;
        File out = new File(folder, name);
        ContentResolver cr = getContentResolver();
        try (InputStream in = cr.openInputStream(uri); OutputStream os = new FileOutputStream(out)) {
            if (in == null) return null;
            copy(in, os);
        } catch (Exception e) {
            Log.w(TAG, "couldn't read " + uri, e);
            return null;
        }
        return new String[] {out.getPath(), name, writable ? uri.toString() : ""};
    }

    private String displayName(Uri uri) {
        String name = null;
        try (Cursor c = getContentResolver().query(uri, new String[] {OpenableColumns.DISPLAY_NAME}, null, null, null)) {
            if (c != null && c.moveToFirst()) name = c.getString(0);
        } catch (Exception ignored) {
        }
        if (name == null) name = uri.getLastPathSegment();
        if (name == null || name.isEmpty()) name = "document";
        // A display name is not a path: keep only its last segment.
        name = name.replace('\\', '/');
        name = name.substring(name.lastIndexOf('/') + 1);
        if (name.equals(".") || name.equals("..") || name.isEmpty()) name = "document";
        return name;
    }

    private static void copy(InputStream in, OutputStream out) throws java.io.IOException {
        byte[] buf = new byte[64 * 1024];
        int n;
        while ((n = in.read(buf)) > 0) out.write(buf, 0, n);
    }
}
