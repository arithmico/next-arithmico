export function getCalculatorHostname(environment: string): string {
    return environment === "production"
        ? "next.arithmico.com"
        : `${environment}.calculator.arithmico.com`;
}