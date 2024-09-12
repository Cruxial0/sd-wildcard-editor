import { DocumentModel } from "./document";
import { Position } from "./types";

interface Command {
    execute(): void;
    undo(): void;
}
  
class InsertTextCommand implements Command {
    constructor(private model: DocumentModel, private position: Position, private text: string) {}

    execute() {
        this.model.insertText(this.position, this.text);
    }

    undo() {
        this.model.deleteText(this.position, this.text.length);
    }
}
  
class CommandManager {
    private undoStack: Command[] = [];
    private redoStack: Command[] = [];

    execute(command: Command) {
        command.execute();
        this.undoStack.push(command);
        this.redoStack = [];
    }

    undo() {
        const command = this.undoStack.pop();
        if (command) {
            command.undo();
            this.redoStack.push(command);
        }
    }

    redo() {
        const command = this.redoStack.pop();
        if (command) {
            command.execute();
            this.undoStack.push(command);
        }
    }
}