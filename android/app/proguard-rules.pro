# Called from Rust through JNI by name.
-keep class org.mahay.shell.MahayActivity { *; }
-keep class com.google.androidgamesdk.** { *; }
# Mahay HUB's update job calls into the hub's native library.
-keepclasseswithmembernames class org.mahay.shell.** { native <methods>; }
