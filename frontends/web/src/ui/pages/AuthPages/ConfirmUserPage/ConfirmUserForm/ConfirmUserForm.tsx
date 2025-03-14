import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { cn } from "src/ui/helpers/utils";

import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import { AlertCircle } from "lucide-react";
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
import { Input } from "src/ui/components/shadcnComponents/input";
import {
  Card,
  CardContent,
  CardHeader,
  CardDescription,
  CardTitle,
} from "src/ui/components/shadcnComponents/formCardContainer";

// Define a schema with two password fields.
const formSchema = z
  .object({
    password: z.string().min(8, "Password must be at least 8 characters long."),
    confirmPassword: z.string().min(8, "Password must be at least 8 characters long."),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match",
    path: ["confirmPassword"], // associate the error with the confirmPassword field
  });

export type ConfirmUserFormSchema = z.infer<typeof formSchema>;

type ConfirmUserFormProps = {
  firstName?: string;
  lastName?: string;
  username?: string;
  getUserError: string;
  getUserFailure: boolean;
  confirmUserError: string;
  confirmUserFailure: boolean;
  disabled: boolean;
  submitConfirmUserForm: (data: ConfirmUserFormSchema) => void;
} & React.ComponentPropsWithoutRef<"div">;

export function ConfirmUserForm({
  firstName,
  lastName,
  username,
  getUserError,
  getUserFailure,
  confirmUserError,
  confirmUserFailure,
  disabled,
  submitConfirmUserForm,
  className,
  ...props
}: ConfirmUserFormProps) {
  const form = useForm<ConfirmUserFormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      password: "",
      confirmPassword: "",
    },
  });

  function onSubmit(values: ConfirmUserFormSchema) {
    submitConfirmUserForm(values);
  }

  // Ref for the confirm user error alert.
  const alertRef = React.useRef<HTMLDivElement>(null);

  // When a confirmUser API failure occurs, focus and scroll to the alert.
  React.useEffect(() => {
    if (confirmUserFailure && alertRef.current) {
      alertRef.current.focus();
      alertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }, [confirmUserFailure]);

  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">Confirm Account</CardTitle>
          <CardDescription>
            {getUserFailure ? (
              // Show an alert within the card description if fetching user details failed.
              <Alert variant="destructive" className="text-red-600">
                <AlertCircle className="h-4 w-4" />
                <div>
                  <AlertTitle>Error</AlertTitle>
                  <AlertDescription>{getUserError}</AlertDescription>
                </div>
              </Alert>
            ) : (
              // Otherwise, show the user details and instructions.
              <div>
                <div className="flex flex-col gap-1">
                  <span>
                    <strong>First Name -</strong> {firstName}
                  </span>
                  <span>
                    <strong>Last Name -</strong> {lastName}
                  </span>
                  <span>
                    <strong>Username -</strong> {username}
                  </span>
                </div>
                <p className="mt-2">
                  Please confirm your account by creating a password below. The two password fields must match.
                </p>
              </div>
            )}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Password</FormLabel>
                    <FormControl>
                      <Input placeholder="********" type="password" {...field} disabled={disabled} />
                    </FormControl>
                    <FormDescription>At least 8 characters.</FormDescription>
                    <FormMessage className="text-red-600" />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="confirmPassword"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Confirm Password</FormLabel>
                    <FormControl>
                      <Input placeholder="********" type="password" {...field} disabled={disabled} />
                    </FormControl>
                    <FormDescription>Must match your password.</FormDescription>
                    <FormMessage className="text-red-600" />
                  </FormItem>
                )}
              />
              <FormButton type="submit" disabled={disabled}>
                Confirm Account
              </FormButton>
            </form>
          </Form>

          {confirmUserFailure && (
            <Alert ref={alertRef} variant="destructive" className="mt-4 global-jiggle">
              <AlertCircle className="h-4 w-4" />
              <div>
                <AlertTitle>Error</AlertTitle>
                <AlertDescription>{confirmUserError}</AlertDescription>
              </div>
            </Alert>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
