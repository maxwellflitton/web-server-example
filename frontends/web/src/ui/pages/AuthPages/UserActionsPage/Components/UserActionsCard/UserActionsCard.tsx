/** @jsxImportSource @emotion/react */
import React from "react";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardDescription
} from "src/ui/components/shadcnComponents/formCardContainer";
import { Separator } from "src/ui/components/shadcnComponents/separator";
import { UserRolesForm } from "../UserRolesForm/UserRolesForm";
import { BlockUserForm } from "../BlockUserForm/BlockUserForm";

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "src/ui/components/shadcnComponents/alert-dialog";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import { AlertCircle } from "lucide-react";
import { FormButton } from "src/ui/components/components/Button/Button";
import { LoadingSpinner } from "src/ui/components/components/LoadingSpinner/LoadingSpinner";
import * as styles from "./UserActionsCard.styles";


interface UserActionsCardProps {
  refetchingUser: boolean;
  currentRoles: string[];
  initialBlockedValue: boolean;
  showResendConfirmation: boolean;
  onBlockToggle: (newBlockedValue: boolean) => void;
  blockUserError: string;
  blockUserFailure: boolean;
  onRolesUpdate: (newRoles: string[]) => void;
  updateRolesError: string;
  updateRolesFailure: boolean;
  onResendConfirmation: () => void;
  resendConfirmationEmailError: string;
  resendConfirmationEmailFailure: boolean;
  resendConfirmationEmailSuccess: boolean;
  onDeleteUser: () => void;
  deleteUserError: string;
  deleteUserFailure: boolean;
}

export function UserActionsCard({
  refetchingUser,
  currentRoles,
  initialBlockedValue,
  showResendConfirmation,
  onBlockToggle,
  blockUserError,
  blockUserFailure,
  onRolesUpdate,
  updateRolesError,
  updateRolesFailure,
  onResendConfirmation,
  resendConfirmationEmailError,
  resendConfirmationEmailFailure,
  resendConfirmationEmailSuccess,
  onDeleteUser,
  deleteUserError,
  deleteUserFailure,
}: UserActionsCardProps) {
    // Create a ref for the resend confirmation email alert component.
    const resendConfirmationEmailAlertRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => {
        if (resendConfirmationEmailFailure && resendConfirmationEmailAlertRef.current) {
        resendConfirmationEmailAlertRef.current.focus();
        resendConfirmationEmailAlertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [resendConfirmationEmailFailure]);

    // Create a ref for the resend confirmation email success component.
    const resendConfirmationEmailSuccessRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => {
        if (resendConfirmationEmailSuccess && resendConfirmationEmailSuccessRef.current) {
        resendConfirmationEmailSuccessRef.current.focus();
        resendConfirmationEmailSuccessRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [resendConfirmationEmailSuccess]);

    // Create a ref for the delete user alert component.
    const deleteUserAlertRef = React.useRef<HTMLDivElement>(null);
    React.useEffect(() => {
        if (deleteUserFailure && deleteUserAlertRef.current) {
        deleteUserAlertRef.current.focus();
        deleteUserAlertRef.current.scrollIntoView({ behavior: "smooth", block: "center" });
        }
    }, [deleteUserFailure]);
    
    
  return (
    <Card className="relative">
      <CardHeader>
        <CardTitle className="text-2xl">User Actions</CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">

        <BlockUserForm
          initialBlockedValue={initialBlockedValue}
          blockUserError={blockUserError}
          blockUserFailure={blockUserFailure}
          onSubmit={onBlockToggle}
        />

        <UserRolesForm 
          currentRoles={currentRoles} 
          updateRolesError={updateRolesError}
          updateRolesFailure={updateRolesFailure}
          onSubmit={onRolesUpdate} 
        />

        {showResendConfirmation && (
          <>
            <Card className="border border-gray-300 shadow-sm">
              <CardHeader>
                <CardTitle>Resend Confirmation Email</CardTitle>
                <CardDescription>
                  Resend the confirmation email to the user so they can confirm their account.
                </CardDescription>
              </CardHeader>
              <CardContent>
                <FormButton onClick={onResendConfirmation}>
                  Resend Email
                </FormButton>

                {resendConfirmationEmailFailure && (
                    <div>
                        <Alert ref={resendConfirmationEmailAlertRef} variant="destructive" className="global-jiggle mt-6">
                        <AlertCircle className="h-4 w-4" />
                        <AlertTitle>Error</AlertTitle>
                        <AlertDescription>
                            {resendConfirmationEmailError}
                        </AlertDescription>
                        </Alert>
                    </div>
                )}

                {resendConfirmationEmailSuccess && (
                    <div>
                        <Alert ref={resendConfirmationEmailSuccessRef} className="global-jiggle text-green-600 border-green-600 w-fit min-w-[25%] mt-6">
                        <AlertCircle className="h-4 w-4 stroke-green-600" />
                        <AlertTitle>Success</AlertTitle>
                        <AlertDescription>
                            Confirmation email sent successfully.
                        </AlertDescription>
                        </Alert>
                    </div>
                )}
              </CardContent>
            </Card>
          </>
        )}

        <Card className="border border-gray-300 shadow-sm">
          <CardHeader>
            <CardTitle>Delete User</CardTitle>
            <CardDescription>
              Permanently delete this user. This action is irreversible.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <AlertDialog>
              <AlertDialogTrigger asChild>
              <FormButton
                css={{
                  background: "red",
                  color: "white",
                  "&:hover": {
                    background: "darkred",
                  },
                }}
              >
                Delete User
              </FormButton>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Are you sure you want to delete this user?</AlertDialogTitle>
                  <AlertDialogDescription>
                    This action cannot be undone. This will permanently delete the user and remove all associated data.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>Cancel</AlertDialogCancel>
                  <AlertDialogAction onClick={onDeleteUser}>Continue</AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
            
            {deleteUserFailure && (
                <div>
                    <Alert ref={deleteUserAlertRef} variant="destructive" className="global-jiggle mt-6">
                    <AlertCircle className="h-4 w-4" />
                    <AlertTitle>Error</AlertTitle>
                    <AlertDescription>
                        {deleteUserError}
                    </AlertDescription>
                    </Alert>
                </div>
            )}

          </CardContent>
        </Card>
      </CardContent>

      {refetchingUser && (
          <div css={styles.overlay}>
          <div css={styles.SpinnerContainer}>
              <LoadingSpinner size="48px" />
              <p>Refetching User...</p>
          </div>
          </div>
      )}
    </Card>
  );
}
