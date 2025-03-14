/** @jsxImportSource @emotion/react */
import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { LoadingSpinner } from "src/ui/components/components/LoadingSpinner/LoadingSpinner";
import { FormButton } from "src/ui/components/components/Button/Button";
import { AlertCircle } from "lucide-react"
import { Alert, AlertDescription, AlertTitle } from "src/ui/components/shadcnComponents/alert";
import { UserTable } from "./Components/UserTable/UserTable";
import { columns } from "./Components/UserTable/Columns";
import { getAllUsers } from "#serverApi/auth/users/getAllUsers";
import { getJwt } from "#auth/localStorage";
import { createUser } from "#serverApi/auth/users/createUser";
import type { User } from "./Components/UserTable/Columns";
import * as styles from "./SuperAdminPanelPage.styles";
import { GlobalModal } from "src/ui/components/components/GlobalModal/GlobalModal";
import { CreateUserForm, CreateUserFormSchema } from "./Components/CreateUserForm/CreateUserForm";
import { sleep } from "src/ui/helpers/sleep";


export const SuperAdminPanelPage: React.FC = () => {
    const navigate = useNavigate();
    const [users, setUsers] = useState<User[]>([]);
    const [loading, setLoading] = useState(true);
    const [modalIsOpen, setModalIsOpen] = useState(false);

    const [getUsersError, setGetUsersError] = useState("");
    const [getUsersFailure, setGetUsersFailure] = useState(false);

    const [createUserError, setCreateUserError] = useState("");
    const [createUserFailure, setCreateUserFailure] = useState(false);
    const [createUserSuccess, setCreateUserSuccess] = useState(false);
    const [isSubmittingCreateUserForm, setIsSubmittingCreateUserForm] = useState(false);

    const fetchUsers = async (initialLoad: boolean) => {
        setGetUsersError("");
        setGetUsersFailure(false);
        if (initialLoad) {
            setLoading(true);
        };

        try {
          const jwt = getJwt();
          if (!jwt) {
            setGetUsersError("No JWT found. Please log in and try again.");
            setGetUsersFailure(true);
            if (initialLoad) {
                setLoading(false);
            }            
            return;
          }
      
          const response = await getAllUsers(jwt); 
      
          if (response.status === 200) {
            const transformedUsers = response.body.map((userProfile) => {
              const { user, role_permissions } = userProfile;
              const createdDateObj = new Date(user.date_created);
              const lastLoggedDateObj = new Date(user.last_logged_in);
              const dateCreatedFormatted = createdDateObj.toLocaleDateString('en-GB');
              const lastLoggedInFormatted = lastLoggedDateObj.toLocaleDateString('en-GB');
              const roles = role_permissions.map(rp => rp.role); 
      
              return {
                id: user.id.toString(),
                username: user.username,
                email: user.email,
                firstName: user.first_name,
                lastName: user.last_name,
                roles, 
                dateCreated: dateCreatedFormatted,
                lastLoggedIn: lastLoggedInFormatted,
                blocked: user.blocked,
                confirmed: user.confirmed,
              };
            });
            
            if (initialLoad) {
                await sleep(500);
            };
            setUsers(transformedUsers);
          } else if (response.status === 0) {
            console.error(response.body);
            setGetUsersError("Failed to fetch users due to an internet connection issue. Please check your internet and try again.");
            setGetUsersFailure(true);
          } else {
            console.error(response.body);
            setGetUsersError("Failed to fetch users. Please try again.");
            setGetUsersFailure(true);
          }
        } catch (err) {
          console.error(err);
          setGetUsersError("An unexpected error occurred. Please try again.");
          setGetUsersFailure(true);
        } finally {
            if (initialLoad) {
                setLoading(false);
            }  
        }
    };

    useEffect(() => {
        fetchUsers(true);
    }, []);

    const submitCreateUserForm = async (data: CreateUserFormSchema) => {
        setCreateUserError("");
        setCreateUserFailure(false);
        setIsSubmittingCreateUserForm(true);
        
        try {
            const jwt = getJwt();
            if (!jwt) {
                setCreateUserError("No JWT found. Please log in and try again.");
                setCreateUserFailure(true);
                setIsSubmittingCreateUserForm(false);
                return;
            }

            const requestPayload: any = {
                email: data.email,
                username: data.username,
                first_name: data.firstName,
                last_name: data.lastName,
                user_role: data.role
            };
    
            const response = await createUser(requestPayload, jwt);
        
            if (response.status === 201) {
                await fetchUsers(false);
                await sleep(300);
                setCreateUserSuccess(true);
                setTimeout(() => {
                    setCreateUserSuccess(false);
                }, 5000);
                setIsSubmittingCreateUserForm(false);
                setModalIsOpen(false);
            } else if (response.status === 0) {
                console.log(response.body);
                setCreateUserError(`Account creation failed due to an internet connection issue. Please check your internet and try again.`);
                setCreateUserFailure(true);
                setIsSubmittingCreateUserForm(false);
            } else {
                console.log(response.body);
                setCreateUserError(`Account creation failed. Please try again.`);
                setCreateUserFailure(true);
                setIsSubmittingCreateUserForm(false);
            }
        } catch (err) {
            console.error(err);
            setCreateUserError("An unexpected error occurred. Please try again.");
            setCreateUserFailure(true);
            setIsSubmittingCreateUserForm(false);
        };
    };

    if (loading) {
        return (
            <div css={styles.LoadingPageContainer}>
                <div css={styles.SpinnerContainer}>
                    <LoadingSpinner size="48px" />
                    <p>Fetching All Users...</p>
                </div>
            </div>
        );
    }

    return (
        <div css={styles.Container}>
            <div css={styles.TableWrapper}>
                <div css={styles.HeaderSection}>
                    <h1>User Management</h1>
                    <FormButton onClick={() => setModalIsOpen(true)}>
                        Create User
                    </FormButton>
                </div>

                {getUsersFailure && (
                    <div>
                        <Alert variant="destructive" className="global-jiggle w-fit min-w-[25%]">
                        <AlertCircle className="h-4 w-4" />
                        <AlertTitle>Error</AlertTitle>
                        <AlertDescription>
                            {getUsersError}
                        </AlertDescription>
                        </Alert>
                    </div>
                )}

                {createUserSuccess && !getUsersFailure && (
                    <div>
                        <Alert className="global-jiggle text-green-600 border-green-600 w-fit min-w-[25%]">
                        <AlertCircle className="h-4 w-4 stroke-green-600" />
                        <AlertTitle>Success</AlertTitle>
                        <AlertDescription>
                            User successfully created.
                        </AlertDescription>
                        </Alert>
                    </div>
                )}

                <UserTable 
                    columns={columns} 
                    data={users}
                />
            </div>

            <GlobalModal 
                isOpen={modalIsOpen} 
                onRequestClose={() => setModalIsOpen(false)}
                contentCss={styles.CreateUserFormWrapper}
                disableCloseModal={isSubmittingCreateUserForm}
            >
                <CreateUserForm
                    apiError={createUserError}
                    apiFailure={createUserFailure}
                    submitCreateUserForm={submitCreateUserForm}
                    isSubmittingCreateUserForm={isSubmittingCreateUserForm}
                />
            </GlobalModal>
        </div>
    );
};