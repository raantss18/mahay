// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.ContentResolver;
import android.content.Context;
import android.app.PendingIntent;
import android.content.BroadcastReceiver;
import android.content.Intent;
import android.content.IntentFilter;
import android.content.pm.PackageInfo;
import android.content.pm.PackageInstaller;
import android.database.Cursor;
import android.net.Uri;
import android.os.Bundle;
import android.provider.OpenableColumns;
import android.util.Log;
import android.view.MotionEvent;

import com.google.androidgamesdk.GameActivity;
import com.google.androidgamesdk.gametextinput.State;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.util.ArrayDeque;

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
    private static native void nativeOnInstalled(String path, boolean ok, String message);
    private static native void nativeOnImeChanged(boolean dismissed);
    private static native void nativeOnToolType(int pointerId, int toolType);

    private static final String INSTALL_ACTION = "org.mahay.shell.INSTALL_STATUS";
    /** APKs waiting to be installed, one confirmation at a time. */
    private final ArrayDeque<String> installQueue = new ArrayDeque<>();
    private String installing;

    private final BroadcastReceiver installReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            int status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE);
            if (status == PackageInstaller.STATUS_PENDING_USER_ACTION) {
                // Android asks the user to confirm: show its dialog.
                Intent confirm = intent.getParcelableExtra(Intent.EXTRA_INTENT);
                if (confirm != null) {
                    confirm.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                    startActivity(confirm);
                }
                return;
            }
            String path = installing;
            installing = null;
            String msg = intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE);
            if (path != null) nativeOnInstalled(path, status == PackageInstaller.STATUS_SUCCESS, msg == null ? "" : msg);
            installNext();
        }
    };

    // The soft keyboard changed its text, or was dismissed. winit drops GameActivity's text
    // events, so without this the app would only see the text at its next repaint: tell it now.
    @Override
    public void stateChanged(State newState, boolean dismissed) {
        super.stateChanged(newState, dismissed);
        nativeOnImeChanged(dismissed);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        launchIntent = getIntent();
        super.onCreate(savedInstanceState);
        androidx.core.content.ContextCompat.registerReceiver(this, installReceiver,
                new IntentFilter(INSTALL_ACTION), androidx.core.content.ContextCompat.RECEIVER_NOT_EXPORTED);
    }

    @Override
    protected void onNewIntent(Intent intent) {
        super.onNewIntent(intent);
        String[] got = copyFromIntent(intent, new File(getFilesDir(), "docs/in-" + System.nanoTime()).getPath());
        if (got != null) {
            nativeOnOpened(0, got[0], got[1], got[2].isEmpty() ? null : got[2]);
        }
    }

    // ---- Touch tool types --------------------------------------------------------------------

    /**
     * winit's touch events carry no tool type: tell Rust, before GameActivity queues the event,
     * which pointer is a finger and which a stylus, so a finger draws at full pressure. The
     * native call only stores two ints; it never waits for the Rust thread.
     */
    private void noteToolTypes(MotionEvent ev) {
        if (noToolTypes) return;
        try {
            for (int i = 0; i < ev.getPointerCount(); i++) {
                nativeOnToolType(ev.getPointerId(i), ev.getToolType(i));
            }
        } catch (UnsatisfiedLinkError e) {
            // A native library built before this call existed: touch still works, at its pressure.
            noToolTypes = true;
        }
    }

    private static boolean noToolTypes;

    @Override
    public boolean dispatchTouchEvent(MotionEvent ev) {
        noteToolTypes(ev);
        return super.dispatchTouchEvent(ev);
    }

    @Override
    public boolean dispatchGenericMotionEvent(MotionEvent ev) {
        // A hovering stylus.
        noteToolTypes(ev);
        return super.dispatchGenericMotionEvent(ev);
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
        if (!isContentUri(Uri.parse(uri))) return false;
        // Only files inside this app's own document folder are ever copied out.
        try {
            String docs = new File(getFilesDir(), "docs").getCanonicalPath() + File.separator;
            if (!new File(src).getCanonicalPath().startsWith(docs)) return false;
        } catch (java.io.IOException e) {
            return false;
        }
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
        int l, t, r, b;
        if (android.os.Build.VERSION.SDK_INT >= 30) {
            android.graphics.Insets bars = wi.getInsets(android.view.WindowInsets.Type.systemBars()
                    | android.view.WindowInsets.Type.displayCutout());
            android.graphics.Insets ime = wi.getInsets(android.view.WindowInsets.Type.ime());
            l = bars.left; t = bars.top; r = bars.right; b = Math.max(bars.bottom, ime.bottom);
        } else {
            l = wi.getSystemWindowInsetLeft(); t = wi.getSystemWindowInsetTop();
            r = wi.getSystemWindowInsetRight(); b = wi.getSystemWindowInsetBottom();
        }
        // Only what really overlaps the app's content counts. Before Android 15 the window may
        // already sit below the bars (and above the keyboard), and a free-form or split-screen
        // window may not touch them at all: the root insets would then be taken off twice and
        // leave empty bands. Edge to edge (the content fills the window) keeps them all.
        android.view.View content = getWindow().findViewById(android.R.id.content);
        if (content != null && content.getWidth() > 0 && content.getHeight() > 0) {
            int[] at = new int[2];
            content.getLocationInWindow(at);
            int[] decorAt = new int[2];
            view.getLocationInWindow(decorAt);
            int left = at[0] - decorAt[0];
            int top = at[1] - decorAt[1];
            int right = view.getWidth() - (left + content.getWidth());
            int bottom = view.getHeight() - (top + content.getHeight());
            l = Math.max(0, l - left);
            t = Math.max(0, t - top);
            r = Math.max(0, r - Math.max(0, right));
            b = Math.max(0, b - Math.max(0, bottom));
        }
        return new int[] {l, t, r, b};
    }

    /**
     * Tell the user that the app could not start (the graphics could not be set up), instead of
     * closing without a word. The next start tries another graphics backend by itself.
     */
    public void showError(final String message) {
        runOnUiThread(() -> {
            try {
                new android.app.AlertDialog.Builder(this)
                        .setTitle(getApplicationInfo().loadLabel(getPackageManager()))
                        .setMessage(message + "\n\nOpen the app again: it will try another way to draw.")
                        .setPositiveButton(android.R.string.ok, (d, w) -> finish())
                        .setCancelable(false)
                        .show();
            } catch (Exception e) {
                Log.w(TAG, "couldn't show the error", e);
            }
        });
    }

    /** Bar icons that stay readable over the app's theme (Android 11+; older keeps the default). */
    public void setDarkBars(final boolean dark) {
        if (android.os.Build.VERSION.SDK_INT < 30) return;
        runOnUiThread(() -> {
            android.view.WindowInsetsController c = getWindow().getInsetsController();
            if (c == null) return;
            int light = android.view.WindowInsetsController.APPEARANCE_LIGHT_STATUS_BARS
                    | android.view.WindowInsetsController.APPEARANCE_LIGHT_NAVIGATION_BARS;
            c.setSystemBarsAppearance(dark ? 0 : light, light);
        });
    }

    // ---- Installing apps and updates --------------------------------------------------------

    /**
     * Queue an APK (already verified by Rust: signed checksum list, SHA-256) for installation
     * through PackageInstaller. Returns 0 when queued, 1 when Android first needs the user to
     * allow this app to install apps (the settings page is opened), 2 on error.
     */
    public int installApk(final String path) {
        if (android.os.Build.VERSION.SDK_INT >= 26 && !getPackageManager().canRequestPackageInstalls()) {
            runOnUiThread(() -> {
                try {
                    startActivity(new Intent(android.provider.Settings.ACTION_MANAGE_UNKNOWN_APP_SOURCES,
                            Uri.parse("package:" + getPackageName())));
                } catch (Exception e) {
                    Log.w(TAG, "no install-permission settings page", e);
                }
            });
            return 1;
        }
        // Only files this app downloaded itself, under its own storage.
        try {
            String own = getFilesDir().getParentFile().getCanonicalPath() + File.separator;
            String real = new File(path).getCanonicalPath();
            if (!real.startsWith(own) || !real.endsWith(".apk")) return 2;
        } catch (java.io.IOException e) {
            return 2;
        }
        runOnUiThread(() -> {
            installQueue.add(path);
            if (installing == null) installNext();
        });
        return 0;
    }

    private void installNext() {
        String path = installQueue.poll();
        if (path == null) return;
        installing = path;
        try {
            PackageInstaller pi = getPackageManager().getPackageInstaller();
            PackageInstaller.SessionParams params =
                    new PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL);
            File apk = new File(path);
            int id = pi.createSession(params);
            try (PackageInstaller.Session session = pi.openSession(id)) {
                try (InputStream in = new FileInputStream(apk);
                     OutputStream out = session.openWrite("mahay.apk", 0, apk.length())) {
                    copy(in, out);
                    session.fsync(out);
                }
                Intent cb = new Intent(INSTALL_ACTION).setPackage(getPackageName());
                int flags = PendingIntent.FLAG_UPDATE_CURRENT
                        | (android.os.Build.VERSION.SDK_INT >= 31 ? PendingIntent.FLAG_MUTABLE : 0);
                session.commit(PendingIntent.getBroadcast(this, id, cb, flags).getIntentSender());
            }
        } catch (Exception e) {
            Log.w(TAG, "install failed: " + path, e);
            installing = null;
            nativeOnInstalled(path, false, String.valueOf(e.getMessage()));
            installNext();
        }
    }

    /** The installed version of a package ("0.2.0"), or null when it isn't installed. */
    public String installedVersion(String pkg) {
        try {
            PackageInfo info = getPackageManager().getPackageInfo(pkg, 0);
            return info.versionName;
        } catch (Exception e) {
            return null;
        }
    }

    public String localeTag() {
        return java.util.Locale.getDefault().toLanguageTag();
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

    /**
     * Only content:// URIs are accepted. The activity is exported, so another app could send a
     * file:// URI pointing into this app's private storage; copying it out, or writing edits
     * back to it, must not be possible.
     */
    private static boolean isContentUri(Uri uri) {
        return uri != null && ContentResolver.SCHEME_CONTENT.equals(uri.getScheme());
    }

    /** Copy uri into dir under its display name: {path, name, uri if writable else ""}. */
    private String[] copyIn(Uri uri, String dir, boolean writable) {
        if (!isContentUri(uri)) {
            Log.w(TAG, "ignored a non-content URI");
            return null;
        }
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
