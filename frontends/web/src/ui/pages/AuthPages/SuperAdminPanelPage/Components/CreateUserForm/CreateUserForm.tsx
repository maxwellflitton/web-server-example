/** @jsxImportSource @emotion/react */
import React, { useState, useEffect } from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { cn } from "src/ui/helpers/utils";
import * as styles from "./CreateUserForm.styles";

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
    CardTitle
} from "src/ui/components/shadcnComponents/formCardContainer";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "src/ui/components/shadcnComponents/select-modal";
import { LoadingSpinner } from "src/ui/components/components/LoadingSpinner/LoadingSpinner";
  

const formSchema = z.object({
    email: z
      .string()
      .email("Please enter a valid email address"),
    username: z
      .string()
      .min(1, "Username must be at least 1 character")
      .max(20, "Username must be 20 characters or less"),
    firstName: z
      .string()
      .min(1, "First name must be at least 1 character")
      .max(20, "First name must be 20 characters or less"),
    lastName: z
      .string()
      .min(1, "Last name must be at least 1 character")
      .max(20, "Last name must be 20 characters or less"),
    role: z.enum(["Admin", "Worker"], {
      errorMap: () => ({ message: "Please select a valid role" }),
    }),
});  
export type CreateUserFormSchema = z.infer<typeof formSchema>;
  
type CreateUserFormProps = {
    apiError: string;
    apiFailure: boolean;
    isSubmittingCreateUserForm: boolean;
    submitCreateUserForm: (data: CreateUserFormSchema) => void;
} & React.ComponentPropsWithoutRef<"div">;
  

// This create user form is used by the superadmin and pops up as an admin panel modal
// In the backend a confirmation email is sent to the user as their account is created
export function CreateUserForm({
    apiError,
    apiFailure,
    isSubmittingCreateUserForm,
    submitCreateUserForm,
    ...props
}: CreateUserFormProps) {

    const form = useForm<z.infer<typeof formSchema>>({
      resolver: zodResolver(formSchema),
      defaultValues: {
        email: "",
        username: "",
        firstName: "",
        lastName: "",
        role: "Worker", 
      },
    });
  
    function onSubmit(values: z.infer<typeof formSchema>) {
      submitCreateUserForm(values);
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
        <div className="flex flex-col gap-6" {...props}>
            {/* relative so overlay can fill it */}
            <Card className="relative">
            <CardHeader>
                <CardTitle className="text-2xl">Create User</CardTitle>
                <CardDescription>Create a new user below</CardDescription>
            </CardHeader>
            <CardContent>
                <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
                    <FormField
                    control={form.control}
                    name="email"
                    render={({ field }) => (
                        <FormItem>
                        <FormLabel>Email</FormLabel>
                        <FormControl>
                            <Input placeholder="email@example.com" type="email" {...field} />
                        </FormControl>
                        <FormDescription>Please enter a valid email address</FormDescription>
                        <FormMessage className="text-red-600" />
                        </FormItem>
                    )}
                    />

                    <FormField
                    control={form.control}
                    name="username"
                    render={({ field }) => (
                        <FormItem>
                        <FormLabel>Username</FormLabel>
                        <FormControl>
                            <Input placeholder="username" {...field} />
                        </FormControl>
                        <FormDescription>Up to 20 characters only</FormDescription>
                        <FormMessage className="text-red-600" />
                        </FormItem>
                    )}
                    />

                    <FormField
                    control={form.control}
                    name="firstName"
                    render={({ field }) => (
                        <FormItem>
                        <FormLabel>First Name</FormLabel>
                        <FormControl>
                            <Input placeholder="e.g. John" {...field} />
                        </FormControl>
                        <FormDescription>Up to 20 characters only</FormDescription>
                        <FormMessage className="text-red-600" />
                        </FormItem>
                    )}
                    />

                    <FormField
                    control={form.control}
                    name="lastName"
                    render={({ field }) => (
                        <FormItem>
                        <FormLabel>Last Name</FormLabel>
                        <FormControl>
                            <Input placeholder="e.g. Doe" {...field} />
                        </FormControl>
                        <FormDescription>Up to 20 characters only</FormDescription>
                        <FormMessage className="text-red-600" />
                        </FormItem>
                    )}
                    />

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
                                <SelectItem value="Admin" className="hover:bg-gray-100">
                                    Admin
                                </SelectItem>
                                <SelectItem value="Worker" className="hover:bg-gray-100">
                                    Worker
                                </SelectItem>
                                </SelectContent>
                            </Select>
                            <FormDescription>Select the user's role</FormDescription>
                            <FormMessage className="text-red-600" />
                        </FormItem>
                        )}
                    />

                    <FormButton type="submit">
                        Create Account
                    </FormButton>
                    
                    {apiFailure && (
                        <div>
                            <Alert ref={alertRef} variant="destructive" className="global-jiggle">
                            <AlertCircle className="h-4 w-4" />
                            <AlertTitle>Error</AlertTitle>
                            <AlertDescription>
                                {apiError}
                            </AlertDescription>
                            </Alert>
                        </div>
                    )}
                </form>
                </Form>
            </CardContent>
            
            {/* The "light grey overlay" + spinner - shown conditionally */}
            {isSubmittingCreateUserForm && (
                <div css={styles.overlay}>
                <div css={styles.SpinnerContainer}>
                    <LoadingSpinner size="48px" />
                    <p>Creating user...</p>
                </div>
                </div>
            )}
            </Card>
        </div>
    );
}