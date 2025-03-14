import React from "react";
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
import {
  Form,
  FormField,
  FormItem,
  FormControl,
  FormLabel,
  FormDescription,
  FormMessage,
} from "src/ui/components/shadcnComponents/form";
import { Switch } from "src/ui/components/shadcnComponents/switch";
import { FormButton } from "src/ui/components/components/Button/Button";
import { AlertCircle } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";


// Define our form schema
const BlockUserSchema = z.object({
  blocked: z.boolean().default(false),
});

type BlockUserFormValues = z.infer<typeof BlockUserSchema>;

interface BlockUserFormProps {
  initialBlockedValue: boolean;
  blockUserError: string;
  blockUserFailure: boolean;
  onSubmit: (newBlockedValue: boolean) => void;
}

export function BlockUserForm({
  initialBlockedValue,
  blockUserError,
  blockUserFailure,
  onSubmit,
}: BlockUserFormProps) {

  const form = useForm<BlockUserFormValues>({
    resolver: zodResolver(BlockUserSchema),
    defaultValues: {
      blocked: initialBlockedValue,
    },
  });

  function handleSubmit(data: BlockUserFormValues) {
    onSubmit(data.blocked);
  }

  // Create a ref for the Alert component.
  const alertRef = React.useRef<HTMLDivElement>(null);

  // When an API failure occurs, focus and scroll to the alert.
  React.useEffect(() => {
      if (blockUserFailure && alertRef.current) {
      alertRef.current.focus();
      alertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
      }
  }, [blockUserFailure]);

  return (
    <Card
      className="w-full max-w-3xl border border-gray-300 shadow-sm"
    >
      <CardHeader>
        <CardTitle>Block / Unblock User</CardTitle>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(handleSubmit)}
            className="space-y-6"
          >
            <FormField
              control={form.control}
              name="blocked"
              render={({ field }) => (
                <FormItem className="flex flex-row items-center justify-between rounded-lg border-1 border-gray-200 gap-3 p-4 shadow-sm">
                  <div>
                    <FormLabel>Blocked</FormLabel>
                    <FormDescription>
                      Toggle on to block this user and off to unblock them.
                    </FormDescription>
                  </div>
                  <FormControl>
                    <Switch
                      checked={field.value}
                      onCheckedChange={field.onChange}
                      className="
                        data-[state=unchecked]:bg-gray-300
                        data-[state=checked]:bg-red-500
                        [&>span.bg-background]:bg-white
                      "
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormButton type="submit">
              Submit Change
            </FormButton>

            {blockUserFailure && (
                <div>
                    <Alert ref={alertRef} variant="destructive" className="global-jiggle">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>
                        {blockUserError}
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
}
