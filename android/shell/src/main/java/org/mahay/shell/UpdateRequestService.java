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
    /** An app says whether it is on screen, so background updates leave it alone. */
    static final String ACTION_STATE = "org.mahay.hub.APP_STATE";
    static final String EXTRA_PACKAGE = "package";
    static final String EXTRA_FOREGROUND = "foreground";

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        String action = intent == null ? null : intent.getAction();
        String pkg = intent == null ? null : intent.getStringExtra(EXTRA_PACKAGE);
        // Cheap sanity check here; the real check (an exact match against the suite) is Rust's.
        boolean valid = pkg != null && pkg.length() <= 64 && pkg.matches("[a-z][a-z0-9_.]*");
        if (valid && ACTION.equals(action)) {
            HubUpdates.request(this, pkg);
        } else if (valid && ACTION_STATE.equals(action)) {
            HubUpdates.setForeground(this, pkg, intent.getBooleanExtra(EXTRA_FOREGROUND, false));
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
