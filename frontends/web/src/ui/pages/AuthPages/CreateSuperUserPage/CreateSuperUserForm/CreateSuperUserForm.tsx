import React from "react";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod"
import { cn } from "src/ui/helpers/utils"

import { AlertCircle } from "lucide-react"
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
    password: z
        .string()
        .min(8, "Password must be at least 8 characters long"),
});
export type CreateSuperUserFormSchema = z.infer<typeof formSchema>;
  
type CreateSuperUserFormProps = {
    apiError: string;
    apiFailure: boolean;
    submitCreateSuperUserForm: (data: CreateSuperUserFormSchema) => void;
} & React.ComponentPropsWithoutRef<"div">;
  
  
export function CreateSuperUserForm({
    apiError,
    apiFailure,
    submitCreateSuperUserForm,
    ...props
}: CreateSuperUserFormProps) {

    const form = useForm<z.infer<typeof formSchema>>({
      resolver: zodResolver(formSchema),
      defaultValues: {
        email: "",
        username: "",
        firstName: "",
        lastName: "",
        password: "",
      },
    });
  
    function onSubmit(values: z.infer<typeof formSchema>) {
      submitCreateSuperUserForm(values);
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
            <Card>
            <CardHeader>
                <CardTitle className="text-2xl">Sign Up</CardTitle>
                <CardDescription>Create a super admin account below</CardDescription>
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
            </Card>
        </div>
    );
}