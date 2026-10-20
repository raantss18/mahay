// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.content.ClipData;
import android.content.ClipboardManager;
import android.content.ContentResolver;
import android.content.Context;
import android.app.PendingIntent;
import android.content.BroadcastReceiver;
import android.content.Intent;
import android.content.ComponentName;
import android.content.IntentFilter;
import android.content.pm.PackageManager;
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
    private static native void nativeOnResumed();
    private static native void nativeOnToolType(int pointerId, int toolType);

    private static final String INSTALL_ACTION = "org.mahay.shell.INSTALL_STATUS";
    /** APKs waiting to be installed, one confirmation at a time. */
    private final ArrayDeque<String> installQueue = new ArrayDeque<>();
    private String installing;

    /** Whether this activity is in the foreground: Android only lets a foreground app open the installer's dialog. */
    private boolean resumed;
    /** The installer's confirmation dialog, waiting for this activity to be in the foreground. */
    private Intent pendingConfirm;

    private final BroadcastReceiver installReceiver = new BroadcastReceiver() {
        @Override
        public void onReceive(Context context, Intent intent) {
            int status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE);
            if (status == PackageInstaller.STATUS_PENDING_USER_ACTION) {
                // Android asks the user to confirm: show its dialog, now or as soon as the hub is in
                // front. Opening it from the background is silently refused (Android 10+, and
                // stricter on some makers' systems): that left "Install all" stuck after the first app.
                Intent confirm = intent.getParcelableExtra(Intent.EXTRA_INTENT);
                if (confirm != null) {
                    confirm.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
                    pendingConfirm = confirm;
                    showPendingConfirm();
                }
                return;
            }
            pendingConfirm = null;
            String path = installing;
            installing = null;
            String msg = intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE);
            if (path != null) nativeOnInstalled(path, status == PackageInstaller.STATUS_SUCCESS, msg == null ? "" : msg);
            // The next install is committed only while the hub is in front (its dialog needs that);
            // otherwise onResume continues the queue.
            if (resumed) installNext();
        }
    };

    /** Open the installer's confirmation dialog if the hub is in front; otherwise onResume does. */
    private void showPendingConfirm() {
        Intent confirm = pendingConfirm;
        if (confirm == null || !resumed) return;
        pendingConfirm = null;
        try {
            startActivity(confirm);
        } catch (Exception e) {
            Log.w(TAG, "the installer's dialog could not be opened", e);
            pendingConfirm = confirm;
        }
    }

    /**
     * The user asked to go on (a tap on the card that says "Confirm the installation"): show the
     * dialog again, or start the install again when Android no longer has a session for it.
     * Returns whether there was something to continue.
     */
    public boolean retryInstall() {
        runOnUiThread(() -> {
            if (pendingConfirm != null) {
                showPendingConfirm();
            } else if (installing != null) {
                try {
                    if (getPackageManager().getPackageInstaller().getMySessions().isEmpty()) {
                        installQueue.addFirst(installing);
                        installing = null;
                        installNext();
                    }
                } catch (Exception e) {
                    Log.w(TAG, "could not check the install sessions", e);
                }
            } else {
                installNext();
            }
        });
        return installing != null || pendingConfirm != null || !installQueue.isEmpty();
    }

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
        if (android.os.Build.VERSION.SDK_INT >= 30) {
            android.graphics.Insets bars = wi.getInsets(android.view.WindowInsets.Type.systemBars()
                    | android.view.WindowInsets.Type.displayCutout());
            android.graphics.Insets ime = wi.getInsets(android.view.WindowInsets.Type.ime());
            return new int[] {bars.left, bars.top, bars.right, Math.max(bars.bottom, ime.bottom)};
        }
        return new int[] {wi.getSystemWindowInsetLeft(), wi.getSystemWindowInsetTop(),
                wi.getSystemWindowInsetRight(), wi.getSystemWindowInsetBottom()};
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
        if (!ApkInstaller.isOwnApk(this, path)) return 2;
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
            ApkInstaller.commit(this, path, null, new Intent(INSTALL_ACTION).setPackage(getPackageName()));
        } catch (Exception e) {
            Log.w(TAG, "install failed: " + path, e);
            installing = null;
            nativeOnInstalled(path, false, String.valueOf(e.getMessage()));
            installNext();
        }
    }

    // ---- Mahay HUB and the apps it manages (decision D21) -----------------------------------

    private static final String HUB = "org.mahay.hub";
    private static final int REQ_NOTIFICATIONS = 0x4d03;

    /**
     * Ask Mahay HUB to update this app. 0: asked; 1: the hub isn't installed; 2: the hub isn't
     * signed with the same certificate as this app; 3: the hub didn't take the request; 4: this
     * app doesn't hold the hub's permission (it was installed before the hub). Any other answer
     * than 0 leaves the app to update itself.
     */
    public int handOffToHub() {
        PackageManager pm = getPackageManager();
        try {
            pm.getPackageInfo(HUB, 0);
        } catch (PackageManager.NameNotFoundException e) {
            return 1;
        }
        if (!hubSignedLikeUs(pm)) return 2;
        Intent i = new Intent("org.mahay.hub.UPDATE_APP")
                .setComponent(new ComponentName(HUB, "org.mahay.shell.UpdateRequestService"))
                .putExtra("package", getPackageName());
        try {
            return startService(i) == null ? 3 : 0;
        } catch (SecurityException e) {
            return 4;
        } catch (IllegalStateException e) {
            return 3;
        }
    }

    /** Whether Mahay HUB is installed and signed with the certificate this app is signed with. */
    public boolean hubUsable() {
        PackageManager pm = getPackageManager();
        try {
            pm.getPackageInfo(HUB, 0);
        } catch (PackageManager.NameNotFoundException e) {
            return false;
        }
        return hubSignedLikeUs(pm);
    }

    private boolean hubSignedLikeUs(PackageManager pm) {
        try {
            if (android.os.Build.VERSION.SDK_INT >= 28) {
                PackageInfo mine = pm.getPackageInfo(getPackageName(), PackageManager.GET_SIGNING_CERTIFICATES);
                if (mine.signingInfo == null || mine.signingInfo.hasMultipleSigners()) return false;
                android.content.pm.Signature[] signers = mine.signingInfo.getApkContentsSigners();
                if (signers == null || signers.length != 1) return false;
                return pm.hasSigningCertificate(HUB, signers[0].toByteArray(), PackageManager.CERT_INPUT_RAW_X509);
            }
            return pm.checkSignatures(getPackageName(), HUB) == PackageManager.SIGNATURE_MATCH;
        } catch (Exception e) {
            return false;
        }
    }

    /** Hub: whether an app says it is on screen (background updates skip it). */
    public boolean isAppForeground(String pkg) {
        return HubUpdates.isForeground(this, pkg);
    }

    /** Tell Mahay HUB, when it manages this app, whether this app is on screen. */
    private void tellHub(boolean foreground) {
        if (HUB.equals(getPackageName())) return;
        try {
            if (!hubUsable()) return;
            startService(new Intent("org.mahay.hub.APP_STATE")
                    .setComponent(new ComponentName(HUB, "org.mahay.shell.UpdateRequestService"))
                    .putExtra("package", getPackageName())
                    .putExtra("foreground", foreground));
        } catch (Exception e) {
            // No hub, no permission, or Android refused: nothing to do.
        }
    }

    @Override
    protected void onResume() {
        super.onResume();
        resumed = true;
        // The app's frames stop while another window (Android's installer) is in front, and nothing
        // draws one again by itself: wake the app so its queues move on.
        nativeOnResumed();
        tellHub(true);
        showPendingConfirm();
        if (pendingConfirm == null && installing == null) installNext();
    }

    @Override
    protected void onPause() {
        resumed = false;
        tellHub(false);
        super.onPause();
    }

    /** Hub: (re)schedule the daily background update, or cancel it. */
    public void scheduleUpdates(boolean auto, boolean mobile) {
        HubUpdates.schedule(this, auto, mobile);
    }

    /** Hub: 0 no network, 1 unmetered, 2 metered. */
    public int networkState() {
        return HubUpdates.network(this);
    }

    /** Hub: install a download Rust verified, with no dialog of ours; Android asks only if it must. */
    public boolean installBackground(String id, String path, String sha256) {
        return HubUpdates.install(this, id, path, sha256, false);
    }

    /** Hub: how many updates are waiting for the user's tap. */
    public int waitingUpdates() {
        return HubUpdates.waitingCount(this);
    }

    /** Hub: whether the "updates ready" notification can be shown. */
    public boolean notificationsAllowed() {
        android.app.NotificationManager nm = (android.app.NotificationManager) getSystemService(Context.NOTIFICATION_SERVICE);
        return nm != null && nm.areNotificationsEnabled();
    }

    /** Hub: ask Android 13+ for the notification permission (only after the user chose to). */
    public void requestNotifications() {
        if (android.os.Build.VERSION.SDK_INT >= 33) {
            runOnUiThread(() -> requestPermissions(new String[] {"android.permission.POST_NOTIFICATIONS"}, REQ_NOTIFICATIONS));
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
