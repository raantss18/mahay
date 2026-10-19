// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.app.PendingIntent;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageInstaller;

import java.io.File;
import java.io.FileInputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.security.MessageDigest;

/**
 * One APK into one PackageInstaller session. Shared by the activity (an install the user asked
 * for) and the hub's background updates, which run with no activity.
 */
final class ApkInstaller {
    private ApkInstaller() {
    }

    /**
     * Copy {@code apk} into a new install session and commit it; the result goes to
     * {@code callback} (an explicit intent, completed here with the session id as request code).
     * Android 12+ then skips its confirmation when this app is the installer of record of the
     * package and the app isn't in use; otherwise it answers STATUS_PENDING_USER_ACTION.
     *
     * <p>When {@code sha256} is given, the bytes written to the session are hashed on the way and
     * the session is abandoned unless they match: what Rust verified is what gets installed,
     * with no gap in which the file could be swapped.
     */
    static int commit(Context ctx, String path, String sha256, Intent callback) throws Exception {
        PackageInstaller pi = ctx.getPackageManager().getPackageInstaller();
        PackageInstaller.SessionParams params =
                new PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL);
        if (android.os.Build.VERSION.SDK_INT >= 31) {
            params.setRequireUserAction(PackageInstaller.SessionParams.USER_ACTION_NOT_REQUIRED);
        }
        File apk = new File(path);
        int id = pi.createSession(params);
        try (PackageInstaller.Session session = pi.openSession(id)) {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            try (InputStream in = new FileInputStream(apk);
                 OutputStream out = session.openWrite("mahay.apk", 0, apk.length())) {
                byte[] buf = new byte[64 * 1024];
                int n;
                while ((n = in.read(buf)) > 0) {
                    digest.update(buf, 0, n);
                    out.write(buf, 0, n);
                }
                session.fsync(out);
            }
            if (sha256 != null && !sha256.equalsIgnoreCase(hex(digest.digest()))) {
                session.abandon();
                throw new java.io.IOException("the file changed after it was checked");
            }
            int flags = PendingIntent.FLAG_UPDATE_CURRENT
                    | (android.os.Build.VERSION.SDK_INT >= 31 ? PendingIntent.FLAG_MUTABLE : 0);
            session.commit(PendingIntent.getBroadcast(ctx, id, callback, flags).getIntentSender());
        } catch (Exception e) {
            try {
                pi.abandonSession(id);
            } catch (Exception ignored) {
                // Already abandoned or committed.
            }
            throw e;
        }
        return id;
    }

    /** Whether {@code path} is an APK inside this app's own storage (the only place it may come from). */
    static boolean isOwnApk(Context ctx, String path) {
        try {
            String own = ctx.getFilesDir().getParentFile().getCanonicalPath() + File.separator;
            String real = new File(path).getCanonicalPath();
            return real.startsWith(own) && real.endsWith(".apk");
        } catch (java.io.IOException e) {
            return false;
        }
    }

    private static String hex(byte[] bytes) {
        StringBuilder sb = new StringBuilder(bytes.length * 2);
        for (byte b : bytes) sb.append(String.format("%02x", b));
        return sb.toString();
    }
}
