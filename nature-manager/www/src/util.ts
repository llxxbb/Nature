export const range = (start: number, end: number) => {
    const length = end - start;
    return Array.from({ length }, (_, i) => start + i);
}