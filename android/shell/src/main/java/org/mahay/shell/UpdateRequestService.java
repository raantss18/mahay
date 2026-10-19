// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.app.Service;
import android.content.Intent;
import android.os.IBinder;
import android.util.Log;

/**
 * Mahay apps ask Mahay HUB to update them here (decision D21). The manifest requires the
 * signature permission {@code org.mahay.permission.MANAGE}, so only apps signed with Mahay's
 * certificate can start it. The only input is the requester's package name, and it is not
 * trusted: the update job checks it against the suite's apps before doing anything. It is never
 * a path or a URI. The service only records the request and schedules the job, then stops.
 */
public class UpdateRequestService extends Service {
    static final String ACTION = "org.mahay.hub.UPDATE_APP";
    static final String EXTRA_PACKAGE = "package";

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        String pkg = intent == null || !ACTION.equals(intent.getAction()) ? null : intent.getStringExtra(EXTRA_PACKAGE);
        // Cheap sanity check here; the real check (an exact match against the suite) is Rust's.
        if (pkg != null && pkg.length() <= 64 && pkg.matches("[a-z][a-z0-9_.]*")) {
            HubUpdates.request(this, pkg);
        } else {
            Log.w("MahayHub", "ignored an update request without a valid package name");
        }
        stopSelf(startId);
        return START_NOT_STICKY;
    }

    @Override
    public IBinder onBind(Intent intent) {
        return null;
    }
}
