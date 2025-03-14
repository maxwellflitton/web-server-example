import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { cn } from "src/ui/helpers/utils";

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
  CardDescription,
  CardHeader,
  CardTitle,
} from "src/ui/components/shadcnComponents/formCardContainer";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "src/ui/components/shadcnComponents/select"; // Import shadcn select components

// Define a Zod schema for the Login form.
const formSchema = z.object({
  email: z
    .string()
    .email("Please enter a valid email address"),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters long"),
  role: z.enum(["Super Admin", "Admin", "Worker"], {
    errorMap: () => ({ message: "Please select a valid role" }),
  }),
});

export type LoginFormSchema = z.infer<typeof formSchema>;

type LoginFormProps = {
  apiError: string;
  apiFailure: boolean;
  submitLoginForm: (data: LoginFormSchema) => void;
} & React.ComponentPropsWithoutRef<"div">;

export function LoginForm({
  apiError,
  apiFailure,
  submitLoginForm,
  className,
  ...props
}: LoginFormProps) {
  const form = useForm<LoginFormSchema>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
      role: "Worker", // Default role; change as needed.
    },
  });

  function onSubmit(values: LoginFormSchema) {
    submitLoginForm(values);
  }

  // Create a ref for the Alert component.
  const alertRef = React.useRef<HTMLDivElement>(null);

  // When an API failure occurs, focus and scroll to the alert.
  React.useEffect(() => {
    if (apiFailure && alertRef.current) {
      alertRef.current.focus();
      alertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }, [apiFailure]);

  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <Card>
        <CardHeader>
          <CardTitle className="text-2xl">Login</CardTitle>
          <CardDescription>Login to your account</CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
              {/* Email Field */}
              <FormField
                control={form.control}
                name="email"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Email</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="email@example.com"
                        type="email"
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>Please enter a valid email address</FormDescription>
                    <FormMessage className="text-red-600" />
                  </FormItem>
                )}
              />

              {/* Password Field */}
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Password</FormLabel>
                    <FormControl>
                      <Input placeholder="********" type="password" {...field} />
                    </FormControl>
                    <FormDescription>At least 8 characters</FormDescription>
                    <FormMessage className="text-red-600" />
                  </FormItem>
                )}
              />

              {/* Role Dropdown Field */}
              <FormField
                control={form.control}
                name="role"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Role</FormLabel>
                    <Select onValueChange={field.onChange} value={field.value}>
                      <FormControl>
                        <SelectTrigger>
                          <SelectValue placeholder="Select your role" />
                        </SelectTrigger>
                      </FormControl>
                        <SelectContent className="bg-white">
                        <SelectItem value="Super Admin" className="hover:bg-gray-100">
                            Super Admin
                        </SelectItem>
                        <SelectItem value="Admin" className="hover:bg-gray-100">
                            Admin
                        </SelectItem>
                        <SelectItem value="Worker" className="hover:bg-gray-100">
                            Worker
                        </SelectItem>
                        </SelectContent>
                    </Select>
                    <FormDescription>Select your role</FormDescription>
                    <FormMessage className="text-red-600" />
                  </FormItem>
                )}
              />

              <FormButton type="submit">Login</FormButton>

              {apiFailure && (
                <div>
                  <Alert
                    ref={alertRef}
                    variant="destructive"
                    className="global-jiggle"
                  >
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>{apiError}</AlertDescription>
                  </Alert>
                </div>
              )}
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
