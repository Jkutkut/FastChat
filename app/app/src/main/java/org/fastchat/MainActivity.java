package org.fastchat;

import androidx.appcompat.app.AppCompatActivity;

import android.content.Intent;
import android.os.Bundle;
import android.widget.Button;

import org.fastchat.screens.ChatActivity;

public class MainActivity extends AppCompatActivity {

    // Declare elements here
    Button btnGoToChatActivity;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        btnGoToChatActivity = findViewById(R.id.btnGoToChatActivity);

        btnGoToChatActivity.setOnClickListener(v -> {

            String ip = "192.168.1.135"; // IP WebSocket server
            String port = "4242"; // Port WebSocket server
            String endpoint = "test"; // Path WebSocket server

            Intent intent = new Intent(MainActivity.this, ChatActivity.class);
            intent.putExtra("ip", ip);
            intent.putExtra("port", port);
            intent.putExtra("endpoint", endpoint);

            startActivity(intent);
        });

    }
}