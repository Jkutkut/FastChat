package org.fastchat.models;

import android.widget.TextView;

import java.util.ArrayList;

public class MessagesData {
    private ArrayList<Messages> messages;

    TextView tvMsgItemName;
    TextView tvMsgItemContent;

    public MessagesData() {
        messages = new ArrayList<Messages>();
        //loadMessages();
    }

    private void loadMessages() {
        messages.add(new Messages("Hello", "user"));
        messages.add(new Messages("Hi", "friend"));
        messages.add(new Messages("How are you?", "user"));
        messages.add(new Messages("I'm fine, thanks", "friend"));
        messages.add(new Messages("What about you?", "user"));
        messages.add(new Messages("I'm fine too", "friend"));
        messages.add(new Messages("See you later", "user"));
        messages.add(new Messages("Bye", "friend"));
    }

    public ArrayList<Messages> getMessages() {
        return messages;
    }

    public void addMessage(String message) {
    messages.add(new Messages(message, "user"));
        System.out.println("Message added");
        for (Messages m : messages) {
            System.out.println(m.getMessage());
        }
    }


}
