export function add(a: number, b: number): number {
    return a + b;
}

export function multiply(a: number, b: number): number {
    return a * b;
}


describe('Math functions', () => {
    test('adds two numbers', () => {
        expect(add(2, 3)).toBe(5);
    });

    test('multiplies two numbers', () => {
        expect(multiply(4, 5)).toBe(20);
    });
});