import ToDoItem from "src/core/ToDoItem";
import NewTodo from "src/core/ToDoItem";
import { ToDoResponse } from "./utils";

export interface HasJWT {
    jwt: string;
}

export interface ItemGetter extends HasJWT {
    getItems(jwt: string, userId: number, cutOffDate: Date): Promise<ToDoResponse>;
}

export interface CompleteItemProcess extends HasJWT {
    completeItem(jwt: string, userId: number, toDoId: number, cutOffDate: Date): Promise<ToDoResponse>;
}

export interface DeleteItemProcess extends HasJWT {
    deleteItem(jwt: string, userId: number, toDoId: number, cutOffDate: Date): Promise<ToDoResponse>;
}

export interface CreateItemProcess extends HasJWT {
    createItem(jwt: string, userId: number, item: NewTodo, cutOffDate: Date): Promise<ToDoResponse>;
}

export interface ReassignItemProcess extends HasJWT {
    reassignItem(jwt: string, userId: number, toDoId: number, newUserId: number, cutOffDate: Date): Promise<ToDoResponse>;
}

export type FullItemProcess = ItemGetter &
    CompleteItemProcess &
    DeleteItemProcess &
    CreateItemProcess &
    ReassignItemProcess;

export type Operation =
    | {type: "Complete", cutOffDate: Date}
    | {type: "Create", cutOffDate: Date}
    | {type: "Delete", cutOffDate: Date}
    | {type: "Reassign", newId: number, cutOffDate: Date};

