#!/bin/bash

askPassword() {
	concept="$1"
	while true; do
		read -s -p "Password for $concept: " PASSWD;
		echo;
		if [ "$PASSWD" = "" ]; then
			echo "The password must not be empty";
			continue;
		fi
		read -s -p "Confirm password : " PASSWD_CONFIRM;
		echo;
		if [ "$PASSWD" = "$PASSWD_CONFIRM" ]; then
			break;
		fi
		echo "Passwords do not match";
		echo;
	done
}

SECRET_FILE=".env"

askData() {
	question="$1"
	default="$2"

	if [ ! "$default" = "" ]; then
		question="$question [$default]: ";
	fi

	read -p "$question" data

	if [ "$data" = "" ]; then
		data="$default"
	fi
}


echo "DB:";
askData "DB_NAME" "fast_chat_db"; DB_NAME="$data";
askData "DB_PORT" "4242"; DB_PORT="$data";
askData "DB_USR" "fast_chat_admin"; DB_USR="$data";
askPassword "$DB_USR"; DB_USR_PASSWD="$PASSWD";

echo "
DB_NAME='$DB_NAME'
DB_PORT='$DB_PORT'
DB_USR='$DB_USR'
DB_USR_PASSWD='$DB_USR_PASSWD'
" > $SECRET_FILE
echo "Done!"
echo
