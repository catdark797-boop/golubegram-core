package com.golubegram.core.ui

import androidx.compose.animation.animateColorAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

object NightCity {
    val Background = Color(0xFF0D0D14)
    val NeonCyan = Color(0xFF00E5E5)
    val NeonMagenta = Color(0xFFFF00CC)
    val AlertRed = Color(0xFFFF3333)
}

// Wrapper for Rust JNI
object NativeBridge {
    init {
        System.loadLibrary("golubegram_core")
    }
    external fun initCore()
    // Passes raw X/Y and timestamp to Rust for Matryoshka hash calculation
    external fun reportRawTouch(x: Float, y: Float, timestamp: Long): Boolean
}

@Composable
fun MainMeshScreen() {
    var isConnected by remember { mutableStateOf(false) }
    var peerCount by remember { mutableStateOf(0) }
    var showBabushkaMode by remember { mutableStateOf(false) }

    val statusColor by animateColorAsState(
        targetValue = if (isConnected) NightCity.NeonCyan else NightCity.AlertRed
    )

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(NightCity.Background)
            .padding(24.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.SpaceEvenly
    ) {
        // App Title
        Text(
            text = "GOLUBEGRAM",
            color = NightCity.NeonCyan,
            fontSize = 32.sp,
            fontFamily = FontFamily.Monospace,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.shadow(10.dp, ambientColor = NightCity.NeonCyan, spotColor = NightCity.NeonCyan)
        )

        // Radar / Status
        Box(
            modifier = Modifier
                .size(220.dp)
                .border(4.dp, statusColor, CircleShape)
                .background(statusColor.copy(alpha = 0.1f), CircleShape)
                .shadow(25.dp, ambientColor = statusColor, spotColor = statusColor),
            contentAlignment = Alignment.Center
        ) {
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                Text(
                    text = if (isConnected) "SYSTEM ONLINE" else "ISOLATED",
                    color = Color.White,
                    fontWeight = FontWeight.Bold,
                    fontSize = 18.sp
                )
                if (isConnected) {
                    Text(
                        text = "PEERS: $peerCount",
                        color = NightCity.NeonCyan,
                        fontSize = 14.sp
                    )
                }
            }
        }

        // Action Button (One-Click)
        Button(
            onClick = {
                if (!isConnected) {
                    try {
                        // NativeBridge.initCore() // Uncomment in real build
                    } catch (e: Exception) {
                        // Hide all technical errors behind "Searching"
                    }
                }
                isConnected = !isConnected
                peerCount = if (isConnected) (1..50).random() else 0
            },
            colors = ButtonDefaults.buttonColors(containerColor = Color.Transparent),
            modifier = Modifier
                .fillMaxWidth()
                .border(2.dp, NightCity.NeonMagenta, RoundedCornerShape(8.dp))
                .shadow(15.dp, ambientColor = NightCity.NeonMagenta, spotColor = NightCity.NeonMagenta)
        ) {
            Text(
                text = if (isConnected) "SEARCHING FOR PEERS..." else "ESTABLISH LINK",
                color = NightCity.NeonMagenta,
                fontWeight = FontWeight.Bold,
                modifier = Modifier.padding(8.dp)
            )
        }
        
        Spacer(modifier = Modifier.height(20.dp))
        
        TextButton(onClick = { showBabushkaMode = true }) {
            Text("БАБУШКА РЕЖИМ (Комикс-Гайд)", color = Color.Gray)
        }
    }

    if (showBabushkaMode) {
        // Comic Guide Overlay
        Box(modifier = Modifier.fillMaxSize().background(Color.Black.copy(alpha=0.9f))) {
            Column(modifier = Modifier.align(Alignment.Center), horizontalAlignment = Alignment.CenterHorizontally) {
                Text("КОМИКС-ГАЙД", color = Color.Yellow, fontSize = 24.sp, fontWeight = FontWeight.Bold)
                Spacer(modifier = Modifier.height(16.dp))
                Text("1. Нажми сюда.\n2. Установи MDM.\n3. Подтверди Swarm Root CA.", color = Color.White)
                Spacer(modifier = Modifier.height(16.dp))
                Button(onClick = { showBabushkaMode = false }) { Text("ПОНЯТНО") }
            }
        }
    }
}
