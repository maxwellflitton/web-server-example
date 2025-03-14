import * as React from "react"
import {
  ColumnDef,
  ColumnFiltersState,
  flexRender,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useNavigate } from "react-router-dom";
import { Button } from "src/ui/components/shadcnComponents/button";
import { Input } from "src/ui/components/shadcnComponents/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "src/ui/components/shadcnComponents/select";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "src/ui/components/shadcnComponents/table";

import { User } from "./Columns";

interface UserTableProps {
  columns: ColumnDef<User, any>[]
  data: User[]
};

export function UserTable({
  columns,
  data,
}: UserTableProps) {
  const navigate = useNavigate();
  const [columnFilters, setColumnFilters] = React.useState<ColumnFiltersState>([]);
  
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnFiltersChange: setColumnFilters,
    state: {
      columnFilters,
    },
    initialState: {
      pagination: {
        pageSize: 10,
      },
    },
  });

  const handleRowClick = (userId: string) => {
    navigate(`/user-actions/${userId}`)
  };

  // Define a constant row height (48px for h-12)
  const rowHeight = 48;
  const pageSize = table.getState().pagination.pageSize;
  // maxRows is the fixed table height: if there are less than pageSize items, we use data.length.
  const maxRows = data.length > 0 && data.length < pageSize ? data.length : pageSize;

  return (
    <div>
      <div className="flex items-center gap-4 py-4">
        <Input
          placeholder="Filter by email..."
          value={(table.getColumn("email")?.getFilterValue() as string) ?? ""}
          onChange={(event) =>
            table.getColumn("email")?.setFilterValue(event.target.value)
          }
          className="max-w-sm"
        />
        
        <Select
          value={(table.getColumn("roles")?.getFilterValue() as string) ?? ""}
          onValueChange={(value) => 
            table.getColumn("roles")?.setFilterValue(value)
          }
        >
          <SelectTrigger className="w-[180px] bg-white">
            <SelectValue placeholder="Filter by role" />
          </SelectTrigger>
          <SelectContent className="bg-white">
            <SelectItem value="all" className="cursor-pointer hover:bg-gray-100">All Roles</SelectItem>
            <SelectItem value="Admin" className="cursor-pointer hover:bg-gray-100">Admin</SelectItem>
            <SelectItem value="Worker" className="cursor-pointer hover:bg-gray-100">Worker</SelectItem>
            <SelectItem value="Super Admin" className="cursor-pointer hover:bg-gray-100">Super Admin</SelectItem>
          </SelectContent>
        </Select>

        <Select
          value={
            table.getColumn("blocked")?.getFilterValue() === true
              ? "true"
              : table.getColumn("blocked")?.getFilterValue() === false
              ? "false"
              : "all"
          }
          onValueChange={(value) => {
            if (value === "all") {
              table.getColumn("blocked")?.setFilterValue("");
            } else {
              table.getColumn("blocked")?.setFilterValue(value === "true");
            }
          }}
        >
          <SelectTrigger className="w-[180px] bg-white">
            <SelectValue placeholder="Filter by status" />
          </SelectTrigger>
          <SelectContent className="bg-white">
            <SelectItem value="all" className="cursor-pointer hover:bg-gray-100">
              All Status
            </SelectItem>
            <SelectItem value="false" className="cursor-pointer hover:bg-gray-100">
              Active
            </SelectItem>
            <SelectItem value="true" className="cursor-pointer hover:bg-gray-100">
              Blocked
            </SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((headerGroup) => (
              <TableRow key={headerGroup.id}>
                {headerGroup.headers.map((header) => (
                  <TableHead key={header.id}>
                    {header.isPlaceholder
                      ? null
                      : flexRender(
                          header.column.columnDef.header,
                          header.getContext()
                        )}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows?.length ? (
              <>
                {table.getRowModel().rows.map((row) => (
                  <TableRow
                    key={row.id}
                    onClick={() => handleRowClick(row.original.id)}
                    className="cursor-pointer hover:bg-gray-100 h-12 py-2 border-b border-gray-300"
                  >
                    {row.getVisibleCells().map((cell) => {
                      if (cell.column.id === "blocked") {
                        return (
                          <TableCell key={cell.id}>
                            {cell.getValue() ? "Blocked - ❌" : "Active - ✅"}
                          </TableCell>
                        );
                      } else if (cell.column.id === "confirmed") {
                        return (
                          <TableCell key={cell.id}>
                            {cell.getValue() ? "Confirmed - ✅" : "Confirmed - ❌"}
                          </TableCell>
                        );
                      } else if (cell.column.id === "roles") {
                        return (
                          <TableCell key={cell.id}>
                            {(cell.getValue() as string[]).join(", ")}
                          </TableCell>
                        );
                      }

                      return (
                        <TableCell key={cell.id}>
                          {flexRender(cell.column.columnDef.cell, cell.getContext())}
                        </TableCell>
                      );
                    })}
                  </TableRow>
                ))}
                {(() => {
                  const emptyRowsCount = maxRows - table.getRowModel().rows.length;
                  return emptyRowsCount > 0 ? (
                    <TableRow key="empty" className="border-0">
                      <TableCell
                        colSpan={columns.length}
                        className="border-0"
                        style={{ height: emptyRowsCount * rowHeight }}
                      />
                    </TableRow>
                  ) : null;
                })()}
              </>
            ) : (
              <TableRow>
                <TableCell
                  colSpan={columns.length}
                  className="text-center"
                  style={{ height: maxRows * rowHeight }}
                >
                  No Results
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>
      
      <div className="flex items-center justify-between space-x-2 py-4">
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.setPageIndex(0)}
            disabled={!table.getCanPreviousPage()}
          >
            First
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            Previous
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            Next
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => table.setPageIndex(table.getPageCount() - 1)}
            disabled={!table.getCanNextPage()}
          >
            Last
          </Button>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-sm text-gray-700">
            Page {table.getState().pagination.pageIndex + 1} of{" "}
            {table.getPageCount()}
          </span>
          <span className="text-sm text-gray-700">
            | Total: {table.getFilteredRowModel().rows.length} users
          </span>
        </div>
      </div>
    </div>
  );
}
