package com.plugin.keepScreenOn

import android.app.Activity
import android.view.WindowManager
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class KeepScreenOnPlugin(private val activity: Activity): Plugin(activity) {

    @Command
    fun keepScreenOn(invoke: Invoke) {
        
        activity.getWindow().addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        val ret = JSObject()
        invoke.resolve(ret)
    }
}
