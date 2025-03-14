/*
Here is where we define the to-do items for data interaction we need the following

- []
 */
import {Operation} from "src/core/interfaces";

export default class Todo {
    id: number;                     // SERIAL PRIMARY KEY
    name: string;                   // VARCHAR NOT NULL
    dueDate?: Date | null;          // TIMESTAMP (optional)
    assignedBy: number;             // INTEGER NOT NULL (FK users.id)
    assignedTo: number;             // INTEGER NOT NULL (FK users.id)
    description?: string | null;    // TEXT (optional)
    dateAssigned: Date;             // TIMESTAMP NOT NULL DEFAULT NOW()
    dateFinished?: Date | null;     // TIMESTAMP (optional)
    finished: boolean;              // BOOLEAN NOT NULL DEFAULT FALSE
    handleTrigger: ((item: Todo, operation: Operation) => Promise<void>) | null;

    constructor(params: {
        id: number;
        name: string;
        assignedBy: number;
        assignedTo: number;
        dateAssigned?: Date;
        dueDate?: Date | null;
        description?: string | null;
        dateFinished?: Date | null;
        finished?: boolean;
    }) {
        this.id = params.id;
        this.name = params.name;
        this.assignedBy = params.assignedBy;
        this.assignedTo = params.assignedTo;
        this.description = params.description ?? null;
        this.dueDate = params.dueDate ?? null;
        this.dateAssigned = params.dateAssigned ?? new Date();
        this.dateFinished = params.dateFinished ?? null;
        this.finished = params.finished ?? false;
        this.handleTrigger = null;
    }

    async performOperation(operation: Operation) {
        if (this.handleTrigger) {
            await this.handleTrigger(this, operation);
        }
    }
}


export class NewTodo {
    name: string;                      // String
    dueDate?: Date | null;             // Option<NaiveDateTime>
    assignedBy: number;                // i32
    assignedTo: number;                // i32
    description?: string | null;       // Option<String>
    dateAssigned?: Date | null;        // Option<NaiveDateTime>

    constructor(params: {
        name: string;
        assignedBy: number;
        assignedTo: number;
        dueDate?: Date | null;
        description?: string | null;
        dateAssigned?: Date | null;
    }) {
        this.name = params.name;
        this.assignedBy = params.assignedBy;
        this.assignedTo = params.assignedTo;
        this.dueDate = params.dueDate ?? null;
        this.description = params.description ?? null;
        this.dateAssigned = params.dateAssigned ?? null;
    }

    assignDate(date: Date = new Date()) {
        this.dateAssigned = date;
    }
}

