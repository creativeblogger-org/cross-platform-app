package org.creativeblogger.org

sealed class Screen(val route: String) {
    object HomeScreen : Screen("home_screen")
    object ThingScreen : Screen("thing_screen")
}
