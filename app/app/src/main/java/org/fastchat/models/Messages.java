package org.fastchat.models;

public class Messages {
    private String message;
    private String sender;

    public Messages(String message, String sender) {
        this.message = message;
        this.sender = sender;
    }

    public String getMessage() {
        return message;
    }

    public String getSender() {
        return sender;
    }
}
