package org.creativeblogger.org

import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import org.creativeblogger.org.screens.HomeScreen

@Composable
fun AppNavHost() {
    val navController = rememberNavController()
    
    NavHost(navController = navController, startDestination = Screen.HomeScreen.route) {
        composable(Screen.HomeScreen.route) {
            HomeScreen(navController = navController);
        }
        composable(Screen.ThingScreen.route) {
            Text(text = "Thing screen")
            TextButton(onClick = {
                navController.popBackStack()
            }) {
                Text(text = "Go back")
            }
        }
    }
}