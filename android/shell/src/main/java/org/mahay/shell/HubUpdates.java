// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.PendingIntent;
import android.app.job.JobInfo;
import android.app.job.JobScheduler;
import android.content.ComponentName;
import android.content.Context;
import android.content.Intent;
import android.content.SharedPreferences;
import android.net.ConnectivityManager;
import android.net.Network;
import android.net.NetworkCapabilities;
import android.util.Log;

import java.util.HashSet;
import java.util.Set;

/**
 * What Mahay HUB does for the other apps when no screen is open (decision D21): the periodic and
 * requested update jobs, the install of what Rust downloaded and verified, and the "N updates
 * ready" notification. Used only in the hub flavour, whose manifest declares the components.
 */
final class HubUpdates {
    private static final String TAG = "MahayHub";
    static final int JOB_PERIODIC = 0x4d10;
    static final int JOB_REQUEST = 0x4d11;
    static final String RESULT_ACTION = "org.mahay.hub.INSTALL_RESULT";
    private static final String PREFS = "mahay_managed";
    private static final String REQUESTED = "requested";
    private static final String WAITING = "waiting";
    private static final String CHANNEL = "updates";
    private static final int NOTIFICATION = 0x4d20;

    private HubUpdates() {
    }

    // ---- Scheduling -------------------------------------------------------------------------

    /**
     * (Re)schedule the daily background update, kept across reboots, or cancel it. Unmetered
     * networks only unless {@code mobile}: the user pays for mobile data by the megabyte.
     */
    static void schedule(Context ctx, boolean auto, boolean mobile) {
        JobScheduler js = (JobScheduler) ctx.getSystemService(Context.JOB_SCHEDULER_SERVICE);
        if (js == null) return;
        js.cancel(JOB_PERIODIC);
        if (!auto) return;
        JobInfo job = new JobInfo.Builder(JOB_PERIODIC, new ComponentName(ctx, UpdateJobService.class))
                .setRequiredNetworkType(mobile ? JobInfo.NETWORK_TYPE_ANY : JobInfo.NETWORK_TYPE_UNMETERED)
                .setRequiresBatteryNotLow(true)
                .setPeriodic(24L * 3600 * 1000)
                .setPersisted(true)
                .build();
        if (js.schedule(job) != JobScheduler.RESULT_SUCCESS) Log.w(TAG, "couldn't schedule the daily update");
    }

    /**
     * An app asked to be updated: remember which (only a package name, checked by Rust against the
     * suite's apps when the job runs) and run the job now.
     */
    static void request(Context ctx, String packageName) {
        SharedPreferences p = ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE);
        synchronized (HubUpdates.class) {
            Set<String> set = new HashSet<>(p.getStringSet(REQUESTED, new HashSet<>()));
            set.add(packageName);
            p.edit().putStringSet(REQUESTED, set).apply();
        }
        JobScheduler js = (JobScheduler) ctx.getSystemService(Context.JOB_SCHEDULER_SERVICE);
        if (js == null) return;
        JobInfo.Builder b = new JobInfo.Builder(JOB_REQUEST, new ComponentName(ctx, UpdateJobService.class))
                .setRequiredNetworkType(JobInfo.NETWORK_TYPE_ANY);
        if (android.os.Build.VERSION.SDK_INT >= 31) b.setExpedited(true);
        if (js.schedule(b.build()) != JobScheduler.RESULT_SUCCESS) Log.w(TAG, "couldn't schedule the requested update");
    }

    // ---- Apps in use ------------------------------------------------------------------------

    private static final long FOREGROUND_MAX_MS = 6L * 3600 * 1000;

    /**
     * An app tells the hub it is on screen (or not). Android replaces an open app without asking,
     * which could lose unsaved work, so background updates skip an app that says it is open. The
     * mark expires after six hours in case the app was killed before it could say it left.
     */
    static void setForeground(Context ctx, String packageName, boolean foreground) {
        SharedPreferences.Editor e = ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit();
        if (foreground) e.putLong("fg_" + packageName, System.currentTimeMillis());
        else e.remove("fg_" + packageName);
        e.apply();
    }

    static boolean isForeground(Context ctx, String packageName) {
        long at = ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getLong("fg_" + packageName, 0);
        return at > 0 && System.currentTimeMillis() - at < FOREGROUND_MAX_MS;
    }

    /** The package names asked for since the last call. */
    static Set<String> takeRequested(Context ctx) {
        SharedPreferences p = ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE);
        synchronized (HubUpdates.class) {
            Set<String> set = new HashSet<>(p.getStringSet(REQUESTED, new HashSet<>()));
            p.edit().remove(REQUESTED).apply();
            return set;
        }
    }

    /** 0 no network, 1 unmetered (Wi-Fi), 2 metered (mobile data). */
    static int network(Context ctx) {
        ConnectivityManager cm = (ConnectivityManager) ctx.getSystemService(Context.CONNECTIVITY_SERVICE);
        if (cm == null) return 0;
        Network n = cm.getActiveNetwork();
        NetworkCapabilities nc = n == null ? null : cm.getNetworkCapabilities(n);
        if (nc == null || !nc.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)) return 0;
        return cm.isActiveNetworkMetered() ? 2 : 1;
    }

    // ---- Installing what Rust verified ------------------------------------------------------

    /**
     * Hand one verified download to Android's installer, with no screen involved. Where Android
     * needs the user's tap (the app wasn't installed by the hub, or Android is older than 12) the
     * result receiver records it and the notification says so.
     */
    static boolean install(Context ctx, String id, String path, String sha256, boolean explicit) {
        if (!ApkInstaller.isOwnApk(ctx, path)) return false;
        try {
            Intent cb = new Intent(RESULT_ACTION).setClass(ctx, HubInstallReceiver.class)
                    .putExtra("app", id).putExtra("explicit", explicit);
            ApkInstaller.commit(ctx, path, sha256, cb);
            return true;
        } catch (Exception e) {
            Log.w(TAG, "install of " + id + " failed", e);
            return false;
        }
    }

    // ---- "N updates ready" ------------------------------------------------------------------

    static synchronized void setWaiting(Context ctx, String id, boolean waiting) {
        SharedPreferences p = ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE);
        Set<String> set = new HashSet<>(p.getStringSet(WAITING, new HashSet<>()));
        if (waiting) set.add(id); else set.remove(id);
        p.edit().putStringSet(WAITING, set).apply();
    }

    static int waitingCount(Context ctx) {
        return ctx.getSharedPreferences(PREFS, Context.MODE_PRIVATE).getStringSet(WAITING, new HashSet<>()).size();
    }

    /** Show, update or remove the notification to match the number of updates waiting for a tap. */
    static void notifyWaiting(Context ctx) {
        NotificationManager nm = (NotificationManager) ctx.getSystemService(Context.NOTIFICATION_SERVICE);
        if (nm == null) return;
        int n = waitingCount(ctx);
        if (n == 0) {
            nm.cancel(NOTIFICATION);
            return;
        }
        if (!nm.areNotificationsEnabled()) return;
        nm.createNotificationChannel(new NotificationChannel(CHANNEL,
                ctx.getString(R.string.updates_channel), NotificationManager.IMPORTANCE_LOW));
        Intent open = ctx.getPackageManager().getLaunchIntentForPackage(ctx.getPackageName());
        PendingIntent tap = open == null ? null
                : PendingIntent.getActivity(ctx, 0, open, PendingIntent.FLAG_IMMUTABLE | PendingIntent.FLAG_UPDATE_CURRENT);
        Notification note = new Notification.Builder(ctx, CHANNEL)
                .setSmallIcon(ctx.getApplicationInfo().icon)
                .setContentTitle(ctx.getResources().getQuantityString(R.plurals.updates_ready, n, n))
                .setContentText(ctx.getString(R.string.updates_ready_tap))
                .setContentIntent(tap)
                .setAutoCancel(true)
                .build();
        nm.notify(NOTIFICATION, note);
    }
}
