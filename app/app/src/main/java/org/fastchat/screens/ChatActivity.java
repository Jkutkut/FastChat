package org.fastchat.screens;

import android.os.Bundle;
import android.os.StrictMode;
import android.widget.Button;
import android.widget.EditText;

import androidx.appcompat.app.AppCompatActivity;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import org.fastchat.R;
import org.fastchat.models.MessagesData;
import org.fastchat.utils.Connector;
import org.fastchat.utils.MessagesAdapter;
import org.json.JSONException;
import org.json.JSONObject;

public class ChatActivity extends AppCompatActivity {

    EditText etInputMessage;
    Button btnSendMsg;

    // Declare RecyclerView
    RecyclerView rvMessages;
    MessagesAdapter messagesAdapter;
    RecyclerView.LayoutManager layoutManager;
    MessagesData messagesData;

    private Connector mConnector;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_chat);

        StrictMode.ThreadPolicy policy = new StrictMode.ThreadPolicy.Builder().permitAll().build();
        StrictMode.setThreadPolicy(policy);

        // Get data from MainActivity
        // String ip = getIntent().getStringExtra("ip");
        // String port = getIntent().getStringExtra("port");
        // String endpoint = getIntent().getStringExtra("endpoint");

        mConnector = new Connector("ws://192.168.1.135:4242/test", this);
        mConnector.connect();

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

            if (!message.isEmpty()) {
                mConnector.sendMessage(message);
                etInputMessage.setText("");
            }

        });
    }

    public void onConnected() {
        System.out.println("(TRACE) Connected");
    }
    public void onDisconnected() {
        System.out.println("(TRACE) Disconnected");
    }
    public void onMessageReceived(String message) throws JSONException {
        System.out.println("(TRACE) Msg recived: " + message);
        JSONObject jsonObject = new JSONObject(message);
        messagesData.addMessage(jsonObject.getString("msg"), jsonObject.getString("user"));

        layoutManager.smoothScrollToPosition(rvMessages, null, messagesData.getMessages().size() - 1);
        messagesAdapter.notifyItemInserted(messagesData.getMessages().size() - 1);
    }

}