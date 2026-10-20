export function getStringParameterOrThrow(name: string): string {
    // @ts-ignore
    const value = process.env[name];
    if (!value) {
        throw new Error(`Missing configuration parameter "${name}"`);
    }
    return value;
}