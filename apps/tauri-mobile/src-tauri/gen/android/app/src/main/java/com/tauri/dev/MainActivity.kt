package com.tauri.dev

import android.os.Bundle
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat

class MainActivity : TauriActivity() {
override fun onCreate(savedInstanceState: Bundle?) {
super.onCreate(savedInstanceState)

    // Allow content to extend under the system bars
    WindowCompat.setDecorFitsSystemWindows(window, false)


    // Get the insets controller to manage system UI
    val windowInsetsController = WindowCompat.getInsetsController(window, window.decorView)

    // Ensure status bar is fully hidden
    windowInsetsController?.let {
        // Hide the status bar completely (including icons and text)
        it.hide(WindowInsetsCompat.Type.statusBars())

        // Ensure the system bars are hidden until the user swipes to bring them back
        it.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
    }


    // Ensure the status bar is fully transparent
    window.statusBarColor = android.graphics.Color.TRANSPARENT
}
}