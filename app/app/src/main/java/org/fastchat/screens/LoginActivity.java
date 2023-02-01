package org.fastchat.screens;

import android.os.Bundle;
import android.widget.LinearLayout;

import androidx.appcompat.app.AppCompatActivity;

import org.fastchat.R;

public class LoginActivity extends AppCompatActivity {

    LinearLayout llLoginData;
    LinearLayout llRegisterData;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_login);


        llLoginData = findViewById(R.id.llLoginData);
        llRegisterData = findViewById(R.id.llRegisterData);

        llLoginData.setVisibility(LinearLayout.VISIBLE);
        llRegisterData.setVisibility(LinearLayout.GONE);




    }

}

