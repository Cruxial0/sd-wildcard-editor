import { listen } from "@tauri-apps/api/event";

const LOG_EVENT = "console-log";


export async function setup_log_listener()
{
    await listen(LOG_EVENT, (event) =>
    {
        var payload = event.payload as LogPackage;
        console.log(payload);
        var text = "%c" + payload.strings[0] + " %c[" + payload.strings[1] + "] %c " + payload.strings[2] + " %c| " + payload.strings[3];
        console.log(text, payload.styles[0], payload.styles[1], payload.styles[2], payload.styles[3]);
    });
}

class LogPackage
{
    public strings!: string[];
    public styles!: string[];
    public severity!: LogLevel;
}

enum LogLevel
{
    SEVERE,
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE
}