package org.fastchat.screens;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.os.StrictMode;
import android.webkit.WebStorage;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

import com.neovisionaries.ws.client.HostnameUnverifiedException;
import com.neovisionaries.ws.client.OpeningHandshakeException;
import com.neovisionaries.ws.client.WebSocket;
import com.neovisionaries.ws.client.WebSocketAdapter;
import com.neovisionaries.ws.client.WebSocketException;
import com.neovisionaries.ws.client.WebSocketFactory;

import org.fastchat.R;

import java.io.IOException;

public class ChatActivity extends AppCompatActivity {

    // Declare UIX Elements
    TextView tvChat;
    EditText etInputMessage;
    EditText getEtInputIp;
    Button btnSendMsg;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_chat);

        StrictMode.ThreadPolicy policy = new StrictMode.ThreadPolicy.Builder().permitAll().build();
        StrictMode.setThreadPolicy(policy);

        // Get data from MainActivity
        String ip = getIntent().getStringExtra("ip");
        String port = getIntent().getStringExtra("port");
        String endpoint = getIntent().getStringExtra("endpoint");

        // Create a WebSocket
        WebSocket ws;
        try {
            ws = new WebSocketFactory().createSocket("ws://" + ip + ":" + port + "/" + endpoint);
        } catch (IOException e) {
            System.out.println("Error: " + e.getMessage());
            throw new RuntimeException(e);
        }

        // Create a listener
        ws.addListener(new WebSocketAdapter() {
            @Override
            public void onTextMessage(WebSocket websocket, String message) throws Exception{
                System.out.println("Message: " + message);
                tvChat.setText(message);
            }
        });

        try {
            ws.connect();
        } catch (OpeningHandshakeException | HostnameUnverifiedException e) {
            System.out.println("Error: " + e.getMessage());
        } catch (WebSocketException e) {
            System.out.println("Error: " + e.getMessage());
        }

        // Declare elements
        tvChat = findViewById(R.id.tvChat);
        etInputMessage = findViewById(R.id.etInputMessage);
        btnSendMsg = findViewById(R.id.btnSendMsg);

        btnSendMsg.setOnClickListener(v -> {
            String message = etInputMessage.getText().toString();
            ws.sendText(message);
        });

    }

}