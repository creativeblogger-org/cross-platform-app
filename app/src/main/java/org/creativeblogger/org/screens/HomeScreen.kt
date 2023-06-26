package org.creativeblogger.org.screens

import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.navigation.NavController
import org.creativeblogger.org.Screen

@Composable
fun HomeScreen(
    navController: NavController
) {
    Text(text = "Home screen")
    TextButton(onClick = {
        navController.navigate(Screen.ThingScreen.route)
    }) {
        Text(text = "Go to thing screen")
    }
}