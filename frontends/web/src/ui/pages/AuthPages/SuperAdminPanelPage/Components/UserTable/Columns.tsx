import { ColumnDef } from "@tanstack/react-table"


type RoleOptions = "Admin" | "Worker" | "Super Admin"

export type User = {
  id: string
  username: string
  email: string
  firstName: string
  lastName: string
  roles: RoleOptions[]
  dateCreated: string
  lastLoggedIn: string
  blocked: boolean
  confirmed: boolean  // Added this field
}

export const columns: ColumnDef<User>[] = [
  {
    accessorKey: "username",
    header: "Username",
  },
  {
    accessorKey: "firstName",
    header: "First Name",
  },
  {
    accessorKey: "lastName",
    header: "Last Name",
  },
  {
    accessorKey: "email",
    header: "Email",
  },
  {
    accessorKey: "roles",
    header: "Roles",
    filterFn: (row, columnId, filterValue) => {
      if (!filterValue || filterValue === "all") {
        return true
      }
      const rowValue = row.getValue(columnId) as string[] | undefined
      return Array.isArray(rowValue) && rowValue.includes(filterValue)
    },
  },
  {
    accessorKey: "dateCreated",
    header: "Created",
  },
  {
    accessorKey: "lastLoggedIn",
    header: "Last Login",
  },
  {
    accessorKey: "blocked",
    header: "Blocked",
  },
  {
    accessorKey: "confirmed",
    header: "Email Confirmed",
  }
]

