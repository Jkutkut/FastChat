package org.fastchat.screens;

import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

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
import org.fastchat.models.MessagesData;
import org.fastchat.utils.MessagesAdapter;
import org.json.JSONException;
import org.json.JSONObject;

import java.io.IOException;

public class ChatActivity extends AppCompatActivity {

    EditText etInputMessage;
    Button btnSendMsg;

    // Declare RecyclerView
    RecyclerView rvMessages;
    MessagesAdapter messagesAdapter;
    RecyclerView.LayoutManager layoutManager;
    MessagesData messagesData;


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
                // Declare JSON object and parse it
                JSONObject jsonObject = new JSONObject(message);
                messagesData.addMessage(jsonObject.getString("msg"), jsonObject.getString("user"));
                // Configure RecyclerView and notify adapter
                layoutManager.smoothScrollToPosition(rvMessages, null, messagesData.getMessages().size() - 1);
                messagesAdapter.notifyItemInserted(messagesData.getMessages().size() - 1);
            }
        });

        try {
            ws.connect();
        } catch (WebSocketException e) {
            System.out.println("Error: " + e.getMessage());
        }

        // Declare elementsasd;
        etInputMessage = findViewById(R.id.etInputMessage);
        btnSendMsg = findViewById(R.id.btnSendMsg);

        rvMessages = findViewById(R.id.rvMessages);
        messagesData = new MessagesData();
        messagesAdapter = new MessagesAdapter(messagesData.getMessages(), this);
        layoutManager = new LinearLayoutManager(this);

        // Set adapter
        rvMessages.setLayoutManager(layoutManager);
        rvMessages.setHasFixedSize(true);
        rvMessages.setAdapter(messagesAdapter);

        btnSendMsg.setOnClickListener(v -> {
            String message = etInputMessage.getText().toString();
            ws.sendText(message);
        });

    }

    private String parseJSON(String message) {

        try {
            JSONObject jsonObject = new JSONObject(message);
            String username = jsonObject.getString("user");
            String messageText = jsonObject.getString("msg");

            return username + ": " + messageText;

        } catch (JSONException e) {
            e.printStackTrace();
        }

        return null;
    }

}