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
	else
		question="$question: "
	fi

	read -p "$question" data

	if [ "$data" = "" ]; then
		data="$default"
	fi
}


echo "Firebase App:";
askData "API_KEY" ""; API_KEY="$data";

echo "
# Firebase
API_KEY=$API_KEY
" > $SECRET_FILE
