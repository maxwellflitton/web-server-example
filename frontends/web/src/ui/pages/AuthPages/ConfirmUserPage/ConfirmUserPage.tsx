/** @jsxImportSource @emotion/react */
import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { confirmUser } from "#serverApi/auth/users/confirmUser";
import { getUser } from "#serverApi/auth/users/getUser";
import { resetPassword } from "#serverApi/auth/users/resetPassword";
import { TrimmedUserSchema } from "api-modules/serverApi/auth/users/getUser";
import { ConfirmUserFormSchema } from "./ConfirmUserForm/ConfirmUserForm";
import { ConfirmUserForm } from "./ConfirmUserForm/ConfirmUserForm";
import * as styles from "./ConfirmUserPage.styles";
import { LoadingSpinner } from "src/ui/components/components/LoadingSpinner/LoadingSpinner";
import { sleep } from "src/ui/helpers/sleep";
import { getJwt } from "#auth/localStorage";
import { UserUrl } from "api-modules/serverApi/auth/users/url";


export const ConfirmUserPage: React.FC = () => {
    const navigate = useNavigate();
    const { uuid } = useParams();
  
    // States for the confirm user form
    const [confirmUserError, setConfirmUserError] = useState("");
    const [confirmUserFailure, setConfirmUserFailure] = useState(false);
    const [disabledForm, setDisabledForm] = useState(false);
  
    // States for fetching the user details via getUser
    const [getUserLoading, setGetUserLoading] = useState(false);
    const [getUserError, setGetUserError] = useState("");
    const [getUserFailure, setGetUserFailure] = useState(false);
    const [userData, setUserData] = useState<TrimmedUserSchema | null>(null);
  
    useEffect(() => {
      if (!uuid) {
        setDisabledForm(true);
        setGetUserError( "Cannot get uuid from url. Please reopen this page by clicking the email link and try again.");
        setGetUserFailure(true);
        setGetUserLoading(false);
      } else {
        setDisabledForm(false);
  
        // Fetch user details using the uuid from the url params.
        const fetchUser = async () => {
          setGetUserLoading(true);
          setGetUserError("");
          setGetUserFailure(false);

          try {
            const jwt = getJwt();
            if (!jwt) {
              setGetUserError("No JWT found. Please log in and try again.");
              setGetUserFailure(true);
              setDisabledForm(true);
              setGetUserLoading(false);
              return;
            }
      
            const url = new UserUrl().constructGetUserByUuid(uuid);
            const response = await getUser({ jwt, url });
            await sleep(500);

            if (response.status === 200 && response.body) {
              setUserData(response.body.user);
              setGetUserLoading(false);
            } else {
              console.error(response.body);
              setGetUserError("Unable to fetch user details. Please try again.");
              setGetUserFailure(true);
              setDisabledForm(true);
              setGetUserLoading(false);
            }
          } catch (err) {
            console.error(err);
            setGetUserError("An unexpected error occurred when fetching user details. Please try again.");
            setGetUserFailure(true);
            setDisabledForm(true);
            setGetUserLoading(false);
          }
        };
        fetchUser();
      }
    }, [uuid]);
  
    const submitConfirmUserForm = async (data: ConfirmUserFormSchema) => {
      setConfirmUserError("");
      setConfirmUserFailure(false);
  
      try {
        if (!uuid) return;
  
        // First, confirm the user
        const confirmResponse = await confirmUser({ unique_id: uuid });
        if (confirmResponse.status === 0) {
          console.log(confirmResponse.body);
          setConfirmUserError("Account confirmation failed due to an internet connection issue. Please check your internet and try again.");
          setConfirmUserFailure(true);
          return;
        }
        if (confirmResponse.status !== 201) {
          console.log(confirmResponse.body);
          setConfirmUserError("Account confirmation failed. Please try again or contact support.");
          setConfirmUserFailure(true);
          return;
        }
  
        // Next, reset the password
        const resetResponse = await resetPassword({ password: data.password });
        if (resetResponse.status === 0) {
          console.log(resetResponse.body);
          setConfirmUserError("Password reset failed due to an internet connection issue. Please check your internet and try again.");
          setConfirmUserFailure(true);
          return;
        }
        if (resetResponse.status === 201) {
          navigate("/login");
        } else {
          console.log(resetResponse.body);
          setConfirmUserError("Password reset failed. Please try again or contact support.");
          setConfirmUserFailure(true);
        }
      } catch (err) {
        console.log(err);
        setConfirmUserError("An unexpected error occurred. Please try again.");
        setConfirmUserFailure(true);
      };
    };
  
  
    // While waiting for getUser, show a Loading with some text.
    if (getUserLoading) {
      return (
        <div css={styles.LoadingPageContainer}>
          <div css={styles.SpinnerContainer}>
            <LoadingSpinner size="48px" />
            <p>Fetching User...</p>
          </div>
        </div>
      );
    };
  
    return (
        <div css={styles.PageContainer}>
            <div css={styles.FormWrapper}>
            <ConfirmUserForm
                firstName={userData?.first_name}
                lastName={userData?.last_name}
                username={userData?.username}
                getUserError={getUserError}
                getUserFailure={getUserFailure}
                confirmUserError={confirmUserError}
                confirmUserFailure={confirmUserFailure}
                disabled={disabledForm}
                submitConfirmUserForm={submitConfirmUserForm}
            />
            </div>
        </div>
    );
  };
  