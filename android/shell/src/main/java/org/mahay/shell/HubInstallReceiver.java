// SPDX-License-Identifier: MIT OR Apache-2.0
package org.mahay.shell;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageInstaller;
import android.util.Log;

/** The answer of Android's installer to an update the hub started without a screen. Not exported. */
public class HubInstallReceiver extends BroadcastReceiver {
    @Override
    public void onReceive(Context context, Intent intent) {
        int status = intent.getIntExtra(PackageInstaller.EXTRA_STATUS, PackageInstaller.STATUS_FAILURE);
        String app = intent.getStringExtra("app");
        if (app == null) return;
        if (status == PackageInstaller.STATUS_PENDING_USER_ACTION) {
            // Android needs a tap (the app wasn't installed by the hub, it is open, or this is
            // Android 11 or older). Say so in the notification. An app that asked for the update
            // just now also gets Android's dialog, if Android lets the hub show it from here.
            HubUpdates.setWaiting(context, app, true);
            if (intent.getBooleanExtra("explicit", false)) {
                Intent confirm = intent.getParcelableExtra(Intent.EXTRA_INTENT);
                if (confirm != null) {
                    try {
                        context.startActivity(confirm.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK));
                    } catch (Exception e) {
                        Log.i("MahayHub", "the confirmation can't be shown from the background");
                    }
                }
            } else {
                int session = intent.getIntExtra(PackageInstaller.EXTRA_SESSION_ID, -1);
                try {
                    // The download is kept; "Update" in the hub installs it with the user's tap.
                    if (session >= 0) context.getPackageManager().getPackageInstaller().abandonSession(session);
                } catch (Exception ignored) {
                    // Already gone.
                }
            }
        } else {
            HubUpdates.setWaiting(context, app, false);
            if (status != PackageInstaller.STATUS_SUCCESS) {
                Log.w("MahayHub", "update of " + app + " ended with status " + status + ": "
                        + intent.getStringExtra(PackageInstaller.EXTRA_STATUS_MESSAGE));
            }
        }
        HubUpdates.notifyWaiting(context);
    }
}
