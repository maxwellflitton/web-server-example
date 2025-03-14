/** @jsxImportSource @emotion/react */
import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { AlertCircle } from "lucide-react";

import { LoadingSpinner } from "src/ui/components/components/LoadingSpinner/LoadingSpinner";
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import { UserDetailsCard } from "./Components/UserDetailsCard/UserDetailsCard";
import { UserActionsCard } from "./Components/UserActionsCard/UserActionsCard";
import * as styles from "./UserActionsPage.styles";

import { getUser } from "#serverApi/auth/users/getUser";
import { blockUser } from "#serverApi/auth/users/blockUser";
import { unblockUser } from "#serverApi/auth/users/unblockUser";
import { resendConfirmationEmail } from "#serverApi/auth/auth/resendConfirmationEmail";
import { deleteUser } from "#serverApi/auth/users/deleteUser";        
import { updateRoles } from "#serverApi/auth/roles/updateRoles";  
import { getJwt } from "#auth/localStorage";
import { UserUrl } from "api-modules/serverApi/auth/users/url";
import { sleep } from "src/ui/helpers/sleep";
import { BlockUserOutputSchema } from "#serverApi/auth/users/blockUser";
import { UnblockUserOutputSchema } from "#serverApi/auth/users/unblockUser";

interface UserData {
  confirmed: boolean;
  username: string;
  email: string;
  firstName: string;
  lastName: string;
  roles: string[]; 
  dateCreated: string;
  lastLoggedIn: string;
  blocked: boolean;
  id: number;      
}

export const UserActionsPage: React.FC = () => {
  const { userId } = useParams();
  const navigate = useNavigate();

  const [userData, setUserData] = useState<UserData | null>(null);
  const [getUserLoading, setGetUserLoading] = useState(true);
  const [getUserError, setGetUserError] = useState("");
  const [getUserFailure, setGetUserFailure] = useState(false);

  const [blockUserError, setBlockUserError] = useState("");
  const [blockUserFailure, setBlockUserFailure] = useState(false);

  const [updateRolesError, setUpdateRolesError] = useState("");
  const [updateRolesFailure, setUpdateRolesFailure] = useState(false);

  const [resendConfirmationEmailError, setResendConfirmationEmailError] = useState("");
  const [resendConfirmationEmailSuccess, setResendConfirmationEmailSuccess] = useState(false);
  const [resendConfirmationEmailFailure, setResendConfirmationEmailFailure] = useState(false);

  const [deleteUserError, setDeleteUserError] = useState("");
  const [deleteUserFailure, setDeleteUserFailure] = useState(false);

  const [refetchingUser, setRefetchingUser] = useState(false);
 
  useEffect(() => {
    // If no userId in URL, show error
    if (!userId) {
      setGetUserError("User ID not provided in URL. Please re-navigate to this page and try again.");
      setGetUserFailure(true);
      setGetUserLoading(false);
      return;
    }

    fetchUser(userId, true);
  }, [userId]);

  const fetchUser = async (id: string, initialLoad: boolean) => {
    if (initialLoad) {
      setGetUserLoading(true);
    };
    setGetUserError("");
    setGetUserFailure(false);

    try {
      const jwt = getJwt();
      if (!jwt) {
        setGetUserError("No JWT found. Please log in and try again.");
        setGetUserFailure(true);
        return;
      }

      const url = new UserUrl().constructGetUserById(Number(id));
      const response = await getUser({ jwt, url });
      if (initialLoad) {
        await sleep(500);
      };

      if (response.status === 200) {
        /**
         * Response body shape:
         * {
         *   user: {
         *     id: number,
         *     confirmed: boolean,
         *     username: string,
         *     email: string,
         *     first_name: string,
         *     last_name: string,
         *     user_role: "Admin" | "Worker" | "Super Admin" | "Unreachable",
         *     date_created: string,
         *     last_logged_in: string,
         *     blocked: boolean,
         *     uuid: string
         *   },
         *   roles: string[] // e.g. ["Worker", "Admin", ...]
         * }
         */
        const fetchedUser = response.body.user;
        const fetchedRoles = response.body.roles || [];

        setUserData({
          id: fetchedUser.id,
          confirmed: fetchedUser.confirmed,
          username: fetchedUser.username,
          email: fetchedUser.email,
          firstName: fetchedUser.first_name,
          lastName: fetchedUser.last_name,
          roles: fetchedRoles,
          dateCreated: fetchedUser.date_created,
          lastLoggedIn: fetchedUser.last_logged_in,
          blocked: fetchedUser.blocked,
        });
      } else if (response.status === 0) {
        setGetUserError(
          "Failed to fetch user due to a network issue. Please check your internet connection and try again."
        );
        setGetUserFailure(true);
      } else {
        setGetUserError("Failed to fetch user. Please try again later.");
        setGetUserFailure(true);
      }
    } catch (error) {
      console.error(error);
      setGetUserError("An unexpected error occurred. Please try again.");
      setGetUserFailure(true);
    } finally {
      if (initialLoad) {
        setGetUserLoading(false);
      }
    }
  };

  const handleBlockToggle = async (newBlockedValue: boolean) => {
    if (!userData) return;
    setBlockUserError("");
    setBlockUserFailure(false);
    setRefetchingUser(true);

    try {
      const jwt = getJwt();
      if (!jwt) {
        setBlockUserError("No JWT found. Please log in and try again.");
        setBlockUserFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setBlockUserFailure(false);
          setBlockUserError("");
        }, 5000);
        return;
      }

      let response: UnblockUserOutputSchema | BlockUserOutputSchema;

      if (newBlockedValue) {
        response = await blockUser({ user_id: userData.id }, jwt);
      } else {
        response = await unblockUser({ user_id: userData.id }, jwt);
      };

      if (response.status == 200) {
        await fetchUser(String(userData.id), false);
        await sleep(300);
        setRefetchingUser(false);

      } else if (response.status == 0) {
        console.log(response.body);
        setBlockUserError("Blocking user failed due to an internet connection issue. Please check your internet and try again.");
        setBlockUserFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setBlockUserFailure(false);
          setBlockUserError("");
        }, 5000);

      } else {
        console.log(response.body);
        setBlockUserError("Blocking user failed. Please try again.");
        setBlockUserFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setBlockUserFailure(false);
          setBlockUserError("");
        }, 5000);
      }

    } catch (error) {
      console.error(error);
      setBlockUserError("An unexpected error occurred. Please try again.");
      setBlockUserFailure(true);
      setRefetchingUser(false);
      setTimeout(() => {
        setBlockUserFailure(false);
        setBlockUserError("");
      }, 5000);
    }
  };

  const handleRolesUpdate = async (newRoles: string[]) => {
    if (!userData) return;
    setUpdateRolesError("");
    setUpdateRolesFailure(false);
    setRefetchingUser(true);

    try {
      const jwt = getJwt();
      if (!jwt) {
        setUpdateRolesError("No JWT found. Please log in and try again.");
        setUpdateRolesFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setUpdateRolesFailure(false);
          setUpdateRolesError("");
        }, 5000);
        return;
      }

      const response = await updateRoles({ user_id: userData.id, roles: newRoles, jwt });

      if (response.status === 200) {
        await fetchUser(String(userData.id), false);
        await sleep(300);
        setRefetchingUser(false);

      } else if (response.status === 0) {
        console.log(response.body);
        setUpdateRolesError(
          "Updating roles failed due to an internet connection issue. Please check your connection and try again."
        );
        setUpdateRolesFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setUpdateRolesFailure(false);
          setUpdateRolesError("");
        }, 5000);

      } else {
        console.log(response.body);
        setUpdateRolesError("Failed to update roles. Please try again.");
        setUpdateRolesFailure(true);
        setRefetchingUser(false);
        setTimeout(() => {
          setUpdateRolesFailure(false);
          setUpdateRolesError("");
        }, 5000);
      }
    } catch (error) {
      console.error(error);
      setUpdateRolesError("An unexpected error occurred. Please try again.");
      setUpdateRolesFailure(true);
      setRefetchingUser(false);
      setTimeout(() => {
        setUpdateRolesFailure(false);
        setUpdateRolesError("");
      }, 5000);
    }
  };

  const handleResendConfirmation = async () => {
    if (!userData) return;
    setResendConfirmationEmailError("");
    setResendConfirmationEmailFailure(false);

    try {
      const jwt = getJwt();
      if (!jwt) {
        setResendConfirmationEmailError("No JWT found. Please log in and try again.");
        setResendConfirmationEmailFailure(true);
        setTimeout(() => {
          setResendConfirmationEmailFailure(false);
          setResendConfirmationEmailError("");
        }, 5000);
        return;
      }

      const response = await resendConfirmationEmail({ email: userData.email }, jwt);

      if (response.status === 200) {
        setResendConfirmationEmailSuccess(true);
        setTimeout(() => {
            setResendConfirmationEmailSuccess(false);
        }, 5000);
        
      } else if (response.status === 0) {
        console.log(response.body);
        setResendConfirmationEmailError(
          "Resending confirmation email failed due to an internet connection issue. Please check your connection and try again."
        );
        setResendConfirmationEmailFailure(true);
        setTimeout(() => {
          setResendConfirmationEmailFailure(false);
          setResendConfirmationEmailError("");
        }, 5000);

      } else {
        console.log(response.body);
        setResendConfirmationEmailError("Failed to resend confirmation email. Please try again.");
        setResendConfirmationEmailFailure(true);
        setTimeout(() => {
          setResendConfirmationEmailFailure(false);
          setResendConfirmationEmailError("");
        }, 5000);
      }
    } catch (error) {
      console.error(error);
      setResendConfirmationEmailError("An unexpected error occurred. Please try again.");
      setResendConfirmationEmailFailure(true);
      setTimeout(() => {
        setResendConfirmationEmailFailure(false);
        setResendConfirmationEmailError("");
      }, 5000);
    }
  };

  const handleDeleteUser = async () => {
    if (!userData) return;
    setDeleteUserError("");
    setDeleteUserFailure(false);

    try {
      const jwt = getJwt();
      if (!jwt) {
        setDeleteUserError("No JWT found. Please log in and try again.");
        setDeleteUserFailure(true);
        setTimeout(() => {
          setDeleteUserFailure(false);
          setDeleteUserError("");
        }, 5000);
        return;
      }

      const response = await deleteUser({ user_id: userData.id }, jwt);

      if (response.status === 201) {
        navigate("/superadmin-panel");

      } else if (response.status === 0) {
        console.log(response.body);
        setDeleteUserError(
          "Deleting user failed due to an internet connection issue. Please check your connection and try again."
        );
        setDeleteUserFailure(true);
        setTimeout(() => {
            setDeleteUserFailure(false);
            setDeleteUserError("");
        }, 5000);

      } else {
        console.log(response.body);
        setDeleteUserError("Failed to delete user. Please try again.");
        setDeleteUserFailure(true);
        setTimeout(() => {
          setDeleteUserFailure(false);
          setDeleteUserError("");
        }, 5000);
      }
    } catch (error) {
      console.error(error);
      setDeleteUserError("An unexpected error occurred. Please try again.");
      setDeleteUserFailure(true);
      setTimeout(() => {
        setDeleteUserFailure(false);
        setDeleteUserError("");
      }, 5000);
    }
  };


  if (getUserLoading) {
    return (
      <div css={styles.LoadingPageContainer}>
        <div css={styles.SpinnerContainer}>
          <LoadingSpinner size="48px" />
          <p>Fetching User...</p>
        </div>
      </div>
    );
  }

  // If there's an error retrieving the user, show a destructive alert
  if (getUserFailure) {
    return (
      <div css={styles.PageContainer}>
          <div css={styles.ContentErrorWrapper}>
            <Alert variant="destructive" className="w-fit min-w-[25%]">
              <AlertCircle className="h-4 w-4" />
              <AlertTitle>Error</AlertTitle>
              <AlertDescription>{getUserError}</AlertDescription>
            </Alert>
          </div>
      </div>
    );
  }

  // If userData is still null after fetch, also show a friendly message
  if (!userData) {
    return (
      <div css={styles.PageContainer}>
        <div css={styles.ContentErrorWrapper}>
          <Alert variant="destructive" className="w-fit min-w-[25%]">
            <AlertCircle className="h-4 w-4" />
            <AlertTitle>Error</AlertTitle>
            <AlertDescription>No user data found. Please re-navigate to this page and try again.</AlertDescription>
          </Alert>
        </div>
      </div>
    );
  }

  return (
    <div css={styles.PageContainer}>
      <div css={styles.ContentWrapper}>
        <UserDetailsCard user={userData} />
        <UserActionsCard
          refetchingUser={refetchingUser}
          currentRoles={userData.roles}
          initialBlockedValue={userData.blocked}
          showResendConfirmation={!userData.confirmed}
          onBlockToggle={handleBlockToggle}
          blockUserError={blockUserError}
          blockUserFailure={blockUserFailure}
          onRolesUpdate={handleRolesUpdate}
          updateRolesError={updateRolesError}
          updateRolesFailure={updateRolesFailure}
          onResendConfirmation={handleResendConfirmation}
          resendConfirmationEmailError={resendConfirmationEmailError}
          resendConfirmationEmailSuccess={resendConfirmationEmailSuccess}
          resendConfirmationEmailFailure={resendConfirmationEmailFailure}
          onDeleteUser={handleDeleteUser}
          deleteUserError={deleteUserError}
          deleteUserFailure={deleteUserFailure}
       />
      </div>
    </div>
  );
};
