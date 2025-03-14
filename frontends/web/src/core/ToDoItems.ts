/*
Here we define the collections for to do items. The idea behind this class is to
act as an abstraction layer between items, getting items, and sorting them.

Ideal use would be you have a React component that accepts a class that has the
ItemGetter interface, and this is then called to populate the React component
 */
import Todo from "./ToDoItem";
import { ItemGetter, Operation, FullItemProcess } from "src/core/interfaces";
import { EmptyResponse, ToDoResponse } from "./utils";


export class ToDoItems {
    userId: number;
    jwt: string;
    pendingItems: Todo[];
    doneItems: Todo[];
    interfaceHandle: FullItemProcess

    constructor(
        id: number,
        jwt: string,
        interfaceHandle: FullItemProcess

    ) {
        this.userId = id;
        this.jwt = jwt;
        this.pendingItems = [];
        this.doneItems = [];
        this.interfaceHandle = interfaceHandle;
    }

    private callbackHandle = async (item: Todo, operation: Operation) => {

        switch (operation.type) {
            case "Complete":
                this.processItems(await this.completeItem(item, operation.cutOffDate));
                break;
            case "Delete":
                this.processItems(await this.deleteItem(item, operation.cutOffDate));
                break;
            case "Create":
                this.processItems(await this.createItem(item, operation.cutOffDate));
                break;
            case "Reassign":
                this.processItems(await this.reassignItem(item, operation.newId, operation.cutOffDate));
                break;
        }
    }

    handleResponse(response: ToDoResponse): Todo[] {
        if (response.toDos) {
            return response.toDos
        }
        throw new Error(response.errorMessage!)
    }

    async completeItem(item: Todo, cutOffDate: Date): Promise<Todo[]> {
        return this.handleResponse(await this.interfaceHandle.completeItem(
            this.jwt, this.userId, item.id, cutOffDate
        ));
    }
    async deleteItem(item: Todo, cutOffDate: Date): Promise<Todo[]> {
        return this.handleResponse(await this.interfaceHandle.deleteItem(
            this.jwt, this.userId, item.id, cutOffDate
        ));
    }
    async createItem(item: Todo, cutOffDate: Date): Promise<Todo[]> {
        return this.handleResponse(await this.interfaceHandle.createItem(
            this.jwt, this.userId, item, cutOffDate
        ));
    }
    async reassignItem(item: Todo, newId: number, cutOffDate: Date): Promise<Todo[]> {
        return this.handleResponse(await this.interfaceHandle.reassignItem(
            this.jwt, this.userId, item.id, newId, cutOffDate
        ));
    }

    processItems(items: Todo[]) {
        for (const item of items) {
            item.handleTrigger = this.callbackHandle;
            if (item.finished) {
                this.doneItems.push(item);
            }
            else {
                this.pendingItems.push(item);
            }
        }
    }

    async populateItems(getter: ItemGetter, userId: number, cutOffDate: Date): Promise<EmptyResponse> {
        const items = await getter.getItems(getter.jwt, userId, cutOffDate);
        if (!items.errorMessage) {
            this.processItems(items.toDos!)
            return new EmptyResponse({ code: 0, message: null })
        }
        return new EmptyResponse({ code: 1, message: items.errorMessage })
    }
}
