package org.fastchat.utils;

import com.neovisionaries.ws.client.*;

import org.fastchat.screens.ChatActivity;
import org.json.JSONException;

import java.io.IOException;
import java.util.List;
import java.util.Map;

public class Connector {
    private WebSocket mWebSocket;
    private final ChatActivity mChatActivity;
    private final String mServerUri;

    public Connector(String serverUri, ChatActivity chatActivity) {
        mServerUri = serverUri;
        mChatActivity = chatActivity;
    }

    public void connect() {
        WebSocketFactory factory = new WebSocketFactory();
        try {
            mWebSocket = factory.createSocket(mServerUri);
            mWebSocket.addListener(new WebsocketListener());
            mWebSocket.connect();
        } catch (IOException | WebSocketException e) {
            e.printStackTrace();
        }
    }

    public void disconnect() {
        mWebSocket.disconnect();
    }

    public void sendMessage(String message) {
        mWebSocket.sendText(message);
    }

    private class WebsocketListener extends WebSocketAdapter {
        @Override
        public void onTextMessage(WebSocket websocket, String message) throws JSONException {
            mChatActivity.onMessageReceived(message);
        }

        @Override
        public void onConnected(WebSocket websocket, Map<String, List<String>> headers) {
            mChatActivity.onConnected();
        }

        @Override
        public void onDisconnected(WebSocket websocket, WebSocketFrame serverCloseFrame, WebSocketFrame clientCloseFrame, boolean closedByServer) {
            mChatActivity.onDisconnected();
        }
    }
}
