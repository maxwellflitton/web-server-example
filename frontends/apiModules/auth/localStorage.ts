export function setJwt(jwt: string): void {
    localStorage.setItem('jwt', jwt)
}

export function getJwt(): string | null {
    return localStorage.getItem('jwt')
}

type Role = "Super Admin" | "Admin" | "Worker";

export function setRole(role: Role): void {
    localStorage.setItem('role', role)
}

export function getRole(): string | null {
    return localStorage.getItem('role')
}