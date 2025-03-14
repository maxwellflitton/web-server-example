import { Todo } from '../ToDoItem'; // Update the path as needed

describe('Todo Class', () => {

    it('should create a todo with required fields', () => {
        const todo = new Todo({
            id: 1,
            name: 'Write Jest tests',
            assignedBy: 10,
            assignedTo: 20
        });

        expect(todo.id).toBe(1);
        expect(todo.name).toBe('Write Jest tests');
        expect(todo.assignedBy).toBe(10);
        expect(todo.assignedTo).toBe(20);

        // Check default values
        expect(todo.finished).toBe(false);
        expect(todo.dateFinished).toBeNull();
        expect(todo.description).toBeNull();
        expect(todo.dueDate).toBeNull();
        expect(todo.dateAssigned).toBeInstanceOf(Date);
    });

    it('should mark the todo as finished', () => {
        const todo = new Todo({
            id: 2,
            name: 'Complete testing',
            assignedBy: 11,
            assignedTo: 22
        });

        // Initially unfinished
        expect(todo.finished).toBe(false);
        expect(todo.dateFinished).toBeNull();

        todo.markAsFinished();

        expect(todo.finished).toBe(true);
        expect(todo.dateFinished).toBeInstanceOf(Date);
    });

    it('should assign the todo to another user and reset finish state', () => {
        const todo = new Todo({
            id: 3,
            name: 'Review code',
            assignedBy: 12,
            assignedTo: 25,
            finished: true,
            dateFinished: new Date()
        });

        todo.assignTo(30);

        expect(todo.assignedTo).toBe(30);
        expect(todo.finished).toBe(false);
        expect(todo.dateFinished).toBeNull();
        expect(todo.dateAssigned).toBeInstanceOf(Date);
    });

});
