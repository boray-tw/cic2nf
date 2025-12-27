#!/bin/bash
# This entrypoint create an user in the image at runtime,
# with the UID and username from the environment variables
# HOST_USERNAME and HOST_UID, or fallback to user and 9001
# (a high UID to indicate something went wrong).
#
# reference: https://denibertovic.com/posts/handling-permissions-with-docker-volumes/

USER_ID=${HOST_UID:-9001}
USERNAME=${HOST_USERNAME:-user}

useradd --shell /bin/bash -u $USER_ID -o -c "" -m "$USERNAME"
export HOME="/home/$USERNAME"

cat <<EOT
Will execute commands as user $USERNAME (UID $USER_ID)

WARNING: The following shell is in the container.
EOT

exec gosu "$USERNAME" "$@"
