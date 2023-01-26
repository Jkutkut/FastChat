package org.fastchat.utils;

import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.TextView;

import androidx.annotation.NonNull;
import androidx.recyclerview.widget.RecyclerView;

import org.fastchat.R;
import org.fastchat.models.Messages;
import org.fastchat.screens.ChatActivity;

import java.util.ArrayList;

public class MessagesAdapter extends RecyclerView.Adapter<MessagesAdapter.ViewHolder> {

    private ArrayList<Messages> messagesList;

    public MessagesAdapter(ArrayList<Messages> messagesList, ChatActivity chatActivity) {
        this.messagesList = messagesList;
    }

    @NonNull
    @Override
    public MessagesAdapter.ViewHolder onCreateViewHolder(@NonNull ViewGroup parent, int viewType) {
        View v = LayoutInflater.from(parent.getContext()).inflate(R.layout.msg_item, parent, false);
        return new ViewHolder((ViewGroup) v);
    }

    @Override
    public void onBindViewHolder(@NonNull MessagesAdapter.ViewHolder holder, int position) {
        holder.bind(messagesList.get(position));
    }

    @Override
    public int getItemCount() {
        return messagesList.size();
    }


    public class ViewHolder extends RecyclerView.ViewHolder {

        private TextView message;
        private TextView sender;

        public ViewHolder(@NonNull ViewGroup parent) {
            super(parent);
            message = parent.findViewById(R.id.tvMsgItemContent);
            sender = parent.findViewById(R.id.tvMsgItemName);
        }

        public void bind(Messages messages) {
            message.setText(messages.getMessage());
            sender.setText(messages.getSender());
        }
    }

}
