#!/usr/bin/env bash

# Function to display usage information
usage() {
    echo "Usage: chirper [options] -- command [args...]"
    echo "Options:"
    echo "  -t, --title TITLE    Set the notification title (default: Command Execution)"
    echo "  -m, --message MSG    Add a custom message to the notification"
    echo "  -s, --success        Use success notification (default)"
    echo "  -f, --failure        Use failure notification"
    echo "  -p, --priority PRIO  Set priority (lowest, low, normal, high, emergency)"
    echo "  -h, --help           Display this help message"
    echo
    echo "Example: chirper -t 'Backup Process' -m 'Weekly backup' -p high -- rsync -avz /src /dst"
}

# Default values
TITLE="Command Execution"
NOTIFICATION_TYPE="success"
PRIORITY="normal"
CUSTOM_MESSAGE=""

# Parse options
while [[ $# -gt 0 ]]; do
    case $1 in
        -t|--title)
            TITLE="$2"
            shift 2
            ;;
        -m|--message)
            CUSTOM_MESSAGE="$2"
            shift 2
            ;;
        -s|--success)
            NOTIFICATION_TYPE="success"
            shift
            ;;
        -f|--failure)
            NOTIFICATION_TYPE="failure"
            shift
            ;;
        -p|--priority)
            PRIORITY="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        --)
            shift
            break
            ;;
        *)
            echo "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Check if a command was provided
if [ $# -eq 0 ]; then
    echo "Error: No command specified."
    usage
    exit 1
fi

# Run the command and measure execution time
START_TIME=$(date +%s)
"$@"
EXIT_CODE=$?
END_TIME=$(date +%s)

# Calculate duration
DURATION=$((END_TIME - START_TIME))
DURATION_STR=$(printf '%dh %dm %ds' $((DURATION/3600)) $((DURATION%3600/60)) $((DURATION%60)))

# Prepare message
if [ $EXIT_CODE -eq 0 ]; then
    STATUS="successful"
    [ "$NOTIFICATION_TYPE" = "failure" ] && NOTIFICATION_TYPE="success"
else
    STATUS="failed"
    [ "$NOTIFICATION_TYPE" = "success" ] && NOTIFICATION_TYPE="failure"
fi

MESSAGE="Command $STATUS: $* (Exit code: $EXIT_CODE, Duration: $DURATION_STR)"

# Add custom message if provided
if [ -n "$CUSTOM_MESSAGE" ]; then
    MESSAGE="$CUSTOM_MESSAGE $MESSAGE"
fi

# Send notification using chirp
chirp $NOTIFICATION_TYPE --message "$MESSAGE" --priority "$PRIORITY"

# Exit with the same code as the wrapped command
exit $EXIT_CODE
