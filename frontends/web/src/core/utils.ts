import Todo from "./ToDoItem";


type CustomErrorOptions =
    | { code: 1; message: string }
    | { code: 0; message: null };

type ToDoResponseOptions =
    | { errorMessage: string; toDos: null }
    | { errorMessage: null; toDos: Todo[] };


export class EmptyResponse {
    code: number;
    message: string | null;

    constructor(options: CustomErrorOptions) {
        this.code = options.code;
        this.message = options.message;
    }
}

export class ToDoResponse {
    errorMessage: string | null;
    toDos: Todo[] | null;

    constructor(options: ToDoResponseOptions) {
        this.errorMessage = options.errorMessage;
        this.toDos = options.toDos;
    }
}
