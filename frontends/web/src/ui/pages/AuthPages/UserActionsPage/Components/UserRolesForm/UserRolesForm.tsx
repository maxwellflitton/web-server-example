import React, { useMemo } from "react";
import { css } from "@emotion/react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "src/ui/components/shadcnComponents/formCardContainer";
import { FormButton } from "src/ui/components/components/Button/Button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "src/ui/components/shadcnComponents/form";
import { Checkbox } from "src/ui/components/shadcnComponents/checkbox";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import { AlertCircle } from "lucide-react";


// Create a typed object that stores booleans for each role
const rolesSchema = z.object({
  worker: z.boolean().default(false),
  admin: z.boolean().default(false),
  superAdmin: z.boolean().default(false),
});

type RolesFormValues = z.infer<typeof rolesSchema>;

interface UserRolesFormProps {
  currentRoles: string[];
  updateRolesError: string;
  updateRolesFailure: boolean;
  onSubmit: (newRoles: string[]) => void;
}

export function UserRolesForm({ currentRoles, updateRolesError, updateRolesFailure, onSubmit }: UserRolesFormProps) {
  // For default values, figure out which roles are in currentRoles
  const defaultValues: RolesFormValues = useMemo(() => {
    return {
      worker: currentRoles.includes("Worker"),
      admin: currentRoles.includes("Admin"),
      superAdmin: currentRoles.includes("Super Admin"),
    };
  }, [currentRoles]);

  const form = useForm<RolesFormValues>({
    resolver: zodResolver(rolesSchema),
    defaultValues,
  });

  function handleSubmit(data: RolesFormValues) {
    // Convert the checkboxes to an array
    const updatedRoles: string[] = [];
    if (data.worker) updatedRoles.push("Worker");
    if (data.admin) updatedRoles.push("Admin");
    if (data.superAdmin) updatedRoles.push("Super Admin");
    onSubmit(updatedRoles);
  }

  // Utility to handle row-click toggling without double-toggling
  function rowClickToggle(e: React.MouseEvent<HTMLDivElement>, isChecked: boolean, onChange: (v: boolean) => void) {
    // If user clicked directly on the checkbox input, let the checkbox handle it.
    if (e.target instanceof HTMLInputElement) return;
    onChange(!isChecked);
  }

  // Create a ref for the Alert component.
  const alertRef = React.useRef<HTMLDivElement>(null);

  // When an API failure occurs, focus and scroll to the alert.
  React.useEffect(() => {
      if (updateRolesFailure && alertRef.current) {
      alertRef.current.focus();
      alertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
      }
  }, [updateRolesFailure]);

  return (
    <Card className="w-full max-w-3xl border border-gray-300 shadow-sm">
      <CardHeader>
        <CardTitle>Assign or Remove Roles</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(handleSubmit)} className="space-y-6">
            <FormField
              control={form.control}
              name="worker"
              render={({ field }) => (
                <FormItem
                  onClick={(e) => rowClickToggle(e, field.value, field.onChange)}
                  className="flex flex-row items-start space-x-3 rounded-md border-1 border-gray-200 p-4 shadow-sm 
                             hover:bg-gray-100 cursor-pointer"
                >
                  <FormControl>
                    <Checkbox
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                  <div className="space-y-2">
                    <FormLabel>Worker Role</FormLabel>
                    <FormDescription>
                      Check to assign or remove the Worker role from this user.
                    </FormDescription>
                  </div>
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="admin"
              render={({ field }) => (
                <FormItem
                  onClick={(e) => rowClickToggle(e, field.value, field.onChange)}
                  className="flex flex-row items-start space-x-3 rounded-md border-1 border-gray-200 p-4 shadow-sm 
                             hover:bg-gray-100 cursor-pointer"
                >
                  <FormControl>
                    <Checkbox
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                  <div className="space-y-2">
                    <FormLabel>Admin Role</FormLabel>
                    <FormDescription>
                      Check to assign or remove the Admin role from this user.
                    </FormDescription>
                  </div>
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="superAdmin"
              render={({ field }) => (
                <FormItem
                  onClick={(e) => rowClickToggle(e, field.value, field.onChange)}
                  className="flex flex-row items-start space-x-3 rounded-md border-1 border-gray-200 p-4 shadow-sm 
                             hover:bg-gray-100 cursor-pointer"
                >
                  <FormControl>
                    <Checkbox
                      checked={field.value}
                      onCheckedChange={field.onChange}
                    />
                  </FormControl>
                  <div className="space-y-2">
                    <FormLabel>Super Admin Role</FormLabel>
                    <FormDescription>
                      Check to assign or remove the Super Admin role from this user.
                    </FormDescription>
                  </div>
                </FormItem>
              )}
            />

            <FormButton type="submit" variant="default">
              Submit Role Changes
            </FormButton>

            {updateRolesFailure && (
                <div>
                    <Alert ref={alertRef} variant="destructive" className="global-jiggle">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>
                        {updateRolesError}
                    </AlertDescription>
                    </Alert>
                </div>
            )}
          </form>
        </Form>
      </CardContent>
      <CardFooter />
    </Card>
  );
};