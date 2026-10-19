// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.app.job.JobParameters;
import android.app.job.JobService;
import android.content.pm.PackageInfo;
import android.util.Log;

import java.util.ArrayList;
import java.util.List;
import java.util.Set;

/**
 * Updates the other Mahay apps with no screen open: the daily job and the jobs started by
 * {@link UpdateRequestService}. Downloading and verifying is the hub's native library (the same
 * signed-checksum code the app uses); this class gathers the installed versions, runs it and
 * hands what it verified to Android's installer.
 */
public class UpdateJobService extends JobService {
    private static final String TAG = "MahayHub";

    private static native String[] nativeApps();

    /** First element: status ("ok", "skip:<reason>", "refused", "error:<key>"); then "id\tversion\tsha256\tpath". */
    private static native String[] nativeRun(String dataDir, String[] installed, String only, int network, boolean explicit);

    @Override
    public boolean onStartJob(final JobParameters params) {
        final boolean requested = params.getJobId() == HubUpdates.JOB_REQUEST;
        new Thread(() -> {
            try {
                run(requested);
            } catch (Throwable t) {
                Log.w(TAG, "update job failed", t);
            } finally {
                jobFinished(params, false);
            }
        }, "mahay-update-job").start();
        return true;
    }

    @Override
    public boolean onStopJob(JobParameters params) {
        return false;
    }

    private void run(boolean requested) {
        // The native library of the hub: the same file the screen runs.
        System.loadLibrary("hub_android");
        if (requested) {
            Set<String> packages = HubUpdates.takeRequested(this);
            for (String pkg : packages) runOnce(pkg, true);
        } else {
            runOnce(null, false);
        }
    }

    private void runOnce(String only, boolean explicit) {
        List<String> installed = new ArrayList<>();
        for (String id : nativeApps()) {
            // An open app is left for later unless the user just asked for its update.
            if (!explicit && HubUpdates.isForeground(this, "org.mahay." + id)) continue;
            try {
                PackageInfo info = getPackageManager().getPackageInfo("org.mahay." + id, 0);
                if (info.versionName != null) installed.add(id + "=" + info.versionName);
            } catch (Exception notInstalled) {
                // Not installed: nothing to update.
            }
        }
        String[] res = nativeRun(getFilesDir().getPath(), installed.toArray(new String[0]), only,
                HubUpdates.network(this), explicit);
        if (res == null || res.length == 0) return;
        Log.i(TAG, "update run: " + res[0] + ", " + (res.length - 1) + " ready");
        for (int i = 1; i < res.length; i++) {
            String[] f = res[i].split("\t", 4);
            if (f.length == 4) HubUpdates.install(this, f[0], f[3], f[2], explicit);
        }
    }
}
